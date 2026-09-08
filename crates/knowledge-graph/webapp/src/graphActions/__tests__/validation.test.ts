import { describe, expect, it } from 'vitest';
import { createEditNodeFormState } from '../propertyRows';
import {
  getValidationErrorKeyForProperty,
  validateConnectionFormState,
  validateDeleteNodeAlias,
  validateNodeFormState,
} from '../validation';
import { CONNECTION_RELATION_COLORS, CONNECTION_RELATION_OPTIONS } from '../connectionRelations';
import type { NodeFormState } from '../nodeAuthoringTypes';
import type { ConnectionFormState } from '../connectionAuthoringTypes';

function formState(overrides: Partial<NodeFormState> = {}): NodeFormState {
  return {
    alias: 'node-1',
    nodeType: 'Fetcher',
    properties: [],
    source: 'pane-context-menu',
    ...overrides,
  };
}

describe('validateNodeFormState', () => {
  it('requires alias', () => {
    const result = validateNodeFormState(formState({ alias: '   ' }));
    expect(result.errors.alias).toBeDefined();
  });

  it('rejects reserved aliases case-insensitively', () => {
    const result = validateNodeFormState(formState({ alias: 'Input' }));
    expect(result.errors.alias).toContain('reserved');
  });

  it('rejects duplicate aliases known in current graph data case-insensitively', () => {
    const result = validateNodeFormState(formState({ alias: 'Root' }), {
      graphData: {
        nodes: [{ alias: 'root', types: ['Root'], properties: {} }],
        connections: [],
      },
    });
    expect(result.errors.alias).toContain('already exists');
  });

  it('rejects invalid node type token', () => {
    const result = validateNodeFormState(formState({ nodeType: 'Root.Type' }));
    expect(result.errors.nodeType).toBeDefined();
  });

  it('ignores fully blank property rows', () => {
    const result = validateNodeFormState(formState({
      properties: [{ id: 'p1', key: ' ', value: ' ' }],
    }));
    expect(result.valid).toBe(true);
  });

  it('allows a non-blank key with blank value', () => {
    const result = validateNodeFormState(formState({
      properties: [{ id: 'p1', key: 'name', value: ' ' }],
    }));
    expect(result.valid).toBe(true);
  });

  it('rejects a blank key with non-blank value', () => {
    const result = validateNodeFormState(formState({
      properties: [{ id: 'p1', key: ' ', value: 'demo' }],
    }));
    expect(result.errors[getValidationErrorKeyForProperty('p1', 'key')]).toBeDefined();
  });

  it('accepts dot/bracket path keys and multiline values in create mode', () => {
    // Create and edit share one grammar: the backend parses both commands'
    // property lines identically (path keys, [] appends, ''' multiline).
    const result = validateNodeFormState(formState({
      properties: [
        { id: 'p1', key: 'a.b', value: 'demo' },
        { id: 'p2', key: 'input[]', value: 'person_id' },
        { id: 'p3', key: 'statement', value: 'IF: true\nTHEN: next' },
      ],
    }));
    expect(result.valid).toBe(true);
  });

  it('rejects malformed property keys and triple-quote values', () => {
    const badKey = validateNodeFormState(formState({
      properties: [{ id: 'p1', key: 'bad key', value: 'demo' }],
    }));
    const tripleQuote = validateNodeFormState(formState({
      properties: [{ id: 'p2', key: 'name', value: "a'''b" }],
    }));
    expect(badKey.errors[getValidationErrorKeyForProperty('p1', 'key')]).toBeDefined();
    expect(tripleQuote.errors[getValidationErrorKeyForProperty('p2', 'value')]).toBeDefined();
  });
});

describe('validateNodeFormState edit mode', () => {
  it('does not duplicate-check the display alias', () => {
    const result = validateNodeFormState(formState({ alias: 'Root', source: 'edit-node' }), {
      mode: 'edit',
      originalAlias: 'root',
      graphData: {
        nodes: [{ alias: 'root', types: ['Root'], properties: {} }],
        connections: [],
      },
    });
    expect(result.valid).toBe(true);
  });

  it('rejects unsupported original aliases', () => {
    const result = validateNodeFormState(formState({ source: 'edit-node' }), {
      mode: 'edit',
      originalAlias: 'bad.alias',
    });
    expect(result.errors.alias).toBeDefined();
  });

  it('allows flattened path keys and multiline values', () => {
    const result = validateNodeFormState(formState({
      source: 'edit-node',
      properties: [
        { id: 'p1', key: 'mapping[0]', value: 'text(hello) -> output.body' },
        { id: 'p2', key: 'config.items[10].value', value: 'line one\nline two' },
      ],
    }), {
      mode: 'edit',
      originalAlias: 'root',
    });
    expect(result.valid).toBe(true);
  });

  it('accepts the [] append signature and repeated append keys', () => {
    const result = validateNodeFormState(formState({
      source: 'edit-node',
      properties: [
        { id: 'p1', key: 'mapping[]', value: 'text(hello) -> output.body' },
        { id: 'p2', key: 'mapping[]', value: 'text(done) -> output.status' },
      ],
    }), {
      mode: 'edit',
      originalAlias: 'root',
    });
    expect(result.valid).toBe(true);
  });

  it('rejects malformed flattened path keys', () => {
    const result = validateNodeFormState(formState({
      source: 'edit-node',
      properties: [{ id: 'p1', key: 'mapping[01]', value: 'demo' }],
    }), {
      mode: 'edit',
      originalAlias: 'root',
    });
    expect(result.errors[getValidationErrorKeyForProperty('p1', 'key')]).toBeDefined();
  });
});

describe('validateDeleteNodeAlias', () => {
  it('requires the selected alias to exist in current graph data when provided', () => {
    const result = validateDeleteNodeAlias('missing', {
      graphData: {
        nodes: [{ alias: 'root', types: ['Root'], properties: {} }],
        connections: [],
      },
    });
    expect(result.errors.alias).toContain('no longer available');
  });

  it('accepts an existing alias', () => {
    const result = validateDeleteNodeAlias('root', {
      graphData: {
        nodes: [{ alias: 'root', types: ['Root'], properties: {} }],
        connections: [],
      },
    });
    expect(result.valid).toBe(true);
  });
});

describe('createEditNodeFormState', () => {
  it('converts flat scalar properties to editable rows', () => {
    const result = createEditNodeFormState({
      alias: 'root',
      types: ['Root'],
      properties: { name: 'demo', active: true, count: 3 },
    });
    expect(result.valid).toBe(true);
    expect(result.formState).toMatchObject({
      alias: 'root',
      nodeType: 'Root',
      source: 'edit-node',
    });
    expect(result.formState?.properties.map(row => [row.key, row.value])).toEqual([
      ['active', 'true'],
      ['count', '3'],
      ['name', 'demo'],
    ]);
  });

  it('flattens arrays and nested objects into sorted []-signature rows', () => {
    const result = createEditNodeFormState({
      alias: 'root',
      types: ['Root'],
      properties: {
        mapping: ['text(hello) -> output.body', 'text(done) -> output.status'],
        config: { items: [{ value: 'one' }, { value: 'two\nlines' }] },
      },
    });
    expect(result.valid).toBe(true);
    // Keys are sorted (matching the backend edit-node listing) and array
    // indices render as the [] append signature; row order = array order.
    expect(result.formState?.properties.map(row => [row.key, row.value])).toEqual([
      ['config.items[].value', 'one'],
      ['config.items[].value', 'two\nlines'],
      ['mapping[]', 'text(hello) -> output.body'],
      ['mapping[]', 'text(done) -> output.status'],
    ]);
  });

  it('keeps array order for ten or more entries via zero-fill sorting', () => {
    const values = Array.from({ length: 12 }, (_, index) => `entry-${index}`);
    const result = createEditNodeFormState({
      alias: 'root',
      types: ['Root'],
      properties: { mapping: values },
    });
    expect(result.valid).toBe(true);
    // A plain lexicographic sort would put mapping[10] before mapping[2];
    // the zero-fill compare preserves the true array order.
    expect(result.formState?.properties.map(row => row.value)).toEqual(values);
  });

  it('rejects edit surfaces that cannot be represented safely', () => {
    const multipleTypes = createEditNodeFormState({ alias: 'root', types: ['Root', 'Other'], properties: {} });
    const invalidKey = createEditNodeFormState({ alias: 'root', types: ['Root'], properties: { name: { 'bad key': true } } });
    const emptyArray = createEditNodeFormState({ alias: 'root', types: ['Root'], properties: { name: [] } });
    const tripleQuote = createEditNodeFormState({ alias: 'root', types: ['Root'], properties: { name: "a'''b" } });
    expect(multipleTypes.valid).toBe(false);
    expect(invalidKey.valid).toBe(false);
    expect(emptyArray.valid).toBe(false);
    expect(tripleQuote.valid).toBe(false);
  });
});

describe('validateConnectionFormState', () => {
  const graphData = {
    nodes: [
      { alias: 'root', types: ['Root'], properties: {} },
      { alias: 'end', types: ['End'], properties: {} },
    ],
    connections: [],
  };

  function connectionState(overrides: Partial<ConnectionFormState> = {}): ConnectionFormState {
    return {
      sourceAlias: 'root',
      targetAlias: 'end',
      relation: 'done',
      ...overrides,
    };
  }

  it('accepts an existing source, existing target, and supported relation', () => {
    const result = validateConnectionFormState(connectionState(), { graphData, connected: true });
    expect(result.valid).toBe(true);
  });

  it('rejects missing source and target aliases', () => {
    const result = validateConnectionFormState(connectionState({
      sourceAlias: '',
      targetAlias: '',
    }), { graphData });
    expect(result.errors.sourceAlias).toBeDefined();
    expect(result.errors.targetAlias).toBeDefined();
  });

  it('rejects aliases not present in current graph data', () => {
    const result = validateConnectionFormState(connectionState({
      sourceAlias: 'missing-source',
      targetAlias: 'missing-target',
    }), { graphData });
    expect(result.errors.sourceAlias).toContain('no longer available');
    expect(result.errors.targetAlias).toContain('no longer available');
  });

  it('rejects same-node connections case-insensitively', () => {
    const result = validateConnectionFormState(connectionState({
      sourceAlias: 'Root',
      targetAlias: 'root',
    }));
    expect(result.errors.targetAlias).toContain('different');
  });

  it('rejects a missing relation but accepts any command-safe free-text relation', () => {
    const missing = validateConnectionFormState(connectionState({ relation: '' }));
    const custom = validateConnectionFormState(connectionState({ relation: 'custom' }), { graphData });
    expect(missing.errors.relation).toBeDefined();
    expect(custom.errors.relation).toBeUndefined();
    expect(custom.valid).toBe(true);
  });

  it('rejects a relation with invalid token characters', () => {
    const result = validateConnectionFormState(connectionState({ relation: 'bad relation' }));
    expect(result.errors.relation).toContain('letters, numbers');
  });

  it('records disconnected state as a command-level validation error', () => {
    const result = validateConnectionFormState(connectionState(), { graphData, connected: false });
    expect(result.errors.command).toContain('disconnected');
  });
});

describe('connection relation registry', () => {
  it('keeps relation options and known color keys in sync', () => {
    expect(Object.keys(CONNECTION_RELATION_COLORS).sort()).toEqual([...CONNECTION_RELATION_OPTIONS].sort());
  });
});
