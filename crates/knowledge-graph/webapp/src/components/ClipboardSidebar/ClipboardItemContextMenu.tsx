import { useEffect, useLayoutEffect, useRef, useState } from 'react';
import styles from './ClipboardItemContextMenu.module.css';

interface ClipboardItemContextMenuProps {
  open: boolean;
  x: number;
  y: number;
  canPasteToInput: boolean;
  onPasteToInput: () => void;
  onInspect: () => void;
  onClose: () => void;
}

const VIEWPORT_INSET = 16;

function clampCoordinate(coordinate: number, size: number, viewportSize: number) {
  const min = VIEWPORT_INSET;
  const max = Math.max(VIEWPORT_INSET, viewportSize - size - VIEWPORT_INSET);
  return Math.min(Math.max(coordinate, min), max);
}

export function ClipboardItemContextMenu({
  open,
  x,
  y,
  canPasteToInput,
  onPasteToInput,
  onInspect,
  onClose,
}: ClipboardItemContextMenuProps) {
  const menuRef = useRef<HTMLDivElement>(null);
  const pasteRef = useRef<HTMLButtonElement>(null);
  const describeRef = useRef<HTMLButtonElement>(null);
  const [position, setPosition] = useState({ left: x, top: y });

  useLayoutEffect(() => {
    if (!open || !menuRef.current) return;

    const rect = menuRef.current.getBoundingClientRect();
    setPosition({
      left: clampCoordinate(x, rect.width, window.innerWidth),
      top: clampCoordinate(y, rect.height, window.innerHeight),
    });
  }, [open, x, y]);

  useEffect(() => {
    if (!open) return;

    if (canPasteToInput) {
      pasteRef.current?.focus();
    } else {
      describeRef.current?.focus();
    }

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
  }, [open, canPasteToInput, onClose]);

  if (!open) return null;

  return (
    <div
      ref={menuRef}
      className={styles.menu}
      style={{ left: position.left, top: position.top }}
      role="menu"
      aria-label="Clipboard item actions"
    >
      <button
        ref={pasteRef}
        role="menuitem"
        type="button"
        className={styles.menuItem}
        disabled={!canPasteToInput}
        onClick={() => {
          if (!canPasteToInput) return;
          onPasteToInput();
        }}
      >
        Paste to Input
      </button>
      <button
        ref={describeRef}
        role="menuitem"
        type="button"
        className={styles.menuItem}
        onClick={onInspect}
      >
        Inspect
      </button>
    </div>
  );
}