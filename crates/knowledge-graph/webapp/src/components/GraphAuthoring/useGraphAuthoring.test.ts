import { describe, expect, it } from 'vitest';
import type { NodeActionTextResultEvent } from '../../protocol/events';
import type { ConnectionFormState } from '../../graphActions/connectionAuthoringTypes';
import {
  resolveAuthoringDisconnect,
  resolveAuthoringSendFailure,
  resolveAuthoringTimeout,
  resolvePendingAuthoringEvent,
  type AuthoringState,
  type PendingNodeActionSubmit,
} from './useGraphAuthoring';

const CONNECTION_FORM: ConnectionFormState = {
  sourceAlias: 'root',
  targetAlias: 'end',
  relation: 'done',
};

function connectionPending(
  overrides: Partial<PendingNodeActionSubmit> = {},
): PendingNodeActionSubmit {
  return {
    action: 'create-connection',
    alias: 'root',
    targetAlias: 'end',
    command: 'connect root to end with done',
    sentAt: '2026-07-24T00:00:00.000Z',
    ...overrides,
  };
}

function connectionState(
  overrides: Partial<Extract<AuthoringState, { status: 'open'; action: 'create-connection' }>> = {},
): Extract<AuthoringState, { status: 'open'; action: 'create-connection' }> {
  return {
    status: 'open',
    action: 'create-connection',
    phase: 'sending',
    formState: CONNECTION_FORM,
    pendingSubmit: connectionPending(),
    serverMessage: null,
    connectionLost: false,
    ...overrides,
  };
}

function connectionEvent(
  overrides: Partial<NodeActionTextResultEvent> = {},
): NodeActionTextResultEvent {
  return {
    kind: 'minigraph.nodeAction.textResult',
    msgId: 1,
    raw: 'node root connected to end',
    status: 'accepted',
    action: 'create-connection',
    alias: 'root',
    targetAlias: 'end',
    message: 'node root connected to end',
    ...overrides,
  };
}

describe('connection authoring lifecycle', () => {
  it('accepts only the result matching both pending endpoints', () => {
    const accepted = resolvePendingAuthoringEvent(
      connectionState(),
      connectionEvent(),
    );
    const otherTarget = resolvePendingAuthoringEvent(
      connectionState(),
      connectionEvent({
        raw: 'node root connected to other',
        targetAlias: 'other',
        message: 'node root connected to other',
      }),
    );

    expect(accepted).toMatchObject({
      state: { status: 'closed', pendingSubmit: null },
      acceptedResult: {
        action: 'create-connection',
        alias: 'root',
        targetAlias: 'end',
      },
    });
    expect(otherTarget).toBeNull();
  });

  it('returns a rejected connection submit to editable state', () => {
    const transition = resolvePendingAuthoringEvent(
      connectionState(),
      connectionEvent({
        raw: 'Source and target nodes must be different',
        status: 'rejected',
        alias: null,
        targetAlias: null,
        message: 'Source and target nodes must be different',
      }),
    );

    expect(transition).toMatchObject({
      state: {
        status: 'open',
        phase: 'editing',
        pendingSubmit: null,
        serverMessage: 'Source and target nodes must be different',
      },
    });
  });

  it('keeps form values editable when transport send returns false', () => {
    expect(resolveAuthoringSendFailure(connectionState())).toEqual({
      ...connectionState(),
      phase: 'editing',
      pendingSubmit: null,
      serverMessage: 'Could not send the create-connection command because the WebSocket is not open. The form values remain in this dialog.',
    });
  });

  it('returns an open pending submit to editing after timeout', () => {
    expect(resolveAuthoringTimeout(connectionState())).toMatchObject({
      state: {
        status: 'open',
        phase: 'editing',
        pendingSubmit: null,
        serverMessage: expect.stringContaining('outcome is unknown'),
      },
    });
  });

  it('locks an open connection draft when the WebSocket disconnects', () => {
    expect(resolveAuthoringDisconnect(connectionState())).toMatchObject({
      state: {
        status: 'open',
        phase: 'editing',
        pendingSubmit: null,
        connectionLost: true,
        serverMessage: expect.stringContaining('outcome is unknown'),
      },
    });
  });
});
