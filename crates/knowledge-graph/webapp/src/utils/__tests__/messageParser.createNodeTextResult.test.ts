import { describe, expect, it } from 'vitest';
import { parseCreateNodeTextResult, parseNodeActionTextResult } from '../messageParser';

describe('parseCreateNodeTextResult', () => {
  it('parses accepted create-node text', () => {
    expect(parseCreateNodeTextResult('node root created')).toEqual({
      status: 'accepted',
      alias: 'root',
      message: 'node root created',
    });
  });

  it('parses duplicate create-node text as rejected', () => {
    expect(parseCreateNodeTextResult('node root already exists')).toEqual({
      status: 'rejected',
      alias: 'root',
      message: 'node root already exists',
    });
  });

  it('parses generic backend errors without alias', () => {
    expect(parseCreateNodeTextResult('ERROR: alias is reserved')).toEqual({
      status: 'error',
      alias: null,
      message: 'ERROR: alias is reserved',
    });
  });

  it('ignores command echoes and unrelated node operations', () => {
    expect(parseCreateNodeTextResult('> create node root')).toBeNull();
    expect(parseCreateNodeTextResult('node root updated')).toBeNull();
    expect(parseCreateNodeTextResult('{"type":"info"}')).toBeNull();
  });
});

describe('parseNodeActionTextResult', () => {
  it('parses create, edit, and delete accepted text', () => {
    expect(parseNodeActionTextResult('node root created')).toMatchObject({
      status: 'accepted',
      action: 'create-node',
      alias: 'root',
    });
    expect(parseNodeActionTextResult('Node root updated')).toMatchObject({
      status: 'accepted',
      action: 'edit-node',
      alias: 'root',
    });
    expect(parseNodeActionTextResult('node root deleted')).toMatchObject({
      status: 'accepted',
      action: 'delete-node',
      alias: 'root',
    });
  });

  it('parses rejected and generic error text', () => {
    expect(parseNodeActionTextResult('node root already exists')).toMatchObject({
      status: 'rejected',
      action: 'create-node',
      alias: 'root',
    });
    expect(parseNodeActionTextResult('node root not found')).toMatchObject({
      status: 'rejected',
      action: null,
      alias: 'root',
    });
    expect(parseNodeActionTextResult('ERROR: bad command')).toMatchObject({
      status: 'error',
      action: null,
      alias: null,
    });
  });

  it('ignores echoed commands', () => {
    expect(parseNodeActionTextResult('> update node root...')).toBeNull();
  });
});
