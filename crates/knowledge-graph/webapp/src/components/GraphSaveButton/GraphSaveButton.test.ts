import { createElement } from 'react';
import { renderToStaticMarkup } from 'react-dom/server';
import { describe, expect, it, vi } from 'vitest';
import GraphSaveButton, { formatGraphSaveButtonLabel } from './GraphSaveButton';

describe('formatGraphSaveButtonLabel', () => {
  it('shows the confirmed export name in the saved state', () => {
    expect(formatGraphSaveButtonLabel('test')).toBe('Saved: test');
  });

  it('shows the save action when no export has been confirmed', () => {
    expect(formatGraphSaveButtonLabel(null)).toBe('Save Graph');
  });

  it('renders the saved name and accessible save-again label', () => {
    const markup = renderToStaticMarkup(createElement(GraphSaveButton, {
      disabled: false,
      connected: true,
      defaultName: 'test',
      savedName: 'test',
      onSave: vi.fn(),
    }));

    expect(markup).toContain('Saved: test');
    expect(markup).toContain('aria-label="Graph saved as test. Save again"');
  });
});
