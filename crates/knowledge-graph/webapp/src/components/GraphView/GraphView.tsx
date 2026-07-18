import { useCallback, useEffect, useMemo, useRef, useState } from 'react';
import {
  ReactFlow,
  Background,
  Controls,
  MiniMap,
  useNodesState,
  useEdgesState,
  BackgroundVariant,
  type Edge,
  type Node,
} from '@xyflow/react';
import '@xyflow/react/dist/style.css';

import { nodeTypes } from './NodeTypes';
import { GraphViewErrorBoundary } from './GraphViewErrorBoundary';
import { transformGraphData, type GraphNodeData, type GraphEdgeData } from '../../utils/graphTransformer';
import type { MinigraphGraphData, MinigraphNode, MinigraphConnection } from '../../utils/graphTypes';
import { hasClipboardItemType, readClipboardItemId } from '../../clipboard/dnd';
import { findNodeByAlias, extractDirectConnections } from '../../clipboard/helpers';
import GraphToolbar from '../GraphToolbar/GraphToolbar';
import GraphContextMenu from './GraphContextMenu';
import NodeContextMenu from './NodeContextMenu';
import styles from './GraphView.module.css';

interface GraphViewProps {
  graphData:       MinigraphGraphData | null;
  /** Resolved display name for the graph (shown in the toolbar). */
  graphName?:      string;
  /** Called after the raw graph JSON is successfully copied to the clipboard. */
  onCopySuccess?:  () => void;
  /** Called when the clipboard write fails. */
  onCopyError?:    () => void;
  onRenderError?:  (message: string) => void;
  /** When true, renders a semi-transparent overlay with a spinner to indicate a background re-fetch. */
  isRefreshing?:   boolean;
  /** Callback for "Clip to Workspace" from the node context menu. */
  onClipNode?:     (node: MinigraphNode, connections: MinigraphConnection[]) => void;
  onClipboardDrop?: (itemId: string) => void;
  isConnected:     boolean;
  supportsAuthoring?: boolean;
  onCreateNode?:   (source: 'empty-graph' | 'pane-context-menu') => void;
  onEditNode?:     (node: MinigraphNode) => void;
  onDeleteNode?:   (node: MinigraphNode) => void;
}

const EMPTY_NODES: Node<GraphNodeData>[]  = [];
const EMPTY_EDGES: Edge<GraphEdgeData>[]  = [];

export default function GraphView({
  graphData,
  graphName,
  onCopySuccess,
  onCopyError,
  onRenderError,
  isRefreshing = false,
  onClipNode,
  onClipboardDrop,
  isConnected,
  supportsAuthoring = false,
  onCreateNode,
  onEditNode,
  onDeleteNode,
}: GraphViewProps) {

  // ── Context menu state ──────────────────────────────────────────────────
  const [contextMenu, setContextMenu] = useState<{
    x: number;
    y: number;
    nodeAlias: string;
  } | null>(null);
  const [paneMenu, setPaneMenu] = useState<{ x: number; y: number } | null>(null);
  const [clipboardDragActive, setClipboardDragActive] = useState(false);
  const clipboardDragDepthRef = useRef(0);
  const canCreateNode = Boolean(supportsAuthoring && onCreateNode && isConnected);
  const canClipNode = Boolean(onClipNode);
  const canEditNode = Boolean(supportsAuthoring && onEditNode && isConnected);
  const canDeleteNode = Boolean(supportsAuthoring && onDeleteNode && isConnected);
  const canOpenNodeContextMenu = canClipNode || canEditNode || canDeleteNode;
  const canAcceptClipboardDrop = Boolean(onClipboardDrop && isConnected);

  const resetClipboardDragState = useCallback(() => {
    clipboardDragDepthRef.current = 0;
    setClipboardDragActive(false);
  }, []);

  useEffect(() => {
    if (!paneMenu) return;

    const handleKeyDown = (event: KeyboardEvent) => {
      if (event.key === 'Escape') setPaneMenu(null);
    };
    const handleScrollOrResize = () => setPaneMenu(null);

    document.addEventListener('keydown', handleKeyDown);
    window.addEventListener('scroll', handleScrollOrResize, true);
    window.addEventListener('resize', handleScrollOrResize);
    return () => {
      document.removeEventListener('keydown', handleKeyDown);
      window.removeEventListener('scroll', handleScrollOrResize, true);
      window.removeEventListener('resize', handleScrollOrResize);
    };
  }, [paneMenu]);

  useEffect(() => {
    const handleGlobalDragCleanup = () => resetClipboardDragState();

    window.addEventListener('dragend', handleGlobalDragCleanup);
    window.addEventListener('drop', handleGlobalDragCleanup);
    return () => {
      window.removeEventListener('dragend', handleGlobalDragCleanup);
      window.removeEventListener('drop', handleGlobalDragCleanup);
      resetClipboardDragState();
    };
  }, [resetClipboardDragState]);

  // Keep a stable ref so the useEffect below can fire the error callback without
  // needing onRenderError in the useMemo dependency array.
  const onRenderErrorRef = useRef(onRenderError);
  useEffect(() => { onRenderErrorRef.current = onRenderError; }, [onRenderError]);

  const { nodes: initialNodes, edges: initialEdges, transformError } = useMemo(() => {
    if (!graphData) return { nodes: EMPTY_NODES, edges: EMPTY_EDGES, transformError: null };
    try {
      const result = transformGraphData(graphData);
      return { ...result, transformError: null };
    } catch (err) {
      const message = err instanceof Error ? err.message : String(err);
      // Do NOT fire side-effects (toasts, setState) inside useMemo — useMemo must
      // be pure.  The error message is surfaced via state and picked up by the
      // useEffect below, which fires the callback safely after the render cycle.
      return { nodes: EMPTY_NODES, edges: EMPTY_EDGES, transformError: message };
    }
  }, [graphData]);

  // Fire the render-error callback whenever the transform produces a new error.
  // A useEffect is the correct place for side-effects that react to derived state.
  // The ref ensures the callback is always current without adding it to the dep array.
  useEffect(() => {
    if (transformError) {
      onRenderErrorRef.current?.(`Graph render failed: ${transformError}`);
    }
  }, [transformError]);

  // Memoize the boundary key so JSON.stringify only runs when graphData actually
  // changes, not on every render of GraphView triggered by unrelated parent state.
  const boundaryKey = useMemo(
    () => graphData ? JSON.stringify(graphData.nodes.map(n => n.alias)) : 'empty',
    [graphData],
  );

  const [nodes, setNodes, onNodesChange] = useNodesState<Node<GraphNodeData>>(initialNodes);
  const [edges, setEdges, onEdgesChange] = useEdgesState<Edge<GraphEdgeData>>(initialEdges);

  // Re-sync whenever the upstream graphData changes
  useEffect(() => {
    setNodes(initialNodes);
    setEdges(initialEdges);
  }, [initialNodes, initialEdges, setNodes, setEdges]);

  const handleClipboardDragEnter = (event: React.DragEvent<HTMLDivElement>) => {
    if (!canAcceptClipboardDrop) return;
    if (!hasClipboardItemType(Array.from(event.dataTransfer.types))) return;

    event.preventDefault();
    clipboardDragDepthRef.current += 1;
    setClipboardDragActive(true);
  };

  const handleClipboardDragOver = (event: React.DragEvent<HTMLDivElement>) => {
    if (!canAcceptClipboardDrop) return;
    if (!hasClipboardItemType(Array.from(event.dataTransfer.types))) return;

    event.preventDefault();
    event.dataTransfer.dropEffect = 'copy';
    setClipboardDragActive(true);
  };

  const handleClipboardDragLeave = (event: React.DragEvent<HTMLDivElement>) => {
    if (!hasClipboardItemType(Array.from(event.dataTransfer.types))) return;

    clipboardDragDepthRef.current = Math.max(0, clipboardDragDepthRef.current - 1);
    if (clipboardDragDepthRef.current === 0) {
      setClipboardDragActive(false);
    }
  };

  const handleClipboardDrop = (event: React.DragEvent<HTMLDivElement>) => {
    if (!canAcceptClipboardDrop) return;
    if (!hasClipboardItemType(Array.from(event.dataTransfer.types))) return;

    event.preventDefault();
    const itemId = readClipboardItemId(event.dataTransfer);
    resetClipboardDragState();
    if (itemId) {
      onClipboardDrop?.(itemId);
    }
  };

  const hasGraphData = Boolean(graphData && graphData.nodes.length > 0);
  const contextNode = contextMenu && graphData
    ? findNodeByAlias(graphData, contextMenu.nodeAlias)
    : null;

  if (transformError) {
    return (
      <div className={styles.empty}>
        <span className={styles.emptyIcon}>⚠️</span>
        <span>Graph could not be rendered.</span>
        <span>{transformError}</span>
      </div>
    );
  }

  return (
    // key resets the boundary whenever the node set changes, so a corrected graph
    // after a previous render failure renders cleanly without a page reload.
    <GraphViewErrorBoundary
      key={boundaryKey}
      onRenderError={onRenderError}
    >
      <div className={styles.graphWrapper} aria-busy={isRefreshing}>
        {hasGraphData && graphData && (
          <GraphToolbar
            graphData={graphData}
            graphName={graphName}
            onCopySuccess={onCopySuccess}
            onCopyError={onCopyError}
          />
        )}

        <div
          className={styles.graphSurface}
          onDragEnter={handleClipboardDragEnter}
          onDragOver={handleClipboardDragOver}
          onDragLeave={handleClipboardDragLeave}
          onDrop={handleClipboardDrop}
        >
          {hasGraphData ? (
            <ReactFlow
              nodes={nodes}
              edges={edges}
              onNodesChange={onNodesChange}
              onEdgesChange={onEdgesChange}
              nodeTypes={nodeTypes}
              fitView
              fitViewOptions={{ padding: 0.25 }}
              minZoom={0.2}
              maxZoom={2.5}
              // colorMode="dark" // enable for dark mode
              proOptions={{ hideAttribution: false }}
              onNodeContextMenu={(event, node) => {
                event.preventDefault();
                event.stopPropagation();
                setPaneMenu(null);
                if (!canOpenNodeContextMenu) return;
                setContextMenu({ x: event.clientX, y: event.clientY, nodeAlias: node.data.alias });
              }}
              onPaneContextMenu={(event) => {
                event.preventDefault();
                if (!canCreateNode) return;
                setContextMenu(null);
                setPaneMenu({ x: event.clientX, y: event.clientY });
              }}
              onPaneClick={() => {
                setContextMenu(null);
                setPaneMenu(null);
              }}
            >
              <Background variant={BackgroundVariant.Dots} gap={18} size={1} color="rgba(255,255,255,0.07)" />
              <Controls showInteractive={false} />
              <MiniMap
                nodeColor={(node) => {
                  const colorMap: Record<string, string> = {
                    Root:        '#15803d',
                    End:         '#dc2626',
                    Fetcher:     '#2563eb',
                    mapper:      '#ea580c',
                    Math:        '#a16207',
                    JavaScript:  '#7e22ce',
                    Provider:    '#be185d',
                    Dictionary:  '#0e7490',
                    Join:        '#65a30d',
                    Extension:   '#4338ca',
                    Island:      '#475569',
                    Decision:    '#b45309',
                  };
                  return colorMap[node.type ?? ''] ?? '#6c7086';
                }}
                maskColor="rgba(0,0,0,0.3)"
                style={{ background: '#fff' }}
              />
            </ReactFlow>
          ) : (
            <div className={styles.empty}>
              <span className={styles.emptyIcon}>🕸️</span>
              <span>No graph data yet.</span>
              <span>Run <strong>describe graph</strong> or <strong>export graph</strong> in the playground.</span>
              {supportsAuthoring && onCreateNode && (
                <>
                  <button
                    type="button"
                    className={styles.emptyCreateButton}
                    disabled={!isConnected}
                    onClick={() => onCreateNode('empty-graph')}
                  >
                    Create Node
                  </button>
                  {!isConnected && (
                    <span className={styles.emptyHint}>Connect WebSocket to create a node.</span>
                  )}
                </>
              )}
            </div>
          )}

          {isRefreshing && (
            <div className={styles.refreshingOverlay}>
              <div
                className={styles.refreshingSpinner}
                role="status"
                aria-label="Graph refreshing"
              />
            </div>
          )}

          {clipboardDragActive && (
            <div className={styles.clipboardDropOverlay}>
              <div className={styles.clipboardDropMessage}>Drop to paste workspace node</div>
            </div>
          )}

          <GraphContextMenu
            open={paneMenu !== null}
            x={paneMenu?.x ?? 0}
            y={paneMenu?.y ?? 0}
            canCreateNode={canCreateNode}
            onCreateNode={() => onCreateNode?.('pane-context-menu')}
            onClose={() => setPaneMenu(null)}
          />
          <NodeContextMenu
            open={contextMenu !== null && contextNode !== null && canOpenNodeContextMenu}
            x={contextMenu?.x ?? 0}
            y={contextMenu?.y ?? 0}
            nodeAlias={contextMenu?.nodeAlias ?? ''}
            canClipNode={canClipNode && contextNode !== null}
            canEditNode={canEditNode && contextNode !== null}
            canDeleteNode={canDeleteNode && contextNode !== null}
            onClipNode={() => {
              if (!contextNode || !graphData) return;
              const connections = extractDirectConnections(graphData, contextNode.alias);
              onClipNode?.(contextNode, connections);
            }}
            onEditNode={() => {
              if (!contextNode) return;
              onEditNode?.(contextNode);
            }}
            onDeleteNode={() => {
              if (!contextNode) return;
              onDeleteNode?.(contextNode);
            }}
            onClose={() => setContextMenu(null)}
          />
        </div>
      </div>
    </GraphViewErrorBoundary>
  );
}
