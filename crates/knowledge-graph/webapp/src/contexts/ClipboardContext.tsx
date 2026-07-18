import { createContext, useCallback, useContext, useEffect, useReducer, useRef, type ReactNode } from 'react';
import * as db from '../clipboard/db';
import type { ClipboardItemRecord } from '../clipboard/db';
import { createClipboardChannel, type ClipboardChannelMessage } from '../clipboard/channel';
import type { MinigraphNode, MinigraphConnection } from '../utils/graphTypes';

// ── Context State ────────────────────────────────────────────────────────────

interface ClipboardState {
  items: ClipboardItemRecord[];
  isLoading: boolean;
}

type ClipboardAction =
  | { type: 'HYDRATE';        items: ClipboardItemRecord[] }
  | { type: 'ITEM_ADDED';     item: ClipboardItemRecord }
  | { type: 'ITEM_REPLACED';  item: ClipboardItemRecord; previousId: string }
  | { type: 'ITEM_REMOVED';   id: string }
  | { type: 'ITEMS_CLEARED' };

function clipboardReducer(state: ClipboardState, action: ClipboardAction): ClipboardState {
  switch (action.type) {
    case 'HYDRATE':
      return { items: action.items, isLoading: false };
    case 'ITEM_ADDED':
      return { ...state, items: [action.item, ...state.items] };
    case 'ITEM_REPLACED': {
      const filtered = state.items.filter(i => i.id !== action.previousId);
      return { ...state, items: [action.item, ...filtered] };
    }
    case 'ITEM_REMOVED':
      return { ...state, items: state.items.filter(i => i.id !== action.id) };
    case 'ITEMS_CLEARED':
      return { ...state, items: [] };
    default:
      return state;
  }
}

// ── Public Types ─────────────────────────────────────────────────────────────

export interface ClipMeta {
  sourceWsPath: string;
  sourceLabel: string;
}

export type ClipResult =
  | { status: 'added' }
  | { status: 'duplicate'; existingItem: ClipboardItemRecord; pendingItem: ClipboardItemRecord }
  | { status: 'error'; message: string };

export interface ClipboardContextValue {
  items: ClipboardItemRecord[];
  isLoading: boolean;
  clipNode(
    node: MinigraphNode,
    connections: MinigraphConnection[],
    meta: ClipMeta,
  ): Promise<ClipResult>;
  confirmReplace(pendingItem: ClipboardItemRecord, previousId: string): Promise<void>;
  removeItem(id: string): Promise<void>;
  clearAll(): Promise<void>;
}

// ── Provider ─────────────────────────────────────────────────────────────────

const ClipboardContext = createContext<ClipboardContextValue | null>(null);

export function ClipboardProvider({ children }: { children: ReactNode }) {
  const [state, dispatch] = useReducer(clipboardReducer, { items: [], isLoading: true });
  const channelRef = useRef<BroadcastChannel | null>(null);

  // Hydrate from IndexedDB on mount
  useEffect(() => {
    db.getAllItems().then(items => dispatch({ type: 'HYDRATE', items }));
  }, []);

  // BroadcastChannel setup
  useEffect(() => {
    let channel: BroadcastChannel;
    try {
      channel = createClipboardChannel();
    } catch {
      return;
    }
    channelRef.current = channel;

    channel.onmessage = (event: MessageEvent<ClipboardChannelMessage>) => {
      const msg = event.data;
      switch (msg.type) {
        case 'item-added':
          dispatch({ type: 'ITEM_ADDED', item: msg.item });
          break;
        case 'item-replaced':
          dispatch({ type: 'ITEM_REPLACED', item: msg.item, previousId: msg.previousId });
          break;
        case 'item-removed':
          dispatch({ type: 'ITEM_REMOVED', id: msg.id });
          break;
        case 'items-cleared':
          dispatch({ type: 'ITEMS_CLEARED' });
          break;
      }
    };

    return () => { channel.close(); channelRef.current = null; };
  }, []);

  const broadcast = useCallback((msg: ClipboardChannelMessage) => {
    channelRef.current?.postMessage(msg);
  }, []);

  // ── clipNode ──────────────────────────────────────────────────────────────
  const clipNode = useCallback(async (
    node: MinigraphNode,
    connections: MinigraphConnection[],
    meta: ClipMeta,
  ): Promise<ClipResult> => {
    try {
      const pendingItem: ClipboardItemRecord = {
        id: crypto.randomUUID(),
        clippedAt: new Date().toISOString(),
        sourceWsPath: meta.sourceWsPath,
        sourceLabel: meta.sourceLabel,
        node,
        connections,
      };

      const existing = await db.findByAlias(node.alias);
      if (existing) {
        return { status: 'duplicate', existingItem: existing, pendingItem };
      }

      try {
        await db.addItem(pendingItem);
      } catch (err) {
        if (err instanceof DOMException && err.name === 'ConstraintError') {
          const raceExisting = await db.findByAlias(node.alias);
          if (raceExisting) {
            return { status: 'duplicate', existingItem: raceExisting, pendingItem };
          }
        }
        throw err;
      }

      dispatch({ type: 'ITEM_ADDED', item: pendingItem });
      broadcast({ type: 'item-added', item: pendingItem });
      return { status: 'added' };
    } catch (err) {
      return { status: 'error', message: err instanceof Error ? err.message : String(err) };
    }
  }, [broadcast]);

  // ── confirmReplace ────────────────────────────────────────────────────────
  const confirmReplace = useCallback(async (
    pendingItem: ClipboardItemRecord,
    previousId: string,
  ): Promise<void> => {
    await db.replaceItem(previousId, pendingItem);
    dispatch({ type: 'ITEM_REPLACED', item: pendingItem, previousId });
    broadcast({ type: 'item-replaced', item: pendingItem, previousId });
  }, [broadcast]);

  // ── removeItem ────────────────────────────────────────────────────────────
  const removeItem = useCallback(async (id: string): Promise<void> => {
    await db.removeItem(id);
    dispatch({ type: 'ITEM_REMOVED', id });
    broadcast({ type: 'item-removed', id });
  }, [broadcast]);

  // ── clearAll ──────────────────────────────────────────────────────────────
  const clearAll = useCallback(async (): Promise<void> => {
    await db.clearAll();
    dispatch({ type: 'ITEMS_CLEARED' });
    broadcast({ type: 'items-cleared' });
  }, [broadcast]);

  return (
    <ClipboardContext.Provider value={{
      items: state.items,
      isLoading: state.isLoading,
      clipNode,
      confirmReplace,
      removeItem,
      clearAll,
    }}>
      {children}
    </ClipboardContext.Provider>
  );
}

/** Consumer hook. Must be called inside <ClipboardProvider>. */
export function useClipboardContext(): ClipboardContextValue {
  const ctx = useContext(ClipboardContext);
  if (!ctx) throw new Error('useClipboardContext must be used inside <ClipboardProvider>');
  return ctx;
}
