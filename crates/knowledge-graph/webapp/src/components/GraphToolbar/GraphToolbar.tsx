import { useCallback, type ReactNode } from 'react';
import type { MinigraphGraphData } from '../../utils/graphTypes';
import styles from './GraphToolbar.module.css';

interface GraphToolbarProps {
  graphData:      MinigraphGraphData | null;
  /** Resolved display name for the graph (root node name or fallback). */
  graphName?:     string;
  onCopySuccess?: () => void;
  onCopyError?:   () => void;
  /** Optional extra action buttons rendered before the copy button. */
  extraActions?:  ReactNode;
}

export default function GraphToolbar({
  graphData,
  graphName,
  onCopySuccess,
  onCopyError,
  extraActions,
}: GraphToolbarProps) {
  const handleCopy = useCallback(() => {
    if (!graphData) return;
    navigator.clipboard
      .writeText(JSON.stringify(graphData, null, 2))
      .then(() => onCopySuccess?.())
      .catch(() => onCopyError?.());
  }, [graphData, onCopySuccess, onCopyError]);

  const nodeCount       = graphData?.nodes.length ?? 0;
  const connectionCount = (graphData?.connections ?? []).length;

  return (
    <div className={styles.toolbar}>
      <div className={styles.nameGroup}>
        <span className={styles.graphName}>{graphName ?? 'Untitled'}</span>
        <span className={styles.stats}>
          {nodeCount} node{nodeCount !== 1 ? 's' : ''}
          {' · '}
          {connectionCount} connection{connectionCount !== 1 ? 's' : ''}
        </span>
      </div>

      <div className={styles.toolbarActions}>
        {extraActions}
        <button
          className={styles.toolbarButton}
          onClick={handleCopy}
          title="Copy raw graph JSON to clipboard"
          aria-label="Copy raw graph JSON to clipboard"
        >
          📑
        </button>
      </div>
    </div>
  );
}
