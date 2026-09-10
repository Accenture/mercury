import { useState, useEffect, useRef, useCallback } from 'react';
import { isMinigraphGraphData, type MinigraphGraphData } from '../utils/graphTypes';
import { type ToastType } from './useToast';
import { type RightTab } from '../components/RightPanel/RightPanel';
import { useLocalStorage } from './useLocalStorage';

export function normalizeRightTab(
  value: unknown,
  validTabs: readonly RightTab[],
  fallbackTab: RightTab,
): RightTab {
  const safeFallback = validTabs.includes(fallbackTab)
    ? fallbackTab
    : validTabs[0] ?? 'graph';

  if (typeof value === 'string' && validTabs.includes(value as RightTab)) {
    return value as RightTab;
  }

  return safeFallback;
}

/**
 * A session with no graph content yet (a fresh session exports an empty
 * graph) renders the canvas empty state, not an empty canvas — treat a
 * zero-node payload the same as "no graph".
 */
function toRenderableGraph(json: unknown): MinigraphGraphData | null {
  return isMinigraphGraphData(json) && json.nodes.length > 0 ? json : null;
}

export interface UseGraphDataReturn {
  graphData:    MinigraphGraphData | null;
  setGraphData: React.Dispatch<React.SetStateAction<MinigraphGraphData | null>>;
  rightTab:     RightTab;
  setRightTab:  React.Dispatch<React.SetStateAction<RightTab>>;
  /** True while an auto-refresh re-fetch is in-flight (NOT set during initial load). */
  isRefreshing: boolean;
  /**
   * Imperatively re-fetch the live session graph.
   * - Does NOT null graphData while in-flight — stale graph remains visible under the overlay.
   * - Reveals the Graph tab only when the fetch delivers a graph and none was shown before.
   * - Sets isRefreshing = true while the fetch is in-flight.
   * - Stable reference (empty dep array) — safe to include in useEffect dep arrays.
   */
  refetchGraph: () => void;
}

/**
 * Manages all graph-data state for the Playground.
 *
 * The graph view's single source is the live session endpoint
 * (`GET /api/graph/session/{id}`) — the graph as the backend session holds it
 * right now. The described temp-model links (`/api/graph/model/…`) remain a
 * human-operator surface (`describe graph` / `export graph` console output);
 * the UI no longer round-trips through the temp file system to render.
 *
 *  Initial-load path (triggered by sessionGraphPath changing — the session id
 *  arrives asynchronously via the `session` round-trip on mount, and again
 *  after SPA navigation back to the playground):
 *   - Clears graphData while the fetch is in-flight (a new path means a new
 *     session — any previous graph belongs to another lifetime).
 *   - QUIET on failure or empty content: a fresh session (empty graph) and an
 *     unknown/closed session (HTTP 404) are normal lifecycles, not errors —
 *     the canvas empty state is the honest outcome.
 *   - Auto-switches to the Graph tab only when a graph is actually delivered
 *     (the SPA-return restore moment).
 *   - Cancels in-flight requests on path change or unmount.
 *
 *  Auto-refresh path (refetchGraph(), called after graph mutations):
 *   - Does NOT clear graphData — stale graph stays visible under the overlay.
 *   - Reveals the Graph tab when the fetch delivers the first graph.
 *   - A zero-node result clears the view (e.g. the last node was deleted).
 *   - Failures toast: a mutation just happened, so the session should be live.
 *   - Cancels the previous in-flight request if called again.
 *
 * @param sessionGraphPath  Relative API path e.g. `/api/graph/session/ws-123-4`,
 *                          or null until the session id is known.
 * @param addToast          Toast callback from the parent's useToast hook.
 * @param initialTab        The tab to show when no persisted selection exists.
 *                          Should be the first entry in the playground's `tabs` config.
 * @param validTabs         The set of tabs currently rendered for this playground.
 *                          Used to normalize stale persisted values (e.g. a tab
 *                          removed in a later UI version) before render.
 * @param storageKeyTab     localStorage key for persisting the selected tab across
 *                          navigation. Each playground supplies its own key so
 *                          selections are independent and survive page refreshes.
 */
export function useGraphData(
  sessionGraphPath: string | null,
  addToast: (message: string, type?: ToastType) => void,
  initialTab: RightTab,
  validTabs: readonly RightTab[],
  storageKeyTab: string,
): UseGraphDataReturn {
  const [graphData, setGraphData] = useState<MinigraphGraphData | null>(null);
  // useLocalStorage re-reads from storage whenever `storageKeyTab` changes
  // (playground switch), so the correct persisted tab is restored immediately
  // without any additional synchronisation effect.
  const [storedRightTab, setStoredRightTab] = useLocalStorage<RightTab | string>(storageKeyTab, initialTab);
  const rightTab = normalizeRightTab(storedRightTab, validTabs, initialTab);
  const [isRefreshing, setIsRefreshing] = useState(false);

  const setRightTab = useCallback<React.Dispatch<React.SetStateAction<RightTab>>>(
    (value) => {
      setStoredRightTab((prev) => {
        const normalizedPrev = normalizeRightTab(prev, validTabs, initialTab);
        const nextValue = typeof value === 'function'
          ? value(normalizedPrev)
          : value;
        return normalizeRightTab(nextValue, validTabs, initialTab);
      });
    },
    [setStoredRightTab, validTabs, initialTab],
  );

  // Persist a normalized tab value back to localStorage so legacy entries
  // like "preview" are migrated after the first render.
  useEffect(() => {
    if (storedRightTab !== rightTab) {
      setStoredRightTab(rightTab);
    }
  }, [storedRightTab, rightTab, setStoredRightTab]);

  // Keep a ref in sync with the prop so that refetchGraph() (which has an
  // empty dep array) always reads the latest path rather than a stale closure.
  const sessionGraphPathRef = useRef<string | null>(sessionGraphPath);
  useEffect(() => {
    sessionGraphPathRef.current = sessionGraphPath;
  }, [sessionGraphPath]);

  // refetchGraph()'s reveal decision needs the latest graphData without
  // re-creating the callback per render.
  const hasGraphRef = useRef(false);
  useEffect(() => {
    hasGraphRef.current = graphData !== null;
  }, [graphData]);

  // Ref to the AbortController used by refetchGraph() so successive calls
  // cancel the previous in-flight request.
  const refetchAbortRef = useRef<AbortController | null>(null);

  // ── Initial-load / session-change effect ────────────────────────────────
  // Runs whenever sessionGraphPath changes (including to null) and on mount —
  // the mount run is what restores the live graph after SPA navigation.
  // Quiet by design: no toast for a session that simply has no graph yet.
  // Uses an AbortController so the in-flight request is actually cancelled at
  // the network level (not just guarded by a flag) when the path changes or
  // the component unmounts.
  useEffect(() => {
    if (!sessionGraphPath) {
      setGraphData(null);
      return;
    }

    const controller = new AbortController();
    setGraphData(null); // clear stale data while the new fetch is in-flight

    fetch(sessionGraphPath, { signal: controller.signal })
      .then(res => {
        if (!res.ok) throw new Error(`HTTP ${res.status}`);
        return res.json();
      })
      .then((json: unknown) => {
        const graph = toRenderableGraph(json);
        if (graph) {
          setGraphData(graph);
          setRightTab('graph'); // reveal the restored graph
        }
      })
      .catch(() => {
        // Silent: no live graph in this session (fresh session, closed
        // session, or a network hiccup) — the canvas empty state already
        // says how to get a graph.
      });

    return () => { controller.abort(); };
  }, [sessionGraphPath]); // eslint-disable-line react-hooks/exhaustive-deps
  // setRightTab is intentionally excluded: its identity follows validTabs and
  // would re-trigger the fetch without the path having changed.

  // ── Imperative re-fetch (auto-refresh path) ─────────────────────────────
  // Empty dep array — this function is intentionally stable across renders.
  // It reads sessionGraphPath via sessionGraphPathRef, never via closure.
  const refetchGraph = useCallback(() => {
    const path = sessionGraphPathRef.current;
    if (!path) return;

    // Cancel any previous in-flight refetch.
    refetchAbortRef.current?.abort();
    const controller = new AbortController();
    refetchAbortRef.current = controller;

    setIsRefreshing(true);

    fetch(path, { signal: controller.signal })
      .then(res => {
        if (!res.ok) throw new Error(`HTTP ${res.status}`);
        return res.json();
      })
      .then((json: unknown) => {
        const graph = toRenderableGraph(json);
        const revealing = graph !== null && !hasGraphRef.current;
        setGraphData(graph);
        if (revealing) {
          setRightTab('graph'); // first content — bring the Graph tab forward
        }
        setIsRefreshing(false);
      })
      .catch((err: Error) => {
        // This request was superseded by a newer refetchGraph() call — a new
        // fetch is already in-flight and owns isRefreshing, so don't reset it.
        if (err.name === 'AbortError') return;
        addToast(`Graph refresh failed: ${err.message}`, 'error');
        setIsRefreshing(false);
      });
  }, []); // eslint-disable-line react-hooks/exhaustive-deps
  // The empty dep array is intentional — see sessionGraphPathRef for path access.
  // addToast is intentionally excluded: it is stable (from useToast) and including
  // it would require listing it which would force the hook consumer to stabilise it.

  // ── Abort any in-flight refetch on unmount ──────────────────────────────
  // The initial-load fetch is already cleaned up by its own effect's teardown.
  // This handles the case where refetchGraph() is called and the component
  // unmounts before the response arrives (e.g. user navigates away mid-refresh).
  useEffect(() => {
    return () => { refetchAbortRef.current?.abort(); };
  }, []);

  return { graphData, setGraphData, rightTab, setRightTab, isRefreshing, refetchGraph };
}
