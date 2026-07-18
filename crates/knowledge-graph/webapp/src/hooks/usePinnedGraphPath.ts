import { useState, useCallback } from 'react';

/**
 * Module-scoped store for pinned graph API paths, keyed by wsPath.
 *
 * Graph API paths (e.g. `/api/graph/model/my-graph/ws-123-1`) are bound to
 * the server's WebSocket session — they become invalid when the connection
 * drops (hard refresh, tab close, server restart).  A module-scoped Map
 * matches this lifetime exactly: it survives Playground remounts caused by
 * SPA navigation (`key={cfg.path}` in App.tsx) but resets on page load /
 * hard refresh because the module is re-evaluated.
 *
 * Previously this used `sessionStorage`, which also survives hard refreshes.
 * That caused a stale-path fetch (HTTP 400) because the server session was
 * gone while the stored path remained.
 */
const pinnedPaths = new Map<string, string | null>();

/**
 * Persistence for the pinned graph API path that survives Playground
 * remounts (SPA navigation) but resets on page load / hard refresh.
 *
 * This hook replaces the plain `useState<string | null>(null)` that
 * previously lived inside Playground.tsx, giving the pinned path a home that
 * outlives Playground remounts (caused by `key={cfg.path}` in App.tsx)
 * without adding any new responsibility to WebSocketContext or Playground.
 */
export function usePinnedGraphPath(
  wsPath: string,
): [string | null, (path: string | null) => void] {
  const [path, setPath] = useState<string | null>(
    () => pinnedPaths.get(wsPath) ?? null,
  );

  const setPinnedGraphPath = useCallback(
    (next: string | null) => {
      setPath(next);
      if (next === null) {
        pinnedPaths.delete(wsPath);
      } else {
        pinnedPaths.set(wsPath, next);
      }
    },
    [wsPath],
  );

  return [path, setPinnedGraphPath];
}
