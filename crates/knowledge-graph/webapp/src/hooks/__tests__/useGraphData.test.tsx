// @vitest-environment happy-dom

import { act, renderHook } from '@testing-library/react';
import { afterEach, beforeEach, describe, expect, it, vi } from 'vitest';
import { useGraphData } from '../useGraphData';

const GRAPH = {
  nodes: [{ alias: 'root', types: ['Root'], properties: {} }],
  connections: [],
};

const EMPTY_GRAPH = { nodes: [], connections: [] };

function okResponse(body: unknown) {
  return { ok: true, status: 200, json: async () => body };
}

function errorResponse(status: number) {
  return { ok: false, status, json: async () => ({}) };
}

describe('useGraphData (live session endpoint)', () => {
  const fetchMock = vi.fn();

  beforeEach(() => {
    fetchMock.mockReset();
    vi.stubGlobal('fetch', fetchMock);
    localStorage.clear();
  });

  afterEach(() => {
    vi.unstubAllGlobals();
  });

  function renderGraphData(addToast = vi.fn(), initialPath: string | null = '/api/graph/session/ws-1-1') {
    const rendered = renderHook(
      ({ path }: { path: string | null }) =>
        useGraphData(path, addToast, 'graph-data', ['graph', 'graph-data'], 'test-right-tab'),
      { initialProps: { path: initialPath } },
    );
    return { ...rendered, addToast };
  }

  it('loads the live graph on mount and reveals the Graph tab', async () => {
    fetchMock.mockResolvedValue(okResponse(GRAPH));
    const { result } = renderGraphData();

    await vi.waitFor(() => expect(result.current.graphData).toEqual(GRAPH));
    expect(fetchMock).toHaveBeenCalledWith('/api/graph/session/ws-1-1', expect.anything());
    expect(result.current.rightTab).toBe('graph');
  });

  it('stays quiet when the session has no live graph (HTTP 404)', async () => {
    fetchMock.mockResolvedValue(errorResponse(404));
    const { result, addToast } = renderGraphData();

    await vi.waitFor(() => expect(fetchMock).toHaveBeenCalled());
    expect(addToast).not.toHaveBeenCalled();
    expect(result.current.graphData).toBeNull();
    expect(result.current.rightTab).toBe('graph-data'); // no reveal without content
  });

  it('treats a fresh session (zero-node graph) as no graph — empty state, no tab switch', async () => {
    fetchMock.mockResolvedValue(okResponse(EMPTY_GRAPH));
    const { result, addToast } = renderGraphData();

    await vi.waitFor(() => expect(fetchMock).toHaveBeenCalled());
    expect(result.current.graphData).toBeNull();
    expect(result.current.rightTab).toBe('graph-data');
    expect(addToast).not.toHaveBeenCalled();
  });

  it('does not fetch until the session id is known, then loads when the path appears', async () => {
    fetchMock.mockResolvedValue(okResponse(GRAPH));
    const { result, rerender } = renderGraphData(vi.fn(), null);

    expect(fetchMock).not.toHaveBeenCalled();
    rerender({ path: '/api/graph/session/ws-2-7' });

    await vi.waitFor(() => expect(result.current.graphData).toEqual(GRAPH));
    expect(fetchMock).toHaveBeenCalledWith('/api/graph/session/ws-2-7', expect.anything());
  });

  it('refetchGraph re-fetches in place and reports isRefreshing', async () => {
    fetchMock.mockResolvedValueOnce(okResponse(GRAPH));
    const { result } = renderGraphData();
    await vi.waitFor(() => expect(result.current.graphData).toEqual(GRAPH));

    const UPDATED = { ...GRAPH, nodes: [...GRAPH.nodes, { alias: 'child', types: ['Task'], properties: {} }] };
    fetchMock.mockResolvedValueOnce(okResponse(UPDATED));
    act(() => result.current.refetchGraph());

    await vi.waitFor(() => expect(result.current.graphData).toEqual(UPDATED));
    expect(result.current.isRefreshing).toBe(false);
  });

  it('refetchGraph reveals the Graph tab when the first content arrives', async () => {
    fetchMock.mockResolvedValueOnce(okResponse(EMPTY_GRAPH)); // initial: fresh session
    const { result } = renderGraphData();
    await vi.waitFor(() => expect(fetchMock).toHaveBeenCalled());
    expect(result.current.rightTab).toBe('graph-data');

    fetchMock.mockResolvedValueOnce(okResponse(GRAPH)); // first node created → mutation refetch
    act(() => result.current.refetchGraph());

    await vi.waitFor(() => expect(result.current.graphData).toEqual(GRAPH));
    expect(result.current.rightTab).toBe('graph');
  });

  it('refetchGraph clears the view when the live graph emptied, and toasts on failure', async () => {
    fetchMock.mockResolvedValueOnce(okResponse(GRAPH));
    const { result, addToast } = renderGraphData();
    await vi.waitFor(() => expect(result.current.graphData).toEqual(GRAPH));

    fetchMock.mockResolvedValueOnce(okResponse(EMPTY_GRAPH)); // last node deleted
    act(() => result.current.refetchGraph());
    await vi.waitFor(() => expect(result.current.graphData).toBeNull());
    expect(addToast).not.toHaveBeenCalled();

    fetchMock.mockResolvedValueOnce(errorResponse(500));
    act(() => result.current.refetchGraph());
    await vi.waitFor(() =>
      expect(addToast).toHaveBeenCalledWith('Graph refresh failed: HTTP 500', 'error'));
  });

  it('clears the previous graph while a new session path loads', async () => {
    fetchMock.mockResolvedValueOnce(okResponse(GRAPH));
    const { result, rerender } = renderGraphData();
    await vi.waitFor(() => expect(result.current.graphData).toEqual(GRAPH));

    // New session id → the old session's graph must not linger.
    fetchMock.mockReturnValueOnce(new Promise(() => {})); // never resolves
    rerender({ path: '/api/graph/session/ws-9-9' });
    expect(result.current.graphData).toBeNull();
  });
});
