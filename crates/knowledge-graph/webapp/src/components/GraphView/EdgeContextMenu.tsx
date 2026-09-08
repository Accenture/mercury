import { useEffect, useLayoutEffect, useRef, useState } from 'react';
import styles from './NodeContextMenu.module.css';

interface EdgeContextMenuProps {
  open: boolean;
  x: number;
  y: number;
  sourceAlias: string;
  targetAlias: string;
  /** Relation types carried by this directed edge, in model order. */
  relations: string[];
  /** `relation` undefined deletes the whole directed edge (all relations). */
  onDeleteRelation: (relation?: string) => void;
  onClose: () => void;
}

const VIEWPORT_MARGIN = 8;

// Edge-level menu: one delete item per relation, so a multi-relation edge
// (e.g. "fetch, test") can shed a single relation.  No confirmation step —
// edge deletion is already a one-keystroke act (Delete on a selected edge)
// and every path is undoable via the toast/Ctrl+Z.
export default function EdgeContextMenu({
  open,
  x,
  y,
  sourceAlias,
  targetAlias,
  relations,
  onDeleteRelation,
  onClose,
}: EdgeContextMenuProps) {
  const [position, setPosition] = useState({ left: x, top: y });
  const menuRef = useRef<HTMLDivElement>(null);
  const firstItemRef = useRef<HTMLButtonElement>(null);

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
  }, [open, relations, x, y]);

  useEffect(() => {
    if (open) firstItemRef.current?.focus();
  }, [open]);

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

  if (!open || relations.length === 0) return null;

  return (
    <div
      ref={menuRef}
      className={styles.menu}
      style={{ left: position.left, top: position.top }}
      role="menu"
      aria-label={`Connection actions for ${sourceAlias} → ${targetAlias}`}
    >
      {relations.map((relation, index) => (
        <button
          key={`${relation}-${index}`}
          ref={index === 0 ? firstItemRef : undefined}
          role="menuitem"
          type="button"
          className={`${styles.menuItem} ${styles.dangerItem}`}
          onClick={() => {
            onDeleteRelation(relation);
            onClose();
          }}
        >
          Delete '{relation}'
        </button>
      ))}
      {relations.length > 1 && (
        <button
          role="menuitem"
          type="button"
          className={`${styles.menuItem} ${styles.dangerItem}`}
          onClick={() => {
            onDeleteRelation(undefined);
            onClose();
          }}
        >
          Delete all ({relations.length})
        </button>
      )}
    </div>
  );
}
