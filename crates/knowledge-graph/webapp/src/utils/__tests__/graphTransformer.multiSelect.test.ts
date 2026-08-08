import { describe, expect, it } from 'vitest';
import { transformGraphData } from '../graphTransformer';

describe('transformGraphData multi-select interaction contract', () => {
  it('marks graph nodes so modifier clicks reach React Flow node selection', () => {
    const result = transformGraphData({
      nodes: [{ alias: 'root', types: ['Root'], properties: {} }],
      connections: [],
    });

    expect(result.nodes[0]).toMatchObject({
      id: 'root',
      className: 'nokey',
    });
  });
});
