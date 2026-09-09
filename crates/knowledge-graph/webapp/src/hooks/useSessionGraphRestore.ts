import { useEffect, useRef } from 'react';
import { isMinigraphGraphData, type MinigraphGraphData } from '../utils/graphTypes';
import { type RightTab } from '../components/RightPanel/RightPanel';

export interface UseSessionGraphRestoreOptions {
  /** Only playgrounds with a graph view and a known session lifecycle opt in. */
  enabled: boolean;
  /** `/api/graph/session/{id}` for the current session, or null until the id is known. */
  sessionGraphPath: string | null;
  /** The described temp-model path useGraphData fetches first (authoritative when it resolves). */
  pinnedGraphPath: string | null;
  /** useGraphData's verdict that the CURRENT pinned path is a dead end (e.g. expired → HTTP 400). */
  initialFetchFailed: boolean;
  graphData: MinigraphGraphData | null;
  setGraphData: React.Dispatch<React.SetStateAction<MinigraphGraphData | null>>;
  setRightTab: React.Dispatch<React.SetStateAction<RightTab>>;
}

/**
 * Restores the live session graph after SPA navigation.
 *
 * Described temp-model paths (`/api/graph/model/…`) expire server-side about a
 * minute after `describe graph`, so returning from another playground can find
 * the pinned path answering HTTP 400 while the session still holds the live
 * graph. When the model path is a dead end (failed, or none pinned) and the
 * session id is known, quietly fetch `GET /api/graph/session/{id}` and render
 * that instead — no error toast for this expected lifecycle. The pinned path
 * re-arms by itself on the next describe / mutation auto-refresh cycle.
 *
 * The session id arrives asynchronously (the `session` round-trip on mount),
 * so this reacts to `sessionGraphPath` appearing after the pinned fetch has
 * already failed. One attempt per (session path, pinned path) pair: a session
 * without a live graph answers an error status, and retrying it on unrelated
 * re-renders would be noise — the empty state is the honest outcome then.
 */
export function useSessionGraphRestore({
  enabled,
  sessionGraphPath,
  pinnedGraphPath,
  initialFetchFailed,
  graphData,
  setGraphData,
  setRightTab,
}: UseSessionGraphRestoreOptions): void {
  const attemptedKeyRef = useRef<string | null>(null);

  useEffect(() => {
    if (!enabled || sessionGraphPath === null || graphData !== null) return;
    // While a pinned model path has not delivered its verdict, it stays the
    // authoritative source — only fall back once it failed (or none exists).
    if (pinnedGraphPath !== null && !initialFetchFailed) return;

    const attemptKey = `${sessionGraphPath}|${pinnedGraphPath ?? ''}`;
    if (attemptedKeyRef.current === attemptKey) return;
    attemptedKeyRef.current = attemptKey;

    const controller = new AbortController();
    fetch(sessionGraphPath, { signal: controller.signal })
      .then(res => {
        if (!res.ok) throw new Error(`HTTP ${res.status}`);
        return res.json();
      })
      .then((json: unknown) => {
        if (isMinigraphGraphData(json)) {
          setGraphData(json);
          setRightTab('graph'); // same auto-switch as a pinned-path load
        }
      })
      .catch(() => {
        // Silent: no live graph in this session (or a network hiccup) — the
        // canvas empty state already says how to get a graph back.
      });

    return () => { controller.abort(); };
  }, [enabled, graphData, initialFetchFailed, pinnedGraphPath, sessionGraphPath, setGraphData, setRightTab]);
}
