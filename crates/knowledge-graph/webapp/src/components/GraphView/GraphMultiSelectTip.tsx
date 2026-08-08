import styles from './GraphMultiSelectTip.module.css';

interface GraphMultiSelectTipProps {
  visible: boolean;
  fading: boolean;
  onDismiss: () => void;
}

export default function GraphMultiSelectTip({
  visible,
  fading,
  onDismiss,
}: GraphMultiSelectTipProps) {
  if (!visible) return null;

  return (
    <div
      className={`${styles.tip}${fading ? ` ${styles.fading}` : ''}`}
      role="status"
    >
      <button type="button" className={styles.tipButton} onClick={onDismiss}>
        Multi-select: Shift/Ctrl/Cmd + click nodes, or Shift + drag on the canvas.
      </button>
    </div>
  );
}
