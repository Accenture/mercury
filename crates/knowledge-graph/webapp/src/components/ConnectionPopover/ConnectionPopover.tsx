import { useCallback, useEffect, useLayoutEffect, useRef, useState } from 'react';
import type { ConnectionFormState } from '../../graphActions/connectionAuthoringTypes';
import {
  CONNECTION_RELATION_COLORS,
  CONNECTION_RELATION_OPTIONS,
} from '../../graphActions/connectionRelations';
import styles from './ConnectionPopover.module.css';

interface ConnectionPopoverProps {
  formState: ConnectionFormState;
  phase: 'editing' | 'sending';
  lockReason: null | 'sending' | 'disconnected';
  serverMessage: string | null;
  validationErrors: Record<string, string>;
  /** Viewport position of the completing connect gesture; null falls back to top-center. */
  anchor: { x: number; y: number } | null;
  onFormStateChange: (formState: ConnectionFormState) => void;
  onSubmit: () => void;
  onClose: () => void;
}

const VIEWPORT_MARGIN = 12;

/**
 * Inline relation assignment for a just-drawn connection — anchored at the
 * drop point instead of a modal (Neo4j-style).  Source and target are already
 * fixed by the gesture; clicking a known-relation chip creates the connection
 * immediately, and the free-text field covers custom relations (Enter
 * submits).  Esc or clicking anywhere else cancels.
 *
 * Presentational only: useGraphAuthoring owns validation, transport, and
 * result handling; a successful create closes the session and unmounts this.
 */
export default function ConnectionPopover({
  formState,
  phase,
  lockReason,
  serverMessage,
  validationErrors,
  anchor,
  onFormStateChange,
  onSubmit,
  onClose,
}: ConnectionPopoverProps) {
  const popoverRef = useRef<HTMLDivElement>(null);
  const inputRef = useRef<HTMLInputElement>(null);
  const [position, setPosition] = useState<{ left: number; top: number } | null>(null);
  const sending = phase === 'sending';
  const disconnected = lockReason === 'disconnected';
  const controlsDisabled = sending || disconnected;

  // A chip click sets the relation and submits — but submit() reads the
  // authoring state through a ref that only syncs after the state commit, so
  // the submit is armed here and fired once the new relation round-trips
  // through props.
  const pendingChipSubmitRef = useRef<string | null>(null);
  useEffect(() => {
    if (pendingChipSubmitRef.current === null) return;
    if (formState.relation !== pendingChipSubmitRef.current) return;
    pendingChipSubmitRef.current = null;
    if (!controlsDisabled) onSubmit();
  }, [controlsDisabled, formState.relation, onSubmit]);

  useEffect(() => {
    inputRef.current?.focus();
  }, []);

  // Clamp into the viewport once the size is measurable.
  useLayoutEffect(() => {
    const fallback = { x: window.innerWidth / 2, y: 96 };
    const point = anchor ?? fallback;
    const rect = popoverRef.current?.getBoundingClientRect();
    const width = rect?.width ?? 280;
    const height = rect?.height ?? 180;
    setPosition({
      left: Math.min(
        Math.max(point.x - (width / 2), VIEWPORT_MARGIN),
        Math.max(VIEWPORT_MARGIN, window.innerWidth - width - VIEWPORT_MARGIN),
      ),
      top: Math.min(
        Math.max(point.y + 14, VIEWPORT_MARGIN),
        Math.max(VIEWPORT_MARGIN, window.innerHeight - height - VIEWPORT_MARGIN),
      ),
    });
  }, [anchor]);

  useEffect(() => {
    const handleKeyDown = (event: KeyboardEvent) => {
      if (event.key !== 'Escape') return;
      event.preventDefault();
      if (!sending) onClose();
    };
    const handlePointerDown = (event: PointerEvent) => {
      if (sending) return;
      if (popoverRef.current && !popoverRef.current.contains(event.target as Node)) {
        onClose();
      }
    };
    document.addEventListener('keydown', handleKeyDown);
    document.addEventListener('pointerdown', handlePointerDown);
    return () => {
      document.removeEventListener('keydown', handleKeyDown);
      document.removeEventListener('pointerdown', handlePointerDown);
    };
  }, [onClose, sending]);

  const updateRelation = useCallback((relation: string) => {
    onFormStateChange({ ...formState, relation });
  }, [formState, onFormStateChange]);

  const chooseChip = useCallback((relation: string) => {
    if (controlsDisabled) return;
    pendingChipSubmitRef.current = relation;
    onFormStateChange({ ...formState, relation });
  }, [controlsDisabled, formState, onFormStateChange]);

  const handleFormSubmit = useCallback((event: React.SubmitEvent<HTMLFormElement>) => {
    event.preventDefault();
    if (controlsDisabled) return;
    onSubmit();
  }, [controlsDisabled, onSubmit]);

  const relationError = validationErrors.relation ?? validationErrors.command ??
    validationErrors.sourceAlias ?? validationErrors.targetAlias;

  return (
    <div
      ref={popoverRef}
      className={styles.popover}
      style={position ? { left: position.left, top: position.top } : { visibility: 'hidden' }}
      role="dialog"
      aria-label={`Create connection from ${formState.sourceAlias} to ${formState.targetAlias}`}
    >
      <div className={styles.header}>
        <span className={styles.endpoint}>{formState.sourceAlias}</span>
        <span className={styles.arrow} aria-hidden="true">→</span>
        <span className={styles.endpoint}>{formState.targetAlias}</span>
      </div>

      <div className={styles.chips}>
        {CONNECTION_RELATION_OPTIONS.map((relation) => (
          <button
            key={relation}
            type="button"
            className={styles.chip}
            style={{ ['--chip-color' as string]: CONNECTION_RELATION_COLORS[relation] }}
            disabled={controlsDisabled}
            onClick={() => chooseChip(relation)}
          >
            {relation}
          </button>
        ))}
      </div>

      <form className={styles.customRow} onSubmit={handleFormSubmit}>
        <input
          ref={inputRef}
          className={styles.input}
          value={formState.relation}
          placeholder="custom relation…"
          autoComplete="off"
          autoCorrect="off"
          spellCheck={false}
          disabled={controlsDisabled}
          aria-label="Relation name"
          aria-invalid={Boolean(validationErrors.relation)}
          onChange={(event) => updateRelation(event.target.value)}
        />
        <button
          type="submit"
          className={styles.submitButton}
          disabled={controlsDisabled || !formState.relation.trim()}
        >
          {sending ? 'Creating…' : 'Connect'}
        </button>
      </form>

      {relationError && !sending && (
        <div className={styles.errorText} role="alert">{relationError}</div>
      )}
      {serverMessage && (
        <div className={disconnected ? styles.warningText : styles.statusText} role="status">
          {serverMessage}
        </div>
      )}
    </div>
  );
}
