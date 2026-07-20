import { useEffect, useRef } from 'react';
import styles from './GraphContextMenu.module.css';

interface GraphContextMenuProps {
  open: boolean;
  x: number;
  y: number;
  canCreateNode: boolean;
  onCreateNode: () => void;
  onClose: () => void;
}

// Pane-level graph menu. GraphView owns the right-click coordinates; this
// component only renders the pointer-positioned menu and dismissal behavior.
export default function GraphContextMenu({
  open,
  x,
  y,
  canCreateNode,
  onCreateNode,
  onClose,
}: GraphContextMenuProps) {
  const menuRef = useRef<HTMLDivElement>(null);
  const firstItemRef = useRef<HTMLButtonElement>(null);

  // This is a lightweight menu, not a modal: focus the first action, close on
  // outside pointer down or Escape, and do not trap focus.
  useEffect(() => {
    if (!open) return;
    firstItemRef.current?.focus();

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

    document.addEventListener('pointerdown', handlePointerDown);
    document.addEventListener('keydown', handleKeyDown);
    return () => {
      document.removeEventListener('pointerdown', handlePointerDown);
      document.removeEventListener('keydown', handleKeyDown);
    };
  }, [open, onClose]);

  if (!open) return null;

  return (
    <div
      ref={menuRef}
      className={styles.menu}
      style={{ left: x, top: y }}
      role="menu"
      aria-label="Graph actions"
    >
      <button
        ref={firstItemRef}
        role="menuitem"
        type="button"
        className={styles.menuItem}
        disabled={!canCreateNode}
        onClick={() => {
          if (!canCreateNode) return;
          onCreateNode();
          onClose();
        }}
      >
        Create Node
      </button>
    </div>
  );
}
