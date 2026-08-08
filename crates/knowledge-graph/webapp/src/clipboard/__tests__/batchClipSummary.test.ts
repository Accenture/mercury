import { describe, expect, it } from 'vitest';
import { buildBatchClipToast } from '../batchClipSummary';

describe('buildBatchClipToast', () => {
  it('formats all-added results', () => {
    expect(buildBatchClipToast({ added: 3, duplicates: 0, failed: 0 })).toEqual({
      message: '3 nodes clipped to workspace',
      type: 'success',
    });
  });

  it('formats mixed duplicate results without requesting replacement', () => {
    expect(buildBatchClipToast({ added: 3, duplicates: 2, failed: 0 })).toEqual({
      message: '3 nodes clipped to workspace. 2 already existed.',
      type: 'success',
    });
  });

  it('formats an all-duplicate operation', () => {
    expect(buildBatchClipToast({ added: 0, duplicates: 2, failed: 0 })).toEqual({
      message: 'All selected nodes already exist in workspace.',
      type: 'info',
    });
  });

  it('formats mixed failures and full failures', () => {
    expect(buildBatchClipToast({ added: 3, duplicates: 0, failed: 2 })).toEqual({
      message: '3 nodes clipped to workspace. 2 failed.',
      type: 'error',
    });
    expect(buildBatchClipToast({ added: 0, duplicates: 0, failed: 2 })).toEqual({
      message: 'Failed to clip selected nodes to workspace.',
      type: 'error',
    });
  });

  it('formats a no-op selection', () => {
    expect(buildBatchClipToast({ added: 0, duplicates: 0, failed: 0 })).toEqual({
      message: 'No selected nodes are available to clip.',
      type: 'info',
    });
  });
});
