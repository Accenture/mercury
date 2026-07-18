import { describe, it, expect, vi, beforeEach } from 'vitest';
import { resolveBundledHelpTopic } from '../localHelpCommand';

// getHelpContent uses import.meta.glob (Vite-only) — mock at the module level.
vi.mock('../../data/helpContent', () => ({
  getHelpContent: (topic: string): string | null => {
    // Simulate the bundled topics that exist in src/main/resources/help/
    const bundled = new Set(['', 'create', 'export', 'import', 'list',
      'tutorial 1', 'tutorial 2', 'tutorial 3', 'tutorial 4', 'tutorial 5', 'tutorial 6',
      'graph-api-fetcher', 'graph-js', 'graph-join', 'graph-extension', 'graph-data-mapper',
      'edit', 'delete', 'update', 'run', 'upload', 'inspect']);
    return bundled.has(topic) ? `# help ${topic}` : null;
  },
}));

beforeEach(() => {
  vi.clearAllMocks();
});

describe('resolveBundledHelpTopic', () => {
  it('returns "" for plain "help" when supportsHelp is true', () => {
    expect(resolveBundledHelpTopic('help', true)).toBe('');
  });

  it('returns "create" for "help create" when supportsHelp is true', () => {
    expect(resolveBundledHelpTopic('help create', true)).toBe('create');
  });

  it('returns null for an unbundled topic', () => {
    expect(resolveBundledHelpTopic('help missing-topic-xyz', true)).toBeNull();
  });

  it('returns null for a describe command', () => {
    expect(resolveBundledHelpTopic('describe graph', true)).toBeNull();
  });

  it('returns null when supportsHelp is false', () => {
    expect(resolveBundledHelpTopic('help create', false)).toBeNull();
  });

  it('is case-insensitive for the help prefix', () => {
    expect(resolveBundledHelpTopic('HELP Create', true)).toBe('create');
  });

  it('returns null for a non-help, non-describe command', () => {
    expect(resolveBundledHelpTopic('export graph as foo', true)).toBeNull();
  });
});
