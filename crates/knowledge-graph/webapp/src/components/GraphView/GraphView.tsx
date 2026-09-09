import { useCallback, useEffect, useMemo, useRef, useState } from 'react';
import {
  ReactFlow,
  Background,
  ControlButton,
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
  type OnBeforeDelete,
  type ReactFlowInstance,
} from '@xyflow/react';
import '@xyflow/react/dist/style.css';

import { AUTHORING_SOURCE_HANDLE_ID, AUTHORING_TARGET_HANDLE_ID, nodeTypes } from './NodeTypes';
import { GraphViewErrorBoundary } from './GraphViewErrorBoundary';
import { transformGraphData, computeMeasuredPositions, type GraphNodeData, type GraphEdgeData } from '../../utils/graphTransformer';
import { useLocalStorage } from '../../hooks/useLocalStorage';
import type { MinigraphGraphData, MinigraphNode, MinigraphConnection } from '../../utils/graphTypes';
import type { ConnectionRemovalRequest } from '../../graphActions/connectionEdits';
import { hasClipboardItemType, readClipboardItemId } from '../../clipboard/dnd';
import { findNodeByAlias, extractDirectConnections } from '../../clipboard/helpers';
import GraphToolbar from '../GraphToolbar/GraphToolbar';
import GraphRunControls, { type GraphRunControlsProps } from '../GraphToolbar/GraphRunControls';
import GraphContextMenu from './GraphContextMenu';
import NodeContextMenu from './NodeContextMenu';
import EdgeContextMenu from './EdgeContextMenu';
import GraphMinimap from './GraphMinimap';
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
  graphRunControls?: GraphRunControlsProps;
  onRenderError?:  (message: string) => void;
  /** When true, renders a semi-transparent overlay with a spinner to indicate a background re-fetch. */
  isRefreshing?:   boolean;
  /** Callback for "Clip to Workspace" from the node context menu. */
  onClipNode?:     (node: MinigraphNode, connections: MinigraphConnection[]) => void;
  onClipNodes?:    (items: GraphClipItem[]) => void;
  onClipboardDrop?: (itemId: string) => void;
  /** Whether the Graph tab is currently visible and may handle graph-only hotkeys. */
  isActive:        boolean;
  isConnected:     boolean;
  supportsAuthoring?: boolean;
  onCreateNode?:   (source: 'empty-graph' | 'pane-context-menu') => void;
  /** anchor = viewport position of the completing gesture, for the relation popover. */
  onCreateConnection?: (sourceAlias: string, targetAlias: string, anchor?: { x: number; y: number }) => void;
  onEditNode?:     (node: MinigraphNode) => void;
  onDeleteNode?:   (node: MinigraphNode) => void;
  onDeleteNodes?:  (nodes: MinigraphNode[]) => void;
  /**
   * One call per delete gesture, with every requested directed removal:
   * Delete/Backspace passes the selected edges (relation undefined = the whole
   * directed edge); the edge context menu passes one named relation.  Batching
   * lets the handler plan reconnects across edges that share a node pair.
   */
  onDeleteConnections?: (requests: ConnectionRemovalRequest[]) => void;
  /**
   * Changes whenever a panel toggle reshapes the graph pane (console hidden or
   * restored, node editor opened or closed, workspace/help panels).  The graph
   * re-fits on change — but never during continuous separator drags, which
   * must not fight the user's zoom/pan.
   */
  panelLayoutKey?: string;
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
  graphRunControls,
  onRenderError,
  isRefreshing = false,
  onClipNode,
  onClipNodes,
  onClipboardDrop,
  isActive,
  isConnected,
  supportsAuthoring = false,
  onCreateNode,
  onCreateConnection,
  onEditNode,
  onDeleteNode,
  onDeleteNodes,
  onDeleteConnections,
  panelLayoutKey,
}: GraphViewProps) {

  // ── Context menu state ──────────────────────────────────────────────────
  const [contextMenu, setContextMenu] = useState<{
    x: number;
    y: number;
    target: NodeContextTarget;
  } | null>(null);
  const [paneMenu, setPaneMenu] = useState<{ x: number; y: number } | null>(null);
  const [edgeMenu, setEdgeMenu] = useState<{
    x: number;
    y: number;
    source: string;
    target: string;
    relations: string[];
  } | null>(null);
  const [selectedNodeAliases, setSelectedNodeAliases] = useState<string[]>([]);
  const [clipboardDragActive, setClipboardDragActive] = useState(false);
  const [tipVisible, setTipVisible] = useState(false);
  const [tipFading, setTipFading] = useState(false);
  const [minimapOpen, setMinimapOpen] = useState(false);
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
  const canOpenSingleNodeContextMenu = canClipNode || canEditNode || canDeleteNode || canCreateConnection;
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

  // ── Thumbnail / expanded node detail mode ────────────────────────────────
  // Thumbnail nodes render header-only cards for a compact topology overview.
  // Toggling rebuilds the nodes (mode is a transform input), which re-enters
  // the same measure-then-relayout cycle as a fresh graph load.
  const [compactNodes, setCompactNodes] = useLocalStorage<boolean>('graph-nodes-compact', false);

  const { nodes: initialNodes, edges: initialEdges, transformError } = useMemo(() => {
    if (!graphData) return { nodes: EMPTY_NODES, edges: EMPTY_EDGES, transformError: null };
    try {
      const result = transformGraphData(graphData, {
        supportsConnectionAuthoring: canCreateConnection,
        compactNodes,
      });
      return { ...result, transformError: null };
    } catch (err) {
      const message = err instanceof Error ? err.message : String(err);
      // Do NOT fire side-effects (toasts, setState) inside useMemo — useMemo must
      // be pure.  The error message is surfaced via state and picked up by the
      // useEffect below, which fires the callback safely after the render cycle.
      return { nodes: EMPTY_NODES, edges: EMPTY_EDGES, transformError: message };
    }
  }, [canCreateConnection, compactNodes, graphData]);

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
    setEdgeMenu(null);
  }, [initialNodes, initialEdges, setNodes, setEdges]);

  // ── Measured re-layout ────────────────────────────────────────────────────
  // The transformer positions nodes from ESTIMATED heights — real heights only
  // exist after React Flow measures the rendered DOM, because nodes size to
  // their content.  Once every node reports a measured height, re-run the
  // layout with the true values and re-fit the viewport; this is what
  // guarantees nodes never overlap regardless of content.  Runs once per
  // (graphData, detail mode) pair: later dimension changes (a manual
  // NodeResizer drag) are the user's own and must not snap the layout back.
  const rfInstanceRef = useRef<ReactFlowInstance<Node<GraphNodeData>, Edge<GraphEdgeData>> | null>(null);
  const measuredLayoutDoneRef = useRef<{ graph: MinigraphGraphData; compact: boolean } | null>(null);
  useEffect(() => {
    if (!graphData || graphData.nodes.length === 0) return;
    const done = measuredLayoutDoneRef.current;
    if (done && done.graph === graphData && done.compact === compactNodes) return;

    // The nodes state must already derive from THIS graphData and detail mode.
    // On a graph refresh or a mode toggle this effect can fire in the same
    // commit as the re-sync above, while `nodes` still holds the previous
    // build's (measured) nodes — the alias set may even match.  The
    // transformer passes each graph node's `properties` object through by
    // reference (an exact provenance test: parsed server payloads always
    // allocate fresh objects) and stamps the detail mode on `data.compact`.
    const propsByAlias = new Map(graphData.nodes.map(n => [n.alias, n.properties]));
    const nodesMatchGraph = nodes.length === propsByAlias.size &&
      nodes.every(node =>
        propsByAlias.get(node.id) === node.data.properties &&
        node.data.compact === compactNodes,
      );
    if (!nodesMatchGraph) return;

    const measuredHeights = new Map<string, number>();
    for (const node of nodes) {
      const height = node.measured?.height;
      if (typeof height !== 'number') return; // wait until every node is measured
      measuredHeights.set(node.id, height);
    }

    measuredLayoutDoneRef.current = { graph: graphData, compact: compactNodes };
    const measuredPositions = computeMeasuredPositions(graphData, measuredHeights, { compactNodes });
    const moved = nodes.some(node => {
      const position = measuredPositions.get(node.id);
      return position !== undefined &&
        (position.x !== node.position.x || position.y !== node.position.y);
    });
    if (moved) {
      setNodes(currentNodes => currentNodes.map(node => {
        const position = measuredPositions.get(node.id);
        return position ? { ...node, position } : node;
      }));
    }
    // Re-fit even when nothing moved: a detail-mode toggle changes the graph
    // bounds drastically while the estimate layout may already be exact.
    requestAnimationFrame(() => {
      rfInstanceRef.current?.fitView({ padding: 0.1 });
    });
  }, [compactNodes, graphData, nodes, setNodes]);

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

  // Last pointer-up position in viewport coordinates: the capture-phase
  // listener runs before React Flow's own handlers, so when onConnect fires
  // the ref already holds the drop position — the relation popover anchors there.
  const lastPointerUpRef = useRef<{ x: number; y: number } | null>(null);
  useEffect(() => {
    const handlePointerUp = (event: PointerEvent) => {
      lastPointerUpRef.current = { x: event.clientX, y: event.clientY };
    };
    document.addEventListener('pointerup', handlePointerUp, true);
    return () => document.removeEventListener('pointerup', handlePointerUp, true);
  }, []);

  const handleConnect = useCallback<OnConnect>((connection) => {
    if (!isConnectionValid(connection)) return;
    if (!connection.source || !connection.target) return;
    onCreateConnection?.(connection.source, connection.target, lastPointerUpRef.current ?? undefined);
  }, [isConnectionValid, onCreateConnection]);

  // ── Click-to-connect (alternative to the halo drag) ───────────────────────
  // "Connect to…" in the node context menu arms this mode: the next click on
  // any other node creates the connection; Esc, a pane click, or a graph
  // change cancels it.
  const [connectFromAlias, setConnectFromAlias] = useState<string | null>(null);

  useEffect(() => {
    setConnectFromAlias(null);
  }, [graphData, canCreateConnection]);

  useEffect(() => {
    if (connectFromAlias === null) return;
    const handleKeyDown = (event: KeyboardEvent) => {
      if (event.key !== 'Escape') return;
      event.preventDefault();
      setConnectFromAlias(null);
    };
    document.addEventListener('keydown', handleKeyDown);
    return () => document.removeEventListener('keydown', handleKeyDown);
  }, [connectFromAlias]);

  const handleConnectStart = useCallback<OnConnectStart>((_event, params) => {
    if (params.handleId !== AUTHORING_SOURCE_HANDLE_ID || params.handleType !== 'source') return;
    connectionDragSourceRef.current = params.nodeId;
    setContextMenu(null);
    setPaneMenu(null);
    setEdgeMenu(null);
  }, []);

  const handleConnectEnd = useCallback<OnConnectEnd>(() => {
    connectionDragSourceRef.current = null;
  }, []);

  // ── Auto-fit on discrete panel-layout changes ──────────────────────────────
  // Hiding/restoring the console (or the node editor taking/releasing its
  // slot, or the workspace/help panels toggling) reshapes the graph pane; the
  // graph re-fits so it always uses the new real estate. The key carries only
  // panel VISIBILITY, so dragging a separator never re-fits. Double rAF: the
  // first frame lets React Flow's own ResizeObserver ingest the new container
  // size, the second fits against the fresh dimensions.
  const previousPanelLayoutKeyRef = useRef(panelLayoutKey);
  useEffect(() => {
    if (previousPanelLayoutKeyRef.current === panelLayoutKey) return;
    previousPanelLayoutKeyRef.current = panelLayoutKey;
    if (!hasGraphData) return;
    let innerFrame: number | null = null;
    const outerFrame = requestAnimationFrame(() => {
      innerFrame = requestAnimationFrame(() => {
        rfInstanceRef.current?.fitView({ padding: 0.1 });
      });
    });
    return () => {
      cancelAnimationFrame(outerFrame);
      if (innerFrame !== null) cancelAnimationFrame(innerFrame);
    };
  }, [hasGraphData, panelLayoutKey]);

  // ── Keyboard delete of selected connections ────────────────────────────────
  // Click an edge to select it, press Delete (or Backspace) to remove it.
  // React Flow's delete pipeline is only used as the key handler: the callback
  // ALWAYS blocks the local removal and forwards selected edges to the backend
  // command instead — the graph re-renders from the backend's "{a} -> {b}
  // removed" confirmation via the auto-refresh. Selected nodes are ignored
  // here on purpose: node deletion keeps its confirmed context-menu flow.
  const canDeleteConnection = Boolean(supportsAuthoring && onDeleteConnections && isConnected);
  const handleBeforeDelete = useCallback<OnBeforeDelete<Node<GraphNodeData>, Edge<GraphEdgeData>>>(
    async ({ edges: edgesToDelete }) => {
      if (canDeleteConnection && edgesToDelete.length > 0) {
        // One request per directed edge; the handler plans reconnects across
        // edges sharing a pair (the backend deletes a pair in both directions).
        const seenPairs = new Set<string>();
        const requests: ConnectionRemovalRequest[] = [];
        for (const edge of edgesToDelete) {
          const pair = `${edge.source}\t${edge.target}`;
          if (seenPairs.has(pair)) continue;
          seenPairs.add(pair);
          requests.push({ source: edge.source, target: edge.target });
        }
        onDeleteConnections?.(requests);
      }
      return false; // the backend owns graph mutations — never delete locally
    },
    [canDeleteConnection, onDeleteConnections],
  );

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
            extraActions={graphRunControls ? <GraphRunControls {...graphRunControls} /> : undefined}
          />
        )}

        <div
          className={styles.graphSurface}
          data-connect-picking={connectFromAlias !== null || undefined}
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
              onInit={(instance) => { rfInstanceRef.current = instance; }}
              onNodesChange={onNodesChange}
              onEdgesChange={onEdgesChange}
              nodesConnectable={canCreateConnection}
              edgesReconnectable={false}
              connectOnClick={false}
              isValidConnection={isConnectionValid}
              onConnect={handleConnect}
              onConnectStart={handleConnectStart}
              onConnectEnd={handleConnectEnd}
              deleteKeyCode={['Delete', 'Backspace']}
              onBeforeDelete={handleBeforeDelete}
              nodeTypes={nodeTypes}
              fitView
              fitViewOptions={{ padding: 0.1 }}
              minZoom={0.2}
              maxZoom={4}
              // Trackpad pinch sends wheel events with ctrlKey set; zoomOnPinch
              // (mobile/tablet multi-touch) and zoomOnScroll (desktop wheel +
              // trackpad pinch) are spelled out explicitly here — rather than
              // left as library defaults — so a future React Flow upgrade
              // can't silently change this behaviour.
              zoomOnScroll
              zoomOnPinch
              zoomOnDoubleClick
              panOnScroll={false}
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
                setEdgeMenu(null);
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
              onEdgeContextMenu={(event, edge) => {
                event.preventDefault();
                event.stopPropagation();
                dismissMultiSelectTip();
                setContextMenu(null);
                setPaneMenu(null);
                if (!canDeleteConnection) return;
                setEdgeMenu({
                  x: event.clientX,
                  y: event.clientY,
                  source: edge.source,
                  target: edge.target,
                  relations: edge.data?.relationTypes ?? [],
                });
              }}
              onPaneContextMenu={(event) => {
                event.preventDefault();
                dismissMultiSelectTip();
                setContextMenu(null);
                setEdgeMenu(null);
                if (!canCreateNode) return;
                setPaneMenu({ x: event.clientX, y: event.clientY });
              }}
              onPaneClick={() => {
                dismissMultiSelectTip();
                setContextMenu(null);
                setPaneMenu(null);
                setEdgeMenu(null);
                setConnectFromAlias(null);
              }}
              onNodeClick={(event, node) => {
                dismissMultiSelectTip();
                if (connectFromAlias === null) return;
                event.preventDefault();
                event.stopPropagation();
                const targetAlias = node.data.alias;
                if (targetAlias !== connectFromAlias) {
                  onCreateConnection?.(connectFromAlias, targetAlias, {
                    x: event.clientX,
                    y: event.clientY,
                  });
                }
                setConnectFromAlias(null);
              }}
              onNodeDragStart={() => dismissMultiSelectTip()}
              onSelectionStart={() => dismissMultiSelectTip()}
              onMoveStart={(event) => {
                if (event) dismissMultiSelectTip();
              }}
            >
              <Background variant={BackgroundVariant.Dots} gap={18} size={1} color="rgba(255,255,255,0.07)" />
              <GraphMinimap
                open={minimapOpen}
                onOpenChange={setMinimapOpen}
                hotkeyEnabled={isActive}
              >
                {/* Thumbnail / expanded node detail toggle — lives with the
                    other view controls (+ / − / fit). */}
                <ControlButton
                  onClick={() => setCompactNodes(prev => !prev)}
                  title={compactNodes ? 'Show node details' : 'Show thumbnail nodes'}
                  aria-label={compactNodes ? 'Show node details' : 'Show thumbnail nodes'}
                  aria-pressed={compactNodes}
                >
                  {/* Mini node card; body rows appear when the click would
                      expand the details, a bare card when it would collapse. */}
                  <span className={styles.detailToggleIcon} aria-hidden="true">
                    <span className={styles.detailToggleHeader} />
                    {compactNodes && (
                      <>
                        <span className={styles.detailToggleLine} />
                        <span className={styles.detailToggleLine} />
                        <span className={styles.detailToggleLine} />
                      </>
                    )}
                  </span>
                </ControlButton>
              </GraphMinimap>
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

          {connectFromAlias !== null && (
            <div className={styles.connectBanner} role="status">
              <span>
                Connecting from <strong>{connectFromAlias}</strong> — click a target node
              </span>
              <button
                type="button"
                className={styles.connectBannerCancel}
                onClick={() => setConnectFromAlias(null)}
              >
                Cancel (Esc)
              </button>
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
              canConnectNode={canCreateConnection && contextNode !== null}
              canEditNode={canEditNode && contextNode !== null}
              canDeleteNode={canDeleteNode && contextNode !== null}
              onConnectNode={() => {
                if (!contextNode) return;
                setConnectFromAlias(contextNode.alias);
              }}
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
          <EdgeContextMenu
            open={edgeMenu !== null && canDeleteConnection}
            x={edgeMenu?.x ?? 0}
            y={edgeMenu?.y ?? 0}
            sourceAlias={edgeMenu?.source ?? ''}
            targetAlias={edgeMenu?.target ?? ''}
            relations={edgeMenu?.relations ?? []}
            onDeleteRelation={(relation) => {
              if (!edgeMenu) return;
              onDeleteConnections?.([{
                source: edgeMenu.source,
                target: edgeMenu.target,
                relation,
              }]);
            }}
            onClose={() => setEdgeMenu(null)}
          />
        </div>
      </div>
    </GraphViewErrorBoundary>
  );
}
