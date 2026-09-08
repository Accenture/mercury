import type { MinigraphGraphData } from '../utils/graphTypes';
import {
  buildCreateConnectionCommand,
  buildDeleteConnectionCommand,
} from './minigraphCommandBuilder';

/** One requested removal: a named relation, or the whole directed edge. */
export interface ConnectionRemovalRequest {
  source: string;
  target: string;
  /** undefined = every relation of the directed edge source→target. */
  relation?: string;
}

/** A directed relation the plan removes (the unit of the undo recipe). */
export interface RemovedRelation {
  source: string;
  target: string;
  relation: string;
}

export interface ConnectionRemovalPlan {
  /** Console commands realizing the removal, in send order. */
  commands: string[];
  /** Exactly what disappears — re-connecting these is the inverse. */
  removed: RemovedRelation[];
}

/** Relation types of the DIRECTED connection source→target, in model order. */
export function directedRelationTypes(
  graphData: MinigraphGraphData,
  source: string,
  target: string,
): string[] {
  const types: string[] = [];
  for (const connection of graphData.connections ?? []) {
    if (connection.source !== source || connection.target !== target) continue;
    for (const relation of connection.relations) {
      types.push(relation.type);
    }
  }
  return types;
}

/**
 * Plan the removal of directed relations against the CURRENT graph snapshot.
 * The backend's only removal command — `delete connection {a} and {b}` — wipes
 * the pair in BOTH directions, so each touched pair becomes a compound: one
 * delete, then reconnect commands for every surviving relation (kept forward
 * relations and the untouched reverse direction alike).  Requests that no
 * longer match the snapshot (a stale menu) are skipped; returns null when
 * nothing would be removed or a command cannot be built.
 */
export function planConnectionRemoval(
  graphData: MinigraphGraphData,
  requests: ConnectionRemovalRequest[],
): ConnectionRemovalPlan | null {
  try {
    // Working copy of each directed edge's relations, mutated per request so a
    // multi-edge gesture (e.g. both directions selected) composes correctly.
    const keep = new Map<string, string[]>();
    const keepOf = (source: string, target: string): string[] => {
      const key = `${source}\t${target}`;
      let list = keep.get(key);
      if (!list) {
        list = directedRelationTypes(graphData, source, target);
        keep.set(key, list);
      }
      return list;
    };

    const removed: RemovedRelation[] = [];
    const touchedPairs: Array<{ a: string; b: string }> = [];
    const touchedPairKeys = new Set<string>();

    for (const { source, target, relation } of requests) {
      const list = keepOf(source, target);
      if (relation === undefined) {
        if (list.length === 0) continue;
        for (const type of list) removed.push({ source, target, relation: type });
        list.length = 0;
      } else {
        const index = list.indexOf(relation);
        if (index === -1) continue;
        list.splice(index, 1);
        removed.push({ source, target, relation });
      }
      const pairKey = [source, target].sort().join('\t');
      if (!touchedPairKeys.has(pairKey)) {
        touchedPairKeys.add(pairKey);
        touchedPairs.push({ a: source, b: target });
      }
    }

    if (removed.length === 0) return null;

    const commands: string[] = [];
    for (const { a, b } of touchedPairs) {
      commands.push(buildDeleteConnectionCommand(a, b));
      for (const type of keepOf(a, b)) {
        commands.push(buildCreateConnectionCommand({ sourceAlias: a, targetAlias: b, relation: type }));
      }
      for (const type of keepOf(b, a)) {
        commands.push(buildCreateConnectionCommand({ sourceAlias: b, targetAlias: a, relation: type }));
      }
    }
    return { commands, removed };
  } catch {
    return null;
  }
}

/**
 * Human wording for a removal, tense-neutral: "relation 'fetch' (a → b)",
 * "connection a → b", or "3 relations".  Callers prefix "Deleted " / "delete ".
 */
export function describeRemovedRelations(removed: RemovedRelation[]): string {
  const first = removed[0];
  if (removed.length === 1) {
    return `relation '${first.relation}' (${first.source} → ${first.target})`;
  }
  const samePair = removed.every(
    (entry) => entry.source === first.source && entry.target === first.target,
  );
  if (samePair) {
    return `connection ${first.source} → ${first.target}`;
  }
  return `${removed.length} relations`;
}
