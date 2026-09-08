import { useCallback, useEffect, useRef } from 'react';
import type { ProtocolBus } from '../protocol/bus';
import type { UndoEntry } from '../graphActions/undoCommands';
import type { ToastType } from './useToast';

const MAX_UNDO_DEPTH = 20;
// Wait for the previous inverse command's mutation confirmation before sending
// the next one: the backend can interleave a slow multi-line `create node`
// with later single-line commands from the same session (observed live —
// `connect` answered "not found" before its `create` completed). An error
// reply is not a mutation, so a timeout keeps the chain moving.
const CONFIRMATION_TIMEOUT_MS = 2500;

interface StackItem extends UndoEntry {
  id: number;
}

export interface UseGraphUndoOptions {
  bus: ProtocolBus;
  connected: boolean;
  sendRawText: (text: string) => boolean;
  addToast: (message: string, type?: ToastType) => void;
}

export interface UseGraphUndoReturn {
  /** Push an inverse recipe; returns its id (for toast-bound undo) or null when not undoable. */
  push: (entry: UndoEntry | null) => number | null;
  /** Undo the newest entry (Ctrl+Z). */
  undoLast: () => void;
  /** Undo a specific entry — only when it is still the newest one. */
  undoEntry: (id: number) => void;
  /**
   * Run a compound edit as paced commands (each awaits the previous one's
   * mutation confirmation).  Concurrent calls queue behind the running batch;
   * undo shares the busy flag, so an undo never interleaves with a compound.
   */
  runCommands: (commands: string[]) => void;
  hasEntries: () => boolean;
  clear: () => void;
}

/**
 * Frontend-only undo stack of compensating console commands (the backend is a
 * lightweight command executor with no undo journal — minimalist by design).
 * Undoing sends the entry's inverse commands over the WebSocket; the graph
 * redraws from the backend confirmations via the existing auto-refresh.
 *
 * Invalidation is deliberately simple: the stack clears on disconnect and on a
 * graph import/load (a new model makes old inverses meaningless). Collaborative
 * sessions keep the playground's normal last-write-wins semantics — an undo
 * after a co-author's change overwrites it, exactly like any other edit would.
 * Console-typed commands are not tracked; only UI gestures push entries.
 */
export function useGraphUndo({
  bus,
  connected,
  sendRawText,
  addToast,
}: UseGraphUndoOptions): UseGraphUndoReturn {
  const stackRef = useRef<StackItem[]>([]);
  const nextIdRef = useRef(0);

  const clear = useCallback(() => {
    stackRef.current = [];
  }, []);

  // Old inverses are meaningless against a dropped session or a new model.
  useEffect(() => {
    if (!connected) clear();
  }, [clear, connected]);

  useEffect(() => {
    return bus.on('graph.mutation', (event) => {
      if (event.mutationType === 'import-graph') clear();
    });
  }, [bus, clear]);

  const push = useCallback((entry: UndoEntry | null): number | null => {
    if (entry === null || entry.inverseCommands.length === 0) return null;
    const id = ++nextIdRef.current;
    stackRef.current = [
      ...stackRef.current.slice(-(MAX_UNDO_DEPTH - 1)),
      { ...entry, id },
    ];
    return id;
  }, []);

  const executingRef = useRef(false);

  const waitForMutationConfirmation = useCallback(() => {
    return new Promise<void>((resolve) => {
      let settled = false;
      const settle = () => {
        if (settled) return;
        settled = true;
        unsubscribe();
        clearTimeout(timer);
        resolve();
      };
      const unsubscribe = bus.on('graph.mutation', settle);
      const timer = setTimeout(settle, CONFIRMATION_TIMEOUT_MS);
    });
  }, [bus]);

  const pendingBatchesRef = useRef<string[][]>([]);

  const runCommands = useCallback((commands: string[]) => {
    if (commands.length === 0) return;
    pendingBatchesRef.current.push(commands);
    if (executingRef.current) return; // the running drain picks it up
    executingRef.current = true;
    void (async () => {
      try {
        for (;;) {
          const batch = pendingBatchesRef.current.shift();
          if (!batch) break;
          for (const command of batch) {
            if (!sendRawText(command)) {
              addToast('Could not send the command because the WebSocket is not open.', 'error');
              pendingBatchesRef.current = [];
              return;
            }
            await waitForMutationConfirmation();
          }
        }
      } finally {
        executingRef.current = false;
      }
    })();
  }, [addToast, sendRawText, waitForMutationConfirmation]);

  const undoEntry = useCallback((id?: number) => {
    if (executingRef.current) {
      addToast('A graph edit is already in progress.', 'info');
      return;
    }
    const stack = stackRef.current;
    const top = stack[stack.length - 1];
    if (!top) {
      addToast('Nothing to undo.', 'info');
      return;
    }
    if (id !== undefined && top.id !== id) {
      addToast('Newer changes exist — press Ctrl+Z to undo them in order.', 'info');
      return;
    }
    stackRef.current = stack.slice(0, -1);

    executingRef.current = true;
    void (async () => {
      try {
        for (const command of top.inverseCommands) {
          if (!sendRawText(command)) {
            addToast('Could not send the undo command because the WebSocket is not open.', 'error');
            clear();
            return;
          }
          await waitForMutationConfirmation();
        }
        addToast(`Undo: ${top.label}`, 'success');
      } finally {
        executingRef.current = false;
      }
    })();
  }, [addToast, clear, sendRawText, waitForMutationConfirmation]);

  const undoLast = useCallback(() => undoEntry(undefined), [undoEntry]);
  const hasEntries = useCallback(() => stackRef.current.length > 0, []);

  return { push, undoLast, undoEntry, runCommands, hasEntries, clear };
}
