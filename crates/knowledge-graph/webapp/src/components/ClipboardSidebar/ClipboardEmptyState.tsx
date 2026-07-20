import styles from './ClipboardSidebar.module.css';

export function ClipboardEmptyState() {
  return (
    <div className={styles.emptyState}>
      <span className={styles.emptyIcon}>📋</span>
      <span className={styles.emptyTitle}>No items clipped yet.</span>
      <span className={styles.emptyHint}>
        Right-click a node in the Graph view to get started.
      </span>
    </div>
  );
}
