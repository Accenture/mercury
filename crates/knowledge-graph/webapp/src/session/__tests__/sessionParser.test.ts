import { describe, expect, it } from 'vitest';
import {
  parseSessionStarted,
  parseSessionStatus,
  parseSessionCommandResult,
  parseSessionNotification,
} from '../sessionParser';
import { isValidSessionId } from '../sessionTypes';

describe('sessionParser', () => {
  it('validates public session ids', () => {
    expect(isValidSessionId('ws-123456-1')).toBe(true);
    expect(isValidSessionId(' ws-123456-1 ')).toBe(true);
    expect(isValidSessionId('ws-alpha-1')).toBe(false);
    expect(isValidSessionId('session ws-123456-1')).toBe(false);
  });

  it('parses the websocket session-start message', () => {
    expect(parseSessionStarted('session ws-123456-1 started\nCompanion endpoint: /api/companion/ws-123456-1')).toEqual({
      sessionId: 'ws-123456-1',
      companionEndpoint: '/api/companion/ws-123456-1',
    });
  });

  it('parses a full session status response', () => {
    expect(parseSessionStatus([
      'Session ws-178443-2 started since 2026-06-02 10:20:32.054',
      'subscribed to ws-111111-1',
      'subscribed by [ws-485844-4, ws-222222-3]',
    ].join('\n'))).toEqual({
      sessionId: 'ws-178443-2',
      startedSince: '2026-06-02 10:20:32.054',
      subscribedTo: 'ws-111111-1',
      subscribers: ['ws-485844-4', 'ws-222222-3'],
    });
  });

  it('parses a primary session with no subscribers', () => {
    expect(parseSessionStatus('Session ws-178443-2 started since 2026-06-02 10:20:32.054')).toEqual({
      sessionId: 'ws-178443-2',
      startedSince: '2026-06-02 10:20:32.054',
      subscribedTo: null,
      subscribers: [],
    });
  });

  it('parses command results and known rejections', () => {
    expect(parseSessionCommandResult('Subscribed to ws-178443-2')).toMatchObject({
      command: 'subscribe',
      status: 'accepted',
      sessionId: 'ws-178443-2',
    });
    expect(parseSessionCommandResult('Session unsubscribed from ws-178443-2')).toMatchObject({
      command: 'unsubscribe',
      status: 'accepted',
      sessionId: 'ws-178443-2',
    });
    expect(parseSessionCommandResult('Session ws-000000-0 not found')).toMatchObject({
      command: 'subscribe',
      status: 'rejected',
      sessionId: 'ws-000000-0',
    });
    expect(parseSessionCommandResult('You cannot subscribe to yourself')).toMatchObject({
      command: 'subscribe',
      status: 'rejected',
      sessionId: null,
    });
    expect(parseSessionCommandResult("You have already subscribed to ws-178443-2\nPlease do 'session reset' before subscribing to another session")).toMatchObject({
      command: 'subscribe',
      status: 'rejected',
      sessionId: 'ws-178443-2',
    });
  });

  it('parses relationship notifications', () => {
    expect(parseSessionNotification('ws-485844-4 subscribed to your session')).toEqual({
      type: 'subscriber-joined',
      sessionId: 'ws-485844-4',
      message: 'ws-485844-4 subscribed to your session',
    });
    expect(parseSessionNotification('ws-485844-4 unsubscribed from your session')).toEqual({
      type: 'subscriber-left',
      sessionId: 'ws-485844-4',
      message: 'ws-485844-4 unsubscribed from your session',
    });
    expect(parseSessionNotification('Session ws-178443-2 has closed')).toEqual({
      type: 'host-closed',
      sessionId: 'ws-178443-2',
      message: 'Session ws-178443-2 has closed',
    });
  });

  it('ignores echoed commands and unrelated text', () => {
    expect(parseSessionStatus('> session')).toBeNull();
    expect(parseSessionCommandResult('> session subscribe ws-178443-2')).toBeNull();
    expect(parseSessionNotification('node root created')).toBeNull();
  });
});
