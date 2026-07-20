import { useCallback, useEffect, useRef } from 'react';
import type { NodeFormState } from '../../graphActions/nodeAuthoringTypes';
import { createPropertyRow } from '../../graphActions/propertyRows';
import { getValidationErrorKeyForProperty } from '../../graphActions/validation';
import CloseIcon from '../../icons/CloseIcon.svg?react';
import styles from './NodeDialog.module.css';

interface NodeDialogProps {
  open: boolean;
  mode: 'create' | 'edit';
  aliasReadOnly: boolean;
  formState: NodeFormState;
  phase: 'editing' | 'sending';
  lockReason: null | 'sending' | 'disconnected';
  serverMessage: string | null;
  validationErrors: Record<string, string>;
  onFormStateChange: (formState: NodeFormState) => void;
  onSubmit: () => void;
  onClose: () => void;
}

const MIN_TEXTAREA_ROWS = 2;
const MAX_TEXTAREA_ROWS = 8;
const APPROX_TEXTAREA_CHARS_PER_ROW = 42;

function estimateTextareaRows(value: string): number {
  const rows = value.split('\n').reduce((total, line) => {
    return total + Math.max(1, Math.ceil(line.length / APPROX_TEXTAREA_CHARS_PER_ROW));
  }, 0);
  return Math.min(Math.max(rows, MIN_TEXTAREA_ROWS), MAX_TEXTAREA_ROWS);
}

// Presentational modal only. It edits a NodeFormState and reports submit/close
// intents upward; useGraphAuthoring owns validation, transport, and result handling.
export default function NodeDialog({
  open,
  mode,
  aliasReadOnly,
  formState,
  phase,
  lockReason,
  serverMessage,
  validationErrors,
  onFormStateChange,
  onSubmit,
  onClose,
}: NodeDialogProps) {
  const aliasRef = useRef<HTMLInputElement>(null);
  const nodeTypeRef = useRef<HTMLInputElement>(null);
  const propertyKeyRefs = useRef(new Map<string, HTMLInputElement>());
  const pendingFocusPropertyIdRef = useRef<string | null>(null);
  const editingExistingNode = mode === 'edit';
  const sending = phase === 'sending';
  const disconnected = lockReason === 'disconnected';
  const controlsDisabled = sending || disconnected;
  const title = editingExistingNode ? 'Edit Node' : 'Create Node';
  const closeLabel = editingExistingNode ? 'Close edit node dialog' : 'Close create node dialog';
  const submitLabel = editingExistingNode ? 'Save Changes' : 'Create Node';
  const sendingLabel = editingExistingNode ? 'Saving...' : 'Creating...';
  const disconnectedMessage = editingExistingNode
    ? 'Connection disconnected. Refresh the page and edit the node again after the app reconnects.'
    : 'Connection disconnected. Refresh the page and create the node again after the app reconnects.';

  // The overlay is a real fixed element, not a native dialog backdrop, so it
  // reliably absorbs pointer events before underlying resize handles can drag.
  useEffect(() => {
    if (!open) return;
    if (editingExistingNode) {
      nodeTypeRef.current?.focus();
    } else {
      aliasRef.current?.focus();
    }
    const handleKeyDown = (event: KeyboardEvent) => {
      if (event.key !== 'Escape') return;
      event.preventDefault();
      if (!sending) onClose();
    };
    document.addEventListener('keydown', handleKeyDown);
    return () => {
      document.removeEventListener('keydown', handleKeyDown);
    };
  }, [editingExistingNode, onClose, open, sending]);

  useEffect(() => {
    const rowId = pendingFocusPropertyIdRef.current;
    if (!rowId) return;

    const input = propertyKeyRefs.current.get(rowId);
    if (!input) return;

    input.focus();
    pendingFocusPropertyIdRef.current = null;
  }, [formState.properties]);

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

  const updateFormState = useCallback((patch: Partial<NodeFormState>) => {
    onFormStateChange({ ...formState, ...patch });
  }, [formState, onFormStateChange]);

  const updateProperty = useCallback((rowId: string, patch: { key?: string; value?: string }) => {
    onFormStateChange({
      ...formState,
      properties: formState.properties.map((row) => row.id === rowId ? { ...row, ...patch } : row),
    });
  }, [formState, onFormStateChange]);

  const addProperty = useCallback(() => {
    const nextRow = createPropertyRow();
    pendingFocusPropertyIdRef.current = nextRow.id;
    onFormStateChange({
      ...formState,
      properties: [...formState.properties, nextRow],
    });
  }, [formState, onFormStateChange]);

  const removeProperty = useCallback((rowId: string) => {
    const nextRows = formState.properties.filter((row) => row.id !== rowId);
    onFormStateChange({
      ...formState,
      properties: nextRows.length > 0 ? nextRows : [createPropertyRow()],
    });
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
        aria-labelledby="node-dialog-title"
        onPointerDown={stopPanelPointer}
        onClick={(event) => event.stopPropagation()}
      >
        <header className={styles.header}>
          <div>
            <h2 id="node-dialog-title" className={styles.title}>{title}</h2>
          </div>
          <button
            type="button"
            className={styles.iconButton}
            aria-label={closeLabel}
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
              <span className={styles.label}>Alias</span>
              <input
                ref={aliasRef}
                className={styles.input}
                value={formState.alias}
                disabled={controlsDisabled}
                readOnly={aliasReadOnly}
                aria-invalid={Boolean(validationErrors.alias)}
                aria-describedby={validationErrors.alias ? 'node-alias-error' : undefined}
                onChange={(event) => updateFormState({ alias: event.target.value })}
              />
              {validationErrors.alias && (
                <span id="node-alias-error" className={styles.errorText}>{validationErrors.alias}</span>
              )}
            </label>

            <label className={styles.field}>
              <span className={styles.label}>Node Type</span>
              <input
                ref={nodeTypeRef}
                className={styles.input}
                value={formState.nodeType}
                disabled={controlsDisabled}
                aria-invalid={Boolean(validationErrors.nodeType)}
                aria-describedby={validationErrors.nodeType ? 'node-type-error' : undefined}
                onChange={(event) => updateFormState({ nodeType: event.target.value })}
              />
              {validationErrors.nodeType && (
                <span id="node-type-error" className={styles.errorText}>{validationErrors.nodeType}</span>
              )}
            </label>

            <section className={styles.properties} aria-labelledby="node-properties-title">
              <div className={styles.propertiesHeader}>
                <h3 id="node-properties-title" className={styles.sectionTitle}>Properties</h3>
              </div>

              <div className={styles.propertyRows}>
                {formState.properties.map((row) => {
                  const keyError = validationErrors[getValidationErrorKeyForProperty(row.id, 'key')];
                  const valueError = validationErrors[getValidationErrorKeyForProperty(row.id, 'value')];
                  const valueRows = estimateTextareaRows(row.value);
                  return (
                    <div key={row.id} className={styles.propertyRow}>
                      <label className={styles.propertyField}>
                        <span className={styles.label}>Key</span>
                        <input
                          ref={(element) => {
                            if (element) {
                              propertyKeyRefs.current.set(row.id, element);
                            } else {
                              propertyKeyRefs.current.delete(row.id);
                            }
                          }}
                          className={styles.input}
                          value={row.key}
                          disabled={controlsDisabled}
                          aria-invalid={Boolean(keyError)}
                          onChange={(event) => updateProperty(row.id, { key: event.target.value })}
                        />
                        {keyError && <span className={styles.errorText}>{keyError}</span>}
                      </label>
                      <label className={styles.propertyField}>
                        <span className={styles.label}>Value</span>
                        {editingExistingNode ? (
                          <textarea
                            className={`${styles.input} ${styles.textarea}`}
                            value={row.value}
                            disabled={controlsDisabled}
                            rows={valueRows}
                            aria-invalid={Boolean(valueError)}
                            onChange={(event) => updateProperty(row.id, { value: event.target.value })}
                          />
                        ) : (
                          <input
                            className={styles.input}
                            value={row.value}
                            disabled={controlsDisabled}
                            aria-invalid={Boolean(valueError)}
                            onChange={(event) => updateProperty(row.id, { value: event.target.value })}
                          />
                        )}
                        {valueError && <span className={styles.errorText}>{valueError}</span>}
                      </label>
                      <button
                        type="button"
                        className={styles.removeButton}
                        aria-label="Remove property"
                        disabled={controlsDisabled}
                        onClick={() => removeProperty(row.id)}
                      >
                        <CloseIcon className={styles.buttonIcon} aria-hidden="true" focusable="false" />
                      </button>
                    </div>
                  );
                })}
              </div>
              <div className={styles.propertyActions}>
                <button
                  type="button"
                  className={`${styles.secondaryButton} ${styles.addPropertyButton}`}
                  disabled={controlsDisabled}
                  onClick={addProperty}
                >
                  <span aria-hidden="true">+</span>
                  <span>Add Property</span>
                </button>
              </div>
            </section>
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
              {sending ? sendingLabel : submitLabel}
            </button>
          </footer>
        </form>
      </div>
    </div>
  );
}
