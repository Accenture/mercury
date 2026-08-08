import { useEffect, useLayoutEffect, useRef, useState } from 'react';
import styles from './NodeContextMenu.module.css';

interface BaseNodeContextMenuProps {
  open: boolean;
  x: number;
  y: number;
  onClose: () => void;
}

interface SingleNodeContextMenuProps extends BaseNodeContextMenuProps {
  mode: 'single-node';
  nodeAlias: string;
  canClipNode: boolean;
  canEditNode: boolean;
  canDeleteNode: boolean;
  onClipNode: () => void;
  onEditNode: () => void;
  onDeleteNode: () => void;
}

interface MultiNodeContextMenuProps extends BaseNodeContextMenuProps {
  mode: 'multi-node';
  selectedCount: number;
  canClipSelectedNodes: boolean;
  canDeleteSelectedNodes: boolean;
  onClipSelectedNodes: () => void;
  onDeleteSelectedNodes: () => void;
}

type NodeContextMenuProps = SingleNodeContextMenuProps | MultiNodeContextMenuProps;

const VIEWPORT_MARGIN = 8;

// Node-level menu for actions that require a concrete graph node. This stays
// separate from GraphContextMenu, which is reserved for pane-level actions.
export default function NodeContextMenu(props: NodeContextMenuProps) {
  const { open, x, y, onClose } = props;
  const [confirmingDelete, setConfirmingDelete] = useState(false);
  const [position, setPosition] = useState({ left: x, top: y });
  const menuRef = useRef<HTMLDivElement>(null);
  const firstItemRef = useRef<HTMLButtonElement>(null);
  const confirmDeleteRef = useRef<HTMLButtonElement>(null);
  const selectedCount = props.mode === 'multi-node' ? props.selectedCount : null;
  const isMultiNode = selectedCount !== null && selectedCount > 1;
  const canClip = props.mode === 'multi-node'
    ? isMultiNode && props.canClipSelectedNodes
    : props.canClipNode;
  const canEdit = props.mode === 'single-node' && props.canEditNode;
  const canDelete = props.mode === 'multi-node'
    ? isMultiNode && props.canDeleteSelectedNodes
    : props.canDeleteNode;
  const hasAnyAction = canClip || canEdit || canDelete;
  const targetLabel = isMultiNode
    ? `${selectedCount} selected nodes`
    : props.mode === 'single-node' ? props.nodeAlias : '';

  useLayoutEffect(() => {
    if (open) setConfirmingDelete(false);
  }, [open, targetLabel, x, y]);

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
  }, [canClip, canDelete, canEdit, confirmingDelete, open, targetLabel, x, y]);

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
      aria-label={isMultiNode ? `Actions for ${selectedCount} selected nodes` : `Node actions for ${targetLabel}`}
    >
      {confirmingDelete ? (
        <div className={styles.confirmation} role="group" aria-label={`Confirm delete ${targetLabel}`}>
          <div className={styles.confirmationText}>
            {isMultiNode ? `Delete ${selectedCount} selected nodes?` : `Delete "${targetLabel}"?`}
          </div>
          <div className={styles.confirmationActions}>
            <button
              ref={confirmDeleteRef}
              type="button"
              className={`${styles.menuItem} ${styles.dangerItem}`}
              onClick={() => {
                if (props.mode === 'multi-node') {
                  props.onDeleteSelectedNodes();
                } else {
                  props.onDeleteNode();
                }
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
          {canClip && (
            <button
              ref={firstItemRef}
              role="menuitem"
              type="button"
              className={styles.menuItem}
              onClick={() => {
                if (props.mode === 'multi-node') {
                  props.onClipSelectedNodes();
                } else {
                  props.onClipNode();
                }
                onClose();
              }}
            >
              {isMultiNode ? `Clip ${selectedCount} selected nodes to Workspace` : 'Clip to Workspace'}
            </button>
          )}
          {canEdit && props.mode === 'single-node' && (
            <button
              ref={canClip ? undefined : firstItemRef}
              role="menuitem"
              type="button"
              className={styles.menuItem}
              onClick={() => {
                props.onEditNode();
                onClose();
              }}
            >
              Edit Node
            </button>
          )}
          {canDelete && (
            <button
              ref={!canClip && !canEdit ? firstItemRef : undefined}
              role="menuitem"
              type="button"
              className={`${styles.menuItem} ${styles.dangerItem}`}
              onClick={() => setConfirmingDelete(true)}
            >
              {isMultiNode ? `Delete ${selectedCount} selected nodes` : 'Delete Node'}
            </button>
          )}
        </>
      )}
    </div>
  );
}
