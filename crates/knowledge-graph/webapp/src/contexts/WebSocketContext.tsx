/**
 * WebSocketContext
 *
 * Provides a single, navigation-persistent store of WebSocket connections
 * keyed by wsPath (e.g. "/ws/json/path", "/ws/graph/playground").
 *
 * By living above <Routes> in the component tree, connections survive when the
 * user switches playground tabs — the socket stays open and the status dot in
 * the nav bar correctly reflects every active connection simultaneously.
 */

import {
  createContext,
  useCallback,
  useContext,
  useEffect,
  useReducer,
  useRef,
  useState,
  useMemo,
  type ReactNode,
} from "react";
import {
  MAX_ITEMS,
  PING_INTERVAL,
  PLAYGROUND_CONFIGS,
} from "../config/playgrounds";
import { type ToastType } from "../hooks/useToast";
import { makeWsUrl } from "../utils/urls";

// ---------------------------------------------------------------------------
// Types
// ---------------------------------------------------------------------------

export type WsPhase = "idle" | "connecting" | "connected";

/**
 * Public shape of a WebSocket slot (for external consumers).
 * Internally, per-path state uses the `SlotState` type in the reducer
 * and separate `wsRefs`/`pingRefs`/`msgIdRefs` Records — not `WsSlot`.
 */
export interface WsSlot {
  phase: WsPhase;
  messages: { id: number; raw: string }[];
  wsRef: React.RefObject<WebSocket | null>;
  msgIdRef: React.RefObject<number>;
  pingRef: React.RefObject<ReturnType<typeof setInterval> | null>;
}

/** The shape of the context value exposed to consumers. */
export interface WebSocketContextValue {
  /** Get the reactive state (phase + messages) for a given wsPath. */
  getSlot: (wsPath: string) => {
    phase: WsPhase;
    messages: { id: number; raw: string }[];
  };
  /**
   * Open a WebSocket connection for the given path.
   * When `onToast` is omitted the connection is established silently — no
   * toast is shown for "Connected", "Already connected", or "Disconnected".
   * Useful for background auto-connect at startup or deferred-send flows.
   */
  connect: (
    wsPath: string,
    onToast?: (msg: string, type?: ToastType) => void,
  ) => void;
  /** Close the connection for the given path. */
  disconnect: (wsPath: string) => void;
  /** Send a raw string on the given path's socket. */
  send: (wsPath: string, data: string) => boolean;
  /** Append a local-only error/info message to a slot's console. */
  appendMessage: (wsPath: string, raw: string) => void;
  /** Clear all messages for a given path. */
  clearMessages: (wsPath: string) => void;
  /**
   * Store a payload string for the given wsPath so its Playground can pick
   * it up on the next render without writing to localStorage.
   * Pass `null` to clear a previously-stored pending payload.
   */
  setPendingPayload: (wsPath: string, payload: string | null) => void;
  /**
   * Read the pending payload for the given wsPath without clearing it.
   * Safe to call during render.
   */
  peekPendingPayload: (wsPath: string) => string | null;
  /**
   * Retrieve and immediately clear the pending payload for the given wsPath.
   * Returns `null` when nothing is pending.
   */
  takePendingPayload: (wsPath: string) => string | null;
}

// ---------------------------------------------------------------------------
// Internal reducer
// ---------------------------------------------------------------------------

interface SlotState {
  phase: WsPhase;
  messages: { id: number; raw: string }[];
}

type SlotAction =
  | { type: "CONNECTING"; path: string }
  | { type: "CONNECTED"; path: string; id: number; msg: string }
  | { type: "MESSAGE_RECEIVED"; path: string; id: number; msg: string }
  | { type: "DISCONNECTED"; path: string; id: number; msg: string }
  | { type: "CONNECT_ERROR"; path: string }
  | { type: "CLEAR_MESSAGES"; path: string };

type AllSlots = Record<string, SlotState>;

function appendMsg(
  slots: AllSlots,
  path: string,
  id: number,
  msg: string,
): AllSlots {
  const prev = slots[path] ?? { phase: "idle", messages: [] };
  const msgs = [...prev.messages, { id, raw: msg }];
  if (msgs.length > MAX_ITEMS) msgs.shift();
  return { ...slots, [path]: { ...prev, messages: msgs } };
}

function slotsReducer(state: AllSlots, action: SlotAction): AllSlots {
  const prev = state[action.path] ?? { phase: "idle", messages: [] };

  switch (action.type) {
    case "CONNECTING":
      return { ...state, [action.path]: { ...prev, phase: "connecting" } };

    case "CONNECTED":
      return appendMsg(
        { ...state, [action.path]: { ...prev, phase: "connected" } },
        action.path,
        action.id,
        action.msg,
      );

    case "MESSAGE_RECEIVED":
      return appendMsg(state, action.path, action.id, action.msg);

    case "DISCONNECTED":
      return appendMsg(
        { ...state, [action.path]: { ...prev, phase: "idle" } },
        action.path,
        action.id,
        action.msg,
      );

    case "CONNECT_ERROR":
      return { ...state, [action.path]: { ...prev, phase: "idle" } };

    case "CLEAR_MESSAGES":
      return { ...state, [action.path]: { ...prev, messages: [] } };

    default:
      return state;
  }
}

// ---------------------------------------------------------------------------
// Context
// ---------------------------------------------------------------------------

const WebSocketContext = createContext<WebSocketContextValue | null>(null);

// ---------------------------------------------------------------------------
// Provider
// ---------------------------------------------------------------------------

export function WebSocketProvider({ children }: { children: ReactNode }) {
  const [slots, dispatch] = useReducer(slotsReducer, {} as AllSlots);

  // Per-path refs live outside the reducer so they never cause re-renders.
  // wsRefs[path] = live WebSocket instance (or null)
  // pingRefs[path] = setInterval handle for keep-alive
  // msgIdRefs[path] = monotonic message counter
  const wsRefs = useRef<Record<string, WebSocket | null>>({});
  const pingRefs = useRef<
    Record<string, ReturnType<typeof setInterval> | null>
  >({});
  const msgIdRefs = useRef<Record<string, number>>({});

  // Tear down all connections when the provider unmounts (e.g. in tests, or if
  // the app tree is conditionally rendered) to avoid dangling sockets and intervals.
  useEffect(() => {
    return () => {
      Object.entries(wsRefs.current).forEach(([path, ws]) => {
        ws?.close();
        const ping = pingRefs.current[path];
        if (ping) clearInterval(ping);
      });
    };
  }, []);

  // Helper: derive a stable WebSocket URL from wsPath
  const makeUrl = (wsPath: string) => makeWsUrl(wsPath);

  // Helper: next message id for a path
  const nextId = (path: string) => {
    msgIdRefs.current[path] = (msgIdRefs.current[path] ?? 0) + 1;
    return msgIdRefs.current[path];
  };

  const getTimestamp = () => {
    const s = new Date().toString();
    const gmt = s.indexOf("GMT");
    return gmt > 0 ? s.substring(0, gmt).trim() : s;
  };

  const eventWithTimestamp = (type: string, message: string) =>
    JSON.stringify({ type, message, time: getTimestamp() });

  /**
   * Returns true for keep-alive ping/pong frames that should never appear in
   * the console.  Parses the JSON safely so it works regardless of key order
   * or whitespace differences in the serialised message.
   */
  const isKeepAliveMessage = (data: string): boolean => {
    try {
      const parsed = JSON.parse(data) as unknown;
      if (parsed !== null && typeof parsed === "object") {
        const type = (parsed as Record<string, unknown>).type;
        return type === "ping" || type === "pong";
      }
    } catch {
      // Not JSON — definitely not a keep-alive frame.
    }
    return false;
  };

  // ── connect ──────────────────────────────────────────────────────────────
  const connect = useCallback(
    (wsPath: string, onToast?: (msg: string, type?: ToastType) => void) => {
      if (!window.WebSocket) {
        onToast?.("WebSocket not supported by your browser", "error");
        return;
      }
      const existing = wsRefs.current[wsPath];
      if (
        existing &&
        (existing.readyState === WebSocket.OPEN ||
          existing.readyState === WebSocket.CONNECTING)
      ) {
        onToast?.("Already connected", "error");
        return;
      }

      dispatch({ type: "CONNECTING", path: wsPath });

      const ws = new WebSocket(makeUrl(wsPath));
      wsRefs.current[wsPath] = ws;

      ws.onopen = () => {
        dispatch({
          type: "CONNECTED",
          path: wsPath,
          id: nextId(wsPath),
          msg: eventWithTimestamp("info", "connected"),
        });
        onToast?.("Connected to WebSocket", "success");
        ws.send(JSON.stringify({ type: "welcome" }));

        pingRefs.current[wsPath] = setInterval(() => {
          if (ws.readyState === WebSocket.OPEN) {
            ws.send(eventWithTimestamp("ping", "keep alive"));
          }
        }, PING_INTERVAL);
      };

      ws.onmessage = (evt) => {
        if (!isKeepAliveMessage(evt.data)) {
          dispatch({
            type: "MESSAGE_RECEIVED",
            path: wsPath,
            id: nextId(wsPath),
            msg: evt.data,
          });
        }
      };

      ws.onerror = () => {
        dispatch({ type: "CONNECT_ERROR", path: wsPath });
      };

      ws.onclose = (evt) => {
        const ping = pingRefs.current[wsPath];
        if (ping) {
          clearInterval(ping);
          pingRefs.current[wsPath] = null;
        }
        dispatch({
          type: "DISCONNECTED",
          path: wsPath,
          id: nextId(wsPath),
          msg: eventWithTimestamp(
            "info",
            `disconnected - (${evt.code}) ${evt.reason}`,
          ),
        });
        onToast?.("Disconnected from WebSocket", "info");
        // Only clear the ref if it still points to THIS socket.  If a new
        // connect() call already stored a different socket (e.g. StrictMode
        // remount), leaving the ref alone keeps the new socket reachable.
        if (wsRefs.current[wsPath] === ws) {
          wsRefs.current[wsPath] = null;
        }
      };
    },
    [],
  );

  // ── disconnect ───────────────────────────────────────────────────────────
  const disconnect = useCallback((wsPath: string) => {
    const ws = wsRefs.current[wsPath];
    if (ws) {
      ws.close();
    } else {
      dispatch({
        type: "MESSAGE_RECEIVED",
        path: wsPath,
        id: nextId(wsPath),
        msg: eventWithTimestamp("error", "already disconnected"),
      });
    }
  }, []);

  // ── Auto-connect on startup ───────────────────────────────────────────────
  // Silently opens a WebSocket for every configured playground when the
  // provider first mounts.  Placed here — after connect/disconnect are defined
  // — so the closure captures the final, stable function references.
  //
  // The cleanup closes whatever sockets this effect opened.  This makes the
  // React StrictMode double-invoke cycle safe: the cleanup undoes the first
  // mount's connections so the second mount starts from a clean slate, and the
  // status dots (driven entirely by the reducer) accurately reflect each
  // transition (idle → connecting → connected, and back on cleanup).
  useEffect(() => {
    PLAYGROUND_CONFIGS.forEach((cfg) => {
      connect(cfg.wsPath); // silent — no onToast
    });
    return () => {
      PLAYGROUND_CONFIGS.forEach((cfg) => {
        const ws = wsRefs.current[cfg.wsPath];
        if (ws) ws.close();
      });
    };
    // connect is useCallback([]) — stable for the lifetime of the provider.
    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, []);

  // ── send ─────────────────────────────────────────────────────────────────
  const send = useCallback((wsPath: string, data: string): boolean => {
    const ws = wsRefs.current[wsPath];
    if (ws && ws.readyState === WebSocket.OPEN) {
      ws.send(data);
      return true;
    }
    return false;
  }, []);

  // ── appendMessage ────────────────────────────────────────────────────────
  const appendMessage = useCallback((wsPath: string, raw: string) => {
    dispatch({
      type: "MESSAGE_RECEIVED",
      path: wsPath,
      id: nextId(wsPath),
      msg: raw,
    });
  }, []);

  // ── clearMessages ────────────────────────────────────────────────────────
  const clearMessages = useCallback((wsPath: string) => {
    dispatch({ type: "CLEAR_MESSAGES", path: wsPath });
  }, []);

  // ── pendingPayload ────────────────────────────────────────────────────────
  // useState (not useRef) so that calling setPendingPayload triggers a
  // re-render in any consuming component — specifically the already-mounted
  // JSON-Path Playground whose useEffect polls for this value.
  const [pendingPayloads, setPendingPayloads] = useState<
    Record<string, string>
  >({});

  const setPendingPayload = useCallback(
    (wsPath: string, payload: string | null) => {
      setPendingPayloads((prev) => {
        if (payload === null) {
          const next = { ...prev };
          delete next[wsPath];
          return next;
        }
        return { ...prev, [wsPath]: payload };
      });
    },
    [],
  );

  const peekPendingPayload = useCallback(
    (wsPath: string): string | null => {
      return pendingPayloads[wsPath] ?? null;
    },
    [pendingPayloads],
  );

  const takePendingPayload = useCallback(
    (wsPath: string): string | null => {
      const value = pendingPayloads[wsPath] ?? null;
      if (value !== null) {
        setPendingPayloads((prev) => {
          const next = { ...prev };
          delete next[wsPath];
          return next;
        });
      }
      return value;
    },
    [pendingPayloads],
  );

  // ── getSlot ──────────────────────────────────────────────────────────────
  const getSlot = useCallback(
    (wsPath: string) => {
      return slots[wsPath] ?? { phase: "idle" as WsPhase, messages: [] };
    },
    [slots],
  );

  const value = useMemo(
    () => ({
      getSlot,
      connect,
      disconnect,
      send,
      appendMessage,
      clearMessages,
      setPendingPayload,
      peekPendingPayload,
      takePendingPayload,
    }),
    [
      getSlot,
      connect,
      disconnect,
      send,
      appendMessage,
      clearMessages,
      setPendingPayload,
      peekPendingPayload,
      takePendingPayload,
    ],
  );

  return (
    <WebSocketContext.Provider value={value}>
      {children}
    </WebSocketContext.Provider>
  );
}

// ---------------------------------------------------------------------------
// Consumer hook
// ---------------------------------------------------------------------------

export function useWebSocketContext(): WebSocketContextValue {
  const ctx = useContext(WebSocketContext);
  if (!ctx)
    throw new Error(
      "useWebSocketContext must be used inside <WebSocketProvider>",
    );
  return ctx;
}
