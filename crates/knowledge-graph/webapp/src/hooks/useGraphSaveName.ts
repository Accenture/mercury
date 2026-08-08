import { useState, useEffect, useRef, useCallback } from 'react';
import { useLocalStorage } from './useLocalStorage';
import { type ProtocolBus } from '../protocol/bus';

// ---------------------------------------------------------------------------
// Types
// ---------------------------------------------------------------------------

export interface UseGraphSaveNameReturn {
  /**
   * The name that should pre-fill the save-form input when the user opens it.
   *
   * Priority order:
   *  1. lastSavedName  — the name the working graph was most recently saved as.
   *  2. importedName   — the name that was supplied in the last
   *                      `import graph from {name}` command sent this session.
   *  3. `untitled-{n}` — a monotonically incrementing fallback, persisted to
   *                      localStorage so it never resets across page refreshes
   *                      within the same playground.
   */
  defaultName:      string;
  /**
   * The name confirmed by the most recent successful graph export. Null means
   * the current working graph has not been exported since its last mutation.
   */
  savedName:        string | null;
  /**
   * Call this when the user clears the console / working graph (e.g. via the
   * Clear button). Clears the imported- and last-saved names. The untitled
   * counter only advances when the current `untitled-{n}` slot was actually
   * consumed by a save — so clearing without ever saving reuses the same slot,
   * guaranteeing `untitled-{n}` only exists when `untitled-{n-1}` does (min 1).
   */
  resetName:        () => void;
}

export interface GraphSaveNameState {
  importedName:          string | null;
  lastSavedName:         string | null;
  isSaved:               boolean;
  untitledSlotConsumed:  boolean;
}

export type GraphSaveNameAction =
  | { type: 'imported'; name: string }
  | { type: 'exported'; name: string; consumesUntitled: boolean }
  | { type: 'dirty' }
  | { type: 'reset' };

export const EMPTY_GRAPH_SAVE_NAME_STATE: GraphSaveNameState = {
  importedName: null,
  lastSavedName: null,
  isSaved: false,
  untitledSlotConsumed: false,
};

export function reduceGraphSaveNameState(
  state: GraphSaveNameState,
  action: GraphSaveNameAction,
): GraphSaveNameState {
  switch (action.type) {
    case 'imported':
      return {
        ...state,
        importedName: action.name,
        lastSavedName: null,
        isSaved: false,
      };
    case 'exported':
      return {
        ...state,
        lastSavedName: action.name,
        isSaved: true,
        untitledSlotConsumed: state.untitledSlotConsumed || action.consumesUntitled,
      };
    case 'dirty':
      return { ...state, isSaved: false };
    case 'reset':
      return { ...EMPTY_GRAPH_SAVE_NAME_STATE };
  }
}

// Session-bound state survives Playground remounts during SPA navigation but
// resets on hard refresh because this module is re-evaluated.
interface CachedGraphSaveNameState {
  connectionEpoch: number;
  state: GraphSaveNameState;
}

const graphSaveNameStates = new Map<string, CachedGraphSaveNameState>();

export function shouldRestoreGraphSaveNameState(
  cachedEpoch: number | undefined,
  connected: boolean,
  connectionEpoch: number | null,
): boolean {
  return connected && connectionEpoch !== null && cachedEpoch === connectionEpoch;
}

interface GraphSaveNameEventHandlers {
  onImported: (name: string) => void;
  onExported: (name: string) => void;
  onDirty:    () => void;
  onReset:    () => void;
}

/** Subscribe the save-name state machine to the protocol events it owns. */
export function subscribeGraphSaveNameEvents(
  bus: ProtocolBus,
  handlers: GraphSaveNameEventHandlers,
): () => void {
  const unsubscribeImport = bus.on('command.importGraph', event => {
    handlers.onImported(event.graphName);
  });
  const unsubscribeExport = bus.on('graph.exported', event => {
    handlers.onExported(event.graphName);
  });
  const unsubscribeMutation = bus.on('graph.mutation', () => {
    handlers.onDirty();
  });
  const unsubscribeReset = bus.on('session.reset', () => {
    handlers.onReset();
  });

  return () => {
    unsubscribeImport();
    unsubscribeExport();
    unsubscribeMutation();
    unsubscribeReset();
  };
}

// ---------------------------------------------------------------------------
// Hook
// ---------------------------------------------------------------------------

/**
 * Manages the pre-fill name for the GraphSaveButton input.
 *
 * Design notes:
 *  - `untitledCounter` is stored in localStorage (keyed per playground) so
 *    the counter survives page refreshes.  It only increments when the current
 *    `untitled-{n}` slot has been consumed by an actual save — clearing without
 *    ever saving reuses the same slot number, so numbers are never skipped.
 *  - `untitledSlotConsumed` tracks whether the current slot has been used.
 *  - `importedName` is derived reactively from `command.importGraph` events.
 *  - Name and clean/dirty state live in a module-scoped map so they survive SPA
 *    navigation but not a hard refresh. A mutation clears only the clean flag,
 *    preserving the last export name as the next save-form default.
 *
 * @param storageKey  localStorage key for the untitled counter
 *                    (should be unique per playground, e.g.
 *                    `"minigraph-untitled-counter"`).
 * @param bus         The shared ProtocolBus instance for this playground.
 * @param connected   Whether this playground currently has a live WebSocket
 *                    session. A disconnect invalidates session-bound names.
 * @param connectionEpoch Stable identity for the current WebSocket connection.
 */
export function useGraphSaveName(
  storageKey:      string,
  bus:             ProtocolBus,
  connected:       boolean,
  connectionEpoch: number | null,
): UseGraphSaveNameReturn {

  // ── Untitled counter ──────────────────────────────────────────────────────
  // Persisted per playground so numbers never reuse within a playground's
  // lifetime.  Starts at 1 for a fresh localStorage (i.e. first ever save).
  const [untitledCounter, setUntitledCounter] = useLocalStorage<number>(storageKey, 1);

  // ── Session-bound name state ───────────────────────────────────────────────
  const [nameState, setNameState] = useState<GraphSaveNameState>(
    () => {
      const cached = graphSaveNameStates.get(storageKey);
      if (
        cached &&
        shouldRestoreGraphSaveNameState(
          cached.connectionEpoch,
          connected,
          connectionEpoch,
        )
      ) {
        return cached.state;
      }
      return { ...EMPTY_GRAPH_SAVE_NAME_STATE };
    },
  );
  const nameStateRef = useRef(nameState);

  const updateNameState = useCallback((action: GraphSaveNameAction) => {
    const next = reduceGraphSaveNameState(nameStateRef.current, action);
    nameStateRef.current = next;
    if (action.type === 'reset') {
      graphSaveNameStates.delete(storageKey);
    } else if (connectionEpoch !== null) {
      graphSaveNameStates.set(storageKey, { connectionEpoch, state: next });
    }
    setNameState(next);
  }, [connectionEpoch, storageKey]);

  const setLastSavedName = useCallback((name: string) => {
    updateNameState({
      type: 'exported',
      name,
      consumesUntitled: name === `untitled-${untitledCounter}`,
    });
  }, [untitledCounter, updateNameState]);

  // ── Public reset ───────────────────────────────────────────────────────────
  const resetName = useCallback(() => {
    const cachedState = graphSaveNameStates.get(storageKey)?.state;
    if (
      nameStateRef.current.untitledSlotConsumed ||
      cachedState?.untitledSlotConsumed
    ) {
      setUntitledCounter(prev => prev + 1);
    }
    updateNameState({ type: 'reset' });
  }, [setUntitledCounter, storageKey, updateNameState]);

  const activeConnectionEpochRef = useRef(connectionEpoch);
  useEffect(() => {
    const cachedEpoch = graphSaveNameStates.get(storageKey)?.connectionEpoch;
    if (
      !connected ||
      activeConnectionEpochRef.current !== connectionEpoch ||
      (cachedEpoch !== undefined && cachedEpoch !== connectionEpoch)
    ) {
      resetName();
    }
    activeConnectionEpochRef.current = connectionEpoch;
  }, [connected, connectionEpoch, resetName, storageKey]);

  // ── Protocol-driven saved state ───────────────────────────────────────────
  useEffect(() => {
    return subscribeGraphSaveNameEvents(bus, {
      onImported: (name) => updateNameState({ type: 'imported', name }),
      onExported: setLastSavedName,
      onDirty: () => updateNameState({ type: 'dirty' }),
      onReset: resetName,
    });
  }, [bus, resetName, setLastSavedName, updateNameState]);

  // ── Derived default name ──────────────────────────────────────────────────
  const defaultName =
    nameState.lastSavedName  ??
    nameState.importedName   ??
    `untitled-${untitledCounter}`;

  return {
    defaultName,
    savedName: nameState.isSaved ? nameState.lastSavedName : null,
    resetName,
  };
}
