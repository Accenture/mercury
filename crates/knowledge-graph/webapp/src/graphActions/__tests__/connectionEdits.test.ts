import { describe, expect, it } from 'vitest';
import {
  describeRemovedRelations,
  directedRelationTypes,
  planConnectionRemoval,
} from '../connectionEdits';
import type { MinigraphGraphData } from '../../utils/graphTypes';

function graph(): MinigraphGraphData {
  return {
    nodes: [
      { alias: 'root', types: ['Root'], properties: {} },
      { alias: 'fetcher', types: ['Fetcher'], properties: {} },
      { alias: 'end', types: ['End'], properties: {} },
    ],
    connections: [
      { source: 'root', target: 'fetcher', relations: [{ type: 'fetch', properties: {} }, { type: 'test', properties: {} }] },
      { source: 'fetcher', target: 'end', relations: [{ type: 'complete', properties: {} }, { type: 'done', properties: {} }] },
      { source: 'end', target: 'fetcher', relations: [{ type: 'retry', properties: {} }] },
    ],
  };
}

describe('directedRelationTypes', () => {
  it('returns only the requested direction, in model order', () => {
    expect(directedRelationTypes(graph(), 'fetcher', 'end')).toEqual(['complete', 'done']);
    expect(directedRelationTypes(graph(), 'end', 'fetcher')).toEqual(['retry']);
    expect(directedRelationTypes(graph(), 'root', 'end')).toEqual([]);
  });
});

describe('planConnectionRemoval', () => {
  it('removes one relation and reconnects its directed siblings', () => {
    const plan = planConnectionRemoval(graph(), [
      { source: 'root', target: 'fetcher', relation: 'fetch' },
    ]);
    expect(plan?.commands).toEqual([
      'delete connection root and fetcher',
      'connect root to fetcher with test',
    ]);
    expect(plan?.removed).toEqual([
      { source: 'root', target: 'fetcher', relation: 'fetch' },
    ]);
  });

  it('removes the whole directed edge and reconnects the reverse direction', () => {
    const plan = planConnectionRemoval(graph(), [
      { source: 'fetcher', target: 'end' },
    ]);
    expect(plan?.commands).toEqual([
      'delete connection fetcher and end',
      'connect end to fetcher with retry',
    ]);
    expect(plan?.removed).toEqual([
      { source: 'fetcher', target: 'end', relation: 'complete' },
      { source: 'fetcher', target: 'end', relation: 'done' },
    ]);
  });

  it('wipes a pair with no reconnects when both directions are requested together', () => {
    const plan = planConnectionRemoval(graph(), [
      { source: 'fetcher', target: 'end' },
      { source: 'end', target: 'fetcher' },
    ]);
    expect(plan?.commands).toEqual(['delete connection fetcher and end']);
    expect(plan?.removed).toEqual([
      { source: 'fetcher', target: 'end', relation: 'complete' },
      { source: 'fetcher', target: 'end', relation: 'done' },
      { source: 'end', target: 'fetcher', relation: 'retry' },
    ]);
  });

  it('plans independent pairs as sequential compounds', () => {
    const plan = planConnectionRemoval(graph(), [
      { source: 'root', target: 'fetcher' },
      { source: 'fetcher', target: 'end' },
    ]);
    expect(plan?.commands).toEqual([
      'delete connection root and fetcher',
      'delete connection fetcher and end',
      'connect end to fetcher with retry',
    ]);
  });

  it('skips stale requests and returns null when nothing matches', () => {
    expect(planConnectionRemoval(graph(), [
      { source: 'root', target: 'fetcher', relation: 'nope' },
    ])).toBeNull();
    expect(planConnectionRemoval(graph(), [
      { source: 'root', target: 'end' },
    ])).toBeNull();

    const partial = planConnectionRemoval(graph(), [
      { source: 'root', target: 'fetcher', relation: 'nope' },
      { source: 'root', target: 'fetcher', relation: 'fetch' },
    ]);
    expect(partial?.removed).toEqual([
      { source: 'root', target: 'fetcher', relation: 'fetch' },
    ]);
  });
});

describe('describeRemovedRelations', () => {
  it('names a single relation, a shared directed pair, or a count', () => {
    expect(describeRemovedRelations([
      { source: 'root', target: 'fetcher', relation: 'fetch' },
    ])).toBe("relation 'fetch' (root → fetcher)");
    expect(describeRemovedRelations([
      { source: 'fetcher', target: 'end', relation: 'complete' },
      { source: 'fetcher', target: 'end', relation: 'done' },
    ])).toBe('connection fetcher → end');
    expect(describeRemovedRelations([
      { source: 'fetcher', target: 'end', relation: 'complete' },
      { source: 'end', target: 'fetcher', relation: 'retry' },
    ])).toBe('2 relations');
  });
});
