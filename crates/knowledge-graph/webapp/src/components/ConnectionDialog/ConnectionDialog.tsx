import { useCallback, useEffect, useRef } from 'react';
import type { ConnectionFormState } from '../../graphActions/connectionAuthoringTypes';
import CloseIcon from '../../icons/CloseIcon.svg?react';
import styles from './ConnectionDialog.module.css';

interface ConnectionDialogProps {
  open: boolean;
  formState: ConnectionFormState;
  phase: 'editing' | 'sending';
  lockReason: null | 'sending' | 'disconnected';
  serverMessage: string | null;
  validationErrors: Record<string, string>;
  onFormStateChange: (formState: ConnectionFormState) => void;
  onSubmit: () => void;
  onClose: () => void;
}

export default function ConnectionDialog({
  open,
  formState,
  phase,
  lockReason,
  serverMessage,
  validationErrors,
  onFormStateChange,
  onSubmit,
  onClose,
}: ConnectionDialogProps) {
  const relationRef = useRef<HTMLInputElement>(null);
  const sending = phase === 'sending';
  const disconnected = lockReason === 'disconnected';
  const controlsDisabled = sending || disconnected;
  const disconnectedMessage = 'Connection disconnected. Refresh the page and create the connection again after the app reconnects.';

  useEffect(() => {
    if (!open) return;
    relationRef.current?.focus();
    const handleKeyDown = (event: KeyboardEvent) => {
      if (event.key !== 'Escape') return;
      event.preventDefault();
      if (!sending) onClose();
    };
    document.addEventListener('keydown', handleKeyDown);
    return () => {
      document.removeEventListener('keydown', handleKeyDown);
    };
  }, [onClose, open, sending]);

  const handleOverlayPointerDown = useCallback((event: React.PointerEvent<HTMLDivElement>) => {
    event.preventDefault();
    event.stopPropagation();
  }, []);

  const handleOverlayClick = useCallback((event: React.MouseEvent<HTMLDivElement>) => {
    event.preventDefault();
    event.stopPropagation();
    if (!sending) onClose();
  }, [onClose, sending]);

  const stopPanelPointer = useCallback((event: React.PointerEvent<HTMLDivElement>) => {
    event.stopPropagation();
  }, []);

  const handleFormSubmit = useCallback((event: React.SubmitEvent<HTMLFormElement>) => {
    event.preventDefault();
    if (controlsDisabled) return;
    onSubmit();
  }, [controlsDisabled, onSubmit]);

  const updateRelation = useCallback((relation: string) => {
    onFormStateChange({ ...formState, relation });
  }, [formState, onFormStateChange]);

  if (!open) return null;

  return (
    <div
      className={styles.overlay}
      onPointerDown={handleOverlayPointerDown}
      onClick={handleOverlayClick}
    >
      <div
        className={styles.panel}
        role="dialog"
        aria-modal="true"
        aria-labelledby="connection-dialog-title"
        onPointerDown={stopPanelPointer}
        onClick={(event) => event.stopPropagation()}
      >
        <header className={styles.header}>
          <div>
            <h2 id="connection-dialog-title" className={styles.title}>Create Connection</h2>
          </div>
          <button
            type="button"
            className={styles.iconButton}
            aria-label="Close create connection dialog"
            onClick={onClose}
            disabled={sending}
          >
            <CloseIcon className={styles.buttonIcon} aria-hidden="true" focusable="false" />
          </button>
        </header>

        <form className={styles.form} onSubmit={handleFormSubmit}>
          <div className={styles.body}>
            {serverMessage && !disconnected && (
              <div className={styles.message} role="status">
                {serverMessage}
              </div>
            )}
            {validationErrors.command && (
              <div className={styles.errorMessage} role="alert">
                {validationErrors.command}
              </div>
            )}
            {disconnected && (
              <div className={styles.warningMessage} role="status">
                {serverMessage ?? disconnectedMessage}
              </div>
            )}

            <label className={styles.field}>
              <span className={styles.label}>Source</span>
              <input
                className={styles.input}
                value={formState.sourceAlias}
                readOnly
                disabled={controlsDisabled}
                aria-invalid={Boolean(validationErrors.sourceAlias)}
                aria-describedby={validationErrors.sourceAlias ? 'connection-source-error' : undefined}
              />
              {validationErrors.sourceAlias && (
                <span id="connection-source-error" className={styles.errorText}>{validationErrors.sourceAlias}</span>
              )}
            </label>

            <label className={styles.field}>
              <span className={styles.label}>Target</span>
              <input
                className={styles.input}
                value={formState.targetAlias}
                readOnly
                disabled={controlsDisabled}
                aria-invalid={Boolean(validationErrors.targetAlias)}
                aria-describedby={validationErrors.targetAlias ? 'connection-target-error' : undefined}
              />
              {validationErrors.targetAlias && (
                <span id="connection-target-error" className={styles.errorText}>{validationErrors.targetAlias}</span>
              )}
            </label>

            <label className={styles.field}>
              <span className={styles.label}>Relation</span>
              <input
                ref={relationRef}
                className={styles.input}
                type="text"
                value={formState.relation}
                placeholder="e.g. fetch"
                disabled={controlsDisabled}
                autoComplete="off"
                autoCorrect="off"
                spellCheck={false}
                aria-invalid={Boolean(validationErrors.relation)}
                aria-describedby={validationErrors.relation ? 'connection-relation-error' : undefined}
                onChange={(event) => updateRelation(event.target.value)}
              />
              {validationErrors.relation && (
                <span id="connection-relation-error" className={styles.errorText}>{validationErrors.relation}</span>
              )}
            </label>
          </div>

          <footer className={styles.footer}>
            <button
              type="button"
              className={styles.secondaryButton}
              onClick={onClose}
              disabled={sending}
            >
              Cancel
            </button>
            <button
              type="submit"
              className={styles.primaryButton}
              disabled={controlsDisabled}
            >
              {sending ? 'Creating...' : 'Create Connection'}
            </button>
          </footer>
        </form>
      </div>
    </div>
  );
}
