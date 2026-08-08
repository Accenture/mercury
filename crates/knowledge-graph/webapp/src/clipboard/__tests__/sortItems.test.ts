import { describe, expect, it } from 'vitest';
import type { ClipboardItemRecord } from '../db';
import { sortClipboardItems } from '../sortItems';

function item(
  alias: string,
  options: {
    clippedAt?: string;
    type?: string;
    sourceLabel?: string;
    connectionCount?: number;
    properties?: Record<string, unknown>;
  } = {},
): ClipboardItemRecord {
  const connectionCount = options.connectionCount ?? 0;
  return {
    id: alias,
    clippedAt: options.clippedAt ?? '2026-01-01T00:00:00.000Z',
    sourceWsPath: '/ws/minigraph',
    sourceLabel: options.sourceLabel ?? 'Minigraph',
    node: {
      alias,
      types: options.type ? [options.type] : [],
      properties: options.properties ?? {},
    },
    connections: Array.from({ length: connectionCount }, (_, index) => ({
      source: `${alias}-${index}`,
      target: alias,
      relations: [],
    })),
  };
}

function aliases(items: ClipboardItemRecord[]): string[] {
  return items.map(record => record.node.alias);
}

describe('sortClipboardItems', () => {
  it('sorts recent items newest first', () => {
    const items = [
      item('old', { clippedAt: '2026-01-01T00:00:00.000Z' }),
      item('new', { clippedAt: '2026-01-03T00:00:00.000Z' }),
      item('middle', { clippedAt: '2026-01-02T00:00:00.000Z' }),
    ];

    expect(aliases(sortClipboardItems(items, { field: 'recent' }))).toEqual([
      'new',
      'middle',
      'old',
    ]);
  });

  it('sorts recent items oldest first when ascending', () => {
    const items = [
      item('old', { clippedAt: '2026-01-01T00:00:00.000Z' }),
      item('new', { clippedAt: '2026-01-03T00:00:00.000Z' }),
      item('middle', { clippedAt: '2026-01-02T00:00:00.000Z' }),
    ];

    expect(aliases(sortClipboardItems(items, {
      field: 'recent',
      direction: 'ascending',
    }))).toEqual([
      'old',
      'middle',
      'new',
    ]);
  });

  it('sorts by primary node type', () => {
    const items = [
      item('root', { type: 'Root' }),
      item('fetch', { type: 'Fetcher' }),
      item('dict', { type: 'Dictionary' }),
    ];

    expect(aliases(sortClipboardItems(items, { field: 'type' }))).toEqual([
      'dict',
      'fetch',
      'root',
    ]);
  });

  it('sorts by alias', () => {
    const items = [
      item('root'),
      item('dictionary'),
      item('fetcher'),
    ];

    expect(aliases(sortClipboardItems(items, { field: 'alias' }))).toEqual([
      'dictionary',
      'fetcher',
      'root',
    ]);
  });

  it('sorts by alias descending when requested', () => {
    const items = [
      item('root'),
      item('dictionary'),
      item('fetcher'),
    ];

    expect(aliases(sortClipboardItems(items, {
      field: 'alias',
      direction: 'descending',
    }))).toEqual([
      'root',
      'fetcher',
      'dictionary',
    ]);
  });

  it('sorts by source label', () => {
    const items = [
      item('from-b', { sourceLabel: 'Beta' }),
      item('from-a', { sourceLabel: 'Alpha' }),
      item('from-c', { sourceLabel: 'Gamma' }),
    ];

    expect(aliases(sortClipboardItems(items, { field: 'source' }))).toEqual([
      'from-a',
      'from-b',
      'from-c',
    ]);
  });

  it('sorts by direct connection count from high to low', () => {
    const items = [
      item('one', { connectionCount: 1 }),
      item('three', { connectionCount: 3 }),
      item('zero', { connectionCount: 0 }),
    ];

    expect(aliases(sortClipboardItems(items, { field: 'connections' }))).toEqual([
      'three',
      'one',
      'zero',
    ]);
  });

  it('sorts by direct connection count from low to high when ascending', () => {
    const items = [
      item('one', { connectionCount: 1 }),
      item('three', { connectionCount: 3 }),
      item('zero', { connectionCount: 0 }),
    ];

    expect(aliases(sortClipboardItems(items, {
      field: 'connections',
      direction: 'ascending',
    }))).toEqual([
      'zero',
      'one',
      'three',
    ]);
  });

  it('sorts by a requested property key and moves missing values last', () => {
    const items = [
      item('missing'),
      item('b', { properties: { skill: 'graph.fetcher' } }),
      item('a', { properties: { skill: 'graph.dictionary' } }),
    ];

    expect(aliases(sortClipboardItems(items, {
      field: 'property',
      propertyKey: 'skill',
    }))).toEqual([
      'a',
      'b',
      'missing',
    ]);
  });

  it('sorts property values descending while keeping missing values last', () => {
    const items = [
      item('missing'),
      item('b', { properties: { skill: 'graph.fetcher' } }),
      item('a', { properties: { skill: 'graph.dictionary' } }),
    ];

    expect(aliases(sortClipboardItems(items, {
      field: 'property',
      direction: 'descending',
      propertyKey: 'skill',
    }))).toEqual([
      'b',
      'a',
      'missing',
    ]);
  });

  it('keeps original order when sort values match', () => {
    const items = [
      item('fetch-a', { type: 'Fetcher' }),
      item('fetch-b', { type: 'Fetcher' }),
      item('dict', { type: 'Dictionary' }),
    ];

    expect(aliases(sortClipboardItems(items, { field: 'type' }))).toEqual([
      'dict',
      'fetch-a',
      'fetch-b',
    ]);
  });

  it('does not mutate the source array', () => {
    const items = [
      item('root', { type: 'Root' }),
      item('fetch', { type: 'Fetcher' }),
    ];

    const sorted = sortClipboardItems(items, { field: 'type' });

    expect(sorted).not.toBe(items);
    expect(aliases(items)).toEqual(['root', 'fetch']);
  });
});
