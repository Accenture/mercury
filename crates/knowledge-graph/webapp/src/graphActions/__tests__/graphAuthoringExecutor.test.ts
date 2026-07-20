import { describe, expect, it, vi } from 'vitest';
import { createGraphAuthoringExecutor } from '../graphAuthoringExecutor';

describe('createGraphAuthoringExecutor', () => {
  it('delegates raw command text and returns the send boolean', () => {
    const sendRawText = vi.fn(() => true);
    const executor = createGraphAuthoringExecutor(sendRawText);

    expect(executor.execute('create node root')).toBe(true);
    expect(sendRawText).toHaveBeenCalledWith('create node root');
  });

  it('surfaces send failure', () => {
    const executor = createGraphAuthoringExecutor(() => false);
    expect(executor.execute('create node root')).toBe(false);
  });
});
