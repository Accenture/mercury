import { useState, useCallback, useRef, useEffect } from 'react';
import styles from './GraphSaveButton.module.css';

interface GraphSaveButtonProps {
  /** When true the button is rendered disabled (no graph loaded yet). */
  disabled:     boolean;
  /** Pre-filled name derived by the caller from the current graph data. */
  defaultName:  string;
  /** Last server-confirmed export name, or null when the graph is unsaved/dirty. */
  savedName?:   string | null;
  /** Called with the confirmed name when the user submits the form. */
  onSave:       (name: string) => void;
  /** Returns true if a bookmark with this name already exists. */
  nameExists?:  (name: string) => boolean;
  /** When false the button is disabled — an active connection is required to export. */
  connected?:   boolean;
}

export function formatGraphSaveButtonLabel(savedName: string | null): string {
  return savedName ? `Saved: ${savedName}` : 'Save Graph';
}

/**
 * A self-contained save button for the Playground header.
 *
 * Owns only the transient inline-form state (open/closed, current name value).
 * All domain knowledge (graph data, deriving a default name, persisting) stays
 * in the parent — this component receives only primitives and callbacks.
 */
export default function GraphSaveButton({
  disabled,
  defaultName,
  savedName = null,
  onSave,
  nameExists,
  connected = false,
}: GraphSaveButtonProps) {
  const [isSaving, setIsSaving] = useState(false);
  const [saveName, setSaveName] = useState('');
  const nameInputRef = useRef<HTMLInputElement>(null);
  const triggerRef = useRef<HTMLButtonElement>(null);
  const restoreFocusRef = useRef(false);

  // Open the inline form and pre-fill with the caller-supplied default.
  const handleOpen = useCallback(() => {
    setSaveName(defaultName);
    setIsSaving(true);
  }, [defaultName]);

  const handleCancel = useCallback(() => {
    restoreFocusRef.current = true;
    setIsSaving(false);
    setSaveName('');
  }, []);

  const handleConfirm = useCallback(() => {
    const trimmed = saveName.trim();
    if (!trimmed) return;
    onSave(trimmed);
    restoreFocusRef.current = true;
    setIsSaving(false);
    setSaveName('');
  }, [saveName, onSave]);

  const handleKeyDown = useCallback((e: React.KeyboardEvent<HTMLInputElement>) => {
    if (e.key === 'Enter')  { e.preventDefault(); handleConfirm(); }
    if (e.key === 'Escape') { e.preventDefault(); handleCancel(); }
  }, [handleConfirm, handleCancel]);

  // Focus the input on open and return focus to the trigger on either close path.
  useEffect(() => {
    if (isSaving) {
      nameInputRef.current?.focus();
    } else if (restoreFocusRef.current) {
      restoreFocusRef.current = false;
      triggerRef.current?.focus();
    }
  }, [isSaving]);

  if (isSaving) {
    return (
      <div className={styles.saveForm}>
        <input
          ref={nameInputRef}
          className={`${styles.saveInput}${nameExists?.(saveName.trim()) ? ` ${styles.saveInputWarn}` : ''}`}
          type="text"
          value={saveName}
          onChange={e => setSaveName(e.target.value)}
          onKeyDown={handleKeyDown}
          placeholder="Enter a name…"
          aria-label="Graph save name"
          maxLength={80}
        />
        {nameExists?.(saveName.trim()) && (
          <span className={styles.saveWarnLabel} role="status">Overwrite?</span>
        )}
        <button
          className={styles.saveActionBtn}
          onClick={handleConfirm}
          disabled={!saveName.trim()}
          aria-label="Confirm save"
        >
          ✅
        </button>
        <button
          className={styles.saveActionBtn}
          onClick={handleCancel}
          aria-label="Cancel save"
        >
          ❌
        </button>
      </div>
    );
  }

  return (
    <button
      ref={triggerRef}
      className={`${styles.saveBtn}${savedName ? ` ${styles.saveBtnSaved}` : ''}`}
      onClick={handleOpen}
      disabled={disabled || !connected}
      title={
        disabled    ? 'No graph loaded'
        : !connected ? 'Connect first to save'
        : savedName  ? `Graph saved as ${savedName}. Click to save again`
        :              'Export graph snapshot to server and save bookmark'
      }
      aria-label={savedName ? `Graph saved as ${savedName}. Save again` : 'Save graph snapshot'}
    >
      <span aria-hidden="true">{savedName ? '✅' : '💾'}</span>
      <span className={styles.saveBtnLabel}>
        {formatGraphSaveButtonLabel(savedName)}
      </span>
    </button>
  );
}
