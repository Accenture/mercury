import { describe, expect, it } from 'vitest';
import type { MinigraphGraphData } from '../../utils/graphTypes';
import { collectGraphInputBodyPaths } from '../graphInputPaths';

describe('collectGraphInputBodyPaths', () => {
  it('returns sorted unique input.body references from executable node properties', () => {
    const graph: MinigraphGraphData = {
      nodes: [
        {
          alias: 'mapper',
          types: ['Task'],
          properties: {
            mapping: [
              'input.body.customer.id -> model.customer_id',
              '$.input.body.items[*].sku -> model.skus',
            ],
            nested: { expression: 'f:defaultValue(input.body.enabled, boolean(false))' },
            description: 'Example only: input.body.description_should_not_prompt',
            question: 'Does input.body.question_should_not_prompt exist?',
          },
        },
      ],
      connections: [],
    };

    expect(collectGraphInputBodyPaths(graph)).toEqual([
      'input.body.customer.id',
      'input.body.enabled',
      'input.body.items[*].sku',
    ]);
  });

  it('accepts whole-body references but ignores headers and mid-identifier matches', () => {
    const graph: MinigraphGraphData = {
      nodes: [{
        alias: 'task',
        types: ['Task'],
        properties: {
          input: ['input.body -> *', 'input.header.request-id -> header.id'],
          statement: 'myinput.body.fake and input.bodyish are not body paths',
        },
      }],
      connections: [],
    };

    expect(collectGraphInputBodyPaths(graph)).toEqual(['input.body']);
  });

  it('is deterministic and does not mutate graph data', () => {
    const graph: MinigraphGraphData = {
      nodes: [{
        alias: 'root',
        types: ['Root'],
        properties: { mapping: ['input.body.id -> model.id'] },
      }],
      connections: [],
    };
    const before = JSON.stringify(graph);

    expect(collectGraphInputBodyPaths(graph)).toEqual(['input.body.id']);
    expect(collectGraphInputBodyPaths(graph)).toEqual(['input.body.id']);
    expect(JSON.stringify(graph)).toBe(before);
  });
});
