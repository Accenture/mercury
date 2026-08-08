import { describe, expect, it } from 'vitest';
import {
  buildCreateConnectionCommand,
  buildCreateNodeCommand,
  buildDeleteNodeCommand,
  buildUpdateNodeCommand,
} from '../minigraphCommandBuilder';
import type { NodeFormState } from '../nodeAuthoringTypes';
import type { ConnectionFormState } from '../connectionAuthoringTypes';

function formState(overrides: Partial<NodeFormState> = {}): NodeFormState {
  return {
    alias: 'root',
    nodeType: 'Root',
    properties: [
      { id: 'p1', key: 'name', value: 'demo' },
      { id: 'p2', key: '', value: '' },
    ],
    source: 'empty-graph',
    ...overrides,
  };
}

describe('buildCreateNodeCommand', () => {
  it('emits backend create-node command text without trailing newline', () => {
    expect(buildCreateNodeCommand(formState())).toBe([
      'create node root',
      'with type Root',
      'with properties',
      'name=demo',
    ].join('\n'));
  });

  it('omits node type and properties when blank', () => {
    expect(buildCreateNodeCommand(formState({
      nodeType: '  ',
      properties: [{ id: 'p1', key: ' ', value: ' ' }],
    }))).toBe('create node root');
  });

  it('preserves property row order and allows blank values', () => {
    expect(buildCreateNodeCommand(formState({
      properties: [
        { id: 'p1', key: 'first', value: 'one' },
        { id: 'p2', key: 'second', value: '' },
      ],
    }))).toBe([
      'create node root',
      'with type Root',
      'with properties',
      'first=one',
      'second=',
    ].join('\n'));
  });

  it('rejects alias line injection before serialization', () => {
    expect(() => buildCreateNodeCommand(formState({ alias: 'root\nwith properties' }))).toThrow();
  });

  it('rejects node type line injection before serialization', () => {
    expect(() => buildCreateNodeCommand(formState({ nodeType: 'Root\rwith properties' }))).toThrow();
  });

  it('rejects property keys containing equals before serialization', () => {
    expect(() => buildCreateNodeCommand(formState({
      properties: [{ id: 'p1', key: 'bad=key', value: 'value' }],
    }))).toThrow();
  });

  it('rejects property value newline injection', () => {
    expect(() => buildCreateNodeCommand(formState({
      properties: [{ id: 'p1', key: 'name', value: 'demo\nwith properties' }],
    }))).toThrow();
  });

  it('rejects multiline property delimiters', () => {
    expect(() => buildCreateNodeCommand(formState({
      properties: [{ id: 'p1', key: 'name', value: "'''demo" }],
    }))).toThrow();
  });
});

describe('buildUpdateNodeCommand', () => {
  it('emits backend update-node command text against the original alias', () => {
    expect(buildUpdateNodeCommand(formState({
      alias: 'display-only',
      nodeType: 'Fetcher',
      properties: [{ id: 'p1', key: 'name', value: 'updated' }],
      source: 'edit-node',
    }), 'root')).toBe([
      'update node root',
      'with type Fetcher',
      'with properties',
      'name=updated',
    ].join('\n'));
  });

  it('omits node type and properties when blank', () => {
    expect(buildUpdateNodeCommand(formState({
      nodeType: ' ',
      properties: [{ id: 'p1', key: ' ', value: ' ' }],
      source: 'edit-node',
    }), 'root')).toBe('update node root');
  });

  it('rejects invalid original aliases before serialization', () => {
    expect(() => buildUpdateNodeCommand(formState({ source: 'edit-node' }), 'root\nwith properties')).toThrow();
  });

  it('emits flattened path keys and multiline values for edit mode', () => {
    expect(buildUpdateNodeCommand(formState({
      nodeType: 'Evaluator',
      properties: [
        { id: 'p1', key: 'mapping[0]', value: 'text(hello) -> output.body' },
        { id: 'p2', key: 'statement[0]', value: 'IF: true\nTHEN: next' },
      ],
      source: 'edit-node',
    }), 'root')).toBe([
      'update node root',
      'with type Evaluator',
      'with properties',
      'mapping[0]=text(hello) -> output.body',
      "statement[0]='''",
      'IF: true\nTHEN: next',
      "'''",
    ].join('\n'));
  });
});

describe('buildDeleteNodeCommand', () => {
  it('emits backend delete-node command text', () => {
    expect(buildDeleteNodeCommand('root')).toBe('delete node root');
  });

  it('trims alias input', () => {
    expect(buildDeleteNodeCommand('  root  ')).toBe('delete node root');
  });

  it('rejects invalid aliases before serialization', () => {
    expect(() => buildDeleteNodeCommand('root\nwith properties')).toThrow();
  });
});

describe('buildCreateConnectionCommand', () => {
  function connectionState(overrides: Partial<ConnectionFormState> = {}): ConnectionFormState {
    return {
      sourceAlias: 'root',
      targetAlias: 'end',
      relation: 'done',
      ...overrides,
    };
  }

  it('emits backend connect command text', () => {
    expect(buildCreateConnectionCommand(connectionState())).toBe('connect root to end with done');
  });

  it('trims aliases and relation before serialization', () => {
    expect(buildCreateConnectionCommand(connectionState({
      sourceAlias: '  root  ',
      targetAlias: '  end  ',
      relation: 'done',
    }))).toBe('connect root to end with done');
  });

  it('emits a free-text (non-preset) relation verbatim', () => {
    expect(buildCreateConnectionCommand(connectionState({ relation: 'custom' })))
      .toBe('connect root to end with custom');
  });

  it('rejects invalid aliases, self-connections, missing relation, and malformed relation tokens', () => {
    expect(() => buildCreateConnectionCommand(connectionState({ sourceAlias: 'bad.alias' }))).toThrow();
    expect(() => buildCreateConnectionCommand(connectionState({ targetAlias: 'bad\nalias' }))).toThrow();
    expect(() => buildCreateConnectionCommand(connectionState({ targetAlias: 'root' }))).toThrow();
    expect(() => buildCreateConnectionCommand(connectionState({ relation: '' }))).toThrow();
    expect(() => buildCreateConnectionCommand(connectionState({ relation: 'bad relation' }))).toThrow();
  });
});
