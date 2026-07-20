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
   * Call this after a successful save so the input remembers the chosen name
   * for subsequent opens of the save form.
   *
   * When the name matches the current `untitled-{n}` slot, the slot is marked
   * as consumed so `resetName` knows to advance the counter on the next clear.
   */
  setLastSavedName: (name: string) => void;
  /**
   * Call this when the user clears the console / working graph (e.g. via the
   * Clear button). Clears the imported- and last-saved names. The untitled
   * counter only advances when the current `untitled-{n}` slot was actually
   * consumed by a save — so clearing without ever saving reuses the same slot,
   * guaranteeing `untitled-{n}` only exists when `untitled-{n-1}` does (min 1).
   */
  resetName:        () => void;
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
 *    It is set to true only when setLastSavedName is called with a name that
 *    matches the active untitled fallback, and is reset to false on resetName().
 *  - `importedName` is derived reactively from the WebSocket message stream
 *    using a message-ID watermark (same pattern as useAutoMockUpload /
 *    useAutoGraphRefresh) so stale history is never replayed on mount.
 *  - `lastSavedName` is plain component state — it is intentionally NOT
 *    persisted to localStorage; it is reset on `resetName()` so a cleared
 *    graph starts fresh.
 *
 * @param storageKey  localStorage key for the untitled counter
 *                    (should be unique per playground, e.g.
 *                    `"minigraph-untitled-counter"`).
 * @param bus         The shared ProtocolBus instance for this playground.
 */
export function useGraphSaveName(
  storageKey: string,
  bus:        ProtocolBus,
): UseGraphSaveNameReturn {

  // ── Untitled counter ──────────────────────────────────────────────────────
  // Persisted per playground so numbers never reuse within a playground's
  // lifetime.  Starts at 1 for a fresh localStorage (i.e. first ever save).
  const [untitledCounter, setUntitledCounter] = useLocalStorage<number>(storageKey, 1);

  // ── Untitled slot consumed flag ───────────────────────────────────────────
  // True only after setLastSavedName is called with a name that matches the
  // current `untitled-{n}` fallback.  Tells resetName() whether to advance
  // the counter.  Stored in a ref (not state) because it is write-only from
  // the hook's perspective — it never drives a re-render on its own.
  const untitledSlotConsumedRef = useRef(false);

  // ── Imported name ─────────────────────────────────────────────────────────
  // Updated whenever the user sends `import graph from {name}`.
  // Cleared by resetName().
  const [importedName, setImportedName] = useState<string | null>(null);

  // ── Last-saved name ───────────────────────────────────────────────────────
  // Updated by the caller via setLastSavedName after a successful save.
  // Cleared by resetName().
  const [lastSavedName, setLastSavedNameState] = useState<string | null>(null);

  // ── Message-ID watermark ──────────────────────────────────────────────────
  // Replaced by bus subscription — no watermark needed.

  // ── Scan new messages for import-graph echoes ─────────────────────────────
  useEffect(() => {
    return bus.on('command.importGraph', (event) => {
      setImportedName(event.graphName);
      setLastSavedNameState(null);
    });
  }, [bus]);

  // ── Public setters ────────────────────────────────────────────────────────

  const setLastSavedName = useCallback((name: string) => {
    setLastSavedNameState(name);
    // Mark the current untitled slot as consumed when the user saves with the
    // untitled fallback name — but NOT when they rename it to something else.
    // This is read by resetName() to decide whether to advance the counter.
    if (name === `untitled-${untitledCounter}`) {
      untitledSlotConsumedRef.current = true;
    }
  }, [untitledCounter]);

  const resetName = useCallback(() => {
    setImportedName(null);
    setLastSavedNameState(null);
    // Only advance the counter when the current untitled slot was actually used
    // as a save name.  Clearing without ever saving reuses the same slot so
    // that `untitled-{n}` only exists in storage when `untitled-{n-1}` does.
    if (untitledSlotConsumedRef.current) {
      setUntitledCounter(prev => prev + 1);
    }
    untitledSlotConsumedRef.current = false;
  }, [setUntitledCounter]);

  // ── Derived default name ──────────────────────────────────────────────────
  const defaultName =
    lastSavedName  ??
    importedName   ??
    `untitled-${untitledCounter}`;

  return { defaultName, setLastSavedName, resetName };
}
