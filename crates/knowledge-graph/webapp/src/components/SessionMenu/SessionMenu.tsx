import { useEffect, useRef, useState, type SubmitEvent } from 'react';
import NavMenu, { type DotStatus } from '../NavMenu/NavMenu';
import type { SessionCollaborationController } from '../../session/sessionTypes';
import styles from './SessionMenu.module.css';

interface SessionMenuProps {
  controller: SessionCollaborationController;
}

function CopyIcon({ copied }: { copied: boolean }) {
  if (copied) {
    return (
      <svg viewBox="0 0 16 16" aria-hidden="true" focusable="false">
        <path d="M6.2 11.4 2.9 8.1l1.1-1.1 2.2 2.2 5.8-5.8 1.1 1.1-6.9 6.9Z" />
      </svg>
    );
  }

  return (
    <svg viewBox="0 0 16 16" aria-hidden="true" focusable="false">
      <path d="M5 2.5A1.5 1.5 0 0 1 6.5 1h6A1.5 1.5 0 0 1 14 2.5v6A1.5 1.5 0 0 1 12.5 10H11V8.7h1.5a.2.2 0 0 0 .2-.2v-6a.2.2 0 0 0-.2-.2h-6a.2.2 0 0 0-.2.2V4H5V2.5Z" />
      <path d="M2 6.5A1.5 1.5 0 0 1 3.5 5h6A1.5 1.5 0 0 1 11 6.5v7A1.5 1.5 0 0 1 9.5 15h-6A1.5 1.5 0 0 1 2 13.5v-7Zm1.5-.2a.2.2 0 0 0-.2.2v7a.2.2 0 0 0 .2.2h6a.2.2 0 0 0 .2-.2v-7a.2.2 0 0 0-.2-.2h-6Z" />
    </svg>
  );
}

function getDotStatus(controller: SessionCollaborationController): DotStatus {
  if (!controller.connected) return 'idle';
  if (controller.state.pendingCommand === 'refresh' || controller.state.loading) return 'connecting';
  if (controller.state.sessionId) return 'connected';
  return 'partial';
}

function SessionMenuContent({ controller }: SessionMenuProps) {
  const { state } = controller;
  const [subscribeOpen, setSubscribeOpen] = useState(false);
  const [targetSessionId, setTargetSessionId] = useState('');
  const [copyError, setCopyError] = useState<string | null>(null);
  const [copiedSessionId, setCopiedSessionId] = useState<string | null>(null);
  const copyResetTimerRef = useRef<number | null>(null);

  const showReset = controller.canReset;

  useEffect(() => () => {
    if (copyResetTimerRef.current !== null) {
      window.clearTimeout(copyResetTimerRef.current);
    }
  }, []);

  useEffect(() => {
    if (state.subscribedTo && state.pendingCommand !== 'subscribe') {
      setSubscribeOpen(false);
      setTargetSessionId('');
    }
  }, [state.pendingCommand, state.subscribedTo]);

  const handleSubscribeSubmit = (event: SubmitEvent<HTMLFormElement>) => {
    event.preventDefault();
    controller.subscribeToSession(targetSessionId);
  };

  const handleCopySessionId = async (sessionId: string) => {
    try {
      await navigator.clipboard.writeText(sessionId);
      setCopyError(null);
      setCopiedSessionId(sessionId);
      if (copyResetTimerRef.current !== null) {
        window.clearTimeout(copyResetTimerRef.current);
      }
      copyResetTimerRef.current = window.setTimeout(() => {
        setCopiedSessionId(null);
        copyResetTimerRef.current = null;
      }, 1400);
    } catch {
      setCopiedSessionId(null);
      setCopyError('Could not copy the session ID. Please copy it manually.');
    }
  };

  if (!controller.connected) {
    return (
      <div className={styles.panel}>
        <p className={styles.emptyMessage}>Connect Minigraph to view session details.</p>
      </div>
    );
  }

  return (
    <div className={styles.panel}>
      <section className={styles.section}>
        <div className={styles.sectionHeader}>This session</div>
        <div className={styles.sessionHeaderRow}>
          {state.sessionId ? (
            <code className={styles.sessionId} title={state.sessionId}>{state.sessionId}</code>
          ) : (
            <p className={styles.emptyMessage}>Session details are not loaded yet.</p>
          )}
          {state.sessionId && (
            <button
              type="button"
              className={`${styles.copyButton} ${copiedSessionId === state.sessionId ? styles.copyButtonCopied : ''}`}
              onClick={() => void handleCopySessionId(state.sessionId!)}
              aria-label="Copy session ID"
              title={copiedSessionId === state.sessionId ? 'Copied' : 'Copy session ID'}
            >
              <CopyIcon copied={copiedSessionId === state.sessionId} />
            </button>
          )}
          {controller.canSubscribe && (
            <button
              type="button"
              className={styles.iconButton}
              onClick={() => {
                controller.clearMessage();
                setSubscribeOpen(prev => !prev);
              }}
              aria-expanded={subscribeOpen}
              aria-label={subscribeOpen ? 'Close subscribe form' : 'Subscribe to another session'}
              title={subscribeOpen ? 'Close subscribe form' : 'Subscribe to another session'}
            >
              {subscribeOpen ? '×' : '+'}
            </button>
          )}
        </div>
        {state.startedSince && (
          <div className={styles.metaText}>Started since {state.startedSince}</div>
        )}
        {subscribeOpen && (
          <form className={styles.subscribeForm} onSubmit={handleSubscribeSubmit}>
            <input
              className={styles.subscribeInput}
              value={targetSessionId}
              onChange={event => {
                setTargetSessionId(event.target.value);
                controller.clearMessage();
              }}
              placeholder="ws-123456-1"
              autoComplete="off"
              disabled={!controller.canSubscribe}
            />
            <button
              type="submit"
              className={styles.subscribeButton}
              disabled={!controller.canSubscribe || targetSessionId.trim().length === 0}
            >
              {state.pendingCommand === 'subscribe' ? '...' : 'Subscribe'}
            </button>
          </form>
        )}
      </section>

      {state.subscribedTo && (
        <section className={styles.section}>
          <div className={styles.sectionHeader}>Subscribed to</div>
          <div className={styles.relationshipRow}>
            <span className={styles.statusDot} aria-hidden="true" />
            <code className={styles.sessionId} title={state.subscribedTo}>{state.subscribedTo}</code>
            <button
              type="button"
              className={styles.unsubscribeButton}
              onClick={controller.unsubscribe}
              disabled={!controller.canUnsubscribe}
            >
              {state.pendingCommand === 'unsubscribe' ? '...' : 'Unsubscribe'}
            </button>
          </div>
        </section>
      )}

      {state.subscribers.length > 0 && (
        <section className={styles.section}>
          <div className={styles.sectionHeader}>Subscribers</div>
          <ul className={styles.subscriberList}>
            {state.subscribers.map(sessionId => (
              <li key={sessionId} className={styles.subscriberRow}>
                <span className={styles.statusDot} aria-hidden="true" />
                <code className={styles.sessionId} title={sessionId}>{sessionId}</code>
              </li>
            ))}
          </ul>
        </section>
      )}

      {showReset && (
        <div className={styles.actionsRow}>
          <button
            type="button"
            className={styles.resetButton}
            onClick={controller.resetSession}
            disabled={!controller.canReset}
          >
            {state.pendingCommand === 'reset' ? 'Resetting...' : 'Reset Session'}
          </button>
        </div>
      )}

      {state.error && (
        <div className={styles.errorMessage} role="alert">
          {state.error}
        </div>
      )}

      {copyError && (
        <div className={styles.errorMessage} role="alert">
          {copyError}
        </div>
      )}
    </div>
  );
}

export default function SessionMenu({ controller }: SessionMenuProps) {
  return (
    <NavMenu label="Session" dotStatus={getDotStatus(controller)}>
      <SessionMenuContent controller={controller} />
    </NavMenu>
  );
}
