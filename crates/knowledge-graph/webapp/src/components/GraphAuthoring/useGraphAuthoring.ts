import { useCallback, useEffect, useRef, useState } from 'react';
import type { ProtocolBus } from '../../protocol/bus';
import type { NodeActionTextResultEvent } from '../../protocol/events';
import type { MinigraphGraphData, MinigraphNode } from '../../utils/graphTypes';
import type { NodeActionTextResult } from '../../utils/messageParser';
import type { GraphAuthoringExecutor } from '../../graphActions/graphAuthoringExecutor';
import {
  buildBatchDeleteFeedback,
  isBatchDeleteComplete,
  MAX_BATCH_NODE_ACTIONS,
  recordBatchDeleteResult,
  type PendingBatchDeleteSubmit,
} from '../../graphActions/batchNodeActions';
import {
  buildCreateConnectionCommand,
  buildCreateNodeCommand,
  buildDeleteNodeCommand,
  buildUpdateNodeCommand,
} from '../../graphActions/minigraphCommandBuilder';
import type { ConnectionFormState } from '../../graphActions/connectionAuthoringTypes';
import type {
  NodeAction,
  NodeFormState,
  NodeFormSource,
  NodeFormValidationErrors,
} from '../../graphActions/nodeAuthoringTypes';
import { createDefaultNodeFormState, createEditNodeFormState } from '../../graphActions/propertyRows';
import { validateConnectionFormState, validateDeleteNodeAlias, validateNodeFormState } from '../../graphActions/validation';

export const DEFAULT_AUTHORING_TIMEOUT_MS = 10_000;

type EditableNodeAction = Extract<NodeAction, 'create-node' | 'edit-node'>;
type EditableAuthoringAction = Extract<NodeAction, 'create-node' | 'edit-node' | 'create-connection'>;
type CreateNodeFormSource = Exclude<NodeFormSource, 'edit-node'>;
type UserMessageType = 'info' | 'success' | 'error';

// AuthoringState is the UI-facing state machine for node actions.
// - closed: no modal is open, but a delete action may still be waiting for backend confirmation.
// - open/editing: create/edit modal is editable.
// - open/sending: modal stays open while a create/edit command is waiting for backend confirmation.
export type AuthoringState =
  | {
      status: 'closed';
      pendingSubmit: PendingAuthoringSubmit | null;
      serverMessage: string | null;
    }
  | {
      status: 'open';
      action: EditableNodeAction;
      phase: 'editing' | 'sending';
      formState: NodeFormState;
      originalAlias: string | null;
      pendingSubmit: PendingAuthoringSubmit | null;
      serverMessage: string | null;
      connectionLost: boolean;
    }
  | {
      status: 'open';
      action: 'create-connection';
      phase: 'editing' | 'sending';
      formState: ConnectionFormState;
      pendingSubmit: PendingAuthoringSubmit | null;
      serverMessage: string | null;
      connectionLost: boolean;
    };

// Single-command pending state for create/edit/delete. Batch delete uses the
// wider PendingBatchDeleteSubmit shape from batchNodeActions.ts.
export interface PendingNodeActionSubmit {
  action: NodeAction;
  alias: string | null;
  targetAlias?: string | null;
  command: string;
  sentAt: string;
}

export type PendingAuthoringSubmit = PendingNodeActionSubmit | PendingBatchDeleteSubmit;

export interface UseGraphAuthoringOptions {
  bus: ProtocolBus;
  connected: boolean;
  graphData: MinigraphGraphData | null;
  executor: GraphAuthoringExecutor;
  timeoutMs?: number;
  onAccepted?: (result: NodeActionTextResult) => void;
  onUserMessage?: (message: string, type?: UserMessageType) => void;
}

export interface UseGraphAuthoringReturn {
  state: AuthoringState;
  validationErrors: NodeFormValidationErrors;
  openCreateNode: (source: CreateNodeFormSource) => void;
  openCreateConnection: (sourceAlias: string, targetAlias: string) => void;
  openEditNode: (node: MinigraphNode) => void;
  deleteNode: (node: MinigraphNode) => void;
  deleteNodes: (nodes: MinigraphNode[]) => void;
  updateFormState: (formState: NodeFormState | ConnectionFormState) => void;
  submit: () => void;
  close: () => void;
}

const PENDING_ACTION_MESSAGE = 'A graph authoring action is already pending. Wait for it to finish before starting another.';
const CREATE_SEND_FAILURE_MESSAGE = 'Could not send the create-node command because the WebSocket is not open. The form values remain in this dialog.';
const EDIT_SEND_FAILURE_MESSAGE = 'Could not send the edit-node command because the WebSocket is not open. Your changes remain in this dialog.';
const DELETE_SEND_FAILURE_MESSAGE = 'Could not send the delete-node command because the WebSocket is not open.';
const CONNECTION_SEND_FAILURE_MESSAGE = 'Could not send the create-connection command because the WebSocket is not open. The form values remain in this dialog.';
const DELETE_NODES_SEND_FAILURE_MESSAGE = 'Could not send delete-node commands because the WebSocket is not open.';
const DELETE_NODES_EMPTY_MESSAGE = 'No selected nodes are available to delete.';
const DELETE_NODES_LIMIT_MESSAGE = 'Select 100 or fewer nodes to delete at once.';
const DELETE_NODES_TIMEOUT_MESSAGE = 'Some delete-node commands were sent, but not all backend results were observed yet. Refresh the graph before trying again.';
const NODE_UNAVAILABLE_MESSAGE = 'This node is no longer available in the current graph.';
const CREATE_DISCONNECTED_EDITING_MESSAGE = 'Connection disconnected. Refresh the page and create the node again after the app reconnects.';
const EDIT_DISCONNECTED_EDITING_MESSAGE = 'Connection disconnected. Refresh the page and edit the node again after the app reconnects.';
const CONNECTION_DISCONNECTED_EDITING_MESSAGE = 'Connection disconnected. Refresh the page and create the connection again after the app reconnects.';
const PENDING_DISCONNECTED_MESSAGE = 'Connection disconnected while the graph authoring action was pending. The outcome is unknown. Refresh the page and check the graph before trying again.';

const CLOSED_STATE: AuthoringState = { status: 'closed', pendingSubmit: null, serverMessage: null };

function getPendingSubmit(state: AuthoringState): PendingAuthoringSubmit | null {
  return state.pendingSubmit;
}

function isPendingBatchDelete(
  pending: PendingAuthoringSubmit,
): pending is PendingBatchDeleteSubmit {
  return pending.action === 'delete-nodes';
}

function getSendFailureMessage(action: NodeAction): string {
  if (action === 'create-connection') return CONNECTION_SEND_FAILURE_MESSAGE;
  if (action === 'edit-node') return EDIT_SEND_FAILURE_MESSAGE;
  if (action === 'delete-node') return DELETE_SEND_FAILURE_MESSAGE;
  return CREATE_SEND_FAILURE_MESSAGE;
}

function getTimeoutMessage(pending: PendingAuthoringSubmit): string {
  if (isPendingBatchDelete(pending)) return DELETE_NODES_TIMEOUT_MESSAGE;
  return `The ${pending.action} command was sent, but no backend result was observed yet. The outcome is unknown.`;
}

function getDisconnectedEditingMessage(action: EditableAuthoringAction): string {
  if (action === 'create-connection') return CONNECTION_DISCONNECTED_EDITING_MESSAGE;
  return action === 'edit-node' ? EDIT_DISCONNECTED_EDITING_MESSAGE : CREATE_DISCONNECTED_EDITING_MESSAGE;
}

function aliasesMatch(left: string | null | undefined, right: string | null | undefined): boolean {
  if (!left || !right) return false;
  return left.trim().toLowerCase() === right.trim().toLowerCase();
}

function findCurrentNodeByAlias(
  graphData: MinigraphGraphData | null,
  alias: string,
): MinigraphNode | null {
  return graphData?.nodes.find((node) => node.alias.toLowerCase() === alias.toLowerCase()) ?? null;
}

function eventMatchesPendingAction(
  event: NodeActionTextResultEvent,
  pending: PendingNodeActionSubmit,
): boolean {
  // Backend error text can be generic, so a single-command pending submit treats
  // any error event as relevant. Successful events still need alias/action match.
  if (event.status === 'error') return true;

  if (pending.action === 'create-connection') {
    if (event.action === 'create-connection') {
      if (event.alias === null) return true;
      if (!aliasesMatch(event.alias, pending.alias)) return false;
      return event.status === 'accepted'
        ? aliasesMatch(event.targetAlias, pending.targetAlias)
        : event.targetAlias === null || aliasesMatch(event.targetAlias, pending.targetAlias);
    }
    if (event.action === null) {
      return aliasesMatch(event.alias, pending.alias) || aliasesMatch(event.alias, pending.targetAlias);
    }
    return false;
  }

  if (!aliasesMatch(event.alias, pending.alias)) return false;
  return event.action === null || event.action === pending.action;
}

export interface AuthoringTransition {
  state: AuthoringState;
  acceptedResult?: NodeActionTextResult;
  notification?: {
    message: string;
    type: UserMessageType;
  };
}

export function resolvePendingAuthoringEvent(
  current: AuthoringState,
  event: NodeActionTextResultEvent,
): AuthoringTransition | null {
  const pending = getPendingSubmit(current);
  if (!pending || isPendingBatchDelete(pending)) return null;
  if (!eventMatchesPendingAction(event, pending)) return null;

  if (event.status === 'accepted') {
    return {
      state: CLOSED_STATE,
      acceptedResult: {
        status: event.status,
        action: event.action,
        alias: event.alias,
        targetAlias: event.targetAlias,
        message: event.message,
      },
    };
  }

  if (current.status === 'open') {
    return {
      state: {
        ...current,
        phase: 'editing',
        pendingSubmit: null,
        serverMessage: event.status === 'error'
          ? `Backend returned an error while this submit was pending: ${event.message}`
          : event.message,
      },
    };
  }

  return {
    state: CLOSED_STATE,
    notification: { message: event.message, type: 'error' },
  };
}

export function resolveAuthoringSendFailure(
  current: Extract<AuthoringState, { status: 'open' }>,
): Extract<AuthoringState, { status: 'open' }> {
  return {
    ...current,
    phase: 'editing',
    pendingSubmit: null,
    serverMessage: getSendFailureMessage(current.action),
  };
}

export function resolveAuthoringTimeout(
  current: AuthoringState,
): AuthoringTransition | null {
  const pending = getPendingSubmit(current);
  if (!pending) return null;

  const message = getTimeoutMessage(pending);
  if (current.status === 'open') {
    return {
      state: {
        ...current,
        phase: 'editing',
        pendingSubmit: null,
        serverMessage: message,
      },
    };
  }

  return {
    state: CLOSED_STATE,
    notification: { message, type: 'error' },
  };
}

export function resolveAuthoringDisconnect(
  current: AuthoringState,
): AuthoringTransition | null {
  const pending = getPendingSubmit(current);

  if (current.status === 'open') {
    const message = pending
      ? PENDING_DISCONNECTED_MESSAGE
      : getDisconnectedEditingMessage(current.action);
    return {
      state: {
        ...current,
        phase: 'editing',
        pendingSubmit: null,
        serverMessage: message,
        connectionLost: true,
      },
    };
  }

  if (!pending) return null;
  return {
    state: CLOSED_STATE,
    notification: { message: PENDING_DISCONNECTED_MESSAGE, type: 'error' },
  };
}

function isConnectionFormState(
  formState: NodeFormState | ConnectionFormState,
): formState is ConnectionFormState {
  return 'sourceAlias' in formState;
}

// Owns the complete graph-authoring lifecycle: form state, validation,
// raw-command send, text-result matching, timeout handling, and disconnect
// handling. UI components call this hook instead of sending commands or
// interpreting backend text themselves.
export function useGraphAuthoring({
  bus,
  connected,
  graphData,
  executor,
  timeoutMs = DEFAULT_AUTHORING_TIMEOUT_MS,
  onAccepted,
  onUserMessage,
}: UseGraphAuthoringOptions): UseGraphAuthoringReturn {
  const [state, setState] = useState<AuthoringState>(CLOSED_STATE);
  const [validationErrors, setValidationErrors] = useState<NodeFormValidationErrors>({});

  const stateRef = useRef(state);
  const timeoutRef = useRef<ReturnType<typeof setTimeout> | null>(null);
  const wasConnectedRef = useRef(connected);
  const graphDataRef = useRef(graphData);
  const onAcceptedRef = useRef(onAccepted);
  const onUserMessageRef = useRef(onUserMessage);

  // ProtocolBus and timers can fire after render, so callbacks read refs for
  // the latest state/context without re-subscribing on every form keystroke.
  useEffect(() => { stateRef.current = state; }, [state]);
  useEffect(() => { graphDataRef.current = graphData; }, [graphData]);
  useEffect(() => { onAcceptedRef.current = onAccepted; }, [onAccepted]);
  useEffect(() => { onUserMessageRef.current = onUserMessage; }, [onUserMessage]);

  const notifyUser = useCallback((message: string, type: UserMessageType = 'error') => {
    onUserMessageRef.current?.(message, type);
  }, []);

  const setAuthoringState = useCallback((next: AuthoringState) => {
    stateRef.current = next;
    setState(next);
  }, []);

  // Every submitted action starts one timeout. The timeout message differs
  // depending on whether the modal is still open or the action happened from a
  // context menu with no modal to return to.
  const clearPendingTimer = useCallback(() => {
    if (timeoutRef.current !== null) {
      clearTimeout(timeoutRef.current);
      timeoutRef.current = null;
    }
  }, []);

  const startSubmitTimer = useCallback(() => {
    clearPendingTimer();
    timeoutRef.current = setTimeout(() => {
      const transition = resolveAuthoringTimeout(stateRef.current);
      if (transition) {
        setAuthoringState(transition.state);
        if (transition.notification) {
          notifyUser(transition.notification.message, transition.notification.type);
        }
      }
      timeoutRef.current = null;
    }, timeoutMs);
  }, [clearPendingTimer, notifyUser, setAuthoringState, timeoutMs]);

  // Open create starts from defaults only. Form values live in memory inside
  // this hook; closing or refreshing drops them.
  const openCreateNode = useCallback((source: CreateNodeFormSource) => {
    if (!connected) return;
    if (getPendingSubmit(stateRef.current)) {
      notifyUser(PENDING_ACTION_MESSAGE, 'error');
      return;
    }

    const formState = createDefaultNodeFormState(source);
    setValidationErrors({});
    setAuthoringState({
      status: 'open',
      action: 'create-node',
      phase: 'editing',
      formState,
      originalAlias: null,
      pendingSubmit: null,
      serverMessage: null,
      connectionLost: false,
    });
  }, [connected, notifyUser, setAuthoringState]);

  const openCreateConnection = useCallback((sourceAlias: string, targetAlias: string) => {
    if (!connected) return;
    if (getPendingSubmit(stateRef.current)) {
      notifyUser(PENDING_ACTION_MESSAGE, 'error');
      return;
    }

    const formState: ConnectionFormState = {
      sourceAlias,
      targetAlias,
      // Start empty so the dialog shows only the placeholder; the user types
      // the relation. The open-guard below ignores the resulting
      // "relation required" error since the relation is user-entered.
      relation: '',
    };
    const validation = validateConnectionFormState(formState, {
      graphData: graphDataRef.current,
      connected,
    });
    const { relation: _relationError, ...blockingErrors } = validation.errors;
    if (Object.keys(blockingErrors).length > 0) return;

    setValidationErrors({});
    setAuthoringState({
      status: 'open',
      action: 'create-connection',
      phase: 'editing',
      formState,
      pendingSubmit: null,
      serverMessage: null,
      connectionLost: false,
    });
  }, [connected, notifyUser, setAuthoringState]);

  // Open edit snapshots the node from the latest graphData so the form does not
  // edit stale data passed from an old right-click menu.
  const openEditNode = useCallback((node: MinigraphNode) => {
    if (!connected) {
      notifyUser(EDIT_DISCONNECTED_EDITING_MESSAGE, 'error');
      return;
    }
    if (getPendingSubmit(stateRef.current)) {
      notifyUser(PENDING_ACTION_MESSAGE, 'error');
      return;
    }

    const currentNode = findCurrentNodeByAlias(graphDataRef.current, node.alias);
    if (!currentNode) {
      notifyUser(NODE_UNAVAILABLE_MESSAGE, 'error');
      return;
    }

    const conversion = createEditNodeFormState(currentNode);
    if (!conversion.valid || !conversion.formState) {
      notifyUser(conversion.message ?? 'This node cannot be edited in the UI.', 'error');
      return;
    }

    setValidationErrors({});
    setAuthoringState({
      status: 'open',
      action: 'edit-node',
      phase: 'editing',
      formState: conversion.formState,
      originalAlias: currentNode.alias,
      pendingSubmit: null,
      serverMessage: null,
      connectionLost: false,
    });
  }, [connected, notifyUser, setAuthoringState]);

  // Single-node delete has no modal. It validates the alias against the latest
  // graphData, sends one raw command, then waits for one backend text result.
  const deleteNode = useCallback((node: MinigraphNode) => {
    if (!connected) {
      notifyUser(DELETE_SEND_FAILURE_MESSAGE, 'error');
      return;
    }
    if (getPendingSubmit(stateRef.current)) {
      notifyUser(PENDING_ACTION_MESSAGE, 'error');
      return;
    }

    const validation = validateDeleteNodeAlias(node.alias, { graphData: graphDataRef.current });
    if (!validation.valid) {
      notifyUser(Object.values(validation.errors)[0] ?? 'Invalid node alias.', 'error');
      return;
    }

    let command: string;
    try {
      command = buildDeleteNodeCommand(node.alias, { graphData: graphDataRef.current });
    } catch (err) {
      notifyUser(err instanceof Error ? err.message : String(err), 'error');
      return;
    }

    const sent = executor.execute(command);
    if (!sent) {
      notifyUser(DELETE_SEND_FAILURE_MESSAGE, 'error');
      return;
    }

    const pending: PendingAuthoringSubmit = {
      action: 'delete-node',
      alias: node.alias.trim(),
      command,
      sentAt: new Date().toISOString(),
    };
    setValidationErrors({});
    setAuthoringState({ status: 'closed', pendingSubmit: pending, serverMessage: null });
    startSubmitTimer();
  }, [connected, executor, notifyUser, setAuthoringState, startSubmitTimer]);

  // Multi-select delete is intentionally implemented as N individual delete
  // commands because the backend command protocol is still node-scoped. The
  // batch pending state lets the UI report one combined result after all node
  // responses are observed.
  const deleteNodes = useCallback((selectedNodes: MinigraphNode[]) => {
    if (!connected) {
      notifyUser(DELETE_NODES_SEND_FAILURE_MESSAGE, 'error');
      return;
    }
    if (getPendingSubmit(stateRef.current)) {
      notifyUser(PENDING_ACTION_MESSAGE, 'error');
      return;
    }
    if (selectedNodes.length === 0) {
      notifyUser(DELETE_NODES_EMPTY_MESSAGE, 'info');
      return;
    }
    if (selectedNodes.length > MAX_BATCH_NODE_ACTIONS) {
      notifyUser(DELETE_NODES_LIMIT_MESSAGE, 'error');
      return;
    }

    const seenAliases = new Set<string>();
    const nodes = selectedNodes.filter((node) => {
      const normalized = node.alias.trim().toLowerCase();
      if (seenAliases.has(normalized)) return false;
      seenAliases.add(normalized);
      return true;
    });

    // Build every command before sending any of them. If validation fails, no
    // partial batch is sent.
    const commands: string[] = [];
    const aliases: string[] = [];
    for (const node of nodes) {
      const validation = validateDeleteNodeAlias(node.alias, { graphData: graphDataRef.current });
      if (!validation.valid) {
        notifyUser(Object.values(validation.errors)[0] ?? DELETE_NODES_EMPTY_MESSAGE, 'error');
        return;
      }
      try {
        aliases.push(node.alias.trim());
        commands.push(buildDeleteNodeCommand(node.alias, { graphData: graphDataRef.current }));
      } catch (err) {
        notifyUser(err instanceof Error ? err.message : String(err), 'error');
        return;
      }
    }

    // Send sequentially through the same executor used by create/edit/delete.
    // If the socket fails mid-batch, the user gets an unknown-outcome message
    // because earlier commands may already be processing on the backend.
    for (const [index, command] of commands.entries()) {
      if (!executor.execute(command)) {
        notifyUser(
          index === 0 ? DELETE_NODES_SEND_FAILURE_MESSAGE : PENDING_DISCONNECTED_MESSAGE,
          'error',
        );
        return;
      }
    }

    const pending: PendingBatchDeleteSubmit = {
      action: 'delete-nodes',
      aliases,
      commands,
      sentAt: new Date().toISOString(),
      results: {},
    };
    setValidationErrors({});
    setAuthoringState({ status: 'closed', pendingSubmit: pending, serverMessage: null });
    notifyUser(`${aliases.length} delete-node commands sent. Waiting for backend response.`, 'info');
    startSubmitTimer();
  }, [connected, executor, notifyUser, setAuthoringState, startSubmitTimer]);

  // Form changes are ignored while sending or after connection loss so the
  // visible modal cannot drift away from the command/pending state.
  const updateFormState = useCallback((formState: NodeFormState | ConnectionFormState) => {
    const current = stateRef.current;
    if (current.status !== 'open') return;
    if (current.phase === 'sending' || current.connectionLost) return;

    setValidationErrors({});

    if (current.action === 'create-connection') {
      if (!isConnectionFormState(formState)) return;
      setAuthoringState({
        ...current,
        formState,
        pendingSubmit: null,
        serverMessage: null,
        connectionLost: false,
      });
      return;
    }

    if (isConnectionFormState(formState)) return;
    setAuthoringState({
      ...current,
      formState,
      pendingSubmit: null,
      serverMessage: null,
      connectionLost: false,
    });
  }, [setAuthoringState]);

  // Create/edit submit path: validate the current form, build one raw minigraph
  // command, send it through the executor, then keep the modal in sending state
  // until the backend text result arrives.
  const submit = useCallback(() => {
    const current = stateRef.current;
    if (current.status !== 'open') return;
    if (current.phase === 'sending') return;
    if (current.connectionLost) return;

    const action = current.action;
    if (!connected) {
      setAuthoringState({
        ...current,
        serverMessage: getSendFailureMessage(action),
      });
      return;
    }

    const validation = current.action === 'create-connection'
      ? validateConnectionFormState(current.formState, {
        graphData: graphDataRef.current,
        connected,
      })
      : validateNodeFormState(
        current.formState,
        current.action === 'edit-node'
          ? { mode: 'edit', originalAlias: current.originalAlias }
          : { graphData: graphDataRef.current },
      );
    if (!validation.valid) {
      setValidationErrors(validation.errors);
      return;
    }

    let command: string;
    let pendingAlias: string | null;
    let pendingTargetAlias: string | null = null;
    try {
      if (current.action === 'edit-node') {
        pendingAlias = current.originalAlias?.trim() ?? '';
        command = buildUpdateNodeCommand(current.formState, pendingAlias);
      } else if (current.action === 'create-node') {
        pendingAlias = current.formState.alias.trim();
        command = buildCreateNodeCommand(current.formState);
      } else if (isConnectionFormState(current.formState)) {
        pendingAlias = current.formState.sourceAlias.trim();
        pendingTargetAlias = current.formState.targetAlias.trim();
        command = buildCreateConnectionCommand(current.formState);
      } else {
        setValidationErrors({ command: 'Invalid connection form state.' });
        return;
      }
    } catch (err) {
      setValidationErrors({ command: err instanceof Error ? err.message : String(err) });
      return;
    }

    const sent = executor.execute(command);
    if (!sent) {
      setAuthoringState(resolveAuthoringSendFailure(current));
      return;
    }

    const pending: PendingAuthoringSubmit = {
      action,
      alias: pendingAlias,
      targetAlias: pendingTargetAlias,
      command,
      sentAt: new Date().toISOString(),
    };
    setValidationErrors({});
    setAuthoringState({
      ...current,
      phase: 'sending',
      pendingSubmit: pending,
      serverMessage: null,
      connectionLost: false,
    });
    startSubmitTimer();
  }, [connected, executor, setAuthoringState, startSubmitTimer]);

  // Closing is disabled while a create/edit command is pending so the user does
  // not lose the backend error/success context mid-submit.
  const close = useCallback(() => {
    const current = stateRef.current;
    if (current.status !== 'open') return;
    if (current.phase === 'sending') return;
    clearPendingTimer();
    setValidationErrors({});
    setAuthoringState(CLOSED_STATE);
  }, [clearPendingTimer, setAuthoringState]);

  // All backend node-action text results enter here. The hook first tries the
  // batch-delete matcher, then falls back to the single-command matcher.
  useEffect(() => {
    return bus.on('minigraph.nodeAction.textResult', (event: NodeActionTextResultEvent) => {
      const current = stateRef.current;
      const pending = getPendingSubmit(current);
      if (!pending) return;

      if (isPendingBatchDelete(pending)) {
        // Batch delete may receive results in any order. Keep waiting until all
        // selected aliases have either success or error.
        const nextPending = recordBatchDeleteResult(pending, event);
        if (!nextPending) return;

        if (event.status === 'accepted') {
          onAcceptedRef.current?.({
            status: event.status,
            action: event.action,
            alias: event.alias,
            targetAlias: event.targetAlias,
            message: event.message,
          });
        }

        if (!isBatchDeleteComplete(nextPending)) {
          setAuthoringState({ ...current, pendingSubmit: nextPending });
          return;
        }

        clearPendingTimer();
        setAuthoringState(CLOSED_STATE);
        const feedback = buildBatchDeleteFeedback(nextPending);
        notifyUser(feedback.message, feedback.type);
        return;
      }

      // Single-command create/edit/delete path.
      const transition = resolvePendingAuthoringEvent(current, event);
      if (!transition) return;

      clearPendingTimer();
      if (transition.acceptedResult) setValidationErrors({});
      setAuthoringState(transition.state);
      if (transition.acceptedResult) {
        onAcceptedRef.current?.(transition.acceptedResult);
      }
      if (transition.notification) {
        notifyUser(transition.notification.message, transition.notification.type);
      }
    });
  }, [bus, clearPendingTimer, notifyUser, setAuthoringState]);

  // A disconnect changes the backend WebSocket session. Open modal values are
  // intentionally memory-only; if the connection is lost, fields are locked and
  // the user must refresh before trying the action again.
  useEffect(() => {
    if (wasConnectedRef.current && !connected) {
      const transition = resolveAuthoringDisconnect(stateRef.current);
      if (transition) {
        clearPendingTimer();
        setAuthoringState(transition.state);
        if (transition.notification) {
          notifyUser(transition.notification.message, transition.notification.type);
        }
      }
    }
    wasConnectedRef.current = connected;
  }, [clearPendingTimer, connected, notifyUser, setAuthoringState]);

  useEffect(() => {
    return () => {
      clearPendingTimer();
    };
  }, [clearPendingTimer]);

  return {
    state,
    validationErrors,
    openCreateNode,
    openCreateConnection,
    openEditNode,
    deleteNode,
    deleteNodes,
    updateFormState,
    submit,
    close,
  };
}
