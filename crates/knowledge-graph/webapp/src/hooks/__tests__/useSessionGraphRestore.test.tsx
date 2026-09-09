// @vitest-environment happy-dom

import { renderHook } from '@testing-library/react';
import { afterEach, beforeEach, describe, expect, it, vi } from 'vitest';
import { useSessionGraphRestore, type UseSessionGraphRestoreOptions } from '../useSessionGraphRestore';

const LIVE_GRAPH = {
  nodes: [{ alias: 'root', types: ['Root'], properties: {} }],
  connections: [],
};

function okResponse(body: unknown) {
  return { ok: true, status: 200, json: async () => body };
}

function errorResponse(status: number) {
  return { ok: false, status, json: async () => ({}) };
}

function baseOptions(overrides: Partial<UseSessionGraphRestoreOptions> = {}): UseSessionGraphRestoreOptions {
  return {
    enabled: true,
    sessionGraphPath: '/api/graph/session/ws-1-1',
    pinnedGraphPath: '/api/graph/model/ws-1-1/9-1',
    initialFetchFailed: true,
    graphData: null,
    setGraphData: vi.fn(),
    setRightTab: vi.fn(),
    ...overrides,
  };
}

describe('useSessionGraphRestore', () => {
  const fetchMock = vi.fn();

  beforeEach(() => {
    fetchMock.mockReset();
    vi.stubGlobal('fetch', fetchMock);
  });

  afterEach(() => {
    vi.unstubAllGlobals();
  });

  it('restores the live session graph after the pinned model path failed', async () => {
    fetchMock.mockResolvedValue(okResponse(LIVE_GRAPH));
    const options = baseOptions();

    renderHook(() => useSessionGraphRestore(options));

    await vi.waitFor(() => expect(options.setGraphData).toHaveBeenCalledWith(LIVE_GRAPH));
    expect(fetchMock).toHaveBeenCalledTimes(1);
    expect(fetchMock.mock.calls[0][0]).toBe('/api/graph/session/ws-1-1');
    expect(options.setRightTab).toHaveBeenCalledWith('graph');
  });

  it('waits while a pinned model path has not delivered its verdict', () => {
    const options = baseOptions({ initialFetchFailed: false });

    renderHook(() => useSessionGraphRestore(options));

    expect(fetchMock).not.toHaveBeenCalled();
  });

  it('restores when no model path is pinned at all (return navigation without describe)', async () => {
    fetchMock.mockResolvedValue(okResponse(LIVE_GRAPH));
    const options = baseOptions({ pinnedGraphPath: null, initialFetchFailed: false });

    renderHook(() => useSessionGraphRestore(options));

    await vi.waitFor(() => expect(options.setGraphData).toHaveBeenCalledWith(LIVE_GRAPH));
  });

  it('does nothing while disabled, without a session id, or with graph data already present', () => {
    renderHook(() => useSessionGraphRestore(baseOptions({ enabled: false })));
    renderHook(() => useSessionGraphRestore(baseOptions({ sessionGraphPath: null })));
    renderHook(() => useSessionGraphRestore(baseOptions({ graphData: LIVE_GRAPH })));

    expect(fetchMock).not.toHaveBeenCalled();
  });

  it('stays silent when the session has no live graph, and does not retry the same attempt', async () => {
    fetchMock.mockResolvedValue(errorResponse(404));
    const options = baseOptions();

    const { rerender } = renderHook(
      (props: UseSessionGraphRestoreOptions) => useSessionGraphRestore(props),
      { initialProps: options },
    );

    await vi.waitFor(() => expect(fetchMock).toHaveBeenCalledTimes(1));
    rerender({ ...options });

    expect(fetchMock).toHaveBeenCalledTimes(1);
    expect(options.setGraphData).not.toHaveBeenCalled();
  });

  it('ignores a response that is not a graph payload', async () => {
    fetchMock.mockResolvedValue(okResponse({ message: 'no graph here' }));
    const options = baseOptions();

    renderHook(() => useSessionGraphRestore(options));

    await vi.waitFor(() => expect(fetchMock).toHaveBeenCalledTimes(1));
    expect(options.setGraphData).not.toHaveBeenCalled();
    expect(options.setRightTab).not.toHaveBeenCalled();
  });

  it('retries once the session id changes (a new attempt key)', async () => {
    fetchMock.mockResolvedValue(errorResponse(404));
    const options = baseOptions();

    const { rerender } = renderHook(
      (props: UseSessionGraphRestoreOptions) => useSessionGraphRestore(props),
      { initialProps: options },
    );
    await vi.waitFor(() => expect(fetchMock).toHaveBeenCalledTimes(1));

    fetchMock.mockResolvedValue(okResponse(LIVE_GRAPH));
    rerender({ ...options, sessionGraphPath: '/api/graph/session/ws-2-2' });

    await vi.waitFor(() => expect(options.setGraphData).toHaveBeenCalledWith(LIVE_GRAPH));
    expect(fetchMock).toHaveBeenCalledTimes(2);
    expect(fetchMock.mock.calls[1][0]).toBe('/api/graph/session/ws-2-2');
  });
});
