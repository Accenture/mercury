import { useState } from 'react';
import { Handle, Position, NodeResizer, useConnection, type NodeProps, type Node } from '@xyflow/react';
import type { GraphNodeData } from '../../utils/graphTransformer';
import { MinigraphNodeBody } from './MinigraphNodeBody';
import styles from './NodeTypes.module.css';

/** ReactFlow Node type for minigraph nodes. */
export type MinigraphRFNode = Node<GraphNodeData>;
export const AUTHORING_SOURCE_HANDLE_ID = 'authoring-source';
export const AUTHORING_TARGET_HANDLE_ID = 'authoring-target';

// ─── Resizable node ───────────────────────────────────────────────────────────
//
// Following the official React Flow "node-resizer" example pattern:
//   • The component returns a Fragment — no wrapper <div>.
//   • NodeResizer, Handles and content are all siblings at the top level.
//   • The React Flow wrapper element IS the styled shell; its look is driven
//     by `node.style` set in graphTransformer.ts, not by a CSS class here.
//   • This eliminates every wrapper-sizing workaround that was previously
//     needed (initialWidth/initialHeight tricks, CSS overrides for
//     .react-flow__node-default, overflow:visible hacks, etc.).
function MinigraphNode({ id, data, isConnectable, selected }: NodeProps<MinigraphRFNode>) {
  const [isResizing, setIsResizing] = useState(false);
  const showConnectionAuthoring = data.supportsConnectionAuthoring && !isResizing;
  // Neo4j-style connect gesture: the node BODY moves the node, the halo RING
  // around it starts a connection.  While a connection drag is in progress,
  // every other node becomes a full-body drop target.
  const connection = useConnection();
  const isConnectTarget = showConnectionAuthoring &&
    connection.inProgress &&
    connection.fromNode?.id !== id;

  return (
    <>
      {/* Resize handles — visible only when the node is selected */}
      <NodeResizer
        minWidth={180}
        minHeight={data.minHeight}
        isVisible={selected}
        onResizeStart={() => setIsResizing(true)}
        onResizeEnd={() => setIsResizing(false)}
      />

      {/* Target handles (left) — multiple hidden anchors let edges land a few pixels apart. */}
      {data.targetHandles.map(({ id, offset }) => (
        <Handle
          key={id}
          id={id}
          type="target"
          position={Position.Left}
          isConnectable={isConnectable}
          className={styles.edgeHandle}
          style={{ top: `calc(50% + ${offset}px)` }}
        />
      ))}

      {/* Back-edge source handles (left) — outgoing back-edges exit from the left side. */}
      {data.backSourceHandles.map(({ id, offset }) => (
        <Handle
          key={id}
          id={id}
          type="source"
          position={Position.Left}
          isConnectable={isConnectable}
          className={styles.edgeHandle}
          style={{ top: `calc(50% + ${offset}px)` }}
        />
      ))}

      {/*
        * Content container — fills the React Flow wrapper (which carries the
        * border/background via node.style) and clips its own overflow.
        * width/height:100% make it track the wrapper when the user resizes.
        */}
      <MinigraphNodeBody
        alias={data.alias}
        nodeType={data.nodeType}
        properties={data.properties}
        compact={data.compact}
      />

      {showConnectionAuthoring && (
        <>
          {/*
            * Connection source: a halo ring surrounding the node (Neo4j-style).
            * The clip-path cuts out the node body, so events over the body
            * still move the node while the whole perimeter starts a
            * connection.  The ::before element draws the visible halo on
            * hover/selection.
            */}
          <Handle
            id={AUTHORING_SOURCE_HANDLE_ID}
            type="source"
            position={Position.Right}
            isConnectable={isConnectable}
            isConnectableEnd={false}
            className={styles.authoringRing}
          />
          {/*
            * Connection target: covers the whole node, but stays inert until a
            * connection drag is in progress from another node — then it lights
            * up and accepts a drop anywhere on the node.
            */}
          <Handle
            id={AUTHORING_TARGET_HANDLE_ID}
            type="target"
            position={Position.Left}
            isConnectable={isConnectable}
            isConnectableStart={false}
            className={
              isConnectTarget
                ? `${styles.authoringTargetOverlay} ${styles.authoringTargetOverlayActive}`
                : styles.authoringTargetOverlay
            }
          />
        </>
      )}

      {/* Source handles (right) — paired with target handles for best-effort edge spreading. */}
      {data.sourceHandles.map(({ id, offset }) => (
        <Handle
          key={id}
          id={id}
          type="source"
          position={Position.Right}
          isConnectable={isConnectable}
          className={styles.edgeHandle}
          style={{ top: `calc(50% + ${offset}px)` }}
        />
      ))}

      {/* Back-edge target handles (right) — incoming back-edges enter from the right side. */}
      {data.backTargetHandles.map(({ id, offset }) => (
        <Handle
          key={id}
          id={id}
          type="target"
          position={Position.Right}
          isConnectable={isConnectable}
          className={styles.edgeHandle}
          style={{ top: `calc(50% + ${offset}px)` }}
        />
      ))}
    </>
  );
}

// ─── Exported nodeTypes map for <ReactFlow nodeTypes={...}> ──────────────────
//
// ReactFlow matches the `type` field on each node object to this map.
// Our transformer sets node.type = n.types[0], so we need an entry per known
// type plus a "default" fallback for anything unrecognised.
export const nodeTypes = {
  Root:        MinigraphNode,
  End:         MinigraphNode,
  Fetcher:     MinigraphNode,
  mapper:      MinigraphNode,
  Math:        MinigraphNode,
  JavaScript:  MinigraphNode,
  Provider:    MinigraphNode,
  Dictionary:  MinigraphNode,
  Join:        MinigraphNode,
  Extension:   MinigraphNode,
  Island:      MinigraphNode,
  Decision:    MinigraphNode,
  default:     MinigraphNode,
} as const;
