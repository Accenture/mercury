import { describe, expect, it } from 'vitest';
import type { MinigraphGraphData } from '../../../utils/graphTypes';
import { filterAliasesToGraphNodes, resolveNodeContextTarget } from '../selectionTargets';

const graphData: MinigraphGraphData = {
  nodes: [
    { alias: 'root', types: ['Root'], properties: {} },
    { alias: 'fetch', types: ['Fetcher'], properties: {} },
    { alias: 'end', types: ['End'], properties: {} },
  ],
  connections: [],
};

describe('resolveNodeContextTarget', () => {
  it('returns a multi-node target when the clicked node belongs to a multi-selection', () => {
    expect(resolveNodeContextTarget('root', ['root', 'fetch'])).toEqual({
      kind: 'multi-node',
      aliases: ['root', 'fetch'],
    });
  });

  it('returns a single-node target when the clicked node is outside the selection', () => {
    expect(resolveNodeContextTarget('end', ['root', 'fetch'])).toEqual({
      kind: 'single-node',
      alias: 'end',
    });
  });

  it('keeps a one-node selection on the single-node action path', () => {
    expect(resolveNodeContextTarget('root', ['root'])).toEqual({
      kind: 'single-node',
      alias: 'root',
    });
  });

  it('deduplicates selected aliases before constructing a multi-node target', () => {
    expect(resolveNodeContextTarget('root', ['root', 'ROOT', 'fetch'])).toEqual({
      kind: 'multi-node',
      aliases: ['root', 'fetch'],
    });
  });
});

describe('filterAliasesToGraphNodes', () => {
  it('filters stale aliases and preserves selection order', () => {
    expect(filterAliasesToGraphNodes(['fetch', 'missing', 'root'], graphData).map((node) => node.alias))
      .toEqual(['fetch', 'root']);
  });
});
