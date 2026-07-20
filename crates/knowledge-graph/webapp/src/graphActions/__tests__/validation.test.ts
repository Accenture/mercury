import { describe, expect, it } from 'vitest';
import { createEditNodeFormState } from '../propertyRows';
import { getValidationErrorKeyForProperty, validateDeleteNodeAlias, validateNodeFormState } from '../validation';
import type { NodeFormState } from '../nodeAuthoringTypes';

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

  it('rejects property keys outside the backend name token', () => {
    const result = validateNodeFormState(formState({
      properties: [{ id: 'p1', key: 'a.b', value: 'demo' }],
    }));
    expect(result.errors[getValidationErrorKeyForProperty('p1', 'key')]).toBeDefined();
  });

  it('rejects multiline and triple-quote property values', () => {
    const newline = validateNodeFormState(formState({
      properties: [{ id: 'p1', key: 'name', value: 'a\nb' }],
    }));
    const tripleQuote = validateNodeFormState(formState({
      properties: [{ id: 'p2', key: 'name', value: "a'''b" }],
    }));
    expect(newline.errors[getValidationErrorKeyForProperty('p1', 'value')]).toBeDefined();
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

  it('rejects malformed flattened path keys', () => {
    const result = validateNodeFormState(formState({
      source: 'edit-node',
      properties: [{ id: 'p1', key: 'mapping[]', value: 'demo' }],
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
      ['name', 'demo'],
      ['active', 'true'],
      ['count', '3'],
    ]);
  });

  it('flattens arrays, nested objects, and multiline leaf values', () => {
    const result = createEditNodeFormState({
      alias: 'root',
      types: ['Root'],
      properties: {
        mapping: ['text(hello) -> output.body', 'text(done) -> output.status'],
        config: { items: [{ value: 'one' }, { value: 'two\nlines' }] },
      },
    });
    expect(result.valid).toBe(true);
    expect(result.formState?.properties.map(row => [row.key, row.value])).toEqual([
      ['mapping[0]', 'text(hello) -> output.body'],
      ['mapping[1]', 'text(done) -> output.status'],
      ['config.items[0].value', 'one'],
      ['config.items[1].value', 'two\nlines'],
    ]);
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
