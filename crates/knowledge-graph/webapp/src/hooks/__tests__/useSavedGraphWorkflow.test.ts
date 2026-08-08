import { describe, expect, it, vi } from 'vitest';
import { ProtocolBus } from '../../protocol/bus';
import { subscribeGraphExportBookmarks } from '../useSavedGraphWorkflow';

describe('subscribeGraphExportBookmarks', () => {
  it('bookmarks every confirmed export regardless of its initiating UI', () => {
    const bus = new ProtocolBus();
    const saveGraph = vi.fn();
    const unsubscribe = subscribeGraphExportBookmarks(bus, saveGraph);

    bus.emit({
      kind: 'graph.exported',
      msgId: 1,
      raw: 'Graph exported from command console',
      graphName: 'command-export',
      apiPath: '/api/graph/model/command-export/1',
    });
    bus.emit({
      kind: 'graph.exported',
      msgId: 2,
      raw: 'Graph exported from Save Graph button',
      graphName: 'button-export',
      apiPath: '/api/graph/model/button-export/2',
    });

    expect(saveGraph).toHaveBeenNthCalledWith(1, 'command-export');
    expect(saveGraph).toHaveBeenNthCalledWith(2, 'button-export');

    unsubscribe();
  });

  it('does not bookmark failed exports or events after unsubscribe', () => {
    const bus = new ProtocolBus();
    const saveGraph = vi.fn();
    const unsubscribe = subscribeGraphExportBookmarks(bus, saveGraph);

    bus.emit({
      kind: 'graph.export.failed',
      msgId: 1,
      raw: 'Invalid graph name',
      reason: 'invalid-name',
    });
    unsubscribe();
    bus.emit({
      kind: 'graph.exported',
      msgId: 2,
      raw: 'Late export',
      graphName: 'ignored',
      apiPath: '/api/graph/model/ignored/2',
    });

    expect(saveGraph).not.toHaveBeenCalled();
  });
});
