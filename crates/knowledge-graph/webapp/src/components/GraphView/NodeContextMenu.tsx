import { useEffect, useLayoutEffect, useRef, useState } from 'react';
import styles from './NodeContextMenu.module.css';

interface NodeContextMenuProps {
  open: boolean;
  x: number;
  y: number;
  nodeAlias: string;
  canClipNode: boolean;
  canEditNode: boolean;
  canDeleteNode: boolean;
  onClipNode: () => void;
  onEditNode: () => void;
  onDeleteNode: () => void;
  onClose: () => void;
}

const VIEWPORT_MARGIN = 8;

// Node-level menu for actions that require a concrete graph node. This stays
// separate from GraphContextMenu, which is reserved for pane-level actions.
export default function NodeContextMenu({
  open,
  x,
  y,
  nodeAlias,
  canClipNode,
  canEditNode,
  canDeleteNode,
  onClipNode,
  onEditNode,
  onDeleteNode,
  onClose,
}: NodeContextMenuProps) {
  const [confirmingDelete, setConfirmingDelete] = useState(false);
  const [position, setPosition] = useState({ left: x, top: y });
  const menuRef = useRef<HTMLDivElement>(null);
  const firstItemRef = useRef<HTMLButtonElement>(null);
  const confirmDeleteRef = useRef<HTMLButtonElement>(null);
  const hasAnyAction = canClipNode || canEditNode || canDeleteNode;

  useLayoutEffect(() => {
    if (open) setConfirmingDelete(false);
  }, [nodeAlias, open, x, y]);

  useLayoutEffect(() => {
    if (!open) return;

    const menu = menuRef.current;
    if (!menu) {
      setPosition({ left: x, top: y });
      return;
    }

    const rect = menu.getBoundingClientRect();
    const maxLeft = Math.max(VIEWPORT_MARGIN, window.innerWidth - rect.width - VIEWPORT_MARGIN);
    const maxTop = Math.max(VIEWPORT_MARGIN, window.innerHeight - rect.height - VIEWPORT_MARGIN);
    setPosition({
      left: Math.min(Math.max(x, VIEWPORT_MARGIN), maxLeft),
      top: Math.min(Math.max(y, VIEWPORT_MARGIN), maxTop),
    });
  }, [canClipNode, canDeleteNode, canEditNode, confirmingDelete, nodeAlias, open, x, y]);

  useEffect(() => {
    if (!open) {
      setConfirmingDelete(false);
      return;
    }

    if (confirmingDelete) {
      confirmDeleteRef.current?.focus();
    } else {
      firstItemRef.current?.focus();
    }
  }, [confirmingDelete, open]);

  useEffect(() => {
    if (!open) return;

    const handlePointerDown = (event: PointerEvent) => {
      if (menuRef.current && !menuRef.current.contains(event.target as Node)) {
        onClose();
      }
    };

    const handleKeyDown = (event: KeyboardEvent) => {
      if (event.key === 'Escape') {
        event.preventDefault();
        onClose();
      }
    };

    const handleScrollOrResize = () => onClose();

    document.addEventListener('pointerdown', handlePointerDown);
    document.addEventListener('keydown', handleKeyDown);
    window.addEventListener('scroll', handleScrollOrResize, true);
    window.addEventListener('resize', handleScrollOrResize);
    return () => {
      document.removeEventListener('pointerdown', handlePointerDown);
      document.removeEventListener('keydown', handleKeyDown);
      window.removeEventListener('scroll', handleScrollOrResize, true);
      window.removeEventListener('resize', handleScrollOrResize);
    };
  }, [onClose, open]);

  if (!open || !hasAnyAction) return null;

  return (
    <div
      ref={menuRef}
      className={styles.menu}
      style={{ left: position.left, top: position.top }}
      role="menu"
      aria-label={`Node actions for ${nodeAlias}`}
    >
      {confirmingDelete ? (
        <div className={styles.confirmation} role="group" aria-label={`Confirm delete ${nodeAlias}`}>
          <div className={styles.confirmationText}>Delete "{nodeAlias}"?</div>
          <div className={styles.confirmationActions}>
            <button
              ref={confirmDeleteRef}
              type="button"
              className={`${styles.menuItem} ${styles.dangerItem}`}
              onClick={() => {
                onDeleteNode();
                onClose();
              }}
            >
              Delete
            </button>
            <button
              type="button"
              className={styles.menuItem}
              onClick={() => setConfirmingDelete(false)}
            >
              Cancel
            </button>
          </div>
        </div>
      ) : (
        <>
          {canClipNode && (
            <button
              ref={firstItemRef}
              role="menuitem"
              type="button"
              className={styles.menuItem}
              onClick={() => {
                onClipNode();
                onClose();
              }}
            >
              Clip to Workspace
            </button>
          )}
          {canEditNode && (
            <button
              ref={canClipNode ? undefined : firstItemRef}
              role="menuitem"
              type="button"
              className={styles.menuItem}
              onClick={() => {
                onEditNode();
                onClose();
              }}
            >
              Edit Node
            </button>
          )}
          {canDeleteNode && (
            <button
              ref={!canClipNode && !canEditNode ? firstItemRef : undefined}
              role="menuitem"
              type="button"
              className={`${styles.menuItem} ${styles.dangerItem}`}
              onClick={() => setConfirmingDelete(true)}
            >
              Delete Node
            </button>
          )}
        </>
      )}
    </div>
  );
}
