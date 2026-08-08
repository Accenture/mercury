import { useCallback, useEffect, useMemo, useRef, useState } from 'react';
import type {
  ProtocolEvent,
  SessionCommandResultEvent,
  SessionNotificationEvent,
  SessionResetEvent,
  SessionStartedEvent,
  SessionStatusEvent,
} from '../protocol/events';
import {
  type SessionCollaborationController,
  type SessionCollaborationOptions,
  type SessionUiState,
  isValidSessionId,
} from './sessionTypes';

// Local view-model for the Session dropdown. The backend remains the source of
// truth; this state only mirrors the latest parsed session events and tracks
// short-lived UI concerns such as pending commands and inline errors.
const EMPTY_STATE: SessionUiState = {
  sessionId: null,
  startedSince: null,
  subscribedTo: null,
  subscribers: [],
  loading: false,
  pendingCommand: null,
  error: null,
  lastInfo: null,
};

// Keep subscriber rows stable and deterministic even if status responses or
// notifications arrive out of order or contain duplicate ids.
function uniqueSortedSessionIds(values: string[]): string[] {
  return Array.from(new Set(values)).sort();
}

// Bridges backend session text events into React state and exposes small
// command helpers for SessionMenu. UI actions still go through the same raw
// WebSocket commands that a user could type in the console.
export function useSessionCollaboration({
  enabled,
  connected,
  bus,
  classificationMap,
  sendRawText,
  addToast,
}: SessionCollaborationOptions): SessionCollaborationController {
  const [state, setState] = useState<SessionUiState>(EMPTY_STATE);
  const processedEventKeysRef = useRef(new Set<string>());
  const backlogWatermarkRef = useRef(0);
  const sendRawTextRef = useRef(sendRawText);
  const addToastRef = useRef(addToast);

  useEffect(() => { sendRawTextRef.current = sendRawText; }, [sendRawText]);
  useEffect(() => { addToastRef.current = addToast; }, [addToast]);

  const refreshSession = useCallback((): boolean => {
    if (!enabled || !connected) return false;

    setState(prev => ({
      ...prev,
      loading: true,
      pendingCommand: 'refresh',
      error: null,
      lastInfo: null,
    }));
    const sent = sendRawTextRef.current('session');
    if (!sent) {
      const message = 'Could not load session details because the WebSocket is not open.';
      setState(prev => ({ ...prev, loading: false, pendingCommand: null, error: message }));
      addToastRef.current(message, 'error');
    }
    return sent;
  }, [connected, enabled]);

  // The same WebSocket message can be observed through the live ProtocolBus and
  // the classified console backlog. Deduping by kind+msgId prevents duplicate
  // subscriber rows and repeated error updates.
  const processEvent = useCallback((event: ProtocolEvent): void => {
    const eventKey = `${event.kind}:${event.msgId}`;
    if (processedEventKeysRef.current.has(eventKey)) return;
    processedEventKeysRef.current.add(eventKey);

    if (event.kind === 'minigraph.session.started') {
      // A new backend WebSocket session invalidates every relationship from
      // the previous session, even if old console messages are still visible.
      setState({
        ...EMPTY_STATE,
        sessionId: event.sessionId,
      });
      return;
    }

    if (event.kind === 'minigraph.session.status') {
      // A status response is authoritative. It can be produced by a manual
      // "session" command, so always reconcile all relation fields from it.
      setState(prev => ({
        ...prev,
        sessionId: event.sessionId,
        startedSince: event.startedSince,
        subscribedTo: event.subscribedTo,
        subscribers: uniqueSortedSessionIds(event.subscribers),
        loading: false,
        pendingCommand: null,
        error: null,
        lastInfo: null,
      }));
      return;
    }

    if (event.kind === 'minigraph.session.commandResult') {
      if (event.status === 'accepted') {
        // Accepted command responses update the relation immediately. A later
        // status response can still correct the state if the backend disagrees.
        setState(prev => ({
          ...prev,
          subscribedTo: event.command === 'subscribe'
            ? event.sessionId
            : event.command === 'unsubscribe'
              ? null
              : prev.subscribedTo,
          pendingCommand: null,
          error: null,
          lastInfo: null,
        }));
        return;
      }

      setState(prev => ({ ...prev, pendingCommand: null, error: event.message, lastInfo: null }));
      return;
    }

    if (event.kind === 'minigraph.session.notification') {
      // Notifications are peer-side deltas. They let the dropdown react without
      // requiring the user to manually run "session" after another browser joins
      // or leaves.
      if (event.type === 'host-closed') {
        setState(prev => ({
          ...prev,
          subscribedTo: prev.subscribedTo === event.sessionId ? null : prev.subscribedTo,
          subscribers: prev.subscribers.filter(id => id !== event.sessionId),
          error: null,
          lastInfo: null,
        }));
      } else if (event.type === 'subscriber-joined') {
        setState(prev => ({
          ...prev,
          subscribers: uniqueSortedSessionIds([...prev.subscribers, event.sessionId]),
          error: null,
          lastInfo: null,
        }));
      } else {
        setState(prev => ({
          ...prev,
          subscribers: prev.subscribers.filter(id => id !== event.sessionId),
          error: null,
          lastInfo: null,
        }));
      }
      return;
    }

    if (event.kind === 'session.reset') {
      // Reset creates a new backend GraphSession with a new start time. Clear
      // the old snapshot immediately, then load the authoritative replacement.
      setState(prev => ({
        ...prev,
        startedSince: null,
        subscribedTo: null,
        subscribers: [],
        loading: false,
        pendingCommand: null,
        error: null,
        lastInfo: null,
      }));
      refreshSession();
    }
  }, [refreshSession]);

  // Called when the user edits the subscribe form or toggles the dropdown
  // affordance. It intentionally does not clear pendingCommand.
  const clearMessage = useCallback(() => {
    setState(prev => ({ ...prev, error: null, lastInfo: null }));
  }, []);

  const subscribeToSession = useCallback((rawSessionId: string): boolean => {
    const sessionId = rawSessionId.trim();
    // A session can subscribe to only one primary at a time. Hide the UI entry
    // point and guard here so other callers cannot send an invalid command.
    if (!enabled || !connected || state.pendingCommand !== null || state.subscribedTo !== null) {
      return false;
    }

    if (!isValidSessionId(sessionId)) {
      setState(prev => ({
        ...prev,
        error: 'Enter a valid session ID like ws-123456-1.',
        lastInfo: null,
      }));
      return false;
    }

    setState(prev => ({ ...prev, pendingCommand: 'subscribe', error: null, lastInfo: null }));
    // Send silently; command echo/history are for user-typed console commands,
    // not for button-driven UI actions.
    const sent = sendRawText(`session subscribe ${sessionId}`);
    if (!sent) {
      const message = 'Could not subscribe because the WebSocket is not open.';
      setState(prev => ({ ...prev, pendingCommand: null, error: message }));
      addToast(message, 'error');
    }
    return sent;
  }, [addToast, connected, enabled, sendRawText, state.pendingCommand, state.subscribedTo]);

  const unsubscribe = useCallback((): boolean => {
    // Only a subscriber can unsubscribe. A primary/host with subscribers must
    // use reset if it wants to detach everyone.
    if (!enabled || !connected || state.pendingCommand !== null || state.subscribedTo === null) {
      return false;
    }

    setState(prev => ({ ...prev, pendingCommand: 'unsubscribe', error: null, lastInfo: null }));
    const sent = sendRawText('session unsubscribe');
    if (!sent) {
      const message = 'Could not unsubscribe because the WebSocket is not open.';
      setState(prev => ({ ...prev, pendingCommand: null, error: message }));
      addToast(message, 'error');
    }
    return sent;
  }, [addToast, connected, enabled, sendRawText, state.pendingCommand, state.subscribedTo]);

  const resetSession = useCallback((): boolean => {
    // Product rule: show reset only for a host that currently has subscribers.
    // Subscribers use "unsubscribe" instead; reset is too broad for that role.
    if (
      !enabled ||
      !connected ||
      state.pendingCommand !== null ||
      state.subscribedTo !== null ||
      state.subscribers.length === 0
    ) {
      return false;
    }

    setState(prev => ({ ...prev, pendingCommand: 'reset', error: null, lastInfo: null }));
    const sent = sendRawText('session reset');
    if (!sent) {
      const message = 'Could not reset because the WebSocket is not open.';
      setState(prev => ({ ...prev, pendingCommand: null, error: message }));
      addToast(message, 'error');
    }
    return sent;
  }, [
    addToast,
    connected,
    enabled,
    sendRawText,
    state.pendingCommand,
    state.subscribedTo,
    state.subscribers.length,
  ]);

  useEffect(() => {
    // Disconnects create a new backend session on reconnect, so cached relation
    // state and dedupe keys are no longer valid.
    if (enabled && connected) return;
    processedEventKeysRef.current.clear();
    backlogWatermarkRef.current = 0;
    setState(EMPTY_STATE);
  }, [connected, enabled]);

  useEffect(() => {
    if (!enabled) return;

    const offStarted = bus.on('minigraph.session.started', (event: SessionStartedEvent) => {
      processEvent(event);
    });

    const offStatus = bus.on('minigraph.session.status', (event: SessionStatusEvent) => {
      processEvent(event);
    });

    const offCommandResult = bus.on('minigraph.session.commandResult', (event: SessionCommandResultEvent) => {
      processEvent(event);
    });

    const offNotification = bus.on('minigraph.session.notification', (event: SessionNotificationEvent) => {
      processEvent(event);
    });

    const offReset = bus.on('session.reset', (event: SessionResetEvent) => {
      processEvent(event);
    });

    return () => {
      offStarted();
      offStatus();
      offCommandResult();
      offNotification();
      offReset();
    };
  }, [bus, enabled, processEvent]);

  useEffect(() => {
    if (!enabled || !connected) return;
    refreshSession();
  }, [connected, enabled, refreshSession]);

  // ProtocolBus only emits new events. This catch-up pass consumes the
  // already-classified console backlog so manual `session` commands update the
  // dropdown even when they were sent before the menu was opened.
  useEffect(() => {
    if (!enabled || !classificationMap) return;
    let nextWatermark = backlogWatermarkRef.current;
    for (const [msgId, events] of classificationMap) {
      if (msgId <= backlogWatermarkRef.current) continue;
      for (const event of events) {
        if (
          event.kind === 'minigraph.session.started' ||
          event.kind === 'minigraph.session.status' ||
          event.kind === 'minigraph.session.commandResult' ||
          event.kind === 'minigraph.session.notification' ||
          event.kind === 'session.reset'
        ) {
          processEvent(event);
        }
      }
      nextWatermark = Math.max(nextWatermark, msgId);
    }
    backlogWatermarkRef.current = nextWatermark;
    processedEventKeysRef.current.clear();
  }, [classificationMap, enabled, processEvent]);

  const isPrimary = state.subscribedTo === null;
  const hasSubscribers = state.subscribers.length > 0;

  // Derived permissions are consumed directly by SessionMenu so rendering rules
  // stay centralized with the command guards above.
  return useMemo(() => ({
    state,
    connected,
    isPrimary,
    hasSubscribers,
    canSubscribe: enabled && connected && state.pendingCommand === null && state.subscribedTo === null,
    canUnsubscribe: enabled && connected && state.subscribedTo !== null && state.pendingCommand === null,
    canReset: enabled && connected && state.pendingCommand === null && state.subscribedTo === null && state.subscribers.length > 0,
    subscribeToSession,
    unsubscribe,
    resetSession,
    clearMessage,
  }), [
    clearMessage,
    connected,
    enabled,
    hasSubscribers,
    isPrimary,
    resetSession,
    state,
    subscribeToSession,
    unsubscribe,
  ]);
}
