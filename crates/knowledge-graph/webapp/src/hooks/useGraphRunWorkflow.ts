import { useCallback, useEffect, useMemo, useRef, useState } from 'react';
import type { MinigraphGraphData } from '../utils/graphTypes';
import type { ProtocolBus } from '../protocol/bus';
import type { ToastType } from './useToast';
import { collectGraphInputBodyPaths } from '../graphRun/graphInputPaths';
import {
  GRAPH_RUN_COMMANDS,
  isInstantiateCommandText,
  isRunCommandText,
} from '../graphRun/graphRunProtocol';

export const GRAPH_RUN_SETUP_TIMEOUT_MS = 10_000;

export type GraphRunPhase =
  | 'idle'
  | 'instantiating'
  | 'requesting-input'
  | 'awaiting-input'
  | 'ready'
  | 'running'
  | 'outcome-uncertain';

type GraphRunIntent = 'instantiate-only' | null;
type PendingSignal = 'instance-created' | 'upload-invitation' | 'run-terminal' | null;

interface WorkflowState {
  phase: GraphRunPhase;
  intent: GraphRunIntent;
  pendingSignal: PendingSignal;
  /** The graph changed while an earlier backend command was still outstanding. */
  invalidated: boolean;
}

export interface UseGraphRunWorkflowOptions {
  enabled: boolean;
  bus: ProtocolBus;
  connected: boolean;
  connectionEpoch: number | null;
  graphData: MinigraphGraphData | null;
  graphIdentity: string | null;
  isPrimary: boolean;
  sendRawText: (text: string) => boolean;
  addToast: (message: string, type?: ToastType) => void;
  /** Close the upload panel only when it still belongs to this workflow path. */
  onWorkflowInputInvalidated?: (uploadPath: string) => void;
}

export interface UseGraphRunWorkflowReturn {
  phase: GraphRunPhase;
  ready: boolean;
  busy: boolean;
  canInteract: boolean;
  canInstantiate: boolean;
  canRun: boolean;
  disabledReason: string;
  inputBodyPaths: string[];
  inputIntent: GraphRunIntent;
  isWorkflowInputPanel: boolean;
  runGraph: () => boolean;
  instantiateGraph: () => boolean;
  /** Complete the workflow only when the submitted panel owns this exact path. */
  handleInputUploadSuccess: (uploadPath: string) => boolean;
  /** Cancel the workflow only when the dismissed panel owns this exact path. */
  handleInputCancelled: (uploadPath: string) => boolean;
  /** The invitation path claimed by this workflow, or null before/after input. */
  workflowUploadPath: string | null;
}

const IDLE_STATE: WorkflowState = {
  phase: 'idle',
  intent: null,
  pendingSignal: null,
  invalidated: false,
};

export function useGraphRunWorkflow({
  enabled,
  bus,
  connected,
  connectionEpoch,
  graphData,
  graphIdentity,
  isPrimary,
  sendRawText,
  addToast,
  onWorkflowInputInvalidated,
}: UseGraphRunWorkflowOptions): UseGraphRunWorkflowReturn {
  const [state, setState] = useState<WorkflowState>(IDLE_STATE);
  const stateRef = useRef<WorkflowState>(IDLE_STATE);
  const workflowUploadPathRef = useRef<string | null>(null);
  const sendRawTextRef = useRef(sendRawText);
  const addToastRef = useRef(addToast);
  const onWorkflowInputInvalidatedRef = useRef(onWorkflowInputInvalidated);
  const inputBodyPaths = useMemo(() => collectGraphInputBodyPaths(graphData), [graphData]);
  const inputBodyPathsRef = useRef(inputBodyPaths);

  useEffect(() => { sendRawTextRef.current = sendRawText; }, [sendRawText]);
  useEffect(() => { addToastRef.current = addToast; }, [addToast]);
  useEffect(() => {
    onWorkflowInputInvalidatedRef.current = onWorkflowInputInvalidated;
  }, [onWorkflowInputInvalidated]);
  useEffect(() => { inputBodyPathsRef.current = inputBodyPaths; }, [inputBodyPaths]);

  const transition = useCallback((next: WorkflowState) => {
    stateRef.current = next;
    setState(next);
  }, []);

  const reset = useCallback(() => {
    workflowUploadPathRef.current = null;
    transition({ ...IDLE_STATE });
  }, [transition]);

  const closeWorkflowInput = useCallback(() => {
    const uploadPath = workflowUploadPathRef.current;
    workflowUploadPathRef.current = null;
    if (uploadPath) onWorkflowInputInvalidatedRef.current?.(uploadPath);
  }, []);

  const hardReset = useCallback(() => {
    closeWorkflowInput();
    reset();
  }, [closeWorkflowInput, reset]);

  /**
   * Keep an outstanding backend response quarantined after a graph mutation.
   * The text protocol has no correlation IDs, so unlocking early would let the
   * old acknowledgement complete a later button click.
   */
  const invalidate = useCallback(() => {
    const current = stateRef.current;
    closeWorkflowInput();
    if (current.pendingSignal !== null) {
      transition({
        ...current,
        phase: 'outcome-uncertain',
        invalidated: true,
      });
      return;
    }
    reset();
  }, [closeWorkflowInput, reset, transition]);

  const sendWithState = useCallback((
    command: string,
    next: WorkflowState,
    failureMessage: string,
  ): boolean => {
    transition(next);
    if (sendRawTextRef.current(command)) return true;
    reset();
    addToastRef.current(failureMessage, 'error');
    return false;
  }, [reset, transition]);

  const canInteract = enabled && connected && graphData !== null && isPrimary;
  const canInstantiate = canInteract && (
    state.phase === 'idle' ||
    (state.phase === 'ready' && inputBodyPaths.length > 0)
  );
  const canRun = canInteract && state.phase === 'ready';
  const disabledReason = !enabled
    ? 'Graph run controls are unavailable'
    : !graphData
      ? 'Load a graph first'
      : !connected
        ? 'Connect first to run the graph'
        : !isPrimary
          ? 'Run the graph from the host session'
          : '';

  const runGraph = useCallback((): boolean => {
    if (!canRun) return false;
    const current = stateRef.current;
    if (current.phase !== 'ready') return false;
    return sendWithState(
      GRAPH_RUN_COMMANDS.run,
      { phase: 'running', intent: null, pendingSignal: 'run-terminal', invalidated: false },
      'Could not run graph because the WebSocket is not open.',
    );
  }, [canRun, sendWithState]);

  const instantiateGraph = useCallback((): boolean => {
    if (!canInstantiate) return false;
    const current = stateRef.current;
    if (current.phase !== 'idle' && current.phase !== 'ready') return false;
    return sendWithState(
      GRAPH_RUN_COMMANDS.instantiate,
      {
        phase: 'instantiating',
        intent: 'instantiate-only',
        pendingSignal: 'instance-created',
        invalidated: false,
      },
      'Could not instantiate graph because the WebSocket is not open.',
    );
  }, [canInstantiate, sendWithState]);

  const handleInputUploadSuccess = useCallback((uploadPath: string): boolean => {
    const current = stateRef.current;
    if (
      (current.phase !== 'awaiting-input' && current.phase !== 'requesting-input') ||
      current.intent === null ||
      workflowUploadPathRef.current !== uploadPath
    ) {
      return false;
    }
    workflowUploadPathRef.current = null;
    transition({ phase: 'ready', intent: null, pendingSignal: null, invalidated: false });
    addToastRef.current('Graph instantiated and ready to run.', 'success');
    return true;
  }, [transition]);

  const handleInputCancelled = useCallback((uploadPath: string): boolean => {
    const current = stateRef.current;
    if (
      (current.phase !== 'awaiting-input' && current.phase !== 'requesting-input') ||
      current.intent === null ||
      workflowUploadPathRef.current !== uploadPath
    ) {
      return false;
    }
    reset();
    addToastRef.current('Graph instantiation cancelled.', 'info');
    return true;
  }, [reset]);

  useEffect(() => {
    const offCreated = bus.on('graph.instance.created', () => {
      const current = stateRef.current;
      if (current.pendingSignal !== 'instance-created') return;
      if (current.invalidated) {
        reset();
        return;
      }

      // Manual console commands have no button intent to continue. Mirror the
      // acknowledged instance as Ready without inventing an upload/run action.
      if (current.intent === null) {
        transition({ phase: 'ready', intent: null, pendingSignal: null, invalidated: false });
        return;
      }

      if (inputBodyPathsRef.current.length > 0) {
        sendWithState(
          GRAPH_RUN_COMMANDS.requestInputUpload,
          {
            phase: 'requesting-input',
            intent: current.intent,
            pendingSignal: 'upload-invitation',
            invalidated: false,
          },
          'Graph was instantiated, but the input upload could not be requested.',
        );
      } else {
        transition({ phase: 'ready', intent: null, pendingSignal: null, invalidated: false });
        addToastRef.current('Graph instantiated and ready to run.', 'success');
      }
    });

    const offCleared = bus.on('graph.instance.cleared', hardReset);
    const offMutation = bus.on('graph.mutation', invalidate);
    // A confirmed session restart is authoritative: the old session-bound
    // instance and its acknowledgements can no longer be actionable.
    const offReset = bus.on('session.reset', hardReset);
    const offExported = bus.on('graph.exported', invalidate);

    const offInvitation = bus.on('upload.invitation', (event) => {
      const current = stateRef.current;
      if (current.pendingSignal !== 'upload-invitation') return;
      if (current.invalidated) {
        // useAutoMockUpload opens or queues synchronously earlier in
        // Playground's hook order; remove only this exact workflow invitation.
        onWorkflowInputInvalidatedRef.current?.(event.uploadPath);
        reset();
        return;
      }
      if (current.intent !== null) {
        workflowUploadPathRef.current = event.uploadPath;
        transition({
          phase: 'awaiting-input',
          intent: current.intent,
          pendingSignal: null,
          invalidated: false,
        });
      }
    });

    const offTerminal = bus.on('graph.run.terminal', (event) => {
      const current = stateRef.current;
      if (current.pendingSignal !== 'run-terminal') return;
      const shouldReportAbort = !current.invalidated && event.status === 'aborted';
      reset();
      if (shouldReportAbort) {
        addToastRef.current('Graph run aborted. See the console for details.', 'error');
      }
    });

    const offError = bus.on('command.error', (event) => {
      const current = stateRef.current;
      if (current.pendingSignal === 'instance-created') {
        const shouldReport = !current.invalidated;
        reset();
        if (shouldReport) {
          addToastRef.current(`Could not instantiate graph: ${event.message}`, 'error');
        }
      } else if (current.pendingSignal === 'upload-invitation') {
        const shouldReport = !current.invalidated;
        reset();
        if (shouldReport) {
          addToastRef.current(`Could not request graph input: ${event.message}`, 'error');
        }
      }
    });

    const offEcho = bus.on('command.echo', (event) => {
      const current = stateRef.current;
      if (isInstantiateCommandText(event.commandText)) {
        if (current.phase === 'idle' || current.phase === 'ready') {
          transition({
            phase: 'instantiating',
            intent: null,
            pendingSignal: 'instance-created',
            invalidated: false,
          });
        }
      } else if (
        isRunCommandText(event.commandText) &&
        (current.phase === 'idle' || current.phase === 'ready')
      ) {
        transition({
          phase: 'running',
          intent: null,
          pendingSignal: 'run-terminal',
          invalidated: false,
        });
      }
    });

    return () => {
      offCreated();
      offCleared();
      offMutation();
      offReset();
      offExported();
      offInvitation();
      offTerminal();
      offError();
      offEcho();
    };
  }, [bus, hardReset, invalidate, reset, sendWithState, transition]);

  useEffect(() => {
    if (state.phase !== 'instantiating' && state.phase !== 'requesting-input') return;
    const timer = setTimeout(() => {
      const current = stateRef.current;
      if (current.phase === 'instantiating' || current.phase === 'requesting-input') {
        transition({ ...current, phase: 'outcome-uncertain' });
        addToastRef.current(
          'Graph setup is taking longer than expected. Waiting for the backend outcome…',
          'info',
        );
      }
    }, GRAPH_RUN_SETUP_TIMEOUT_MS);
    return () => clearTimeout(timer);
  }, [state.phase, transition]);

  const previousIdentityRef = useRef(graphIdentity);
  useEffect(() => {
    if (previousIdentityRef.current !== graphIdentity) invalidate();
    previousIdentityRef.current = graphIdentity;
  }, [graphIdentity, invalidate]);

  const previousEpochRef = useRef(connectionEpoch);
  useEffect(() => {
    if (previousEpochRef.current !== connectionEpoch) hardReset();
    previousEpochRef.current = connectionEpoch;
  }, [connectionEpoch, hardReset]);

  useEffect(() => {
    if (!enabled || !connected || !isPrimary) {
      hardReset();
    } else if (!graphData) {
      invalidate();
    }
  }, [enabled, connected, graphData, hardReset, invalidate, isPrimary]);

  const busy = state.phase !== 'idle' && state.phase !== 'ready';
  return {
    phase: state.phase,
    ready: state.phase === 'ready',
    busy,
    canInteract,
    canInstantiate,
    canRun,
    disabledReason,
    inputBodyPaths,
    inputIntent: state.intent,
    workflowUploadPath: workflowUploadPathRef.current,
    isWorkflowInputPanel:
      state.intent !== null &&
      (state.phase === 'requesting-input' || state.phase === 'awaiting-input'),
    runGraph,
    instantiateGraph,
    handleInputUploadSuccess,
    handleInputCancelled,
  };
}
