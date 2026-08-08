import { describe, expect, it, vi } from 'vitest';
import { ProtocolBus } from '../../protocol/bus';
import {
  EMPTY_GRAPH_SAVE_NAME_STATE,
  reduceGraphSaveNameState,
  shouldRestoreGraphSaveNameState,
  subscribeGraphSaveNameEvents,
} from '../useGraphSaveName';

describe('reduceGraphSaveNameState', () => {
  it('keeps the last export name when a mutation makes the graph dirty', () => {
    const saved = reduceGraphSaveNameState(EMPTY_GRAPH_SAVE_NAME_STATE, {
      type: 'exported',
      name: 'test',
      consumesUntitled: false,
    });
    const dirty = reduceGraphSaveNameState(saved, { type: 'dirty' });

    expect(dirty.lastSavedName).toBe('test');
    expect(dirty.isSaved).toBe(false);
  });

  it('preserves untitled-slot consumption across imports until reset', () => {
    const saved = reduceGraphSaveNameState(EMPTY_GRAPH_SAVE_NAME_STATE, {
      type: 'exported',
      name: 'untitled-1',
      consumesUntitled: true,
    });
    const imported = reduceGraphSaveNameState(saved, {
      type: 'imported',
      name: 'tutorial-3',
    });

    expect(imported).toMatchObject({
      importedName: 'tutorial-3',
      lastSavedName: null,
      isSaved: false,
      untitledSlotConsumed: true,
    });
    expect(reduceGraphSaveNameState(imported, { type: 'reset' }))
      .toEqual(EMPTY_GRAPH_SAVE_NAME_STATE);
  });
});

describe('shouldRestoreGraphSaveNameState', () => {
  it('restores only for the same live WebSocket epoch', () => {
    expect(shouldRestoreGraphSaveNameState(12, true, 12)).toBe(true);
    expect(shouldRestoreGraphSaveNameState(12, true, 18)).toBe(false);
    expect(shouldRestoreGraphSaveNameState(12, false, 12)).toBe(false);
    expect(shouldRestoreGraphSaveNameState(undefined, true, 12)).toBe(false);
  });
});

describe('subscribeGraphSaveNameEvents', () => {
  it('tracks confirmed exports and clears saved state when the graph changes', () => {
    const bus = new ProtocolBus();
    const onImported = vi.fn();
    const onExported = vi.fn();
    const onDirty = vi.fn();
    const onReset = vi.fn();
    const unsubscribe = subscribeGraphSaveNameEvents(bus, {
      onImported,
      onExported,
      onDirty,
      onReset,
    });

    bus.emit({
      kind: 'graph.exported',
      msgId: 1,
      raw: 'Graph exported',
      graphName: 'test',
      apiPath: '/api/graph/model/test/1',
    });
    bus.emit({
      kind: 'graph.mutation',
      msgId: 2,
      raw: 'Node fetcher updated',
      mutationType: 'node-mutation',
    });
    bus.emit({
      kind: 'command.importGraph',
      msgId: 3,
      raw: '> import graph from tutorial-3',
      graphName: 'tutorial-3',
    });

    expect(onExported).toHaveBeenCalledOnce();
    expect(onExported).toHaveBeenCalledWith('test');
    expect(onDirty).toHaveBeenCalledOnce();
    expect(onImported).toHaveBeenCalledWith('tutorial-3');

    bus.emit({
      kind: 'session.reset',
      msgId: 4,
      raw: 'Session restarted',
    });
    expect(onReset).toHaveBeenCalledOnce();

    unsubscribe();
    bus.emit({
      kind: 'graph.exported',
      msgId: 5,
      raw: 'Graph exported again',
      graphName: 'ignored',
      apiPath: '/api/graph/model/ignored/2',
    });
    expect(onExported).toHaveBeenCalledOnce();
  });

  it('ignores failed exports', () => {
    const bus = new ProtocolBus();
    const onExported = vi.fn();
    subscribeGraphSaveNameEvents(bus, {
      onImported: vi.fn(),
      onExported,
      onDirty: vi.fn(),
      onReset: vi.fn(),
    });

    bus.emit({
      kind: 'graph.export.failed',
      msgId: 1,
      raw: 'Invalid graph name',
      reason: 'invalid-name',
    });

    expect(onExported).not.toHaveBeenCalled();
  });
});
