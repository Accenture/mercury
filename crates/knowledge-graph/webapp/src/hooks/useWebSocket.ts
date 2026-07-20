import { useReducer, useEffect, useRef, useCallback } from 'react';
import { useLocalStorage } from './useLocalStorage';
import { type ToastType } from './useToast';
import { MAX_BUFFER, MAX_HISTORY } from '../config/playgrounds';
import { useWebSocketContext } from '../contexts/WebSocketContext';
import { extractUploadPath } from '../utils/messageParser';
import { type ProtocolBus } from '../protocol/bus';

// ---------------------------------------------------------------------------
// Types
// ---------------------------------------------------------------------------

/** Local UI state that does NOT need to persist across navigation. */
interface LocalState {
  command:      string;
  historyIndex: number;
  /**
   * Snapshot of the command field taken the moment the user first presses
   * ArrowUp to enter history browsing.  Restored when they press ArrowDown
   * past the newest history entry (index 0) — so in-progress input is never
   * lost by accidentally over-pressing the arrow key.
   */
  draftCommand:  string;
}

/** Every action the local reducer can handle. */
type LocalAction =
  | { type: 'SET_COMMAND';       value: string }
  | { type: 'CLEAR_COMMAND' }
  | { type: 'SET_HISTORY_INDEX'; index: number; command: string }
  | { type: 'ENTER_HISTORY';     command: string }
  | { type: 'EXIT_HISTORY' };

/** Options accepted by useWebSocket. */
export interface UseWebSocketOptions {
  wsPath:            string;
  storageKeyHistory: string;
  payload:           string;
  addToast:          (message: string, type?: ToastType) => void;
  bus?:              ProtocolBus;
  /**
   * Optional local command interception.  Return `true` to signal the command
   * was handled locally and must not be sent over the WebSocket; return `false`
   * (or omit) to fall through to the normal remote-send path.
   */
  handleLocalCommand?: (text: string) => boolean;
}

/** Public API surface returned by useWebSocket. */
export interface UseWebSocketReturn {
  connected:        boolean;
  connecting:       boolean;
  messages:         { id: number; raw: string }[];
  command:          string;
  setCommand:       (value: string) => void;
  connect:          () => void;
  disconnect:       () => void;
  sendCommand:      () => void;
  handleKeyDown:    (e: React.KeyboardEvent<HTMLElement>) => void;
  consoleRef:       React.RefObject<HTMLDivElement | null>;
  copyMessages:     () => void;
  clearMessages:    () => void;
  uploadPayload:    () => void;
  sendRawText:      (text: string) => boolean;
  /** Append a local-only message to this slot's console (no WebSocket round-trip). */
  appendMessage:    (raw: string) => void;
  /** Ordered command history for this playground slot (newest-first). */
  history:          string[];
}

// ---------------------------------------------------------------------------
// Local reducer (UI-only state — not shared across routes)
// ---------------------------------------------------------------------------

const localInitial: LocalState = {
  command:      '',
  historyIndex: -1,
  draftCommand: '',
};

function localReducer(state: LocalState, action: LocalAction): LocalState {
  switch (action.type) {
    case 'SET_COMMAND':
      // Any explicit setCommand call (including suggestion accept) exits history
      // navigation: reset the cursor and discard the draft. Semantically correct —
      // if text is being set externally, history browsing is over.
      return { ...state, command: action.value, historyIndex: -1, draftCommand: '' };
    case 'CLEAR_COMMAND':
      return { ...state, command: '', historyIndex: -1, draftCommand: '' };
    case 'SET_HISTORY_INDEX':
      return { ...state, historyIndex: action.index, command: action.command };
    // Save current input as draft, then load the first history entry.
    case 'ENTER_HISTORY':
      return { ...state, historyIndex: 0, command: action.command, draftCommand: state.command };
    // Restore the draft and leave history browsing mode.
    case 'EXIT_HISTORY':
      return { ...state, historyIndex: -1, command: state.draftCommand, draftCommand: '' };
    default:
      return state;
  }
}

// ---------------------------------------------------------------------------
// Hook
// ---------------------------------------------------------------------------

/**
 * Thin wrapper that delegates connection / message state to WebSocketContext
 * (so it persists across route changes) while keeping purely local UI state
 * (command input, history cursor, auto-scroll) in a component-scoped reducer.
 *
 * Public API is unchanged — Playground.tsx needs no edits.
 */
export function useWebSocket({ wsPath, storageKeyHistory, payload, addToast, bus, handleLocalCommand }: UseWebSocketOptions): UseWebSocketReturn {

  // Shared context — connection phase + messages live here, surviving navigation.
  const ctx = useWebSocketContext();

  // Pull this playground's slot from the shared store.
  const { phase, messages } = ctx.getSlot(wsPath);
  const connected  = phase === 'connected';
  const connecting = phase === 'connecting';

  // Local UI state (not shared, resets on remount — that's intentional).
  const [localState, dispatch] = useReducer(localReducer, localInitial);
  const { command, historyIndex } = localState;

  // Persisted command history (keyed per playground so they stay separate).
  const [history, setHistory] = useLocalStorage<string[]>(storageKeyHistory, []);

  // consoleRef is only needed for auto-scroll; it is purely presentational.
  const consoleRef = useRef<HTMLDivElement>(null);

  // When true, the next incoming message that contains an upload URL is
  // consumed to fire the HTTP POST.  A ref (not state) avoids a re-render.
  const pendingUploadRef = useRef(false);

  // --- Auto-scroll effect ---
  useEffect(() => {
    if (consoleRef.current) {
      consoleRef.current.scrollTop = consoleRef.current.scrollHeight;
    }
  }, [messages]);

  // --- Public: connect ---
  const connect = useCallback(() => {
    ctx.connect(wsPath, addToast);
  }, [ctx, wsPath, addToast]);

  // --- Public: disconnect ---
  const disconnect = useCallback(() => {
    ctx.disconnect(wsPath);
  }, [ctx, wsPath]);

  // --- Public: send the current command ---
  const sendCommand = useCallback(() => {
    if (phase !== 'connected') return;
    const text = command.trim();
    if (text.length === 0) return;

    if (handleLocalCommand?.(text) === true) {
      if (history[0] !== text) {
        setHistory((prev) => [text, ...prev].slice(0, MAX_HISTORY));
      }
      ctx.appendMessage(wsPath, '> ' + text);
      dispatch({ type: 'CLEAR_COMMAND' });
      return;
    }

    ctx.send(wsPath, text);
    if (history[0] !== text) {
      setHistory((prev) => [text, ...prev].slice(0, MAX_HISTORY));
    }

    // Special "load" command — also send the payload as a second message.
    if (text === 'load') {
      if (payload.length === 0) {
        ctx.appendMessage(wsPath, 'ERROR: please paste JSON/XML payload in input text area');
      } else {
        ctx.send(wsPath, payload);
      }
    }

    dispatch({ type: 'CLEAR_COMMAND' });
  }, [ctx, wsPath, phase, command, payload, history, setHistory, handleLocalCommand]);

  // --- Public: keyboard handler (history navigation only) ---
  const handleKeyDown = useCallback((e: React.KeyboardEvent<HTMLElement>) => {
    if (e.key === 'ArrowUp') {
      e.preventDefault();
      if (history.length === 0) return;
      if (historyIndex === -1) {
        // First ArrowUp — snapshot the draft and load the newest history entry.
        dispatch({ type: 'ENTER_HISTORY', command: history[0] });
      } else if (historyIndex < history.length - 1) {
        // Already in history — go further back.
        const newIndex = historyIndex + 1;
        dispatch({ type: 'SET_HISTORY_INDEX', index: newIndex, command: history[newIndex] });
      }
    } else if (e.key === 'ArrowDown') {
      e.preventDefault();
      if (historyIndex <= 0) {
        // At the newest entry (or not in history) — restore the draft instead
        // of clearing, so in-progress input is never lost.
        if (historyIndex === 0) dispatch({ type: 'EXIT_HISTORY' });
      } else {
        const newIndex = historyIndex - 1;
        dispatch({ type: 'SET_HISTORY_INDEX', index: newIndex, command: history[newIndex] });
      }
    }
  }, [history, historyIndex]);

  // --- Watch incoming messages for the upload URL when a POST is pending ---
  // Bus-based subscription (active when bus is provided)
  useEffect(() => {
    if (!bus) return;
    return bus.on('upload.contentPath', (event) => {
      if (!pendingUploadRef.current) return;
      pendingUploadRef.current = false;

      if (payload.length === 0) {
        ctx.appendMessage(wsPath, 'ERROR: please paste JSON/XML payload in the input text area');
        return;
      }

      let body: string;
      try {
        body = JSON.stringify(JSON.parse(payload));
      } catch {
        ctx.appendMessage(wsPath, 'ERROR: payload is not valid JSON — cannot upload');
        return;
      }

      fetch(event.uploadPath, {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body,
      })
        .then(res => {
          if (!res.ok) throw new Error(`HTTP ${res.status}`);
          addToast('Payload uploaded successfully', 'success');
        })
        .catch((err: Error) => {
          ctx.appendMessage(wsPath, `ERROR: upload failed — ${err.message}`);
          addToast(`Upload failed: ${err.message}`, 'error');
        });
    });
  }, [bus, payload, wsPath, ctx, addToast]);

  // Legacy messages-watch fallback (active when bus is NOT provided)
  useEffect(() => {
    if (bus) return;
    if (!pendingUploadRef.current || messages.length === 0) return;
    const latest = messages[messages.length - 1].raw;
    const uploadPath = extractUploadPath(latest);
    if (!uploadPath) return;

    // Consume the pending flag immediately so a duplicate message doesn't
    // re-trigger the POST.
    pendingUploadRef.current = false;

    if (payload.length === 0) {
      ctx.appendMessage(wsPath, 'ERROR: please paste JSON/XML payload in the input text area');
      return;
    }

    let body: string;
    try {
      // Re-serialise through JSON.parse to ensure the server receives clean JSON.
      body = JSON.stringify(JSON.parse(payload));
    } catch {
      ctx.appendMessage(wsPath, 'ERROR: payload is not valid JSON — cannot upload');
      return;
    }

    fetch(uploadPath, {
      method: 'POST',
      headers: { 'Content-Type': 'application/json' },
      body,
    })
      .then(res => {
        if (!res.ok) throw new Error(`HTTP ${res.status}`);
        addToast('Payload uploaded successfully', 'success');
      })
      .catch((err: Error) => {
        ctx.appendMessage(wsPath, `ERROR: upload failed — ${err.message}`);
        addToast(`Upload failed: ${err.message}`, 'error');
      });
  }, [bus, messages, payload, wsPath, ctx, addToast]);

  // --- Public: trigger the two-step upload handshake ---
  const uploadPayload = useCallback(() => {
    if (phase !== 'connected') return;
    if (payload.length === 0) {
      addToast('Nothing to upload — paste a JSON payload first', 'error');
      return;
    }
    // Tell the server we want to upload; it will reply with the upload URL.
    pendingUploadRef.current = true;
    ctx.send(wsPath, 'upload');
  }, [ctx, wsPath, phase, payload, addToast]);

  // --- Public: send raw text directly over the WebSocket without echoing to the console ---
  // Unlike sendCommand(), this bypasses the command-echo and history logic.
  // Used by useAutoGraphRefresh to silently send "describe graph" after a mutation.
  const sendRawText = useCallback((text: string): boolean => {
    if (phase !== 'connected') return false;
    return ctx.send(wsPath, text);
  }, [ctx, wsPath, phase]);

  // --- Console helpers ---
  const copyMessages = useCallback(() => {
    navigator.clipboard.writeText(messages.map((m: { id: number; raw: string }) => m.raw).join('\n'));
    addToast('Console copied to clipboard!', 'success');
  }, [messages, addToast]);

  const clearMessages = useCallback(() => {
    ctx.clearMessages(wsPath);
    addToast('Console cleared', 'info');
  }, [ctx, wsPath, addToast]);

  const appendMessage = useCallback((raw: string) => {
    ctx.appendMessage(wsPath, raw);
  }, [ctx, wsPath]);

  const setCommand = useCallback(
    (value: string) => dispatch({ type: 'SET_COMMAND', value }),
    []
  );

  return {
    connected,
    connecting,
    messages,
    command,
    setCommand,
    connect,
    disconnect,
    sendCommand,
    handleKeyDown,
    consoleRef,
    copyMessages,
    clearMessages,
    uploadPayload,
    sendRawText,
    appendMessage,
    history,
  };
}
