import type { MinigraphGraphData, MinigraphNode } from '../utils/graphTypes';
import {
  buildCreateConnectionCommand,
  buildCreateNodeCommand,
  buildDeleteConnectionCommand,
  buildDeleteNodeCommand,
  buildUpdateNodeCommand,
} from './minigraphCommandBuilder';
import { describeRemovedRelations, type RemovedRelation } from './connectionEdits';
import { createEditNodeFormState } from './propertyRows';

/**
 * Compensating-command undo (frontend-only, by design): the backend stays a
 * lightweight command executor with no undo journal, so every UI-initiated
 * mutation captures enough of the pre-mutation graph snapshot to compute the
 * inverse console commands.  All command text still flows through the
 * minigraphCommandBuilder serialization boundary; a snapshot that cannot be
 * expressed safely yields null — the action simply is not undoable.
 *
 * Known limit: the `connect` command carries the relation TYPE only, so
 * relation properties (rare; empty in the bundled tutorials) are not restored.
 */
export interface UndoEntry {
  label: string;
  inverseCommands: string[];
}

/** Connect commands recreating every relation between the pair, BOTH directions. */
function pairConnectCommands(
  graphData: MinigraphGraphData,
  nodeA: string,
  nodeB: string,
): string[] {
  const commands: string[] = [];
  for (const connection of graphData.connections ?? []) {
    const forward = connection.source === nodeA && connection.target === nodeB;
    const backward = connection.source === nodeB && connection.target === nodeA;
    if (!forward && !backward) continue;
    for (const relation of connection.relations) {
      commands.push(buildCreateConnectionCommand({
        sourceAlias: connection.source,
        targetAlias: connection.target,
        relation: relation.type,
      }));
    }
  }
  return commands;
}

/**
 * Inverse of a connection-removal plan: reconnect exactly the removed directed
 * relations.  (The plan's compound already preserved every survivor — kept
 * forward relations and the reverse direction — so the undo re-adds only what
 * actually disappeared.)
 */
export function buildConnectionRemovalUndo(removed: RemovedRelation[]): UndoEntry | null {
  if (removed.length === 0) return null;
  try {
    return {
      label: `delete ${describeRemovedRelations(removed)}`,
      inverseCommands: removed.map(({ source, target, relation }) =>
        buildCreateConnectionCommand({ sourceAlias: source, targetAlias: target, relation })),
    };
  } catch {
    return null;
  }
}

/** Inverse of an edit: `update node` is full-replace, so re-submitting the pre-edit snapshot is exact. */
export function buildNodeEditUndo(previousNode: MinigraphNode): UndoEntry | null {
  const conversion = createEditNodeFormState(previousNode);
  if (!conversion.valid || conversion.formState === null) return null;
  try {
    return {
      label: `edit node ${previousNode.alias}`,
      inverseCommands: [buildUpdateNodeCommand(conversion.formState, previousNode.alias)],
    };
  } catch {
    return null;
  }
}

/** Inverse of creating a node. */
export function buildNodeCreateUndo(alias: string): UndoEntry | null {
  try {
    return {
      label: `create node ${alias}`,
      inverseCommands: [buildDeleteNodeCommand(alias)],
    };
  } catch {
    return null;
  }
}

/**
 * Inverse of creating a connection: `delete connection` wipes ALL relations
 * between the pair, so the inverse re-adds whatever existed before the create.
 * Pass the PRE-create graph snapshot.
 */
export function buildConnectionCreateUndo(
  graphDataBefore: MinigraphGraphData,
  sourceAlias: string,
  targetAlias: string,
): UndoEntry | null {
  try {
    const restore = pairConnectCommands(graphDataBefore, sourceAlias, targetAlias);
    return {
      label: `create connection ${sourceAlias} → ${targetAlias}`,
      inverseCommands: [
        buildDeleteConnectionCommand(sourceAlias, targetAlias),
        ...restore,
      ],
    };
  } catch {
    return null;
  }
}

/**
 * Inverse of deleting a node: recreate it from the snapshot, then reconnect
 * every relation that touched it (node deletion cascades its connections).
 */
export function buildNodeDeleteUndo(
  graphDataBefore: MinigraphGraphData,
  node: MinigraphNode,
): UndoEntry | null {
  const conversion = createEditNodeFormState(node);
  if (!conversion.valid || conversion.formState === null) return null;
  try {
    const inverseCommands = [buildCreateNodeCommand(conversion.formState)];
    for (const connection of graphDataBefore.connections ?? []) {
      if (connection.source !== node.alias && connection.target !== node.alias) continue;
      for (const relation of connection.relations) {
        inverseCommands.push(buildCreateConnectionCommand({
          sourceAlias: connection.source,
          targetAlias: connection.target,
          relation: relation.type,
        }));
      }
    }
    return { label: `delete node ${node.alias}`, inverseCommands };
  } catch {
    return null;
  }
}
