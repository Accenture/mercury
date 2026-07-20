import { useCallback, useEffect, useRef } from 'react';
import { type WebSocketContextValue } from '../contexts/WebSocketContext';
import { PLAYGROUND_CONFIGS } from '../config/playgrounds';
import { type ToastType } from './useToast';

export interface UseSendToJsonPathOptions {
  /** The WebSocket context (needed for getSlot, setPendingPayload, connect). */
  ctx:      WebSocketContextValue;
  /** react-router-dom navigate function. */
  navigate: (to: string) => void;
  /** Toast callback. */
  addToast: (message: string, type?: ToastType) => void;
  /** The current playground's wsPath — used to prevent sending to self. */
  wsPath:   string;
}

export interface UseSendToJsonPathReturn {
  /**
   * Callback to send JSON to the JSON-Path playground.
   * undefined when JSON-Path config doesn't exist or the current playground IS JSON-Path.
   */
  handleSendToJsonPath: ((json: string) => void) | undefined;
}

/**
 * Manages the cross-playground transfer of inline JSON responses to the
 * JSON-Path playground's payload editor.
 *
 * When the target playground is already connected, the payload is deposited
 * immediately via the WebSocket context and navigation occurs.  When the
 * target is not yet connected, the hook arms a deferred send and auto-connects;
 * a watching effect fires the transfer once the slot reaches 'connected' phase.
 */
export function useSendToJsonPath({
  ctx,
  navigate,
  addToast,
  wsPath,
}: UseSendToJsonPathOptions): UseSendToJsonPathReturn {
  // Find the JSON-Path playground config (static — derived from PLAYGROUND_CONFIGS).
  const jsonPathConfig = PLAYGROUND_CONFIGS.find(
    c => c.tabs.includes('payload') && c.supportsUpload
  );

  // Single-slot mailbox for deferred JSON transfers — last-write-wins while the
  // target playground is connecting.  Only the most recently requested payload
  // is deposited when the connection opens; earlier payloads are intentionally
  // discarded (see D5 in the spec).
  const pendingJsonTransferRef = useRef<{ wsPath: string; json: string } | null>(null);

  // Watch the JSON-Path slot phase; when it reaches 'connected' and there is a
  // deferred send pending, execute it and clear the ref.
  const jsonPathWsPath = jsonPathConfig?.wsPath;
  useEffect(() => {
    if (!jsonPathWsPath || !pendingJsonTransferRef.current) return;
    const slot = ctx.getSlot(jsonPathWsPath);
    if (slot.phase === 'connected') {
      const { wsPath: targetPath, json } = pendingJsonTransferRef.current;
      pendingJsonTransferRef.current = null;
      ctx.setPendingPayload(targetPath, json);
      navigate(jsonPathConfig!.path);
      addToast('JSON loaded into JSON-Path editor ✓', 'success');
    }
  }, [jsonPathWsPath, ctx, navigate, addToast, jsonPathConfig,
      // getSlot returns a new object reference when the slot changes, so
      // reading it inside the effect (keyed on ctx) is sufficient — but we
      // also need to re-run when the slot's phase changes.  ctx.getSlot is
      // wrapped in useCallback(_, [slots]) so it changes whenever slots does,
      // which is exactly what we want.
     ]);

  const handleSendToJsonPathInner = useCallback((json: string) => {
    if (!jsonPathConfig) return;
    const slot = ctx.getSlot(jsonPathConfig.wsPath);
    if (slot.phase === 'connected') {
      // Already connected — deposit immediately.
      ctx.setPendingPayload(jsonPathConfig.wsPath, json);
      navigate(jsonPathConfig.path);
      addToast('JSON loaded into JSON-Path editor ✓', 'success');
    } else if (slot.phase === 'connecting') {
      // Last-write-wins while target is connecting — overwrite the pending slot;
      // do not call ctx.connect() again (connection attempt already in progress).
      pendingJsonTransferRef.current = { wsPath: jsonPathConfig.wsPath, json };
      addToast('Updated pending JSON transfer — latest payload will open when connected', 'info');
    } else {
      // 'idle': store payload and start the connection.
      pendingJsonTransferRef.current = { wsPath: jsonPathConfig.wsPath, json };
      ctx.connect(jsonPathConfig.wsPath, addToast);
      addToast('Connecting to JSON-Path Playground…', 'info');
    }
  }, [ctx, navigate, addToast, jsonPathConfig]);

  // Return undefined when feature is unavailable (no config or sending to self).
  const handleSendToJsonPath =
    jsonPathConfig && wsPath !== jsonPathConfig.wsPath
      ? handleSendToJsonPathInner
      : undefined;

  return { handleSendToJsonPath };
}
