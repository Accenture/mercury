import { useEffect, useRef } from 'react';
import { type ProtocolBus } from '../protocol/bus';
import { type ToastType } from './useToast';

export interface UseAutoGraphRefreshOptions {
  bus:                ProtocolBus;
  pinnedGraphPath:    string | null;
  setPinnedGraphPath: (path: string | null) => void;
  connected:          boolean;
  sendRawText:        (text: string) => void;
  addToast:           (message: string, type?: ToastType) => void;
}

/**
 * Watches the ProtocolBus for graph mutation and graph-link events and
 * automatically re-renders the graph without requiring user interaction.
 */
export function useAutoGraphRefresh({
  bus,
  pinnedGraphPath,
  setPinnedGraphPath,
  connected,
  sendRawText,
  addToast,
}: UseAutoGraphRefreshOptions): void {

  const debounceTimerRef = useRef<ReturnType<typeof setTimeout> | null>(null);
  const waitingForDescribeRef = useRef(false);
  const pinnedGraphPathRef = useRef<string | null>(pinnedGraphPath);
  const connectedRef = useRef(connected);
  const sendRawTextRef = useRef(sendRawText);

  // Stale-closure fixes
  useEffect(() => { pinnedGraphPathRef.current = pinnedGraphPath; }, [pinnedGraphPath]);
  useEffect(() => { connectedRef.current = connected; }, [connected]);
  useEffect(() => { sendRawTextRef.current = sendRawText; }, [sendRawText]);

  // Reset waitingForDescribeRef on disconnect
  useEffect(() => {
    if (!connected) {
      waitingForDescribeRef.current = false;
      if (debounceTimerRef.current !== null) {
        clearTimeout(debounceTimerRef.current);
        debounceTimerRef.current = null;
      }
    }
  }, [connected]);

  // Subscribe to graph.link (consume pending describe response)
  useEffect(() => {
    return bus.on('graph.link', (event) => {
      if (waitingForDescribeRef.current) {
        waitingForDescribeRef.current = false;
        setPinnedGraphPath(event.apiPath);
      }
    });
  }, [bus, setPinnedGraphPath]);

  // Subscribe to graph.mutation
  useEffect(() => {
    return bus.on('graph.mutation', (event) => {
      if (!connectedRef.current) return;

      if (event.mutationType === 'import-graph') {
        if (debounceTimerRef.current !== null) {
          clearTimeout(debounceTimerRef.current);
          debounceTimerRef.current = null;
        }
        waitingForDescribeRef.current = true;
        sendRawTextRef.current('describe graph');
        addToast('Graph imported — refreshing view…', 'info');
        return;
      }

      // Arm gate immediately: a graph.link forwarded by the primary during
      // the debounce window must be accepted before this session sends its
      // own describe graph.
      waitingForDescribeRef.current = true;

      // node-mutation → debounce
      if (debounceTimerRef.current !== null) {
        clearTimeout(debounceTimerRef.current);
      }
      debounceTimerRef.current = setTimeout(() => {
        debounceTimerRef.current = null;
        if (!connectedRef.current) return;
        waitingForDescribeRef.current = true; // re-arm: may have been reset by an early forwarded graph.link
        sendRawTextRef.current('describe graph');
        addToast(
          pinnedGraphPathRef.current !== null
            ? 'Graph updated — refreshing…'
            : 'Graph updated — opening Graph tab…',
          'info',
        );
      }, 300);
    });
  }, [bus, addToast]);

  // Subscribe to session.reset (session restarted — clear stale graph view)
  useEffect(() => {
    return bus.on('session.reset', () => {
      if (debounceTimerRef.current !== null) {
        clearTimeout(debounceTimerRef.current);
        debounceTimerRef.current = null;
      }
      waitingForDescribeRef.current = false;
      setPinnedGraphPath(null);
    });
  }, [bus, setPinnedGraphPath]);

  // Cleanup debounce on unmount
  useEffect(() => {
    return () => {
      if (debounceTimerRef.current !== null) {
        clearTimeout(debounceTimerRef.current);
      }
    };
  }, []);
}
