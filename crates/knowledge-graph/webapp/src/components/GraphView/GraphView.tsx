import { useCallback, useEffect, useMemo, useRef, useState } from 'react';
import {
  ReactFlow,
  Background,
  Controls,
  useNodesState,
  useEdgesState,
  BackgroundVariant,
  SelectionMode,
  type Edge,
  type Node,
  type IsValidConnection,
  type OnConnect,
  type OnConnectStart,
  type OnConnectEnd,
} from '@xyflow/react';
import '@xyflow/react/dist/style.css';

import { AUTHORING_SOURCE_HANDLE_ID, AUTHORING_TARGET_HANDLE_ID, nodeTypes } from './NodeTypes';
import { GraphViewErrorBoundary } from './GraphViewErrorBoundary';
import { transformGraphData, type GraphNodeData, type GraphEdgeData } from '../../utils/graphTransformer';
import type { MinigraphGraphData, MinigraphNode, MinigraphConnection } from '../../utils/graphTypes';
import { hasClipboardItemType, readClipboardItemId } from '../../clipboard/dnd';
import { findNodeByAlias, extractDirectConnections } from '../../clipboard/helpers';
import GraphToolbar from '../GraphToolbar/GraphToolbar';
import GraphContextMenu from './GraphContextMenu';
import NodeContextMenu from './NodeContextMenu';
import GraphMultiSelectTip from './GraphMultiSelectTip';
import {
  filterAliasesToGraphNodes,
  resolveNodeContextTarget,
  type GraphClipItem,
  type NodeContextTarget,
} from './selectionTargets';
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
  onClipNodes?:    (items: GraphClipItem[]) => void;
  onClipboardDrop?: (itemId: string) => void;
  isConnected:     boolean;
  supportsAuthoring?: boolean;
  onCreateNode?:   (source: 'empty-graph' | 'pane-context-menu') => void;
  onCreateConnection?: (sourceAlias: string, targetAlias: string) => void;
  onEditNode?:     (node: MinigraphNode) => void;
  onDeleteNode?:   (node: MinigraphNode) => void;
  onDeleteNodes?:  (nodes: MinigraphNode[]) => void;
}

const EMPTY_NODES: Node<GraphNodeData>[]  = [];
const EMPTY_EDGES: Edge<GraphEdgeData>[]  = [];
const NODE_MULTI_SELECTION_KEYS = ['Shift', 'Control', 'Meta'];

function sameAliasSelection(current: string[], next: string[]): boolean {
  return current.length === next.length
    && current.every((alias, index) => alias === next[index]);
}

export default function GraphView({
  graphData,
  graphName,
  onCopySuccess,
  onCopyError,
  onRenderError,
  isRefreshing = false,
  onClipNode,
  onClipNodes,
  onClipboardDrop,
  isConnected,
  supportsAuthoring = false,
  onCreateNode,
  onCreateConnection,
  onEditNode,
  onDeleteNode,
  onDeleteNodes,
}: GraphViewProps) {

  // ── Context menu state ──────────────────────────────────────────────────
  const [contextMenu, setContextMenu] = useState<{
    x: number;
    y: number;
    target: NodeContextTarget;
  } | null>(null);
  const [paneMenu, setPaneMenu] = useState<{ x: number; y: number } | null>(null);
  const [selectedNodeAliases, setSelectedNodeAliases] = useState<string[]>([]);
  const [clipboardDragActive, setClipboardDragActive] = useState(false);
  const [tipVisible, setTipVisible] = useState(false);
  const [tipFading, setTipFading] = useState(false);
  const clipboardDragDepthRef = useRef(0);
  const tipShownRef = useRef(false);
  const tipFadeTimerRef = useRef<ReturnType<typeof setTimeout> | null>(null);
  const canCreateNode = Boolean(supportsAuthoring && onCreateNode && isConnected);
  const canCreateConnection = Boolean(supportsAuthoring && onCreateConnection && isConnected);
  const canClipNode = Boolean(onClipNode);
  const canClipNodes = Boolean(onClipNodes);
  const canEditNode = Boolean(supportsAuthoring && onEditNode && isConnected);
  const canDeleteNode = Boolean(supportsAuthoring && onDeleteNode && isConnected);
  const canDeleteNodes = Boolean(supportsAuthoring && onDeleteNodes && isConnected);
  const canOpenSingleNodeContextMenu = canClipNode || canEditNode || canDeleteNode;
  const canOpenMultiNodeContextMenu = canClipNodes || canDeleteNodes;
  const canOpenNodeContextMenu = canOpenSingleNodeContextMenu || canOpenMultiNodeContextMenu;
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
      const result = transformGraphData(graphData, {
        supportsConnectionAuthoring: canCreateConnection,
      });
      return { ...result, transformError: null };
    } catch (err) {
      const message = err instanceof Error ? err.message : String(err);
      // Do NOT fire side-effects (toasts, setState) inside useMemo — useMemo must
      // be pure.  The error message is surfaced via state and picked up by the
      // useEffect below, which fires the callback safely after the render cycle.
      return { nodes: EMPTY_NODES, edges: EMPTY_EDGES, transformError: message };
    }
  }, [canCreateConnection, graphData]);

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
  const connectionDragSourceRef = useRef<string | null>(null);
  const hasGraphData = Boolean(graphData && graphData.nodes.length > 0);

  // React Flow can notify selection while it initializes controlled nodes.
  // Keep the handler stable and avoid writing an equivalent alias snapshot,
  // otherwise initialization can feed an unnecessary render/update loop.
  const handleSelectionChange = useCallback(({ nodes: selectedNodes }: {
    nodes: Node<GraphNodeData>[];
  }) => {
    const nextAliases = selectedNodes.map((node) => node.data.alias);
    setSelectedNodeAliases((currentAliases) => (
      sameAliasSelection(currentAliases, nextAliases) ? currentAliases : nextAliases
    ));
  }, []);

  // Re-sync whenever the upstream graphData changes
  useEffect(() => {
    setNodes(initialNodes);
    setEdges(initialEdges);
    setSelectedNodeAliases([]);
    setContextMenu(null);
  }, [initialNodes, initialEdges, setNodes, setEdges]);

  const dismissMultiSelectTip = useCallback(() => {
    if (!tipVisible || tipFading) return;
    setTipFading(true);
    if (tipFadeTimerRef.current !== null) {
      clearTimeout(tipFadeTimerRef.current);
    }
    tipFadeTimerRef.current = setTimeout(() => {
      setTipVisible(false);
      tipFadeTimerRef.current = null;
    }, 400);
  }, [tipFading, tipVisible]);

  // The shortcut is introduced only after the first useful graph reaches the
  // canvas. Keeping the shown flag in memory makes ordinary refreshes quiet.
  useEffect(() => {
    if (!hasGraphData || transformError || tipShownRef.current) return;
    tipShownRef.current = true;
    setTipFading(false);
    setTipVisible(true);
  }, [hasGraphData, transformError]);

  useEffect(() => {
    if (!tipVisible || tipFading) return;
    const timerId = setTimeout(dismissMultiSelectTip, 5000);
    return () => clearTimeout(timerId);
  }, [dismissMultiSelectTip, tipFading, tipVisible]);

  useEffect(() => {
    return () => {
      if (tipFadeTimerRef.current !== null) {
        clearTimeout(tipFadeTimerRef.current);
      }
    };
  }, []);

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

  const graphNodeAliases = useMemo(
    () => new Set(graphData?.nodes.map((node) => node.alias) ?? []),
    [graphData],
  );
  const contextNode = contextMenu?.target.kind === 'single-node' && graphData
    ? findNodeByAlias(graphData, contextMenu.target.alias)
    : null;
  const contextAliases = contextMenu?.target.kind === 'multi-node'
    ? contextMenu.target.aliases
    : [];
  const contextNodes = graphData
    ? filterAliasesToGraphNodes(contextAliases, graphData)
    : [];

  const isConnectionValid = useCallback<IsValidConnection>((connection) => {
    if (!canCreateConnection) return false;
    if (!connection.source || !connection.target) return false;
    if (connection.source === connection.target) return false;
    if (!graphNodeAliases.has(connection.source) || !graphNodeAliases.has(connection.target)) return false;
    return connection.sourceHandle === AUTHORING_SOURCE_HANDLE_ID &&
      connection.targetHandle === AUTHORING_TARGET_HANDLE_ID;
  }, [canCreateConnection, graphNodeAliases]);

  const handleConnect = useCallback<OnConnect>((connection) => {
    if (!isConnectionValid(connection)) return;
    if (!connection.source || !connection.target) return;
    onCreateConnection?.(connection.source, connection.target);
  }, [isConnectionValid, onCreateConnection]);

  const handleConnectStart = useCallback<OnConnectStart>((_event, params) => {
    if (params.handleId !== AUTHORING_SOURCE_HANDLE_ID || params.handleType !== 'source') return;
    connectionDragSourceRef.current = params.nodeId;
    setContextMenu(null);
    setPaneMenu(null);
  }, []);

  const handleConnectEnd = useCallback<OnConnectEnd>(() => {
    connectionDragSourceRef.current = null;
  }, []);

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
          onWheelCapture={dismissMultiSelectTip}
        >
          {hasGraphData ? (
            <ReactFlow
              nodes={nodes}
              edges={edges}
              onNodesChange={onNodesChange}
              onEdgesChange={onEdgesChange}
              nodesConnectable={canCreateConnection}
              edgesReconnectable={false}
              connectOnClick={false}
              isValidConnection={isConnectionValid}
              onConnect={handleConnect}
              onConnectStart={handleConnectStart}
              onConnectEnd={handleConnectEnd}
              nodeTypes={nodeTypes}
              fitView
              fitViewOptions={{ padding: 0.25 }}
              minZoom={0.2}
              maxZoom={2.5}
              selectionKeyCode="Shift"
              multiSelectionKeyCode={NODE_MULTI_SELECTION_KEYS}
              selectionOnDrag={false}
              selectionMode={SelectionMode.Partial}
              // colorMode="dark" // enable for dark mode
              proOptions={{ hideAttribution: false }}
              onSelectionChange={handleSelectionChange}
              onNodeContextMenu={(event, node) => {
                event.preventDefault();
                event.stopPropagation();
                dismissMultiSelectTip();
                setPaneMenu(null);
                if (!canOpenNodeContextMenu) return;
                const target = resolveNodeContextTarget(node.data.alias, selectedNodeAliases);
                if (target.kind === 'single-node' && selectedNodeAliases.length > 1) {
                  setNodes((currentNodes) => currentNodes.map((currentNode) => ({
                    ...currentNode,
                    selected: currentNode.data.alias === node.data.alias,
                  })));
                  setSelectedNodeAliases([node.data.alias]);
                }
                setContextMenu({ x: event.clientX, y: event.clientY, target });
              }}
              onPaneContextMenu={(event) => {
                event.preventDefault();
                dismissMultiSelectTip();
                setContextMenu(null);
                if (!canCreateNode) return;
                setPaneMenu({ x: event.clientX, y: event.clientY });
              }}
              onPaneClick={() => {
                dismissMultiSelectTip();
                setContextMenu(null);
                setPaneMenu(null);
              }}
              onNodeClick={() => dismissMultiSelectTip()}
              onNodeDragStart={() => dismissMultiSelectTip()}
              onSelectionStart={() => dismissMultiSelectTip()}
              onMoveStart={(event) => {
                if (event) dismissMultiSelectTip();
              }}
            >
              <Background variant={BackgroundVariant.Dots} gap={18} size={1} color="rgba(255,255,255,0.07)" />
              <Controls showInteractive={false} />
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

          <GraphMultiSelectTip
            visible={tipVisible}
            fading={tipFading}
            onDismiss={dismissMultiSelectTip}
          />

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
          {contextMenu?.target.kind === 'multi-node' ? (
            <NodeContextMenu
              mode="multi-node"
              open={contextNodes.length > 1 && canOpenMultiNodeContextMenu}
              x={contextMenu.x}
              y={contextMenu.y}
              selectedCount={contextAliases.length}
              canClipSelectedNodes={canClipNodes}
              canDeleteSelectedNodes={canDeleteNodes}
              onClipSelectedNodes={() => {
                if (!graphData) {
                  onClipNodes?.([]);
                  return;
                }
                const items = contextNodes.map((node) => ({
                  node,
                  connections: extractDirectConnections(graphData, node.alias),
                }));
                onClipNodes?.(items);
              }}
              onDeleteSelectedNodes={() => {
                const allTargetsStillExist = contextNodes.length === contextAliases.length;
                onDeleteNodes?.(allTargetsStillExist ? contextNodes : []);
              }}
              onClose={() => setContextMenu(null)}
            />
          ) : (
            <NodeContextMenu
              mode="single-node"
              open={contextMenu !== null && contextNode !== null && canOpenSingleNodeContextMenu}
              x={contextMenu?.x ?? 0}
              y={contextMenu?.y ?? 0}
              nodeAlias={contextMenu?.target.kind === 'single-node' ? contextMenu.target.alias : ''}
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
          )}
        </div>
      </div>
    </GraphViewErrorBoundary>
  );
}
