import { useCallback, useEffect, useRef, useState } from 'react';
import type { NodeFormState } from '../../graphActions/nodeAuthoringTypes';
import { createPropertyRow, sortPropertyRowsByKey } from '../../graphActions/propertyRows';
import { getValidationErrorKeyForProperty } from '../../graphActions/validation';
import { getMinigraphNodeAccent, getMinigraphNodeTypeMeta } from '../../utils/minigraphNodeTheme';
import CloseIcon from '../../icons/CloseIcon.svg?react';
import styles from './NodeEditPanel.module.css';

interface NodeEditPanelProps {
  /** 'edit' updates an existing node (alias fixed); 'create' authors a new one (alias editable). */
  mode: 'create' | 'edit';
  formState: NodeFormState;
  phase: 'editing' | 'sending';
  lockReason: null | 'sending' | 'disconnected';
  serverMessage: string | null;
  validationErrors: Record<string, string>;
  onFormStateChange: (formState: NodeFormState) => void;
  onSubmit: () => void;
  onClose: () => void;
}

const MIN_TEXTAREA_ROWS = 1;
const MAX_TEXTAREA_ROWS = 10;
const APPROX_TEXTAREA_CHARS_PER_ROW = 36;

function estimateTextareaRows(value: string): number {
  const rows = value.split('\n').reduce((total, line) => {
    return total + Math.max(1, Math.ceil(line.length / APPROX_TEXTAREA_CHARS_PER_ROW));
  }, 0);
  return Math.min(Math.max(rows, MIN_TEXTAREA_ROWS), MAX_TEXTAREA_ROWS);
}

/**
 * In-place node authoring panel rendered in the left panel slot (the
 * console's space) instead of a modal — a "magnified node": the
 * accent-colored ribbon mirrors the node header (icon + alias + type badge,
 * colored by the current node type) and the properties render as aligned
 * node-style rows.  The body scrolls, so the number of key-value rows does
 * not matter.  Create and edit share the exact same look and behavior; the
 * only difference is that create edits the alias in the ribbon.
 *
 * Presentational only: it edits a NodeFormState and reports submit/close
 * intents upward; useGraphAuthoring owns validation, transport, and result
 * handling (a successful save closes the session, which unmounts this panel
 * and restores whatever the left slot held before).
 */
export default function NodeEditPanel({
  mode,
  formState,
  phase,
  lockReason,
  serverMessage,
  validationErrors,
  onFormStateChange,
  onSubmit,
  onClose,
}: NodeEditPanelProps) {
  const aliasRef = useRef<HTMLInputElement>(null);
  const nodeTypeRef = useRef<HTMLInputElement>(null);
  const propertyKeyRefs = useRef(new Map<string, HTMLInputElement>());
  const pendingFocusPropertyIdRef = useRef<string | null>(null);
  const creating = mode === 'create';
  const sending = phase === 'sending';
  const disconnected = lockReason === 'disconnected';
  const controlsDisabled = sending || disconnected;
  const submitLabel = creating ? 'Create Node' : 'Save Changes';
  const sendingLabel = creating ? 'Creating...' : 'Saving...';
  const disconnectedMessage = creating
    ? 'Connection disconnected. Refresh the page and create the node again after the app reconnects.'
    : 'Connection disconnected. Refresh the page and edit the node again after the app reconnects.';

  const meta = getMinigraphNodeTypeMeta(formState.nodeType);
  const accent = getMinigraphNodeAccent(formState.nodeType);

  // Escape closes the editor and restores the previous left-panel content.
  useEffect(() => {
    if (creating) {
      aliasRef.current?.focus();
    } else {
      nodeTypeRef.current?.focus();
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
  }, [creating, onClose, sending]);

  useEffect(() => {
    const rowId = pendingFocusPropertyIdRef.current;
    if (!rowId) return;
    const input = propertyKeyRefs.current.get(rowId);
    if (!input) return;
    input.focus();
    pendingFocusPropertyIdRef.current = null;
  }, [formState.properties]);

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

  // ── Row drag-and-drop ─────────────────────────────────────────────────────
  // Rows sharing a key use the [] append signature, so their ORDER is the
  // array order ([0], [1], … on submit).  After a drop the rows re-sort by key
  // ascending (stable), which regroups same-key rows while keeping the user's
  // new relative order inside the group.
  const dragRowIdRef = useRef<string | null>(null);
  const [dropTargetId, setDropTargetId] = useState<string | null>(null); // row id, or 'end'

  const clearDragState = useCallback(() => {
    dragRowIdRef.current = null;
    setDropTargetId(null);
  }, []);

  const handleGripDragStart = useCallback((rowId: string) => (event: React.DragEvent<HTMLElement>) => {
    dragRowIdRef.current = rowId;
    if (event.dataTransfer) {
      event.dataTransfer.effectAllowed = 'move';
      event.dataTransfer.setData('text/plain', rowId);
      const rowElement = (event.currentTarget as HTMLElement).closest('[data-row-id]');
      if (rowElement instanceof HTMLElement && typeof event.dataTransfer.setDragImage === 'function') {
        event.dataTransfer.setDragImage(rowElement, 16, 16);
      }
    }
  }, []);

  const handleRowDragOver = useCallback((targetId: string) => (event: React.DragEvent<HTMLElement>) => {
    if (dragRowIdRef.current === null) return;
    event.preventDefault();
    if (event.dataTransfer) event.dataTransfer.dropEffect = 'move';
    setDropTargetId((current) => (current === targetId ? current : targetId));
  }, []);

  const dropDraggedRow = useCallback((targetId: string | null) => {
    const draggedId = dragRowIdRef.current;
    clearDragState();
    if (draggedId === null || draggedId === targetId) return;

    const dragged = formState.properties.find((row) => row.id === draggedId);
    if (!dragged) return;
    const remaining = formState.properties.filter((row) => row.id !== draggedId);
    let insertAt = remaining.length;
    if (targetId !== null) {
      const targetIndex = remaining.findIndex((row) => row.id === targetId);
      if (targetIndex !== -1) insertAt = targetIndex;
    }
    const reordered = [
      ...remaining.slice(0, insertAt),
      dragged,
      ...remaining.slice(insertAt),
    ];
    onFormStateChange({
      ...formState,
      properties: sortPropertyRowsByKey(reordered),
    });
  }, [clearDragState, formState, onFormStateChange]);

  const handleRowDrop = useCallback((targetId: string | null) => (event: React.DragEvent<HTMLElement>) => {
    if (dragRowIdRef.current === null) return;
    event.preventDefault();
    dropDraggedRow(targetId);
  }, [dropDraggedRow]);

  return (
    <div className={styles.root}>
      <form
        className={styles.card}
        style={{ borderColor: accent, ['--node-accent' as string]: accent }}
        aria-label={creating ? 'Create node' : `Edit node ${formState.alias}`}
        onSubmit={handleFormSubmit}
      >
        <header className={styles.ribbon}>
          <span className={styles.ribbonIcon} aria-hidden="true">{meta.icon}</span>
          {creating ? (
            <input
              ref={aliasRef}
              className={styles.ribbonAliasInput}
              value={formState.alias}
              placeholder="node-alias"
              aria-label="Node alias"
              disabled={controlsDisabled}
              aria-invalid={Boolean(validationErrors.alias)}
              onChange={(event) => updateFormState({ alias: event.target.value })}
            />
          ) : (
            <span className={styles.ribbonAlias}>{formState.alias}</span>
          )}
          <span className={styles.ribbonBadge}>{meta.label}</span>
          <button
            type="button"
            className={styles.ribbonClose}
            aria-label="Close node editor"
            title="Close (Esc)"
            onClick={onClose}
            disabled={sending}
          >
            <CloseIcon className={styles.ribbonCloseIcon} aria-hidden="true" focusable="false" />
          </button>
        </header>

        <div className={styles.body}>
          {serverMessage && !disconnected && (
            <div className={styles.message} role="status">{serverMessage}</div>
          )}
          {validationErrors.command && (
            <div className={styles.errorMessage} role="alert">{validationErrors.command}</div>
          )}
          {validationErrors.alias && (
            <div className={styles.errorMessage} role="alert">{validationErrors.alias}</div>
          )}
          {disconnected && (
            <div className={styles.warningMessage} role="status">
              {serverMessage ?? disconnectedMessage}
            </div>
          )}

          <div className={styles.row}>
            {/* spacer keeps the grid columns aligned with property rows */}
            <span aria-hidden="true" />
            <label className={styles.rowLabel} htmlFor="node-edit-type">type</label>
            <div className={styles.rowValue}>
              <input
                id="node-edit-type"
                ref={nodeTypeRef}
                className={styles.valueInput}
                value={formState.nodeType}
                disabled={controlsDisabled}
                aria-invalid={Boolean(validationErrors.nodeType)}
                onChange={(event) => updateFormState({ nodeType: event.target.value })}
              />
              {validationErrors.nodeType && (
                <span className={styles.errorText}>{validationErrors.nodeType}</span>
              )}
            </div>
            <span className={styles.rowSpacer} aria-hidden="true" />
          </div>

          {formState.properties.map((row) => {
            const keyError = validationErrors[getValidationErrorKeyForProperty(row.id, 'key')];
            const valueError = validationErrors[getValidationErrorKeyForProperty(row.id, 'value')];
            return (
              <div
                key={row.id}
                data-row-id={row.id}
                className={dropTargetId === row.id ? `${styles.row} ${styles.rowDropTarget}` : styles.row}
                onDragOver={handleRowDragOver(row.id)}
                onDrop={handleRowDrop(row.id)}
              >
                <span
                  className={styles.dragGrip}
                  role="button"
                  aria-label={`Reorder property ${row.key.trim() || '(empty)'}`}
                  title="Drag to reorder — same keys append as [0], [1], … in row order"
                  draggable={!controlsDisabled}
                  onDragStart={handleGripDragStart(row.id)}
                  onDragEnd={clearDragState}
                >
                  ⠿
                </span>
                <div className={styles.rowKey}>
                  <input
                    ref={(element) => {
                      if (element) {
                        propertyKeyRefs.current.set(row.id, element);
                      } else {
                        propertyKeyRefs.current.delete(row.id);
                      }
                    }}
                    className={styles.keyInput}
                    value={row.key}
                    placeholder="key"
                    aria-label="Property key"
                    disabled={controlsDisabled}
                    aria-invalid={Boolean(keyError)}
                    onChange={(event) => updateProperty(row.id, { key: event.target.value })}
                  />
                  {keyError && <span className={styles.errorText}>{keyError}</span>}
                </div>
                <div className={styles.rowValue}>
                  <textarea
                    className={styles.valueInput}
                    value={row.value}
                    placeholder="value"
                    aria-label="Property value"
                    disabled={controlsDisabled}
                    rows={estimateTextareaRows(row.value)}
                    aria-invalid={Boolean(valueError)}
                    onChange={(event) => updateProperty(row.id, { value: event.target.value })}
                  />
                  {valueError && <span className={styles.errorText}>{valueError}</span>}
                </div>
                <button
                  type="button"
                  className={styles.removeButton}
                  aria-label="Remove property"
                  disabled={controlsDisabled}
                  onClick={() => removeProperty(row.id)}
                >
                  <CloseIcon className={styles.removeIcon} aria-hidden="true" focusable="false" />
                </button>
              </div>
            );
          })}

          <div
            className={dropTargetId === 'end' ? `${styles.addRow} ${styles.rowDropTarget}` : styles.addRow}
            onDragOver={handleRowDragOver('end')}
            onDrop={handleRowDrop(null)}
          >
            <button
              type="button"
              className={styles.addButton}
              disabled={controlsDisabled}
              onClick={addProperty}
            >
              <span aria-hidden="true">+</span>
              <span>Add Property</span>
            </button>
          </div>
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
  );
}
