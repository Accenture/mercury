import { describe, expect, it } from 'vitest';
import { classifyMessage } from '../../protocol/classifier';
import { GRAPH_RUN_COMMANDS } from '../graphRunProtocol';

describe('graph run protocol classification', () => {
  it('classifies graph-instance creation with its observable metadata', () => {
    const events = classifyMessage(
      11,
      'Graph instance created. Loaded 2 mock entries, model.ttl = 45000 ms',
    );

    expect(events).toContainEqual(expect.objectContaining({
      kind: 'graph.instance.created',
      msgId: 11,
      mockEntries: 2,
      ttlMs: 45000,
    }));
  });

  it('classifies instance clearing, both run terminals, and command errors', () => {
    expect(classifyMessage(1, 'Graph instance cleared')).toContainEqual(expect.objectContaining({
      kind: 'graph.instance.cleared',
    }));
    expect(classifyMessage(2, 'Graph traversal completed in 37 ms')).toContainEqual(expect.objectContaining({
      kind: 'graph.run.terminal',
      status: 'completed',
      elapsedMs: 37,
    }));
    expect(classifyMessage(3, 'Graph traversal aborted')).toContainEqual(expect.objectContaining({
      kind: 'graph.run.terminal',
      status: 'aborted',
      elapsedMs: null,
    }));
    expect(classifyMessage(4, 'ERROR: Root node does not exist')).toContainEqual(expect.objectContaining({
      kind: 'command.error',
      message: 'Root node does not exist',
    }));
  });

  it('centralizes the canonical commands sent by the UI', () => {
    expect(GRAPH_RUN_COMMANDS).toEqual({
      instantiate: 'instantiate graph',
      requestInputUpload: 'upload mock data',
      run: 'run',
    });
  });
});
