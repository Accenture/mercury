import { useEffect, useRef } from 'react';
import { type ProtocolBus } from '../protocol/bus';
import { type ToastType } from './useToast';

export interface UseLargePayloadDownloadOptions {
  bus:            ProtocolBus;
  connected:      boolean;
  appendMessage:  (raw: string) => void;
  addToast:       (message: string, type?: ToastType) => void;
}

/**
 * Subscribes to `payload.large` events on the ProtocolBus.
 * When the server sends a large-payload notification, this hook fetches
 * the payload and appends it to the console.
 */
export function useLargePayloadDownload({
  bus,
  connected,
  appendMessage,
  addToast,
}: UseLargePayloadDownloadOptions): void {

  const abortRef = useRef<AbortController | null>(null);
  const isFetchingRef = useRef<boolean>(false);

  // Ref-wrap unstable callbacks so the bus subscription never re-subscribes.
  // Same pattern as sendRawTextRef in useAutoGraphRefresh.
  const appendMessageRef = useRef(appendMessage);
  useEffect(() => { appendMessageRef.current = appendMessage; }, [appendMessage]);

  const addToastRef = useRef(addToast);
  useEffect(() => { addToastRef.current = addToast; }, [addToast]);

  // Cancel in-flight fetch on disconnect
  useEffect(() => {
    if (!connected) {
      abortRef.current?.abort();
      abortRef.current = null;
      isFetchingRef.current = false;
    }
  }, [connected]);

  // Cancel in-flight fetch on unmount
  useEffect(() => {
    return () => { abortRef.current?.abort(); };
  }, []);

  // Subscribe to payload.large events
  useEffect(() => {
    return bus.on('payload.large', (event) => {
      // Re-entrancy guard: skip while a fetch is in flight
      if (isFetchingRef.current) return;

      const { apiPath, byteSize } = event;

      abortRef.current?.abort();
      const controller = new AbortController();
      abortRef.current = controller;

      const sizeMB = (byteSize / (1024 * 1024)).toFixed(2);
      addToastRef.current(`Fetching large payload (${sizeMB} MB)…`, 'info');

      isFetchingRef.current = true;

      fetch(apiPath, { signal: controller.signal })
        .then(res => {
          if (!res.ok) throw new Error(`HTTP ${res.status}`);
          return res.text();
        })
        .then(text => {
          if (!text.trim()) throw new Error('empty response body');

          let content = text;
          try { content = JSON.stringify(JSON.parse(text), null, 2); } catch { /* not JSON — pass raw */ }

          appendMessageRef.current(content);
          isFetchingRef.current = false;
          abortRef.current = null;
        })
        .catch((err: Error) => {
          if (err.name === 'AbortError') return;
          isFetchingRef.current = false;
          abortRef.current = null;
          appendMessageRef.current(`ERROR: payload fetch failed — ${err.message}`);
          addToastRef.current(`Payload fetch failed: ${err.message}`, 'error');
        });
    });
  }, [bus]);
}
