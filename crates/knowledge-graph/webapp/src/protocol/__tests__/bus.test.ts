import { describe, it, expect, vi } from 'vitest';
import { ProtocolBus } from '../bus';
import type { ProtocolEvent, GraphLinkEvent } from '../events';

describe('ProtocolBus', () => {
  it('on() returns an unsubscribe function that works', () => {
    const bus = new ProtocolBus();
    const listener = vi.fn();
    const unsub = bus.on('graph.link', listener);

    const event: GraphLinkEvent = { kind: 'graph.link', msgId: 1, raw: 'test', apiPath: '/api/graph/model/x' };
    bus.emit(event);
    expect(listener).toHaveBeenCalledTimes(1);

    unsub();
    bus.emit(event);
    expect(listener).toHaveBeenCalledTimes(1); // not called again
  });

  it('emit() calls listeners synchronously in registration order', () => {
    const bus = new ProtocolBus();
    const order: number[] = [];

    bus.on('graph.link', () => order.push(1));
    bus.on('graph.link', () => order.push(2));
    bus.on('graph.link', () => order.push(3));

    const event: GraphLinkEvent = { kind: 'graph.link', msgId: 1, raw: 'test', apiPath: '/x' };
    bus.emit(event);

    expect(order).toEqual([1, 2, 3]);
  });

  it('emit() wraps each listener in try/catch — one throw does not block others', () => {
    const bus = new ProtocolBus();
    const consoleError = vi.spyOn(console, 'error').mockImplementation(() => {});
    const listener1 = vi.fn();
    const listener2 = vi.fn(() => { throw new Error('boom'); });
    const listener3 = vi.fn();

    bus.on('graph.link', listener1);
    bus.on('graph.link', listener2);
    bus.on('graph.link', listener3);

    const event: GraphLinkEvent = { kind: 'graph.link', msgId: 1, raw: 'test', apiPath: '/x' };
    bus.emit(event);

    expect(listener1).toHaveBeenCalledTimes(1);
    expect(listener2).toHaveBeenCalledTimes(1);
    expect(listener3).toHaveBeenCalledTimes(1);
    expect(consoleError).toHaveBeenCalledTimes(1);

    consoleError.mockRestore();
  });

  it('emit() is a no-op for kinds with no listeners', () => {
    const bus = new ProtocolBus();
    // Should not throw
    expect(() => {
      bus.emit({ kind: 'graph.link', msgId: 1, raw: 'test', apiPath: '/x' } as ProtocolEvent);
    }).not.toThrow();
  });

  it('clear() removes all listeners', () => {
    const bus = new ProtocolBus();
    const listener = vi.fn();
    bus.on('graph.link', listener);

    bus.clear();

    const event: GraphLinkEvent = { kind: 'graph.link', msgId: 1, raw: 'test', apiPath: '/x' };
    bus.emit(event);
    expect(listener).not.toHaveBeenCalled();
  });

  it('type narrowing: graph.link listener receives GraphLinkEvent', () => {
    const bus = new ProtocolBus();

    bus.on('graph.link', (event) => {
      // TypeScript should narrow this to GraphLinkEvent
      expect(event.kind).toBe('graph.link');
      expect(event.apiPath).toBe('/api/graph/model/test');
    });

    bus.emit({ kind: 'graph.link', msgId: 1, raw: 'test', apiPath: '/api/graph/model/test' });
  });

  it('listeners for different kinds are independent', () => {
    const bus = new ProtocolBus();
    const graphListener = vi.fn();
    const mutationListener = vi.fn();

    bus.on('graph.link', graphListener);
    bus.on('graph.mutation', mutationListener);

    bus.emit({ kind: 'graph.link', msgId: 1, raw: 'test', apiPath: '/x' });

    expect(graphListener).toHaveBeenCalledTimes(1);
    expect(mutationListener).not.toHaveBeenCalled();
  });
});
