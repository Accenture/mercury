import { useState, useMemo, useCallback, useEffect, useRef } from 'react';
import { useNavigate } from 'react-router';
import { Group, Panel, Separator, useDefaultLayout } from 'react-resizable-panels';
import styles from './Playground.module.css';
import { validatePayload, formatJSON } from '../utils/validators';
import { useToast } from '../hooks/useToast';
import { useLocalStorage } from '../hooks/useLocalStorage';
import { useWebSocket } from '../hooks/useWebSocket';
import { useMediaQuery } from '../hooks/useMediaQuery';
import { useGraphData } from '../hooks/useGraphData';
import { useAutoGraphRefresh } from '../hooks/useAutoGraphRefresh';
import { useAutoHelpNavigate } from '../hooks/useAutoHelpNavigate';
import { useSendToJsonPath } from '../hooks/useSendToJsonPath';
import { useMockUploadModal } from '../hooks/useMockUploadModal';
import { useLargePayloadDownload } from '../hooks/useLargePayloadDownload';
import { useSavedGraphs } from '../hooks/useSavedGraphs';
import { useGraphSaveName } from '../hooks/useGraphSaveName';
import { useSavedGraphWorkflow } from '../hooks/useSavedGraphWorkflow';
import { usePinnedGraphPath } from '../hooks/usePinnedGraphPath';
import { buildClipboardPastePlan } from '../clipboard/paste';
import { buildBatchClipToast } from '../clipboard/batchClipSummary';
import { createGraphAuthoringExecutor } from '../graphActions/graphAuthoringExecutor';
import {
  describeRemovedRelations,
  planConnectionRemoval,
  type ConnectionRemovalRequest,
} from '../graphActions/connectionEdits';
import {
  buildConnectionCreateUndo,
  buildConnectionRemovalUndo,
  buildNodeCreateUndo,
  buildNodeDeleteUndo,
  buildNodeEditUndo,
  type UndoEntry,
} from '../graphActions/undoCommands';
import { useGraphUndo } from '../hooks/useGraphUndo';
import { MAX_BATCH_NODE_ACTIONS } from '../graphActions/batchNodeActions';
import { ToastContainer } from './Toast';
import Navigation from './Navigation';
import GraphSaveButton from './GraphSaveButton/GraphSaveButton';
import SavedGraphsMenu from './SavedGraphsMenu/SavedGraphsMenu';
import RightPanel from './RightPanel/RightPanel';
import LeftPanel from './LeftPanel/LeftPanel';
import NodeEditPanel from './NodeEditPanel/NodeEditPanel';
import { MockUploadModal } from './MockUploadModal/MockUploadModal';
import ConnectionPopover from './ConnectionPopover/ConnectionPopover';
import { useGraphAuthoring } from './GraphAuthoring/useGraphAuthoring';
import { useSessionCollaboration } from '../session/useSessionCollaboration';
import ClipboardSidebar from './ClipboardSidebar/ClipboardSidebar';
import HelpBrowser from './HelpBrowser/HelpBrowser';
import { ClipboardDuplicateDialog } from './ClipboardSidebar/ClipboardDuplicateDialog';
import { type PlaygroundConfig } from '../config/playgrounds';
import { resolveBundledHelpTopic } from '../utils/localHelpCommand';
import { useWebSocketContext } from '../contexts/WebSocketContext';
import { useClipboardContext } from '../contexts/ClipboardContext';
import { ProtocolBus } from '../protocol/bus';
import { useProtocolKernel } from '../protocol/useProtocolKernel';
import type { GraphLinkEvent } from '../protocol/events';
import type { NodeActionTextResult } from '../utils/messageParser';
import type { ClipboardItemRecord } from '../clipboard/db';
import type { MinigraphNode, MinigraphConnection } from '../utils/graphTypes';
import type { GraphClipItem } from './GraphView/selectionTargets';

interface PlaygroundProps {
  config: PlaygroundConfig;
}

export default function Playground({ config }: PlaygroundProps) {
  const { title, wsPath, storageKeyPayload, storageKeyHistory, storageKeyTab, storageKeySavedGraphs, supportsUpload, supportsClipboard, supportsHelp, supportsAuthoring, supportsSessionCollaboration, tabs } = config;

  const navigate = useNavigate();

  // Persisted payload (survives navigation / refresh via localStorage).
  const [storedPayload, setStoredPayload] = useLocalStorage<string>(storageKeyPayload, '');
  const ctx = useWebSocketContext();

  // Cross-playground payload transfer: when the user clicks ➡️ in the
  // Minigraph console to send a JSON result to JSON-Path, Playground.tsx
  // calls ctx.setPendingPayload() and navigates.  The receiving playground
  // peeks it here at mount (useState initialiser) and then consumes it
  // reactively via the effect below (if the playground is already mounted).
  // Using a separate useState means we never write cross-playground payloads
  // to localStorage. null = no override; the stored value is used instead.
  const [payloadOverride, setPayloadOverride] = useState<string | null>(() =>
    ctx.peekPendingPayload(wsPath)
  );

  // Reactively consume a pending payload deposited by useLargePayloadDownload.
  // takePendingPayload has a new identity whenever a payload is deposited
  // (backed by useState in the context), so this effect fires precisely when
  // a new payload arrives — covering both the first-mount and already-mounted cases.
  const { takePendingPayload } = ctx;
  useEffect(() => {
    const pending = takePendingPayload(wsPath);
    if (pending !== null) {
      setPayloadOverride(pending);
    }
  }, [takePendingPayload, wsPath]);

  // The active payload: override wins over the persisted value.
  const payload    = payloadOverride ?? storedPayload;
  // Writers: the textarea and format/clear actions always write to localStorage.
  // Clearing the override on any manual edit lets the user take back control.
  const setPayload = useCallback((value: string | ((prev: string) => string)) => {
    setPayloadOverride(null);
    setStoredPayload(value as string);
  }, [setStoredPayload]);

  // Live payload validation — derived synchronously from payload, no extra render cycle needed
  const payloadValidation = useMemo(
    () => payload ? validatePayload(payload) : { valid: true, error: null, type: null },
    [payload]
  );

  // Toast notifications
  const { toasts, addToast, removeToast } = useToast();

  // ── Protocol Kernel ─────────────────────────────────────────────────────
  const busRef = useRef(new ProtocolBus());
  const bus = busRef.current;

  // Bundled help commands are handled locally — no WebSocket round-trip needed.
  // Define before useWebSocket so it is in scope when the options object is evaluated.
  const handleLocalCommand = useCallback((text: string): boolean => {
    return resolveBundledHelpTopic(text, supportsHelp === true) !== null;
  }, [supportsHelp]);

  // All WebSocket + console logic lives in the hook
  const ws = useWebSocket({ wsPath, storageKeyHistory, payload, addToast, bus, handleLocalCommand });

  const { classificationMap } = useProtocolKernel({
    messages: ws.messages, bus,
  });

  // ── Graph state ────────────────────────────────────────────────────────────────────────
  // The API path extracted from the currently-pinned graph-link message.
  // Survives Playground remounts (SPA navigation between playground tabs)
  // via a module-scoped Map, but resets on hard refresh / page load —
  // matching the server session lifetime (WebSocket-bound).
  const [pinnedGraphPath, setPinnedGraphPath] = usePinnedGraphPath(wsPath);

  // Fetch + parse graph data, auto-switch to Graph tab — logic lives in the hook.
  const { graphData, setGraphData, rightTab, setRightTab, isRefreshing } = useGraphData(
    pinnedGraphPath,
    addToast,
    tabs[0],
    tabs,
    storageKeyTab,
  );

  // ── Mock-upload modal ────────────────────────────────────────────────────
  const {
    modalUploadPath, successfulUploadPaths,
    handleOpenUploadModal, handleCloseUploadModal,
    handleUploadSuccess, handleUploadError, resetSuccessfulPaths,
  } = useMockUploadModal({ bus, addToast });

  // ── Auto-refresh on mutation commands ────────────────────────────────────
  useAutoGraphRefresh({
    bus,
    pinnedGraphPath,
    setPinnedGraphPath,
    connected:   ws.connected,
    sendRawText: ws.sendRawText,
    addToast,
  });

  // ── Undo (compensating commands) ──────────────────────────────────────────
  // The backend stays a lightweight command executor (minimalist principle):
  // every UI-initiated mutation captures a pre-mutation snapshot and pushes
  // the inverse console commands. Ctrl+Z and per-toast Undo buttons replay
  // them; the graph redraws from the backend confirmations.
  const graphUndo = useGraphUndo({
    bus,
    connected: ws.connected,
    sendRawText: ws.sendRawText,
    addToast,
  });

  // Pre-mutation snapshots, keyed to the in-flight authoring action.
  const preEditNodeRef = useRef<MinigraphNode | null>(null);
  const pendingConnectionUndoRef = useRef<UndoEntry | null>(null);
  const pendingNodeDeleteUndoRef = useRef(new Map<string, UndoEntry | null>());

  const toastWithUndo = useCallback((message: string, undoId: number | null) => {
    if (undoId === null) {
      addToast(message, 'success');
      return;
    }
    addToast(message, 'success', {
      durationMs: 6000,
      action: { label: 'Undo', onClick: () => graphUndo.undoEntry(undoId) },
    });
  }, [addToast, graphUndo.undoEntry]);

  // Push the inverse only once the backend ACCEPTS the mutation.
  const handleAuthoringAccepted = useCallback((result: NodeActionTextResult) => {
    if (result.action === 'edit-node') {
      const previous = preEditNodeRef.current;
      preEditNodeRef.current = null;
      if (previous && previous.alias === result.alias) {
        toastWithUndo(`Updated node ${previous.alias}`, graphUndo.push(buildNodeEditUndo(previous)));
      }
      return;
    }
    if (result.action === 'create-node' && result.alias) {
      toastWithUndo(`Created node ${result.alias}`, graphUndo.push(buildNodeCreateUndo(result.alias)));
      return;
    }
    if (result.action === 'create-connection') {
      const entry = pendingConnectionUndoRef.current;
      pendingConnectionUndoRef.current = null;
      toastWithUndo(
        `Connected ${result.alias ?? ''} → ${result.targetAlias ?? ''}`,
        graphUndo.push(entry),
      );
      return;
    }
    if (result.action === 'delete-node' && result.alias) {
      const entry = pendingNodeDeleteUndoRef.current.get(result.alias) ?? null;
      pendingNodeDeleteUndoRef.current.delete(result.alias);
      toastWithUndo(`Deleted node ${result.alias}`, graphUndo.push(entry));
    }
  }, [graphUndo.push, toastWithUndo]);

  // ── Session-bound graph state invalidation ───────────────────────────────
  // Graph API paths are tied to the backend WebSocket session.  When the
  // connection drops, any previously fetched graph data and its API path are
  // invalid for the new session.  Clear both on a real connected→disconnected
  // transition.  The guard ensures this does NOT fire on initial mount when
  // the socket is still in 'idle' or 'connecting' phase.
  const wasConnectedRef = useRef(false);

  useEffect(() => {
    if (wasConnectedRef.current && !ws.connected) {
      setPinnedGraphPath(null);
      setGraphData(null);
    }
    wasConnectedRef.current = ws.connected;
  }, [ws.connected, setPinnedGraphPath, setGraphData]);

  // ── Help panel state ────────────────────────────────────────────────────
  // Persisted last-viewed help topic (empty string = root index).
  const [activeHelpTopic, setActiveHelpTopic] = useLocalStorage<string>(
    config.storageKeyHelpTopic ?? 'help-topic-fallback',
    ''
  );
  const [helpOpen, setHelpOpen] = useLocalStorage<boolean>('help-panel-open', false);

  // ── Help-hint popover (shows on every app load) ─────────────────────────
  const [helpHintVisible, setHelpHintVisible] = useState(
    () => !!supportsHelp && !helpOpen
  );
  const [helpHintFading, setHelpHintFading] = useState(false);
  const helpHintTimerRef = useRef<ReturnType<typeof setTimeout> | null>(null);

  const dismissHelpHint = useCallback(() => {
    if (!helpHintVisible) return;
    setHelpHintFading(true);
    // Remove from DOM after the CSS fade transition completes.
    helpHintTimerRef.current = setTimeout(() => setHelpHintVisible(false), 400);
  }, [helpHintVisible]);

  // Auto-dismiss after 3 seconds.
  useEffect(() => {
    if (!helpHintVisible || helpHintFading) return;
    const id = setTimeout(dismissHelpHint, 3000);
    return () => clearTimeout(id);
  }, [helpHintVisible, helpHintFading, dismissHelpHint]);

  // Dismiss immediately when the help panel opens during the hint window.
  useEffect(() => {
    if (helpOpen && helpHintVisible) dismissHelpHint();
  }, [helpOpen, helpHintVisible, dismissHelpHint]);

  // Cleanup on unmount.
  useEffect(() => {
    return () => { if (helpHintTimerRef.current) clearTimeout(helpHintTimerRef.current); };
  }, []);

  // Ctrl+` toggles the help panel (only for playgrounds that support it).
  useEffect(() => {
    if (!supportsHelp) return;
    const handleKeyDown = (e: KeyboardEvent) => {
      if (e.ctrlKey && e.key === '`') {
        e.preventDefault();
        setHelpOpen((prev: boolean) => !prev);
      }
    };
    window.addEventListener('keydown', handleKeyDown);
    return () => window.removeEventListener('keydown', handleKeyDown);
  }, [supportsHelp, setHelpOpen]);

  useAutoHelpNavigate({
    bus,
    setHelpTopic: setActiveHelpTopic,
    onTabSwitch:  supportsHelp ? () => setHelpOpen(true) : () => {},
  });

  // ── Auto-download large payloads ──────────────────────────────────────────
  // When the server responds with a "Large payload (N) -> GET /api/inspect/…"
  // message (i.e. a namespace value that exceeds the 64 KB inline limit),
  // this hook fetches the data from the provided endpoint and appends the
  // result inline to the console as a collapsible JSON row — no extra user
  // interaction required.
  useLargePayloadDownload({
    bus,
    connected:     ws.connected,
    appendMessage: ws.appendMessage,
    addToast,
  });

  // ── Console panel visibility ─────────────────────────────────────────────
  // Hiding the console gives the right panel the full width, so the graph
  // view can use the whole screen. Persisted like the other panel toggles.
  const [consoleOpen, setConsoleOpen] = useLocalStorage<boolean>('console-panel-open', true);

  // ── Clipboard integration ────────────────────────────────────────────────
  const clipboardCtx = useClipboardContext();

  const [clipboardOpen, setClipboardOpen] = useLocalStorage<boolean>('clipboard-sidebar-open', false);

  const [duplicateDialogState, setDuplicateDialogState] = useState<{
    pendingItem: ClipboardItemRecord;
    existingItem: ClipboardItemRecord;
  } | null>(null);

  const handlePasteToInput = useCallback((item: ClipboardItemRecord) => {
    const plan = buildClipboardPastePlan(item, graphData);
    ws.setCommand(plan.command);
    addToast(`${plan.verb === 'create' ? 'Create' : 'Update'} command for "${item.node.alias}" pasted to input`, 'info');
  }, [graphData, ws.setCommand, addToast]);

  const handleClipboardDrop = useCallback((itemId: string) => {
    const item = clipboardCtx.items.find(entry => entry.id === itemId);
    if (!item) {
      addToast('Clipboard item is no longer available. It may have been removed in another tab.', 'error');
      return;
    }

    const plan = buildClipboardPastePlan(item, graphData);
    if (!ws.sendRawText(plan.command)) {
      addToast('Could not send clipboard paste command because the WebSocket is not open.', 'error');
      return;
    }

    addToast(`Clipboard node "${item.node.alias}" sent as ${plan.verb}. Waiting for backend response.`, 'info');
  }, [clipboardCtx.items, graphData, ws.sendRawText, addToast]);

  const handleClipNode = useCallback(async (
    node: MinigraphNode,
    connections: MinigraphConnection[],
  ) => {
    try {
      const result = await clipboardCtx.clipNode(node, connections, {
        sourceWsPath: wsPath,
        sourceLabel: config.label,
      });

      switch (result.status) {
        case 'added':
          addToast(`Node "${node.alias}" clipped to workspace`, 'success');
          break;
        case 'duplicate':
          setDuplicateDialogState({
            pendingItem: result.pendingItem,
            existingItem: result.existingItem,
          });
          break;
        case 'error':
          addToast(`Clip failed: ${result.message}`, 'error');
          break;
      }
    } catch (err) {
      addToast(`Clip failed: ${err instanceof Error ? err.message : String(err)}`, 'error');
    }
  }, [clipboardCtx, wsPath, config.label, addToast]);

  const handleClipNodes = useCallback(async (items: GraphClipItem[]) => {
    if (items.length === 0) {
      addToast('No selected nodes are available to clip.', 'info');
      return;
    }
    if (items.length > MAX_BATCH_NODE_ACTIONS) {
      addToast('Select 100 or fewer nodes to clip at once.', 'error');
      return;
    }

    const summary = { added: 0, duplicates: 0, failed: 0 };
    for (const item of items) {
      if (!item.node?.alias?.trim()) {
        summary.failed += 1;
        continue;
      }
      try {
        const result = await clipboardCtx.clipNode(item.node, item.connections, {
          sourceWsPath: wsPath,
          sourceLabel: config.label,
        });
        if (result.status === 'added') summary.added += 1;
        if (result.status === 'duplicate') summary.duplicates += 1;
        if (result.status === 'error') summary.failed += 1;
      } catch {
        summary.failed += 1;
      }
    }

    const toast = buildBatchClipToast(summary);
    addToast(toast.message, toast.type);
  }, [addToast, clipboardCtx, config.label, wsPath]);

  // ── Saved graphs (localStorage snapshots) ────────────────────────────────
  // Only instantiated when the playground config provides a storage key so
  // playgrounds that don't use this feature have zero overhead.
  const savedGraphs = useSavedGraphs(storageKeySavedGraphs ?? '');

  // ── Save-form default name ────────────────────────────────────────────────
  // Tracks the pre-fill name for the GraphSaveButton input with priority:
  //   1. last-saved name (if this working graph was previously saved)
  //   2. imported name   (if the graph was loaded via `import graph from …`)
  //   3. untitled-{n}    (monotonically incrementing per-playground fallback)
  //
  // The untitled counter key is derived from the saved-graphs key so it stays
  // isolated per playground (e.g. "minigraph-saved-graphs" →
  // "minigraph-untitled-counter").  When there is no saved-graphs key the hook
  // is still instantiated but is effectively unused (the save button is hidden).
  const { defaultName: graphSaveName, savedName: graphSavedName, resetName: resetSaveName } = useGraphSaveName(
    storageKeySavedGraphs ? `${storageKeySavedGraphs}-untitled-counter` : 'untitled-counter',
    bus,
    ws.connected,
    ws.connectionEpoch,
  );

  // ── Resolved graph display name ────────────────────────────────────────────
  // Priority: root node's "name" property (from graph data) → defaultName from
  // the save-name hook (lastSavedName → importedName → untitled-{n}).
  const reliableGraphName = useMemo(() => {
    const rootNode = graphData?.nodes.find(n => n.types.includes('Root'));
    const rootName = typeof rootNode?.properties?.name === 'string' ? rootNode.properties.name : undefined;
    return rootName?.trim() ? rootName : null;
  }, [graphData]);

  const graphDisplayName = reliableGraphName ?? graphSaveName;

  // ── Graph authoring ───────────────────────────────────────────────────────
  const graphAuthoringExecutor = useMemo(
    () => createGraphAuthoringExecutor(ws.sendRawText),
    [ws.sendRawText],
  );

  const graphAuthoring = useGraphAuthoring({
    bus,
    connected: ws.connected,
    graphData,
    executor: graphAuthoringExecutor,
    onAccepted: handleAuthoringAccepted,
    onUserMessage: addToast,
  });

  // ── In-place node authoring panel ─────────────────────────────────────────
  // An open create-node or edit-node session takes over the left panel slot
  // (the console's space) with the magnified-node editor — the console hides
  // while authoring. consoleOpen itself is never touched, so Esc / Cancel / a
  // successful submit simply unmounts the editor and the slot returns to its
  // previous state (console back if it was open, full-width graph if hidden).
  const authoringState = graphAuthoring.state;
  const nodeEditSession =
    supportsAuthoring && authoringState.status === 'open' &&
    (authoringState.action === 'edit-node' || authoringState.action === 'create-node')
      ? authoringState
      : null;

  // ── Inline connection popover ─────────────────────────────────────────────
  // A create-connection session renders as a popover anchored at the connect
  // gesture's drop point (captured by GraphView), not as a modal.
  const connectionSession =
    supportsAuthoring && authoringState.status === 'open' && authoringState.action === 'create-connection'
      ? authoringState
      : null;
  const [connectionAnchor, setConnectionAnchor] = useState<{ x: number; y: number } | null>(null);

  // Ctrl/Cmd+Z undoes the newest tracked mutation — but never while typing
  // (native text undo) or while an authoring session is open.
  const authoringSessionOpen = nodeEditSession !== null || connectionSession !== null;
  useEffect(() => {
    const handleKeyDown = (event: KeyboardEvent) => {
      if (event.key !== 'z' && event.key !== 'Z') return;
      if (!(event.metaKey || event.ctrlKey) || event.shiftKey || event.altKey) return;
      const target = event.target;
      if (target instanceof Element && target.closest('input, textarea, select, [contenteditable="true"]')) return;
      if (authoringSessionOpen) return;
      if (!graphUndo.hasEntries()) return;
      event.preventDefault();
      graphUndo.undoLast();
    };
    window.addEventListener('keydown', handleKeyDown);
    return () => window.removeEventListener('keydown', handleKeyDown);
  }, [authoringSessionOpen, graphUndo.hasEntries, graphUndo.undoLast]);
  const handleOpenCreateConnection = useCallback(
    (sourceAlias: string, targetAlias: string, anchor?: { x: number; y: number }) => {
      setConnectionAnchor(anchor ?? null);
      // Snapshot the pair's pre-create relations: the inverse must delete the
      // pair and re-add exactly these.
      pendingConnectionUndoRef.current = graphData
        ? buildConnectionCreateUndo(graphData, sourceAlias, targetAlias)
        : null;
      graphAuthoring.openCreateConnection(sourceAlias, targetAlias);
    },
    [graphAuthoring.openCreateConnection, graphData],
  );

  // Connection removal (keyboard Delete on selected edges, or a relation item
  // from the edge context menu): the planner turns the requested DIRECTED
  // removals into backend commands — the pair-wiping `delete connection` plus
  // reconnects for every survivor — sent paced, and the graph redraws from the
  // backend confirmations via the auto-refresh. The removed relations become
  // the undo recipe.
  const handleDeleteConnections = useCallback((requests: ConnectionRemovalRequest[]) => {
    if (!graphData) return;
    const plan = planConnectionRemoval(graphData, requests);
    if (!plan) return; // stale gesture — nothing matches the current graph
    graphUndo.runCommands(plan.commands);
    toastWithUndo(
      `Deleted ${describeRemovedRelations(plan.removed)}`,
      graphUndo.push(buildConnectionRemovalUndo(plan.removed)),
    );
  }, [graphData, graphUndo.push, graphUndo.runCommands, toastWithUndo]);

  // Wraps that capture pre-mutation snapshots before delegating to the
  // authoring hook; the matching undo entry is pushed in onAccepted.
  const handleOpenEditNode = useCallback((node: MinigraphNode) => {
    preEditNodeRef.current = node;
    graphAuthoring.openEditNode(node);
  }, [graphAuthoring.openEditNode]);

  const handleDeleteNode = useCallback((node: MinigraphNode) => {
    pendingNodeDeleteUndoRef.current.set(
      node.alias,
      graphData ? buildNodeDeleteUndo(graphData, node) : null,
    );
    graphAuthoring.deleteNode(node);
  }, [graphAuthoring.deleteNode, graphData]);

  // Restore the scroll position when the console re-mounts (console toggle or
  // node editor closing): the message-driven auto-scroll in useWebSocket only
  // fires on new messages, so an unchanged backlog would otherwise reappear
  // scrolled to the top.
  const consoleRef = ws.consoleRef;
  const consoleVisible = consoleOpen && nodeEditSession === null;
  useEffect(() => {
    if (consoleVisible && consoleRef.current) {
      consoleRef.current.scrollTop = consoleRef.current.scrollHeight;
    }
  }, [consoleVisible, consoleRef]);

  // ── Session collaboration ────────────────────────────────────────────────
  // The Session dropdown is rendered from Navigation, but this hook is created
  // here because Playground owns the Minigraph ProtocolBus and classification map.
  const sessionCollaboration = useSessionCollaboration({
    enabled: supportsSessionCollaboration === true,
    bus,
    classificationMap,
    connected: ws.connected,
    sendRawText: ws.sendRawText,
    addToast,
  });

  // ── Saved graph workflow ──────────────────────────────────────────────────
  // Note: must be called AFTER useAutoGraphRefresh — both listen on graph.link
  // and ProtocolBus fires listeners in insertion order.
  const { handleSaveGraph, handleLoadGraph } = useSavedGraphWorkflow({
    bus,
    connected:    ws.connected,
    sendRawText:  ws.sendRawText,
    saveGraph:    storageKeySavedGraphs ? savedGraphs.saveGraph : null,
    addToast,
  });

  // When a console graph-link row is clicked, load the referenced graph.
  const handleGraphLinkMessage = useCallback((msg: { id: number; raw: string }) => {
    const events = classificationMap.get(msg.id);
    const graphLink = events?.find(e => e.kind === 'graph.link') as GraphLinkEvent | undefined;
    if (graphLink) {
      setPinnedGraphPath(graphLink.apiPath);
    }
  }, [classificationMap]);

  // ── Cross-playground send to JSON-Path ──────────────────────────────────
  const { handleSendToJsonPath } = useSendToJsonPath({
    ctx, navigate, addToast, wsPath,
  });

  // Responsive layout: stack panels vertically on narrow viewports
  const isMobile = useMediaQuery('(max-width: 768px)');

  // Persist panel split ratio per playground route
  const { defaultLayout, onLayoutChanged } = useDefaultLayout({
    id: config.path + '-panel-split',
    storage: localStorage,
  });

  const handleFormatPayload = useCallback(() => setPayload(formatJSON(payload)), [payload]);

  const handleClearMessages = useCallback(() => {
    ws.clearMessages();
    setPinnedGraphPath(null);
    setGraphData(null);
    // Reset mock-upload session state so ✅ badges clear with the console.
    // Modal upload path is NOT reset here — if the modal is open while the
    // user clears the console, it remains open for the current upload attempt.
    resetSuccessfulPaths();
    // Advance the untitled counter so the next saved graph gets a fresh name.
    resetSaveName();
  }, [ws.clearMessages, setGraphData, resetSuccessfulPaths, resetSaveName]);

  return (
    <div className={styles.wrapper}>
      <ToastContainer toasts={toasts} onRemove={removeToast} />

      {modalUploadPath && (
        <MockUploadModal
          uploadPath={modalUploadPath}
          onSuccess={handleUploadSuccess}
          onClose={handleCloseUploadModal}
          onError={handleUploadError}
        />
      )}

      {connectionSession !== null && (
        <ConnectionPopover
          formState={connectionSession.formState}
          phase={connectionSession.phase}
          lockReason={
            connectionSession.phase === 'sending'
              ? 'sending'
              : connectionSession.connectionLost
                ? 'disconnected'
                : null
          }
          serverMessage={connectionSession.serverMessage}
          validationErrors={graphAuthoring.validationErrors}
          anchor={connectionAnchor}
          onFormStateChange={graphAuthoring.updateFormState}
          onSubmit={graphAuthoring.submit}
          onClose={graphAuthoring.close}
        />
      )}

      <header className={styles.header}>
        <h1 className={styles.title}>{title}</h1>
        <div className={styles.headerActions}>
          {storageKeySavedGraphs && (
            <GraphSaveButton
              disabled={!graphData}
              defaultName={graphSaveName}
              savedName={graphSavedName}
              onSave={handleSaveGraph}
              nameExists={savedGraphs.hasGraph}
              connected={ws.connected}
            />
          )}
          {storageKeySavedGraphs && savedGraphs.savedGraphs.length > 0 && (
            <SavedGraphsMenu
              savedGraphs={savedGraphs.savedGraphs}
              onLoad={handleLoadGraph}
              onDelete={savedGraphs.deleteGraph}
              connected={ws.connected}
            />
          )}
          <button
            className={styles.panelToggle}
            onClick={() => {
              // While the node editor occupies the console's slot, this button
              // means "give me the console back": close the editor (same
              // discard semantics as Esc/Cancel) and show the console. A save
              // in flight blocks closing, exactly like Esc/Cancel.
              if (nodeEditSession !== null) {
                if (nodeEditSession.phase !== 'sending') {
                  graphAuthoring.close();
                  setConsoleOpen(true);
                }
                return;
              }
              setConsoleOpen(prev => !prev);
            }}
            aria-label={nodeEditSession !== null
              ? 'Show console panel and close the node editor'
              : consoleOpen ? 'Hide console panel' : 'Show console panel'}
            aria-pressed={consoleVisible}
            title={nodeEditSession !== null ? 'Closes the node editor' : undefined}
          >
            Console
          </button>
          {supportsClipboard && (
            <button
              className={styles.panelToggle}
              onClick={() => setClipboardOpen(prev => !prev)}
              aria-label={clipboardOpen ? 'Close workspace sidebar' : 'Open workspace sidebar'}
              aria-pressed={clipboardOpen}
            >
              Workspace{clipboardCtx.items.length > 0 ? ` (${clipboardCtx.items.length})` : ''}
            </button>
          )}
          <Navigation
            addToast={addToast}
            sessionCollaboration={supportsSessionCollaboration ? sessionCollaboration : null}
          />
          {supportsHelp && (
            <div className={styles.helpButtonWrapper}>
              <button
                className={`${styles.helpToggle}${helpHintVisible && !helpHintFading ? ` ${styles.helpTogglePulsing}` : ''}`}
                onClick={() => setHelpOpen(prev => !prev)}
                aria-label={helpOpen ? 'Close help panel' : 'Open help panel'}
                aria-pressed={helpOpen}
              >
                ?
              </button>
              {helpHintVisible && (
                <div
                  className={`${styles.helpHint}${helpHintFading ? ` ${styles.helpHintFading}` : ''}`}
                  onClick={dismissHelpHint}
                  role="status"
                >
                  <kbd className={styles.helpHintKbd}>Ctrl + `</kbd> to toggle help
                </div>
              )}
            </div>
          )}
        </div>
      </header>

      {duplicateDialogState && (
        <ClipboardDuplicateDialog
          existingItem={duplicateDialogState.existingItem}
          pendingItem={duplicateDialogState.pendingItem}
          onReplace={async () => {
            try {
              await clipboardCtx.confirmReplace(
                duplicateDialogState.pendingItem,
                duplicateDialogState.existingItem.id,
              );
              setDuplicateDialogState(null);
              addToast(`Clipboard item "${duplicateDialogState.pendingItem.node.alias}" replaced`, 'success');
            } catch (err) {
              addToast(`Replace failed: ${err instanceof Error ? err.message : String(err)}`, 'error');
            }
          }}
          onCancel={() => {
            setDuplicateDialogState(null);
            addToast('Clip cancelled', 'info');
          }}
        />
      )}

      <Group
        className={styles.panelGroup}
        orientation={isMobile ? 'vertical' : 'horizontal'}
        defaultLayout={defaultLayout}
        onLayoutChanged={onLayoutChanged}
      >
        {(consoleOpen || nodeEditSession !== null) && (
          <>
            <Panel defaultSize={(helpOpen || clipboardOpen) ? "50%" : "60%"} minSize="25%">
              {nodeEditSession !== null ? (
                <NodeEditPanel
                  mode={nodeEditSession.action === 'edit-node' ? 'edit' : 'create'}
                  formState={nodeEditSession.formState}
                  phase={nodeEditSession.phase}
                  lockReason={
                    nodeEditSession.phase === 'sending'
                      ? 'sending'
                      : nodeEditSession.connectionLost
                        ? 'disconnected'
                        : null
                  }
                  serverMessage={nodeEditSession.serverMessage}
                  validationErrors={graphAuthoring.validationErrors}
                  onFormStateChange={graphAuthoring.updateFormState}
                  onSubmit={graphAuthoring.submit}
                  onClose={graphAuthoring.close}
                />
              ) : (
                <LeftPanel
                  messages={ws.messages}
                  classificationMap={classificationMap}
                  onCopy={ws.copyMessages}
                  onClear={handleClearMessages}
                  consoleRef={ws.consoleRef}
                  command={ws.command}
                  onCommandChange={ws.setCommand}
                  onCommandKeyDown={ws.handleKeyDown}
                  onSend={ws.sendCommand}
                  sendDisabled={!ws.connected || !ws.command.trim()}
                  inputDisabled={!ws.connected}
                  commandHistory={ws.history}
                  onGraphLinkMessage={handleGraphLinkMessage}
                  onCopyMessage={() => addToast('Copied to clipboard', 'success')}
                  onSendToJsonPath={handleSendToJsonPath}
                  onUploadMockData={handleOpenUploadModal}
                  successfulUploadPaths={successfulUploadPaths}
                />
              )}
            </Panel>
            <Separator className={styles.resizeHandle} aria-label="Resize panels" />
          </>
        )}
        <Panel defaultSize={helpOpen ? "50%" : clipboardOpen ? "30%" : "40%"} minSize="20%">
          <RightPanel
            tabs={tabs}
            payload={payload}
            onChange={setPayload}
            validation={payloadValidation}
            onFormat={handleFormatPayload}
            onUpload={supportsUpload ? ws.uploadPayload : undefined}
            graphData={graphData}
            graphName={graphDisplayName}
            activeTab={rightTab}
            onTabChange={setRightTab}
            onGraphRenderError={(msg) => addToast(msg, 'error')}
            onGraphDataCopySuccess={() => addToast('Graph JSON copied to clipboard!', 'success')}
            onGraphDataCopyError={() => addToast('Copy failed', 'error')}
            isGraphRefreshing={isRefreshing}
            onClipNode={supportsClipboard ? handleClipNode : undefined}
            onClipNodes={supportsClipboard ? handleClipNodes : undefined}
            onClipboardDrop={supportsClipboard ? handleClipboardDrop : undefined}
            isConnected={ws.connected}
            supportsAuthoring={supportsAuthoring}
            onCreateNode={supportsAuthoring ? graphAuthoring.openCreateNode : undefined}
            onCreateConnection={supportsAuthoring ? handleOpenCreateConnection : undefined}
            onEditNode={supportsAuthoring ? handleOpenEditNode : undefined}
            onDeleteNode={supportsAuthoring ? handleDeleteNode : undefined}
            onDeleteNodes={supportsAuthoring ? graphAuthoring.deleteNodes : undefined}
            onDeleteConnections={supportsAuthoring ? handleDeleteConnections : undefined}
            panelLayoutKey={`${consoleOpen || nodeEditSession !== null}|${clipboardOpen}|${helpOpen}`}
            helpPanel={supportsHelp && helpOpen ? (
              (onToggleMaximize: () => void, isMaximized: boolean) => (
                <HelpBrowser
                  activeTopic={activeHelpTopic}
                  onNavigate={setActiveHelpTopic}
                  onClose={() => setHelpOpen(false)}
                  onToggleMaximize={onToggleMaximize}
                  isMaximized={isMaximized}
                />
              )
            ) : undefined}
          />
        </Panel>
        {supportsClipboard && clipboardOpen && (
          <>
            <Separator className={styles.resizeHandle} aria-label="Resize clipboard" />
            <Panel defaultSize="20%" minSize="10%" maxSize="40%">
              <ClipboardSidebar
                connected={ws.connected}
                onPasteToInput={handlePasteToInput}
              />
            </Panel>
          </>
        )}
      </Group>
    </div>
  );
}
