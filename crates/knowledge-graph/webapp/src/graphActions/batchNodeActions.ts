import type { ToastType } from '../hooks/useToast';
import type { NodeActionTextResultEvent } from '../protocol/events';

// Batch node actions are still executed as individual raw minigraph commands.
// This module owns the client-side bookkeeping needed to treat those separate
// backend responses as one user-facing operation.
export const MAX_BATCH_NODE_ACTIONS = 100;

export type BatchDeleteResult = 'success' | 'error';

export interface PendingBatchDeleteSubmit {
  action: 'delete-nodes';
  // Aliases are the stable identifiers used to match each backend text result
  // back to the command that produced it.
  aliases: string[];
  // Kept for observability/debugging; the executor sends these commands before
  // this pending state starts waiting for responses.
  commands: string[];
  sentAt: string;
  // Results are filled incrementally as backend events arrive. Missing aliases
  // mean the UI is still waiting or will eventually time out.
  results: Record<string, BatchDeleteResult>;
}

export interface BatchDeleteFeedback {
  message: string;
  type: ToastType;
}

function normalizedAlias(alias: string): string {
  return alias.trim().toLowerCase();
}

// Text result events arrive independently for each command. Generic ERROR
// results have no alias and must stay unmatched so timeout reports uncertainty
// instead of attributing the failure to an arbitrary selected node.
export function recordBatchDeleteResult(
  pending: PendingBatchDeleteSubmit,
  event: NodeActionTextResultEvent,
): PendingBatchDeleteSubmit | null {
  // Only alias-specific delete-node results can safely update this batch.
  // A generic ERROR without an alias is intentionally ignored because the UI
  // cannot know which selected node failed.
  if (!event.alias || (event.action !== null && event.action !== 'delete-node')) {
    return null;
  }

  // Find the first pending alias that matches this event. The result map check
  // prevents duplicate backend events from changing an already recorded node.
  const matchingAlias = pending.aliases.find(
    (alias) => normalizedAlias(alias) === normalizedAlias(event.alias!) && pending.results[alias] === undefined,
  );
  if (!matchingAlias) return null;

  return {
    ...pending,
    results: {
      ...pending.results,
      [matchingAlias]: event.status === 'accepted' ? 'success' : 'error',
    },
  };
}

export function isBatchDeleteComplete(pending: PendingBatchDeleteSubmit): boolean {
  // The batch is complete only when every selected alias has a matched result.
  // Partial completion stays pending so the caller can keep waiting or time out.
  return pending.aliases.every((alias) => pending.results[alias] !== undefined);
}

export function buildBatchDeleteFeedback(pending: PendingBatchDeleteSubmit): BatchDeleteFeedback {
  // The UI reports the whole batch once instead of showing one toast per node.
  const succeeded = Object.values(pending.results).filter((result) => result === 'success').length;
  const failed = Object.values(pending.results).filter((result) => result === 'error').length;
  const deletedText = `${succeeded} selected ${succeeded === 1 ? 'node' : 'nodes'} deleted.`;

  if (succeeded === 0 && failed > 0) {
    return { message: 'Failed to delete selected nodes.', type: 'error' };
  }
  if (failed > 0) {
    return {
      message: `${deletedText} ${failed} failed.`,
      type: 'error',
    };
  }
  return {
    message: deletedText,
    type: 'success',
  };
}
