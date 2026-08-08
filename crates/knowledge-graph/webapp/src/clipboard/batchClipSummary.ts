import type { ToastType } from '../hooks/useToast';

// Batch clip writes are performed by the clipboard context one node at a time.
// This file only converts the accumulated counts into one toast so multi-select
// clip does not spam the user with one notification or duplicate dialog per node.
export interface BatchClipSummary {
  // Newly inserted workspace items.
  added: number;
  // Nodes that matched existing workspace items by alias/content.
  duplicates: number;
  // Nodes that could not be clipped because validation or storage failed.
  failed: number;
}

export interface BatchClipToast {
  message: string;
  type: ToastType;
}

function clippedCountText(count: number): string {
  return `${count} ${count === 1 ? 'node' : 'nodes'} clipped to workspace`;
}

// Batch mode reports one outcome and intentionally does not open the
// single-node duplicate replacement dialog for each repeated alias.
export function buildBatchClipToast(summary: BatchClipSummary): BatchClipToast {
  const { added, duplicates, failed } = summary;

  // Nothing was actionable; keep this informational instead of treating it as
  // an error because it usually means the graph changed before the menu action.
  if (added === 0 && duplicates === 0 && failed === 0) {
    return { message: 'No selected nodes are available to clip.', type: 'info' };
  }
  // Every selected node already exists in workspace. This is not a failure,
  // but it should still explain why no new workspace items appeared.
  if (added === 0 && duplicates > 0 && failed === 0) {
    return { message: 'All selected nodes already exist in workspace.', type: 'info' };
  }
  // Nothing was written and at least one node failed, so the whole batch reads
  // as a failed operation from the user's point of view.
  if (added === 0 && duplicates === 0 && failed > 0) {
    return { message: 'Failed to clip selected nodes to workspace.', type: 'error' };
  }

  // Mixed outcomes still report the successful writes first, then append any
  // skipped duplicates or failures.
  let message = clippedCountText(added);
  if (duplicates > 0) {
    message += `. ${duplicates} already existed.`;
  }
  if (failed > 0) {
    message += `. ${failed} failed.`;
  }
  return {
    message,
    type: failed > 0 ? 'error' : 'success',
  };
}
