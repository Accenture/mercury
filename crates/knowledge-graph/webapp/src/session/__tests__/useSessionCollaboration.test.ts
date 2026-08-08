// @vitest-environment happy-dom

import { act, renderHook, waitFor } from '@testing-library/react';
import { describe, expect, it, vi } from 'vitest';
import { ProtocolBus } from '../../protocol/bus';
import type { ProtocolEvent } from '../../protocol/events';
import { useSessionCollaboration } from '../useSessionCollaboration';

function emit(bus: ProtocolBus, event: ProtocolEvent): void {
  act(() => bus.emit(event));
}

function renderController(sendRawText = vi.fn(() => true)) {
  const bus = new ProtocolBus();
  const addToast = vi.fn();
  const hook = renderHook(() => useSessionCollaboration({
    enabled: true,
    connected: true,
    bus,
    classificationMap: new Map(),
    sendRawText,
    addToast,
  }));
  return { ...hook, addToast, bus, sendRawText };
}

describe('useSessionCollaboration', () => {
  it('loads authoritative session status when mounted on a connected socket', async () => {
    const { bus, result, sendRawText } = renderController();

    await waitFor(() => expect(sendRawText).toHaveBeenCalledWith('session'));
    expect(result.current.state.pendingCommand).toBe('refresh');

    emit(bus, {
      kind: 'minigraph.session.status',
      msgId: 201,
      raw: '',
      sessionId: 'ws-123456-1',
      startedSince: '2026-07-29 10:00:00.000',
      subscribedTo: 'ws-654321-2',
      subscribers: [],
    });

    expect(result.current.state).toMatchObject({
      sessionId: 'ws-123456-1',
      startedSince: '2026-07-29 10:00:00.000',
      subscribedTo: 'ws-654321-2',
      loading: false,
      pendingCommand: null,
    });
  });

  it('does not repeat the mount status request when the sender identity changes during rerenders', async () => {
    const bus = new ProtocolBus();
    const addToast = vi.fn();
    const sendRawText = vi.fn((_text: string) => true);
    renderHook(() => useSessionCollaboration({
      enabled: true,
      connected: true,
      bus,
      classificationMap: new Map(),
      sendRawText: (text: string) => sendRawText(text),
      addToast,
    }));

    await waitFor(() => expect(sendRawText).toHaveBeenCalledTimes(1));

    emit(bus, {
      kind: 'minigraph.session.status',
      msgId: 202,
      raw: '',
      sessionId: 'ws-123456-1',
      startedSince: '2026-07-29 10:00:00.000',
      subscribedTo: null,
      subscribers: [],
    });

    await waitFor(() => expect(sendRawText).toHaveBeenCalledTimes(1));
  });

  it('invalidates the old start time and refreshes after session reset', async () => {
    const { bus, result, sendRawText } = renderController();
    await waitFor(() => expect(sendRawText).toHaveBeenCalledTimes(1));

    emit(bus, {
      kind: 'minigraph.session.status',
      msgId: 1,
      raw: '',
      sessionId: 'ws-123456-1',
      startedSince: 'old-time',
      subscribedTo: null,
      subscribers: ['ws-654321-2'],
    });
    emit(bus, { kind: 'session.reset', msgId: 2, raw: 'Session restarted' });

    expect(sendRawText).toHaveBeenNthCalledWith(2, 'session');
    expect(result.current.state).toMatchObject({
      startedSince: null,
      subscribers: [],
      loading: true,
      pendingCommand: 'refresh',
    });
  });

  it('rolls back pending state and reports a failed status request', async () => {
    const sendRawText = vi.fn(() => false);
    const { addToast, result } = renderController(sendRawText);

    await waitFor(() => expect(sendRawText).toHaveBeenCalledWith('session'));
    expect(result.current.state).toMatchObject({
      loading: false,
      pendingCommand: null,
      error: 'Could not load session details because the WebSocket is not open.',
    });
    expect(addToast).toHaveBeenCalledWith(
      'Could not load session details because the WebSocket is not open.',
      'error',
    );
  });

  it('does not send an invalid subscribe command', async () => {
    const { bus, result, sendRawText } = renderController();
    await waitFor(() => expect(sendRawText).toHaveBeenCalledTimes(1));
    emit(bus, {
      kind: 'minigraph.session.status',
      msgId: 1,
      raw: '',
      sessionId: 'ws-123456-1',
      startedSince: 'now',
      subscribedTo: null,
      subscribers: [],
    });

    act(() => expect(result.current.subscribeToSession('not-a-session')).toBe(false));

    expect(sendRawText).toHaveBeenCalledTimes(1);
    expect(result.current.state.error).toBe('Enter a valid session ID like ws-123456-1.');
  });
});