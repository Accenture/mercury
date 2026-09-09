// @vitest-environment happy-dom

import { cleanup, fireEvent, render, screen } from '@testing-library/react';
import { afterEach, describe, expect, it, vi } from 'vitest';
import HelpBrowser from '../HelpBrowser';

vi.mock('../../../hooks/useHelpScrollNavigation', () => ({
  useHelpScrollNavigation: () => {},
}));

vi.mock('../../../data/helpContent', () => {
  const minigraphCategories = [
    { id: 'overview', label: 'Overview' },
    { id: 'graph-model', label: 'Graph Model' },
  ];
  const jsonPathCategories = [{ id: 'overview', label: 'Overview' }];

  return {
    HELP_CATEGORIES: minigraphCategories,
    ORDERED_HELP_PAGES: ['', 'create'],
    getHelpCategories: (profile = 'minigraph') => (
      profile === 'json-path' ? jsonPathCategories : minigraphCategories
    ),
    getOrderedHelpPages: (profile = 'minigraph') => (
      profile === 'json-path' ? [''] : ['', 'create']
    ),
    getHelpContent: (topic: string, profile = 'minigraph') => {
      if (profile === 'json-path') {
        return topic === '' ? '# JSON-Path Playground Overview\n\n## Quick start' : null;
      }
      return topic === '' ? '# MiniGraph Overview' : '# Create';
    },
    resolveCategory: () => 'overview',
    getCategoryPages: (_category: string, profile = 'minigraph') => (
      profile === 'json-path' ? [''] : ['', 'create']
    ),
    getChipLabel: (topic: string) => topic || 'Overview',
  };
});

describe('HelpBrowser JSON Path profile', () => {
  afterEach(cleanup);

  it('renders only the mock Overview while preserving shared panel controls', () => {
    const onClose = vi.fn();
    const onToggleMaximize = vi.fn();

    render(
      <HelpBrowser
        activeTopic=""
        contentProfile="json-path"
        onNavigate={() => {}}
        onClose={onClose}
        onToggleMaximize={onToggleMaximize}
        isMaximized={false}
      />
    );

    expect(screen.getByRole('region', { name: 'Help browser' })).toBeTruthy();
    expect(screen.getByRole('button', { name: 'Overview' })).toBeTruthy();
    expect(screen.queryByRole('button', { name: 'Graph Model' })).toBeNull();
    expect(screen.getByRole('heading', { name: 'JSON-Path Playground Overview' })).toBeTruthy();

    fireEvent.click(screen.getByRole('button', { name: 'Maximize help panel' }));
    fireEvent.click(screen.getByRole('button', { name: 'Close help panel' }));

    expect(onToggleMaximize).toHaveBeenCalledOnce();
    expect(onClose).toHaveBeenCalledOnce();
  });
});
