import { useId, useState, useCallback, useRef, useEffect, type ReactNode } from 'react';
import { Group, Panel, Separator, type PanelImperativeHandle } from 'react-resizable-panels';
import PayloadEditor from '../PayloadEditor/PayloadEditor';
import GraphView from '../GraphView/GraphView';
import GraphDataView from '../GraphDataView/GraphDataView';
import styles from './RightPanel.module.css';
import { type ValidationResult } from '../../utils/validators';
import type { MinigraphGraphData, MinigraphNode, MinigraphConnection } from '../../utils/graphTypes';

export type RightTab = 'payload' | 'graph' | 'graph-data';

interface RightPanelProps {
  /** Ordered list of tabs to render for this playground (from PlaygroundConfig). */
  tabs:           RightTab[];
  payload:        string;
  onChange:       (value: string) => void;
  validation:     ValidationResult;
  onFormat:       () => void;
  onUpload?:      () => void;
  graphData:      MinigraphGraphData | null;
  /** Resolved display name for the graph (shown in toolbar). */
  graphName?:     string;
  activeTab:      RightTab;
  onTabChange:    (tab: RightTab) => void;
  onGraphRenderError?: (message: string) => void;
  /** Called after the raw graph JSON is successfully copied from the Graph Data tab. */
  onGraphDataCopySuccess?: () => void;
  /** Called when the clipboard write fails from the Graph Data tab. */
  onGraphDataCopyError?:   () => void;
  /** When true, forwards the loading-overlay state to GraphView. */
  isGraphRefreshing?:      boolean;
  /** Callback for "Clip to Clipboard" from the node context menu in GraphView. */
  onClipNode?:             (node: MinigraphNode, connections: MinigraphConnection[]) => void;
  onClipboardDrop?:        (itemId: string) => void;
  isConnected:             boolean;
  supportsAuthoring?:      boolean;
  onCreateNode?:           (source: 'empty-graph' | 'pane-context-menu') => void;
  onEditNode?:             (node: MinigraphNode) => void;
  onDeleteNode?:           (node: MinigraphNode) => void;
  /**
   * When provided and non-null, the right panel renders a vertical split:
   * top = tab content, bottom = help panel.  Accepts either a plain ReactNode
   * or a render function that receives a maximize-toggle callback — the
   * callback lets the help panel request 100% / default height.
   */
  helpPanel?:              ReactNode | ((onToggleMaximize: () => void, isMaximized: boolean) => ReactNode);
}

// ── Help panel split constants (module-level: no closure dependencies) ──────
const STORAGE_KEY      = 'help-split-percent';
const MAXIMIZED_KEY    = 'help-split-maximized';
const DEFAULT_HELP_PCT = 45;
/** Threshold for treating a manual drag as "effectively maximized". */
const MAXIMIZE_THRESHOLD = 98;

export default function RightPanel({
  tabs,
  payload,
  onChange,
  validation,
  onFormat,
  onUpload,
  graphData,
  graphName,
  activeTab,
  onTabChange,
  onGraphRenderError,
  onGraphDataCopySuccess,
  onGraphDataCopyError,
  isGraphRefreshing,
  onClipNode,
  onClipboardDrop,
  isConnected,
  supportsAuthoring,
  onCreateNode,
  onEditNode,
  onDeleteNode,
  helpPanel,
}: RightPanelProps) {
  const uid              = useId();
  const payloadPanelId   = `${uid}-tab-payload`;
  const graphPanelId     = `${uid}-tab-graph`;
  const graphDataPanelId = `${uid}-tab-graph-data`;

  const tabContent = (
    <div className={styles.rightPanel}>
      {/* Tab strip — only tabs listed in `tabs` are rendered */}
      <div className={styles.tabStrip} role="tablist" aria-label="Right panel tabs">
        {tabs.includes('payload') && (
          <button
            role="tab"
            aria-selected={activeTab === 'payload'}
            aria-controls={payloadPanelId}
            className={`${styles.tab}${activeTab === 'payload' ? ` ${styles.tabActive}` : ''}`}
            onClick={() => onTabChange('payload')}
          >
            Payload Editor
          </button>
        )}
        {tabs.includes('graph') && (
          <button
            role="tab"
            aria-selected={activeTab === 'graph'}
            aria-controls={graphPanelId}
            className={`${styles.tab}${activeTab === 'graph' ? ` ${styles.tabActive}` : ''}`}
            onClick={() => onTabChange('graph')}
          >
            Graph
            {graphData !== null && (
              <span className={styles.tabBadge} aria-label="Graph data available">🕸️</span>
            )}
          </button>
        )}
        {tabs.includes('graph-data') && (
          <button
            role="tab"
            aria-selected={activeTab === 'graph-data'}
            aria-controls={graphDataPanelId}
            className={`${styles.tab}${activeTab === 'graph-data' ? ` ${styles.tabActive}` : ''}`}
            onClick={() => onTabChange('graph-data')}
          >
            Graph Data (Raw)
          </button>
        )}
      </div>

      {/* Payload Editor tab body — only mounted when enabled for this playground */}
      {tabs.includes('payload') && (
        <div
          role="tabpanel"
          id={payloadPanelId}
          tabIndex={activeTab === 'payload' ? 0 : -1}
          className={`${styles.tabBody}${activeTab !== 'payload' ? ` ${styles.tabBodyHidden}` : ''}`}
        >
          <PayloadEditor
            payload={payload}
            onChange={onChange}
            validation={validation}
            onFormat={onFormat}
            onUpload={onUpload}
          />
        </div>
      )}

      {/* Graph View tab body — always mounted when enabled to preserve zoom/pan state */}
      {tabs.includes('graph') && (
        <div
          role="tabpanel"
          id={graphPanelId}
          tabIndex={activeTab === 'graph' ? 0 : -1}
          className={`${styles.tabBody}${activeTab !== 'graph' ? ` ${styles.tabBodyHidden}` : ''}`}
        >
          <div className={styles.graphContent}>
            <GraphView
              graphData={graphData}
              graphName={graphName}
              onRenderError={onGraphRenderError}
              isRefreshing={isGraphRefreshing}
              onCopySuccess={onGraphDataCopySuccess}
              onCopyError={onGraphDataCopyError}
              onClipNode={onClipNode}
              onClipboardDrop={onClipboardDrop}
              isConnected={isConnected}
              supportsAuthoring={supportsAuthoring}
              onCreateNode={onCreateNode}
              onEditNode={onEditNode}
              onDeleteNode={onDeleteNode}
            />
          </div>
        </div>
      )}

      {/* Graph Data tab body — always mounted when enabled; shows pretty-printed raw JSON */}
      {tabs.includes('graph-data') && (
        <div
          role="tabpanel"
          id={graphDataPanelId}
          tabIndex={activeTab === 'graph-data' ? 0 : -1}
          className={`${styles.tabBody}${activeTab !== 'graph-data' ? ` ${styles.tabBodyHidden}` : ''}`}
        >
          <GraphDataView
            graphData={graphData}
            graphName={graphName}
            onCopySuccess={onGraphDataCopySuccess}
            onCopyError={onGraphDataCopyError}
          />
        </div>
      )}

    </div>
  );

  // ── Help panel split persistence + maximize/restore ──────────────────
  // Track the user's preferred help split so it survives:
  //   • help open/close toggles (ref persists — RightPanel stays mounted)
  //   • playground navigation   (sessionStorage survives Playground remount)
  // sessionStorage clears on tab close, matching server-session lifetime.
  const helpSizeRef = useRef(
    Number(sessionStorage.getItem(STORAGE_KEY)) || DEFAULT_HELP_PCT
  );

  const helpPanelRef = useRef<PanelImperativeHandle | null>(null);
  const tabPanelRef  = useRef<PanelImperativeHandle | null>(null);

  // Restore maximized state from sessionStorage so close/reopen preserves it.
  const [helpMaximized, setHelpMaximized] = useState(
    () => sessionStorage.getItem(MAXIMIZED_KEY) === '1'
  );
  const helpMaximizedRef = useRef(helpMaximized);

  const handleHelpSplitChanged = useCallback((layout: Record<string, number>) => {
    const helpSize = layout['help-split-help'];
    if (helpSize === undefined) return;

    // Sync maximized flag with actual panel size so the button icon stays
    // consistent even when the user drags to 100% manually.
    const effectivelyMaximized = helpSize >= MAXIMIZE_THRESHOLD;

    if (effectivelyMaximized !== helpMaximizedRef.current) {
      helpMaximizedRef.current = effectivelyMaximized;
      setHelpMaximized(effectivelyMaximized);
      sessionStorage.setItem(MAXIMIZED_KEY, effectivelyMaximized ? '1' : '0');
    }

    // Only persist the user's preferred resting size (not the transient
    // maximized split) so restore returns to a sensible height.
    if (!effectivelyMaximized) {
      helpSizeRef.current = helpSize;
      sessionStorage.setItem(STORAGE_KEY, String(helpSize));
    }
  }, []);

  const toggleHelpMaximize = useCallback(() => {
    const next = !helpMaximizedRef.current;
    helpMaximizedRef.current = next;
    setHelpMaximized(next);
    sessionStorage.setItem(MAXIMIZED_KEY, next ? '1' : '0');
    if (next) {
      tabPanelRef.current?.resize('0%');
      helpPanelRef.current?.resize('100%');
    } else {
      const restored = helpSizeRef.current;
      helpPanelRef.current?.resize(`${restored}%`);
      tabPanelRef.current?.resize(`${100 - restored}%`);
    }
  }, []);

  // When the help panel reopens in maximized state, the Group remounts with
  // defaultSize from helpSizeRef (the resting size).  Imperatively resize to
  // 100% after mount so the visual state matches the persisted flag.
  const helpPanelActive = !!helpPanel;
  useEffect(() => {
    if (helpPanelActive && helpMaximizedRef.current) {
      // Defer to next frame so the panel refs are populated after mount.
      requestAnimationFrame(() => {
        tabPanelRef.current?.resize('0%');
        helpPanelRef.current?.resize('100%');
      });
    }
  }, [helpPanelActive]);

  // When no help panel is active, render the tab content alone.
  if (!helpPanel) return tabContent;

  const resolvedHelpPanel =
    typeof helpPanel === 'function'
      ? helpPanel(toggleHelpMaximize, helpMaximized)
      : helpPanel;

  // Vertical split: tab content on top, help panel on bottom.
  // defaultSize reads from the ref (initialized from sessionStorage).
  // When the persisted state is maximized, mount at 100%/0% directly.
  const mountMaximized = helpMaximizedRef.current;
  const helpDefault = mountMaximized ? 100 : helpSizeRef.current;
  const tabDefault  = 100 - helpDefault;

  return (
    <Group
      orientation="vertical"
      className={styles.rightPanelGroup}
      onLayoutChanged={handleHelpSplitChanged}
    >
      <Panel panelRef={tabPanelRef} defaultSize={`${tabDefault}%`} minSize="0%">
        {tabContent}
      </Panel>
      <Separator className={styles.verticalResizeHandle} aria-label="Resize help panel" />
      <Panel id="help-split-help" panelRef={helpPanelRef} defaultSize={`${helpDefault}%`} minSize="15%">
        {resolvedHelpPanel}
      </Panel>
    </Group>
  );
}
