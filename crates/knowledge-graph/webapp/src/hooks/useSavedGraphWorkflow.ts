import { useCallback, useEffect, useRef } from 'react';
import { type ProtocolBus } from '../protocol/bus';
import { type ToastType } from './useToast';

export interface UseSavedGraphWorkflowOptions {
  bus:              ProtocolBus;
  connected:        boolean;
  sendRawText:      (text: string) => void;
  /** Persists confirmed exports when this playground supports saved graphs. */
  saveGraph:        ((name: string) => void) | null;
  addToast:         (message: string, type?: ToastType) => void;
}

export interface UseSavedGraphWorkflowReturn {
  /** Save the current graph under the given name. Requires an active connection — defers localStorage write until server confirms export. */
  handleSaveGraph: (name: string) => void;
  /** Load a saved graph by sending `import graph from {name}` over the WebSocket. */
  handleLoadGraph: (name: string) => void;
}

type PendingSave = {
  graphName: string;
  timeoutId: ReturnType<typeof setTimeout>;
};

/** Persist every server-confirmed export, regardless of whether UI or console initiated it. */
export function subscribeGraphExportBookmarks(
  bus: ProtocolBus,
  saveGraph: (name: string) => void,
): () => void {
  return bus.on('graph.exported', event => saveGraph(event.graphName));
}

/**
 * Orchestrates the save/load workflow for named graph snapshots.
 *
 * **Save flow (requires active connection):**
 *   1. Sends `export graph as {name}` over the WebSocket.
 *   2. Arms a 10-second timeout as a safety net.
 *   3. Waits for the server's response:
 *      - `graph.exported` with matching graphName → confirms the pending UI save.
 *      - `graph.export.failed` → rejects and shows a typed error toast.
 *      - timeout → rejects with a timeout toast.
 *      - disconnect → clears pending and shows a disconnect toast.
 *   4. If called while disconnected, shows an error toast and returns immediately — no bookmark
 *      is written. A bookmark without a server-side export file cannot be loaded.
 *
 * Every `graph.exported` event writes a bookmark when saved graphs are enabled.
 * This gives exports entered in the command console the same Saved + Load Graph
 * state as exports submitted through the Save Graph button. The protocol-driven
 * saved indicator remains owned by `useGraphSaveName`.
 *
 * **Load flow:**
 *   Sends `import graph from {name}` — the backend reads the previously exported JSON file.
 */
export function useSavedGraphWorkflow({
  bus,
  connected,
  sendRawText,
  saveGraph,
  addToast,
}: UseSavedGraphWorkflowOptions): UseSavedGraphWorkflowReturn {
  // Tracks a save that is waiting for server confirmation.
  const pendingSaveRef = useRef<PendingSave | null>(null);

  // Save the current graph under the given name.
  const handleSaveGraph = useCallback((name: string) => {
    if (!connected) {
      // Disconnected guard: a bookmark without a server-side export file cannot be
      // loaded after reconnect, so optimistic local saves must never be created.
      addToast('Save failed: connection required to export graph', 'error');
      return;
    }
    // Connected path: arm the ref, start the timeout, send the export command.
    // Saved UI state updates independently when graph.exported reaches the bus.
    const timeoutId = setTimeout(() => {
      if (pendingSaveRef.current === null) return; // already resolved — do nothing
      pendingSaveRef.current = null;
      addToast('Save failed: export confirmation timed out', 'error');
    }, 10_000);
    pendingSaveRef.current = { graphName: name, timeoutId };
    sendRawText(`export graph as ${name}`);
  }, [connected, sendRawText, addToast]);

  // A server-confirmed export is loadable regardless of which UI initiated it.
  useEffect(() => {
    if (saveGraph === null) return;
    return subscribeGraphExportBookmarks(bus, saveGraph);
  }, [bus, saveGraph]);

  // Confirm pending UI save and provide its action-specific success feedback.
  useEffect(() => {
    return bus.on('graph.exported', (event) => {
      if (pendingSaveRef.current === null) return;
      if (event.graphName !== pendingSaveRef.current.graphName) return; // not our export
      clearTimeout(pendingSaveRef.current.timeoutId);
      const name = pendingSaveRef.current.graphName;
      pendingSaveRef.current = null;
      addToast(`Graph saved as "${name}"`, 'success');
    });
  }, [bus, addToast]);

  // Reject pending save: server returned an export-failure event.
  useEffect(() => {
    return bus.on('graph.export.failed', (event) => {
      if (pendingSaveRef.current === null) return;
      clearTimeout(pendingSaveRef.current.timeoutId);
      pendingSaveRef.current = null;
      if (event.reason === 'invalid-name') {
        addToast('Save failed: invalid filename (a\u2013z, A\u2013Z, 0\u20139, hyphen only)', 'error');
      } else {
        addToast('Save failed: root node name does not match existing graph', 'error');
      }
    });
  }, [bus, addToast]);

  // Disconnect cleanup: if a save was in-flight when the connection dropped, fail it.
  useEffect(() => {
    if (!connected && pendingSaveRef.current !== null) {
      clearTimeout(pendingSaveRef.current.timeoutId);
      pendingSaveRef.current = null;
      addToast('Save failed: connection closed before export confirmation', 'error');
    }
  }, [connected, addToast]);

  // Unmount cleanup: cancel any in-flight timeout to prevent stale addToast calls.
  useEffect(() => {
    return () => {
      if (pendingSaveRef.current !== null) {
        clearTimeout(pendingSaveRef.current.timeoutId);
      }
    };
  }, []);

  // Load a saved graph by sending `import graph from {name}` over the WebSocket.
  const handleLoadGraph = useCallback((name: string) => {
    if (!connected) return;
    sendRawText(`import graph from ${name}`);
    addToast(`Importing graph "${name}"…`, 'info');
  }, [connected, sendRawText, addToast]);

  return { handleSaveGraph, handleLoadGraph };
}
