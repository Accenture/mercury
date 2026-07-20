import { buildNodeCommand } from '../commandBuilder';
import { buildClipboardPastePlan } from '../paste';
import type { ClipboardItemRecord } from '../db';
import type { MinigraphGraphData } from '../../utils/graphTypes';

function makeClipboardItem(alias: string): ClipboardItemRecord {
  return {
    id: `id-${alias}`,
    clippedAt: '2026-05-07T00:00:00.000Z',
    sourceWsPath: '/ws/minigraph',
    sourceLabel: 'Minigraph',
    node: {
      alias,
      types: ['Fetcher'],
      properties: {
        skill: 'graph.fetch',
        lines: ['one', 'two'],
      },
    },
    connections: [],
  };
}

function makeGraphData(aliases: string[]): MinigraphGraphData {
  return {
    nodes: aliases.map(alias => ({
      alias,
      types: ['Fetcher'],
      properties: {},
    })),
    connections: [],
  };
}

describe('buildClipboardPastePlan', () => {
  it('returns create when the alias is absent from the current graph', () => {
    const item = makeClipboardItem('fetchpersondata');

    expect(buildClipboardPastePlan(item, makeGraphData(['other-node'])).verb).toBe('create');
    expect(buildClipboardPastePlan(item, null).verb).toBe('create');
  });

  it('returns update when the alias is present in the current graph', () => {
    const item = makeClipboardItem('fetchpersondata');

    expect(buildClipboardPastePlan(item, makeGraphData(['fetchpersondata'])).verb).toBe('update');
  });

  it('returns the exact command text produced by buildNodeCommand', () => {
    const item = makeClipboardItem('fetchpersondata');
    const graphData = makeGraphData(['fetchpersondata']);
    const plan = buildClipboardPastePlan(item, graphData);

    expect(plan.command).toBe(buildNodeCommand('update', item.node));
  });
});