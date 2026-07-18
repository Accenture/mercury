import { useEffect, useMemo, useState } from 'react';
import { useClipboardContext } from '../../contexts/ClipboardContext';
import type { ClipboardItemRecord } from '../../clipboard/db';
import { ClipboardItem } from './ClipboardItem';
import { ClipboardItemContextMenu } from './ClipboardItemContextMenu';
import { ClipboardEmptyState } from './ClipboardEmptyState';
import styles from './ClipboardSidebar.module.css';
import { JsonView, darkStyles } from 'react-json-view-lite';
import 'react-json-view-lite/dist/index.css';

interface ClipboardSidebarProps {
  connected: boolean;
  onPasteToInput: (item: ClipboardItemRecord) => void;
}

type ActiveClipboardMenuState = {
  itemId: string;
  x: number;
  y: number;
};

export default function ClipboardSidebar({ connected, onPasteToInput }: ClipboardSidebarProps) {
  const clipboardCtx = useClipboardContext();
  const [inspectItem, setInspectItem] = useState<ClipboardItemRecord | null>(null);
  const [activeItemMenu, setActiveItemMenu] = useState<ActiveClipboardMenuState | null>(null);

  const handleOpenItemMenu = (itemId: string, x: number, y: number) => {
    setActiveItemMenu({ itemId, x, y });
  };

  const handleCloseItemMenu = () => {
    setActiveItemMenu(null);
  };

  const handlePasteToInputFromMenu = (item: ClipboardItemRecord) => {
    handleCloseItemMenu();
    onPasteToInput(item);
  };

  const handleInspect = (item: ClipboardItemRecord) => {
    handleCloseItemMenu();
    setInspectItem(current => (current?.id === item.id ? null : item));
  };

  const handleRemove = (itemId: string) => {
    handleCloseItemMenu();
    setInspectItem(current => (current?.id === itemId ? null : current));
    void clipboardCtx.removeItem(itemId);
  };

  const handleClearAll = () => {
    handleCloseItemMenu();
    setInspectItem(null);
    void clipboardCtx.clearAll();
  };

  useEffect(() => {
    const itemIds = new Set(clipboardCtx.items.map(item => item.id));

    if (activeItemMenu && !itemIds.has(activeItemMenu.itemId)) {
      setActiveItemMenu(null);
    }

    if (inspectItem && !itemIds.has(inspectItem.id)) {
      setInspectItem(null);
    }
  }, [clipboardCtx.items, activeItemMenu, inspectItem]);

  const activeMenuItem = useMemo(
    () => (activeItemMenu
      ? clipboardCtx.items.find(item => item.id === activeItemMenu.itemId) ?? null
      : null),
    [activeItemMenu, clipboardCtx.items],
  );

  return (
    <div className={styles.sidebar}>
      <div className={styles.header}>
        <span className={styles.headerTitle}>Workspace</span>
        {clipboardCtx.items.length > 0 && (
          <button
            className={styles.clearBtn}
            onClick={handleClearAll}
            aria-label="Clear all workspace items"
          >
            Clear
          </button>
        )}
      </div>

      <div className={styles.itemList}>
        {clipboardCtx.isLoading ? (
          <div className={styles.loading}>Loading…</div>
        ) : clipboardCtx.items.length === 0 ? (
          <ClipboardEmptyState />
        ) : (
          clipboardCtx.items.map(item => (
            <ClipboardItem
              key={item.id}
              item={item}
              onRemove={handleRemove}
              onOpenMenu={handleOpenItemMenu}
              onCloseMenu={handleCloseItemMenu}
            />
          ))
        )}
      </div>

      {/* Inspect panel (inline expand) */}
      {inspectItem && (
        <div className={styles.inspectPanel}>
          <div className={styles.inspectHeader}>
            <span>Inspect node {inspectItem.node.alias}</span>
            <button
              className={styles.inspectClose}
              onClick={() => setInspectItem(null)}
              aria-label="Close inspect panel"
            >
              ✕
            </button>
          </div>
          <div className={styles.inspectBody}>
            <JsonView
              data={{ node: inspectItem.node, connections: inspectItem.connections }}
              style={darkStyles}
            />
          </div>
        </div>
      )}

      {activeItemMenu && activeMenuItem && (
        <ClipboardItemContextMenu
          open={true}
          x={activeItemMenu.x}
          y={activeItemMenu.y}
          canPasteToInput={connected}
          onPasteToInput={() => handlePasteToInputFromMenu(activeMenuItem)}
          onInspect={() => handleInspect(activeMenuItem)}
          onClose={handleCloseItemMenu}
        />
      )}
    </div>
  );
}
