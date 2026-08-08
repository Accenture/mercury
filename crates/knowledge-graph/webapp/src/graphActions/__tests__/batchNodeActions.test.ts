import { describe, expect, it } from 'vitest';
import type { NodeActionTextResultEvent } from '../../protocol/events';
import {
  buildBatchDeleteFeedback,
  isBatchDeleteComplete,
  recordBatchDeleteResult,
  type PendingBatchDeleteSubmit,
} from '../batchNodeActions';

function pending(results: PendingBatchDeleteSubmit['results'] = {}): PendingBatchDeleteSubmit {
  return {
    action: 'delete-nodes',
    aliases: ['root', 'fetch'],
    commands: ['delete node root', 'delete node fetch'],
    sentAt: '2026-05-27T00:00:00.000Z',
    results,
  };
}

function event(
  alias: string | null,
  status: NodeActionTextResultEvent['status'],
  action: NodeActionTextResultEvent['action'] = 'delete-node',
): NodeActionTextResultEvent {
  return {
    kind: 'minigraph.nodeAction.textResult',
    msgId: 1,
    raw: '',
    alias,
    targetAlias: null,
    status,
    action,
    message: '',
  };
}

describe('recordBatchDeleteResult', () => {
  it('records accepted and rejected delete results by alias', () => {
    const first = recordBatchDeleteResult(pending(), event('root', 'accepted'));
    expect(first?.results).toEqual({ root: 'success' });

    const second = recordBatchDeleteResult(first!, event('fetch', 'rejected', null));
    expect(second?.results).toEqual({ root: 'success', fetch: 'error' });
    expect(isBatchDeleteComplete(second!)).toBe(true);
  });

  it('ignores generic errors or unrelated aliases that cannot be correlated', () => {
    expect(recordBatchDeleteResult(pending(), event(null, 'error', null))).toBeNull();
    expect(recordBatchDeleteResult(pending(), event('other', 'accepted'))).toBeNull();
  });
});

describe('buildBatchDeleteFeedback', () => {
  it('reports full success, partial failure, and full failure once complete', () => {
    expect(buildBatchDeleteFeedback(pending({ root: 'success', fetch: 'success' }))).toEqual({
      message: '2 selected nodes deleted.',
      type: 'success',
    });
    expect(buildBatchDeleteFeedback(pending({ root: 'success', fetch: 'error' }))).toEqual({
      message: '1 selected node deleted. 1 failed.',
      type: 'error',
    });
    expect(buildBatchDeleteFeedback(pending({ root: 'error', fetch: 'error' }))).toEqual({
      message: 'Failed to delete selected nodes.',
      type: 'error',
    });
  });
});
