import { describe, expect, it } from 'vitest';
import {
  buildConnectionCreateUndo,
  buildConnectionRemovalUndo,
  buildNodeCreateUndo,
  buildNodeDeleteUndo,
  buildNodeEditUndo,
} from '../undoCommands';
import type { MinigraphGraphData } from '../../utils/graphTypes';

function graph(): MinigraphGraphData {
  return {
    nodes: [
      { alias: 'root', types: ['Root'], properties: { skill: 'graph.data.mapper' } },
      { alias: 'fetcher', types: ['Fetcher'], properties: { input: ['a', 'b'], skill: 'graph.api.fetcher' } },
      { alias: 'end', types: ['End'], properties: {} },
    ],
    connections: [
      { source: 'root', target: 'fetcher', relations: [{ type: 'fetch', properties: {} }] },
      { source: 'fetcher', target: 'end', relations: [{ type: 'complete', properties: {} }, { type: 'done', properties: {} }] },
      { source: 'end', target: 'fetcher', relations: [{ type: 'retry', properties: {} }] },
    ],
  };
}

describe('buildConnectionRemovalUndo', () => {
  it('reconnects exactly the removed directed relations — never the reverse direction', () => {
    const entry = buildConnectionRemovalUndo([
      { source: 'fetcher', target: 'end', relation: 'complete' },
      { source: 'fetcher', target: 'end', relation: 'done' },
    ]);
    expect(entry?.label).toBe('delete connection fetcher → end');
    expect(entry?.inverseCommands).toEqual([
      'connect fetcher to end with complete',
      'connect fetcher to end with done',
    ]);
  });

  it('labels a single-relation removal by its relation', () => {
    const entry = buildConnectionRemovalUndo([
      { source: 'root', target: 'fetcher', relation: 'fetch' },
    ]);
    expect(entry?.label).toBe("delete relation 'fetch' (root → fetcher)");
    expect(entry?.inverseCommands).toEqual(['connect root to fetcher with fetch']);
  });

  it('returns null when nothing was removed', () => {
    expect(buildConnectionRemovalUndo([])).toBeNull();
  });
});

describe('buildNodeEditUndo', () => {
  it('rebuilds the full pre-edit node as an update command', () => {
    const entry = buildNodeEditUndo(graph().nodes[1]);
    expect(entry?.label).toBe('edit node fetcher');
    expect(entry?.inverseCommands).toEqual([[
      'update node fetcher',
      'with type Fetcher',
      'with properties',
      'input[]=a',
      'input[]=b',
      'skill=graph.api.fetcher',
    ].join('\n')]);
  });
});

describe('buildNodeCreateUndo', () => {
  it('deletes the created node', () => {
    expect(buildNodeCreateUndo('fetcher')?.inverseCommands).toEqual(['delete node fetcher']);
  });
});

describe('buildConnectionCreateUndo', () => {
  it('deletes the pair and restores the pre-existing relations', () => {
    const entry = buildConnectionCreateUndo(graph(), 'fetcher', 'end');
    expect(entry?.inverseCommands).toEqual([
      'delete connection fetcher and end',
      'connect fetcher to end with complete',
      'connect fetcher to end with done',
      'connect end to fetcher with retry',
    ]);
  });

  it('only deletes when the pair had no prior relations', () => {
    const entry = buildConnectionCreateUndo(graph(), 'root', 'end');
    expect(entry?.inverseCommands).toEqual(['delete connection root and end']);
  });
});

describe('buildNodeDeleteUndo', () => {
  it('recreates the node and reconnects every relation touching it', () => {
    const entry = buildNodeDeleteUndo(graph(), graph().nodes[1]);
    expect(entry?.inverseCommands).toEqual([
      [
        'create node fetcher',
        'with type Fetcher',
        'with properties',
        'input[]=a',
        'input[]=b',
        'skill=graph.api.fetcher',
      ].join('\n'),
      'connect root to fetcher with fetch',
      'connect fetcher to end with complete',
      'connect fetcher to end with done',
      'connect end to fetcher with retry',
    ]);
  });
});
