import { describe, expect, it } from 'vitest';
import {
  getHelpCategories,
  getHelpContent,
  getOrderedHelpPages,
} from '../helpContent';

describe('helpContent', () => {
  it('documents the minimap and Help panel keyboard shortcuts in the overview', () => {
    const overview = getHelpContent('');

    expect(overview).toContain('Keyboard shortcuts');
    expect(overview).toContain('`Ctrl + M`');
    expect(overview).toContain('Toggle the minimap');
    expect(overview).toContain('`Ctrl + backtick`');
    expect(overview).toContain('Toggle the Help panel');
  });

  it('provides an isolated JSON Path Overview profile', () => {
    const overview = getHelpContent('', 'json-path');

    expect(overview).toContain('# JSON-Path Playground Overview');
    expect(overview).toContain('## Quick start');
    expect(overview).toContain('`Ctrl + backtick`');
    expect(getHelpContent('create', 'json-path')).toBeNull();
    expect(getHelpCategories('json-path').map(category => category.label)).toEqual(['Overview']);
    expect(getOrderedHelpPages('json-path')).toEqual(['']);
  });
});
