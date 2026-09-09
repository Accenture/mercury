// @vitest-environment happy-dom

import { act, renderHook } from '@testing-library/react';
import { afterEach, describe, expect, it, vi } from 'vitest';
import type { MinigraphGraphData } from '../../utils/graphTypes';
import { ProtocolBus } from '../../protocol/bus';
import { classifyMessage } from '../../protocol/classifier';
import { GRAPH_RUN_COMMANDS } from '../../graphRun/graphRunProtocol';
import { GRAPH_RUN_SETUP_TIMEOUT_MS, useGraphRunWorkflow } from '../useGraphRunWorkflow';
import { useMockUploadPanel } from '../useMockUploadPanel';

const graphWithoutInput: MinigraphGraphData = {
  nodes: [{ alias: 'root', types: ['Root'], properties: { name: 'hello' } }],
  connections: [],
};

const graphWithInput: MinigraphGraphData = {
  nodes: [{
    alias: 'mapper',
    types: ['Task'],
    properties: { mapping: ['input.body.user.id -> model.user_id'] },
  }],
  connections: [],
};

function emitRaw(bus: ProtocolBus, msgId: number, raw: string) {
  for (const event of classifyMessage(msgId, raw)) bus.emit(event);
}

function setup(graphData: MinigraphGraphData = graphWithoutInput) {
  const bus = new ProtocolBus();
  const sendRawText = vi.fn(() => true);
  const addToast = vi.fn();
  const onWorkflowInputInvalidated = vi.fn();
  const initialProps = {
    connected: true,
    connectionEpoch: 1 as number | null,
    graphData: graphData as MinigraphGraphData | null,
    graphIdentity: '/api/graph/model/current/1' as string | null,
    isPrimary: true,
  };
  const rendered = renderHook(
    (props: typeof initialProps) => useGraphRunWorkflow({
      enabled: true,
      bus,
      sendRawText,
      addToast,
      onWorkflowInputInvalidated,
      ...props,
    }),
    { initialProps },
  );
  return { ...rendered, bus, sendRawText, addToast, onWorkflowInputInvalidated, initialProps };
}

afterEach(() => {
  vi.useRealTimers();
});

describe('useGraphRunWorkflow', () => {
  it('rejects Run until a no-input graph is acknowledged Ready', () => {
    const { result, bus, sendRawText } = setup();

    expect(result.current.canRun).toBe(false);
    act(() => expect(result.current.runGraph()).toBe(false));
    expect(sendRawText).not.toHaveBeenCalled();

    act(() => expect(result.current.instantiateGraph()).toBe(true));
    expect(result.current.phase).toBe('instantiating');
    expect(sendRawText).toHaveBeenCalledTimes(1);
    expect(sendRawText).toHaveBeenNthCalledWith(1, GRAPH_RUN_COMMANDS.instantiate);

    act(() => emitRaw(bus, 1, 'Graph instance created. Loaded 0 mock entries, model.ttl = 30000 ms'));
    expect(result.current.phase).toBe('ready');
    expect(result.current.canRun).toBe(true);
    expect(sendRawText).toHaveBeenCalledTimes(1);

    act(() => expect(result.current.runGraph()).toBe(true));
    expect(sendRawText).toHaveBeenNthCalledWith(2, GRAPH_RUN_COMMANDS.run);
    expect(result.current.phase).toBe('running');

    act(() => emitRaw(bus, 2, 'Graph traversal completed in 8 ms'));
    expect(result.current.phase).toBe('idle');
  });

  it('waits for JSON upload success before enabling Run for a graph with body references', () => {
    const { result, bus, sendRawText } = setup(graphWithInput);

    act(() => expect(result.current.runGraph()).toBe(false));
    act(() => result.current.instantiateGraph());
    act(() => emitRaw(bus, 1, 'Graph instance created. Loaded 0 mock entries, model.ttl = 30000 ms'));

    expect(sendRawText).toHaveBeenNthCalledWith(2, GRAPH_RUN_COMMANDS.requestInputUpload);
    expect(sendRawText).not.toHaveBeenCalledWith(GRAPH_RUN_COMMANDS.run);
    expect(result.current.phase).toBe('requesting-input');

    act(() => emitRaw(bus, 2, 'You may upload JSON payload -> POST /api/mock/ws-123-1'));
    expect(result.current.phase).toBe('awaiting-input');
    expect(result.current.isWorkflowInputPanel).toBe(true);
    expect(result.current.inputBodyPaths).toEqual(['input.body.user.id']);

    act(() => expect(result.current.handleInputUploadSuccess('/api/mock/ws-123-1')).toBe(true));
    expect(result.current.phase).toBe('ready');
    expect(result.current.canRun).toBe(true);
    expect(sendRawText).toHaveBeenCalledTimes(2);

    act(() => expect(result.current.runGraph()).toBe(true));
    expect(sendRawText).toHaveBeenNthCalledWith(3, GRAPH_RUN_COMMANDS.run);
    expect(result.current.phase).toBe('running');
  });

  it('instantiates only, becomes Ready, and then runs without re-instantiating', () => {
    const { result, bus, sendRawText } = setup(graphWithInput);

    act(() => result.current.instantiateGraph());
    act(() => emitRaw(bus, 1, 'Graph instance created. Loaded 0 mock entries, model.ttl = 30000 ms'));
    act(() => emitRaw(bus, 2, 'You may upload JSON payload -> POST /api/mock/ws-123-1'));
    act(() => result.current.handleInputUploadSuccess('/api/mock/ws-123-1'));

    expect(result.current.phase).toBe('ready');
    expect(result.current.ready).toBe(true);
    expect(sendRawText).toHaveBeenCalledTimes(2);

    act(() => result.current.runGraph());
    expect(sendRawText).toHaveBeenNthCalledWith(3, GRAPH_RUN_COMMANDS.run);
    expect(result.current.phase).toBe('running');
  });

  it('mirrors manual console instantiate/run events', () => {
    const { result, bus, sendRawText } = setup();

    act(() => emitRaw(bus, 1, '> instantiate graph'));
    act(() => emitRaw(bus, 2, 'Graph instance created. Loaded 1 mock entry, model.ttl = 30000 ms'));
    expect(result.current.phase).toBe('ready');

    act(() => result.current.runGraph());
    expect(sendRawText).toHaveBeenCalledWith(GRAPH_RUN_COMMANDS.run);
  });

  it('invalidates stale Ready and pending intent on graph/session lifecycle changes', () => {
    const { result, bus, rerender, initialProps } = setup();

    act(() => emitRaw(bus, 1, '> instantiate graph'));
    act(() => emitRaw(bus, 2, 'Graph instance created. Loaded 0 mock entries, model.ttl = 30000 ms'));
    expect(result.current.ready).toBe(true);
    act(() => emitRaw(bus, 3, 'node root updated'));
    expect(result.current.phase).toBe('idle');

    act(() => emitRaw(bus, 4, '> instantiate graph'));
    act(() => emitRaw(bus, 5, 'Graph instance created. Loaded 0 mock entries, model.ttl = 30000 ms'));
    act(() => rerender({ ...initialProps, graphIdentity: '/api/graph/model/other/2' }));
    expect(result.current.phase).toBe('idle');

    act(() => emitRaw(bus, 6, '> instantiate graph'));
    act(() => emitRaw(bus, 7, 'Graph instance created. Loaded 0 mock entries, model.ttl = 30000 ms'));
    act(() => rerender({ ...initialProps, connected: false }));
    expect(result.current.phase).toBe('idle');
  });

  it('quarantines a late instantiate acknowledgement after mutation instead of exposing stale Ready', () => {
    const { result, bus, sendRawText } = setup();

    act(() => emitRaw(bus, 1, '> instantiate graph'));
    expect(result.current.phase).toBe('instantiating');
    act(() => emitRaw(bus, 2, 'node root updated'));
    expect(result.current.phase).toBe('outcome-uncertain');

    act(() => emitRaw(bus, 3, 'Graph instance created. Loaded 0 mock entries, model.ttl = 30000 ms'));
    expect(result.current.phase).toBe('idle');
    expect(sendRawText).not.toHaveBeenCalledWith(GRAPH_RUN_COMMANDS.run);

    act(() => emitRaw(bus, 4, '> instantiate graph'));
    act(() => emitRaw(bus, 5, 'Graph instance created. Loaded 0 mock entries, model.ttl = 30000 ms'));
    expect(result.current.phase).toBe('ready');
  });

  it('uses an acknowledged session reset to clear an uncertain backend outcome', () => {
    vi.useFakeTimers();
    const { result, bus } = setup();

    act(() => result.current.instantiateGraph());
    act(() => vi.advanceTimersByTime(GRAPH_RUN_SETUP_TIMEOUT_MS));
    expect(result.current.phase).toBe('outcome-uncertain');

    act(() => bus.emit({
      kind: 'session.reset',
      msgId: 2,
      raw: 'Session restarted',
    }));
    expect(result.current.phase).toBe('idle');
  });

  it('closes only workflow-owned input when lifecycle invalidation occurs', () => {
    const { result, bus, onWorkflowInputInvalidated } = setup(graphWithInput);

    act(() => result.current.instantiateGraph());
    act(() => emitRaw(bus, 1, 'Graph instance created. Loaded 0 mock entries, model.ttl = 30000 ms'));
    act(() => emitRaw(bus, 2, 'You may upload JSON payload -> POST /api/mock/ws-123-1'));
    act(() => emitRaw(bus, 3, 'node root updated'));

    expect(onWorkflowInputInvalidated).toHaveBeenCalledWith('/api/mock/ws-123-1');
    expect(result.current.phase).toBe('idle');
  });

  it('does not let an unrelated upload panel complete or cancel graph input', () => {
    const { result, bus } = setup(graphWithInput);

    act(() => result.current.instantiateGraph());
    act(() => emitRaw(bus, 1, 'Graph instance created. Loaded 0 mock entries, model.ttl = 30000 ms'));
    act(() => emitRaw(bus, 2, 'You may upload JSON payload -> POST /api/mock/workflow'));

    expect(result.current.workflowUploadPath).toBe('/api/mock/workflow');
    act(() => expect(result.current.handleInputUploadSuccess('/api/mock/manual')).toBe(false));
    act(() => expect(result.current.handleInputCancelled('/api/mock/manual')).toBe(false));
    expect(result.current.phase).toBe('awaiting-input');
    expect(result.current.canRun).toBe(false);

    act(() => expect(result.current.handleInputUploadSuccess('/api/mock/workflow')).toBe(true));
    expect(result.current.phase).toBe('ready');
    expect(result.current.canRun).toBe(true);
  });

  it('queues workflow input behind an open manual upload panel and resumes it after close', () => {
    const bus = new ProtocolBus();
    const sendRawText = vi.fn(() => true);
    const addToast = vi.fn();
    const { result } = renderHook(() => {
      const uploadPanel = useMockUploadPanel({ bus, addToast });
      const graphRun = useGraphRunWorkflow({
        enabled: true,
        bus,
        connected: true,
        connectionEpoch: 1,
        graphData: graphWithInput,
        graphIdentity: '/api/graph/model/current/1',
        isPrimary: true,
        sendRawText,
        addToast,
        onWorkflowInputInvalidated: uploadPanel.handleCloseUploadPath,
      });
      return { uploadPanel, graphRun };
    });

    act(() => result.current.uploadPanel.handleOpenUploadPanel('/api/mock/manual'));
    act(() => result.current.graphRun.instantiateGraph());
    act(() => emitRaw(bus, 1, 'Graph instance created. Loaded 0 mock entries, model.ttl = 30000 ms'));
    act(() => emitRaw(bus, 2, 'You may upload JSON payload -> POST /api/mock/workflow'));

    expect(result.current.uploadPanel.uploadPanelPath).toBe('/api/mock/manual');
    expect(result.current.graphRun.phase).toBe('awaiting-input');
    act(() => {
      result.current.uploadPanel.handleCloseUploadPanel();
      expect(result.current.graphRun.handleInputCancelled('/api/mock/manual')).toBe(false);
    });
    expect(result.current.uploadPanel.uploadPanelPath).toBe('/api/mock/workflow');

    act(() => {
      result.current.uploadPanel.handleUploadSuccess('ok');
      expect(result.current.graphRun.handleInputUploadSuccess('/api/mock/workflow')).toBe(true);
    });
    expect(result.current.uploadPanel.uploadPanelPath).toBeNull();
    expect(result.current.graphRun.phase).toBe('ready');
    expect(result.current.graphRun.canRun).toBe(true);
  });

  it('invalidates Ready when Save Graph confirms an export', () => {
    const { result, bus } = setup();

    act(() => emitRaw(bus, 1, '> instantiate graph'));
    act(() => emitRaw(bus, 2, 'Graph instance created. Loaded 0 mock entries, model.ttl = 30000 ms'));
    expect(result.current.ready).toBe(true);

    act(() => bus.emit({
      kind: 'graph.exported',
      msgId: 3,
      raw: 'Graph exported to /tmp/example\nDescribed in /api/graph/model/example/1',
      graphName: 'example',
      apiPath: '/api/graph/model/example/1',
    }));
    expect(result.current.phase).toBe('idle');
  });

  it('does not repeat a no-input instantiate command while the graph is already Ready', () => {
    const { result, bus, sendRawText } = setup();

    act(() => result.current.instantiateGraph());
    act(() => emitRaw(bus, 1, 'Graph instance created. Loaded 0 mock entries, model.ttl = 30000 ms'));
    expect(result.current.canInstantiate).toBe(false);

    act(() => expect(result.current.instantiateGraph()).toBe(false));
    expect(sendRawText).toHaveBeenCalledTimes(1);
    expect(result.current.phase).toBe('ready');
  });

  it('cancels workflow input without running, handles setup errors, and times out missing acknowledgements', () => {
    vi.useFakeTimers();
    const { result, bus, sendRawText, addToast } = setup(graphWithInput);

    act(() => result.current.instantiateGraph());
    act(() => emitRaw(bus, 1, 'Graph instance created. Loaded 0 mock entries, model.ttl = 30000 ms'));
    act(() => emitRaw(bus, 2, 'You may upload JSON payload -> POST /api/mock/ws-123-1'));
    act(() => expect(result.current.handleInputCancelled('/api/mock/ws-123-1')).toBe(true));
    expect(result.current.phase).toBe('idle');
    expect(sendRawText).not.toHaveBeenCalledWith(GRAPH_RUN_COMMANDS.run);

    act(() => result.current.instantiateGraph());
    act(() => emitRaw(bus, 3, 'ERROR: Root node does not exist'));
    expect(result.current.phase).toBe('idle');
    expect(addToast).toHaveBeenCalledWith('Could not instantiate graph: Root node does not exist', 'error');

    act(() => result.current.instantiateGraph());
    act(() => vi.advanceTimersByTime(GRAPH_RUN_SETUP_TIMEOUT_MS));
    expect(result.current.phase).toBe('outcome-uncertain');
    expect(result.current.busy).toBe(true);
    act(() => expect(result.current.instantiateGraph()).toBe(false));
    expect(addToast).toHaveBeenCalledWith(
      'Graph setup is taking longer than expected. Waiting for the backend outcome…',
      'info',
    );

    act(() => emitRaw(bus, 4, 'Graph instance created. Loaded 0 mock entries, model.ttl = 30000 ms'));
    expect(result.current.phase).toBe('requesting-input');
  });

  it('rejects actions when the graph cannot be controlled by this session', () => {
    const { result, rerender, initialProps, sendRawText } = setup();

    act(() => rerender({ ...initialProps, isPrimary: false }));
    expect(result.current.canInteract).toBe(false);
    act(() => expect(result.current.runGraph()).toBe(false));
    expect(sendRawText).not.toHaveBeenCalled();
  });
});
