// @vitest-environment happy-dom

import { renderHook } from '@testing-library/react';
import { afterEach, beforeEach, describe, expect, it, vi } from 'vitest';
import { useGraphData } from '../useGraphData';

const GRAPH = {
  nodes: [{ alias: 'root', types: ['Root'], properties: {} }],
  connections: [],
};

function okResponse(body: unknown) {
  return { ok: true, status: 200, json: async () => body };
}

function errorResponse(status: number) {
  return { ok: false, status, json: async () => ({}) };
}

describe('useGraphData initial-fetch failure handling', () => {
  const fetchMock = vi.fn();

  beforeEach(() => {
    fetchMock.mockReset();
    vi.stubGlobal('fetch', fetchMock);
    localStorage.clear();
  });

  afterEach(() => {
    vi.unstubAllGlobals();
  });

  function renderGraphData(quiet: boolean, addToast = vi.fn()) {
    const rendered = renderHook(
      ({ path }: { path: string | null }) =>
        useGraphData(path, addToast, 'graph', ['graph', 'graph-data'], 'test-right-tab', quiet),
      { initialProps: { path: '/api/graph/model/ws-1-1/9-1' as string | null } },
    );
    return { ...rendered, addToast };
  }

  it('reports the failure without a toast when the quiet flag is set (session restore owns it)', async () => {
    fetchMock.mockResolvedValue(errorResponse(400));
    const { result, addToast } = renderGraphData(true);

    await vi.waitFor(() => expect(result.current.initialFetchFailed).toBe(true));
    expect(addToast).not.toHaveBeenCalled();
    expect(result.current.graphData).toBeNull();
  });

  it('keeps the legacy failure toast when the quiet flag is not set', async () => {
    fetchMock.mockResolvedValue(errorResponse(400));
    const { result, addToast } = renderGraphData(false);

    await vi.waitFor(() => expect(result.current.initialFetchFailed).toBe(true));
    expect(addToast).toHaveBeenCalledWith('Graph fetch failed: HTTP 400', 'error');
  });

  it('treats a 200 response with a non-graph body (error envelope) as a failed verdict, silently', async () => {
    fetchMock.mockResolvedValue(okResponse({ message: "Draft graph 'ws-1-1' does not exist", type: 'error', status: 400 }));
    const { result, addToast } = renderGraphData(true);

    await vi.waitFor(() => expect(result.current.initialFetchFailed).toBe(true));
    expect(addToast).not.toHaveBeenCalled();
    expect(result.current.graphData).toBeNull();
  });

  it('resets the failure verdict when the pinned path changes and the new fetch succeeds', async () => {
    fetchMock.mockResolvedValueOnce(errorResponse(400));
    const { result, rerender } = renderGraphData(true);
    await vi.waitFor(() => expect(result.current.initialFetchFailed).toBe(true));

    fetchMock.mockResolvedValueOnce(okResponse(GRAPH));
    rerender({ path: '/api/graph/model/ws-1-1/10-2' });

    await vi.waitFor(() => expect(result.current.graphData).toEqual(GRAPH));
    expect(result.current.initialFetchFailed).toBe(false);
  });
});
