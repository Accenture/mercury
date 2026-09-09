// @vitest-environment happy-dom

import { act, renderHook } from '@testing-library/react';
import { describe, expect, it, vi } from 'vitest';
import { ProtocolBus } from '../../protocol/bus';
import { useMockUploadPanel } from '../useMockUploadPanel';

describe('useMockUploadPanel path ownership', () => {
  it('closes an invalidated workflow path without closing a different manual upload panel', () => {
    const { result } = renderHook(() => useMockUploadPanel({
      bus: new ProtocolBus(),
      addToast: vi.fn(),
    }));

    act(() => result.current.handleOpenUploadPanel('/api/mock/manual'));
    act(() => expect(result.current.handleCloseUploadPath('/api/mock/workflow')).toBe(false));
    expect(result.current.uploadPanelPath).toBe('/api/mock/manual');

    act(() => expect(result.current.handleCloseUploadPath('/api/mock/manual')).toBe(true));
    expect(result.current.uploadPanelPath).toBeNull();
  });

  it('can open a later invitation after an invalidated invitation is opened and closed in one event', () => {
    const bus = new ProtocolBus();
    const { result } = renderHook(() => useMockUploadPanel({
      bus,
      addToast: vi.fn(),
    }));
    const offInvalidate = bus.on('upload.invitation', event => {
      result.current.handleCloseUploadPath(event.uploadPath);
    });

    act(() => bus.emit({
      kind: 'upload.invitation',
      msgId: 1,
      raw: 'You may upload JSON payload -> POST /api/mock/stale',
      uploadPath: '/api/mock/stale',
    }));
    expect(result.current.uploadPanelPath).toBeNull();

    offInvalidate();
    act(() => bus.emit({
      kind: 'upload.invitation',
      msgId: 2,
      raw: 'You may upload JSON payload -> POST /api/mock/current',
      uploadPath: '/api/mock/current',
    }));
    expect(result.current.uploadPanelPath).toBe('/api/mock/current');
  });

  it('queues a later invitation instead of losing it behind an open panel', () => {
    const bus = new ProtocolBus();
    const { result } = renderHook(() => useMockUploadPanel({
      bus,
      addToast: vi.fn(),
    }));

    act(() => result.current.handleOpenUploadPanel('/api/mock/manual'));
    act(() => bus.emit({
      kind: 'upload.invitation',
      msgId: 1,
      raw: 'You may upload JSON payload -> POST /api/mock/workflow',
      uploadPath: '/api/mock/workflow',
    }));

    expect(result.current.uploadPanelPath).toBe('/api/mock/manual');
    act(() => result.current.handleCloseUploadPanel());
    expect(result.current.uploadPanelPath).toBe('/api/mock/workflow');
    act(() => result.current.handleCloseUploadPanel());
    expect(result.current.uploadPanelPath).toBeNull();
  });

  it('can invalidate a queued workflow invitation without closing the current panel', () => {
    const bus = new ProtocolBus();
    const { result } = renderHook(() => useMockUploadPanel({
      bus,
      addToast: vi.fn(),
    }));

    act(() => result.current.handleOpenUploadPanel('/api/mock/manual'));
    act(() => bus.emit({
      kind: 'upload.invitation',
      msgId: 1,
      raw: 'You may upload JSON payload -> POST /api/mock/workflow',
      uploadPath: '/api/mock/workflow',
    }));
    act(() => expect(result.current.handleCloseUploadPath('/api/mock/workflow')).toBe(true));

    expect(result.current.uploadPanelPath).toBe('/api/mock/manual');
    act(() => result.current.handleCloseUploadPanel());
    expect(result.current.uploadPanelPath).toBeNull();
  });
});
