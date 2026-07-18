import { useCallback, useMemo } from 'react';
import { useLocalStorage } from './useLocalStorage';

// ---------------------------------------------------------------------------
// Types
// ---------------------------------------------------------------------------

/** A single saved-graph entry stored in localStorage. */
export interface SavedGraphEntry {
  /** The user-supplied name, also the key used with `import graph from {name}`. */
  name:    string;
  /** ISO-8601 timestamp of when this entry was saved. */
  savedAt: string;
}

/** The full map of saved entries, keyed by name. */
export type SavedGraphsMap = Record<string, SavedGraphEntry>;

export interface UseSavedGraphsReturn {
  /** All saved entries, sorted newest-first by savedAt. */
  savedGraphs: SavedGraphEntry[];
  /** Persist an entry under the given name (overwrites if name already exists). */
  saveGraph:   (name: string) => void;
  /** Remove a saved entry by name. */
  deleteGraph: (name: string) => void;
  /** True when an entry with this name already exists in storage. */
  hasGraph:    (name: string) => boolean;
}

// ---------------------------------------------------------------------------
// Hook
// ---------------------------------------------------------------------------

/**
 * Manages a set of named graph bookmarks in localStorage.
 *
 * Only the export name is stored — not the graph data itself.  The name is
 * what the backend uses to locate the file it previously wrote to disk when
 * the user ran `export graph as {name}`.
 *
 * Snapshots are keyed by the `storageKey` arg so each playground has its own
 * isolated namespace (e.g. `"minigraph-saved-graphs"`).
 *
 * The hook is a pure data store — it knows nothing about WebSockets.
 * The save/load orchestration lives in Playground.tsx:
 *
 *  Save:  saveGraph(name) + ws.sendRawText(`export graph as ${name}`)
 *          → bookmark written to localStorage
 *          → server writes {name}.json to its temp directory
 *
 *  Load:  ws.sendRawText(`import graph from ${name}`)
 *          → server reads {name}.json from its temp directory
 *          → auto-refresh hook re-renders the graph
 */
export function useSavedGraphs(storageKey: string): UseSavedGraphsReturn {
  const [map, setMap] = useLocalStorage<SavedGraphsMap>(storageKey, {});

  const saveGraph = useCallback((name: string) => {
    setMap(prev => ({
      ...prev,
      [name]: { name, savedAt: new Date().toISOString() },
    }));
  }, [setMap]);

  const deleteGraph = useCallback((name: string) => {
    setMap(prev => {
      const next = { ...prev };
      delete next[name];
      return next;
    });
  }, [setMap]);

  const hasGraph = useCallback((name: string) => {
    return Object.prototype.hasOwnProperty.call(map, name);
  }, [map]);

  // Sort newest-first so the most recently saved entry appears at the top.
  const savedGraphs = useMemo(() => Object.values(map).sort(
    (a, b) => new Date(b.savedAt).getTime() - new Date(a.savedAt).getTime(),
  ), [map]
  );

  return { savedGraphs, saveGraph, deleteGraph, hasGraph };
}
