import { useEffect, useRef } from 'react';
import type { ClipboardItemRecord } from '../../clipboard/db';
import { formatRelativeTime } from '../../utils/timeFormat';
import styles from './ClipboardSidebar.module.css';

interface ClipboardDuplicateDialogProps {
  existingItem: ClipboardItemRecord;
  pendingItem: ClipboardItemRecord;
  onReplace: () => void;
  onCancel: () => void;
}

export function ClipboardDuplicateDialog({
  existingItem,
  pendingItem,
  onReplace,
  onCancel,
}: ClipboardDuplicateDialogProps) {
  const dialogRef = useRef<HTMLDialogElement>(null);

  useEffect(() => {
    const dialog = dialogRef.current;
    if (dialog && !dialog.open) {
      dialog.showModal();
    }
  }, []);

  return (
    <dialog
      ref={dialogRef}
      className={styles.dialog}
      onClose={onCancel}
      aria-labelledby="duplicate-dialog-title"
    >
      <h2 id="duplicate-dialog-title" className={styles.dialogTitle}>Duplicate Node</h2>
      <p className={styles.dialogBody}>
        A clipboard item with alias <strong>"{pendingItem.node.alias}"</strong> already
        exists (clipped {formatRelativeTime(existingItem.clippedAt)}).
      </p>
      <p className={styles.dialogBody}>
        Replace it with the new snapshot?
      </p>
      <div className={styles.dialogActions}>
        <button className={styles.cancelBtn} onClick={onCancel}>
          Cancel
        </button>
        <button className={styles.replaceBtn} onClick={onReplace}>
          Replace
        </button>
      </div>
    </dialog>
  );
}
