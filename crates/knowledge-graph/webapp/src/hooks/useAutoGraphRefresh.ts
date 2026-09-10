import { useEffect, useRef } from 'react';
import { type ProtocolBus } from '../protocol/bus';
import { type ToastType } from './useToast';

export interface UseAutoGraphRefreshOptions {
  bus:          ProtocolBus;
  /** True while a graph is currently rendered — drives the toast wording. */
  hasGraph:     boolean;
  connected:    boolean;
  /** useGraphData's imperative live-endpoint re-fetch (stable reference). */
  refetchGraph: () => void;
  /** Clears the rendered graph (session restarted — the old graph is gone). */
  clearGraph:   () => void;
  addToast:     (message: string, type?: ToastType) => void;
}

/**
 * Watches the ProtocolBus for graph mutation events and automatically
 * re-fetches the live session graph (`GET /api/graph/session/{id}`) without
 * requiring user interaction.
 *
 * Node mutations are debounced (a burst of authoring commands lands as one
 * re-fetch); a graph import re-fetches immediately. In a collaboration
 * session every member sees the propagated commands' replies in its own
 * console, so each member's bus emits the same mutation events and each
 * re-fetches its own session replica — no graph-link forwarding involved.
 */
export function useAutoGraphRefresh({
  bus,
  hasGraph,
  connected,
  refetchGraph,
  clearGraph,
  addToast,
}: UseAutoGraphRefreshOptions): void {

  const debounceTimerRef = useRef<ReturnType<typeof setTimeout> | null>(null);
  const hasGraphRef = useRef(hasGraph);
  const connectedRef = useRef(connected);
  const refetchGraphRef = useRef(refetchGraph);

  // Stale-closure fixes
  useEffect(() => { hasGraphRef.current = hasGraph; }, [hasGraph]);
  useEffect(() => { connectedRef.current = connected; }, [connected]);
  useEffect(() => { refetchGraphRef.current = refetchGraph; }, [refetchGraph]);

  // Cancel a pending refresh on disconnect
  useEffect(() => {
    if (!connected && debounceTimerRef.current !== null) {
      clearTimeout(debounceTimerRef.current);
      debounceTimerRef.current = null;
    }
  }, [connected]);

  // Subscribe to graph.mutation
  useEffect(() => {
    return bus.on('graph.mutation', (event) => {
      if (!connectedRef.current) return;

      if (event.mutationType === 'import-graph') {
        if (debounceTimerRef.current !== null) {
          clearTimeout(debounceTimerRef.current);
          debounceTimerRef.current = null;
        }
        refetchGraphRef.current();
        addToast('Graph imported — refreshing view…', 'info');
        return;
      }

      // node-mutation → debounce
      if (debounceTimerRef.current !== null) {
        clearTimeout(debounceTimerRef.current);
      }
      debounceTimerRef.current = setTimeout(() => {
        debounceTimerRef.current = null;
        if (!connectedRef.current) return;
        refetchGraphRef.current();
        addToast(
          hasGraphRef.current
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
      clearGraph();
    });
  }, [bus, clearGraph]);

  // Cleanup debounce on unmount
  useEffect(() => {
    return () => {
      if (debounceTimerRef.current !== null) {
        clearTimeout(debounceTimerRef.current);
      }
    };
  }, []);
}
