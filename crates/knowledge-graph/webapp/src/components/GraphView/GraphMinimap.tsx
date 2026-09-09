import {
  useCallback,
  useEffect,
  useRef,
  useState,
  type PointerEvent as ReactPointerEvent,
  type ReactNode,
} from 'react';
import { ControlButton, Controls, MiniMap, type Node } from '@xyflow/react';
import { useLocalStorage } from '../../hooks/useLocalStorage';
import styles from './GraphMinimap.module.css';

interface GraphMinimapProps {
  open: boolean;
  onOpenChange: (open: boolean) => void;
  hotkeyEnabled: boolean;
  /** Additional graph controls that share the native React Flow control stack. */
  children?: ReactNode;
}

const NODE_COLORS: Record<string, string> = {
  Root:       '#15803d',
  End:        '#dc2626',
  Fetcher:    '#2563eb',
  mapper:     '#ea580c',
  Math:       '#a16207',
  JavaScript: '#7e22ce',
  Provider:   '#be185d',
  Dictionary: '#0e7490',
  Join:       '#65a30d',
  Extension:  '#4338ca',
  Island:     '#475569',
  Decision:   '#b45309',
};

interface IslandPosition {
  left:   number;
  bottom: number;
}

/**
 * The minimap is a floating island: grab the title bar to move it anywhere in
 * the graph pane, so it never has to cover a graph node. The default anchor
 * sits beside the control stack (15px panel margin + 26px controls + 9px gap),
 * and the position persists across sessions.
 */
const DEFAULT_ISLAND_POSITION: IslandPosition = { left: 50, bottom: 15 };
/** Minimum gap kept between the island and the pane edges when clamping. */
const ISLAND_EDGE_MARGIN = 8;
const ISLAND_POSITION_STORAGE_KEY = 'graph-minimap-position';

function minimapNodeColor(node: Node): string {
  return NODE_COLORS[node.type ?? ''] ?? '#6c7086';
}

function isEditableTarget(target: EventTarget | null): boolean {
  return target instanceof Element && target.closest(
    'input, textarea, select, [contenteditable]:not([contenteditable="false"])'
  ) !== null;
}

/**
 * Keep the island inside its positioned host (the React Flow pane). Layouts
 * without real dimensions (pre-layout mounts, happy-dom) are left unclamped.
 */
function clampIslandPosition(
  position: IslandPosition,
  host: HTMLElement,
  island: HTMLElement,
): IslandPosition {
  if (host.clientWidth <= 0 || host.clientHeight <= 0
    || island.offsetWidth <= 0 || island.offsetHeight <= 0) {
    return position;
  }
  const maxLeft   = Math.max(host.clientWidth  - island.offsetWidth  - ISLAND_EDGE_MARGIN, ISLAND_EDGE_MARGIN);
  const maxBottom = Math.max(host.clientHeight - island.offsetHeight - ISLAND_EDGE_MARGIN, ISLAND_EDGE_MARGIN);
  return {
    left:   Math.min(Math.max(position.left,   ISLAND_EDGE_MARGIN), maxLeft),
    bottom: Math.min(Math.max(position.bottom, ISLAND_EDGE_MARGIN), maxBottom),
  };
}

export default function GraphMinimap({
  open,
  onOpenChange,
  hotkeyEnabled,
  children,
}: GraphMinimapProps) {
  const toggleLabel = open ? 'Hide minimap' : 'Show minimap';

  const toggleMinimap = useCallback(() => {
    onOpenChange(!open);
  }, [onOpenChange, open]);

  useEffect(() => {
    if (!hotkeyEnabled) return;

    const handleKeyDown = (event: KeyboardEvent) => {
      if (
        event.defaultPrevented
        || event.repeat
        || !event.ctrlKey
        || event.metaKey
        || event.altKey
        || event.shiftKey
        || event.code !== 'KeyM'
        || isEditableTarget(event.target)
      ) {
        return;
      }

      event.preventDefault();
      toggleMinimap();
    };

    window.addEventListener('keydown', handleKeyDown);
    return () => window.removeEventListener('keydown', handleKeyDown);
  }, [hotkeyEnabled, toggleMinimap]);

  // ── Floating island position ──────────────────────────────────────────────
  const [islandPosition, setIslandPosition] = useLocalStorage<IslandPosition>(
    ISLAND_POSITION_STORAGE_KEY,
    DEFAULT_ISLAND_POSITION,
  );
  const islandRef = useRef<HTMLDivElement | null>(null);
  const [dragState, setDragState] = useState<{
    pointerId: number;
    startX:    number;
    startY:    number;
    origin:    IslandPosition;
  } | null>(null);

  const handleGripPointerDown = useCallback((event: ReactPointerEvent<HTMLDivElement>) => {
    if (event.pointerType === 'mouse' && event.button !== 0) return;
    event.preventDefault();
    setDragState({
      pointerId: event.pointerId,
      startX:    event.clientX,
      startY:    event.clientY,
      origin:    islandPosition,
    });
  }, [islandPosition]);

  // Window-level listeners track the drag even when the pointer leaves the
  // island; `bottom` grows upward, so the Y delta is subtracted.
  useEffect(() => {
    if (dragState === null) return;

    const handlePointerMove = (event: PointerEvent) => {
      if (event.pointerId !== dragState.pointerId) return;
      const island = islandRef.current;
      const host = island?.offsetParent instanceof HTMLElement ? island.offsetParent : null;
      const next = {
        left:   dragState.origin.left   + (event.clientX - dragState.startX),
        bottom: dragState.origin.bottom - (event.clientY - dragState.startY),
      };
      setIslandPosition(island && host ? clampIslandPosition(next, host, island) : next);
    };
    const endDrag = (event: PointerEvent) => {
      if (event.pointerId !== dragState.pointerId) return;
      setDragState(null);
    };

    window.addEventListener('pointermove', handlePointerMove);
    window.addEventListener('pointerup', endDrag);
    window.addEventListener('pointercancel', endDrag);
    return () => {
      window.removeEventListener('pointermove', handlePointerMove);
      window.removeEventListener('pointerup', endDrag);
      window.removeEventListener('pointercancel', endDrag);
    };
  }, [dragState, setIslandPosition]);

  // Keep the island reachable when the graph pane shrinks (panel toggles,
  // window resizes): re-clamp whenever the pane resizes while open.
  useEffect(() => {
    if (!open) return;
    const island = islandRef.current;
    const host = island?.offsetParent instanceof HTMLElement ? island.offsetParent : null;
    if (!island || !host) return;

    const clampNow = () => {
      setIslandPosition((current) => {
        const clamped = clampIslandPosition(current, host, island);
        return clamped.left === current.left && clamped.bottom === current.bottom
          ? current
          : clamped;
      });
    };
    clampNow();
    if (typeof ResizeObserver === 'undefined') return;
    const observer = new ResizeObserver(clampNow);
    observer.observe(host);
    return () => observer.disconnect();
  }, [open, setIslandPosition]);

  return (
    <>
      {open && (
        <div
          ref={islandRef}
          className={dragState !== null ? `${styles.island} ${styles.islandDragging}` : styles.island}
          style={{ left: islandPosition.left, bottom: islandPosition.bottom }}
          role="group"
          aria-label="Graph minimap"
        >
          <div
            className={styles.islandGrip}
            role="button"
            aria-label="Move minimap"
            title="Drag to move the minimap"
            onPointerDown={handleGripPointerDown}
          >
            <span className={styles.islandGripDots} aria-hidden="true">⠿</span>
            <span className={styles.islandGripLabel}>Minimap</span>
          </div>
          <MiniMap
            className={styles.minimap}
            nodeColor={minimapNodeColor}
            maskColor="rgba(0,0,0,0.3)"
            pannable
            style={{ position: 'relative', margin: 0, background: '#fff' }}
          />
        </div>
      )}
      <Controls
        position="bottom-left"
        showInteractive={false}
        className={styles.controls}
      >
        {children}
        <ControlButton
          className={`${styles.toggleButton} nodrag nopan`}
          aria-label={toggleLabel}
          aria-keyshortcuts="Control+M"
          aria-pressed={open}
          title={`${toggleLabel} (Ctrl + M)`}
          onClick={toggleMinimap}
        >
          <svg
            className={styles.toggleIcon}
            viewBox="1.5 2 17 16"
            fill="none"
            aria-hidden="true"
            focusable="false"
          >
            <rect x="2.5" y="3" width="15" height="14" rx="1.5" />
            <path d="M6 12.5 9 8l2.5 2 2.5-3" />
            <circle cx="6" cy="12.5" r="1" />
            <circle cx="9" cy="8" r="1" />
            <circle cx="11.5" cy="10" r="1" />
            <circle cx="14" cy="7" r="1" />
          </svg>
        </ControlButton>
      </Controls>
    </>
  );
}
