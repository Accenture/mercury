import { useEffect, useId, useMemo, useRef, useState } from 'react';
import { useClipboardContext } from '../../contexts/ClipboardContext';
import type { ClipboardItemRecord } from '../../clipboard/db';
import {
  getDefaultClipboardSortDirection,
  sortClipboardItems,
  type ClipboardSortDirection,
  type ClipboardSortField,
} from '../../clipboard/sortItems';
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

const SORT_FIELD_OPTIONS: Array<{ value: ClipboardSortField; label: string }> = [
  { value: 'recent', label: 'Recent' },
  { value: 'type', label: 'Type' },
  { value: 'alias', label: 'Alias' },
  { value: 'source', label: 'Source' },
  { value: 'connections', label: 'Connections' },
  { value: 'property', label: 'Property' },
];

const SORT_DIRECTION_OPTIONS: Array<{ value: ClipboardSortDirection; label: string }> = [
  { value: 'ascending', label: 'Ascending' },
  { value: 'descending', label: 'Descending' },
];

const SORT_FIELD_LABELS = SORT_FIELD_OPTIONS.reduce<Record<ClipboardSortField, string>>(
  (labels, option) => ({ ...labels, [option.value]: option.label }),
  {} as Record<ClipboardSortField, string>,
);

export default function ClipboardSidebar({ connected, onPasteToInput }: ClipboardSidebarProps) {
  const sortMenuId = useId();
  const propertyInputId = useId();
  const sortMenuRef = useRef<HTMLDivElement | null>(null);
  const clipboardCtx = useClipboardContext();
  const [inspectItem, setInspectItem] = useState<ClipboardItemRecord | null>(null);
  const [activeItemMenu, setActiveItemMenu] = useState<ActiveClipboardMenuState | null>(null);
  const [sortField, setSortField] = useState<ClipboardSortField>('recent');
  const [sortDirection, setSortDirection] = useState<ClipboardSortDirection>(
    getDefaultClipboardSortDirection('recent'),
  );
  const [sortPropertyKey, setSortPropertyKey] = useState('');
  const [sortMenuOpen, setSortMenuOpen] = useState(false);

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

  const handleSortFieldChange = (field: ClipboardSortField) => {
    setSortField(field);
    setSortDirection(getDefaultClipboardSortDirection(field));
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

  useEffect(() => {
    if (!sortMenuOpen) return;

    const handlePointerDown = (event: PointerEvent) => {
      if (sortMenuRef.current?.contains(event.target as Node)) return;
      setSortMenuOpen(false);
    };
    const handleKeyDown = (event: KeyboardEvent) => {
      if (event.key === 'Escape') setSortMenuOpen(false);
    };

    document.addEventListener('pointerdown', handlePointerDown);
    document.addEventListener('keydown', handleKeyDown);
    return () => {
      document.removeEventListener('pointerdown', handlePointerDown);
      document.removeEventListener('keydown', handleKeyDown);
    };
  }, [sortMenuOpen]);

  const activeMenuItem = useMemo(
    () => (activeItemMenu
      ? clipboardCtx.items.find(item => item.id === activeItemMenu.itemId) ?? null
      : null),
    [activeItemMenu, clipboardCtx.items],
  );
  const visibleItems = useMemo(
    () => sortClipboardItems(clipboardCtx.items, {
      field: sortField,
      direction: sortDirection,
      propertyKey: sortPropertyKey,
    }),
    [clipboardCtx.items, sortDirection, sortField, sortPropertyKey],
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

      {clipboardCtx.items.length > 0 && (
        <div className={styles.sortBar}>
          <div className={styles.sortMenuWrapper} ref={sortMenuRef}>
            <button
              type="button"
              className={styles.sortMenuButton}
              onClick={() => setSortMenuOpen(open => !open)}
              aria-expanded={sortMenuOpen}
              aria-controls={sortMenuId}
            >
              <span className={styles.sortButtonLabel}>Sort</span>
              <span className={styles.sortButtonValue}>{SORT_FIELD_LABELS[sortField]}</span>
              <span className={styles.sortButtonDirection}>
                {sortDirection === 'ascending' ? 'Asc' : 'Desc'}
              </span>
              <span
                className={`${styles.sortButtonCaret}${sortMenuOpen ? ` ${styles.sortButtonCaretOpen}` : ''}`}
                aria-hidden="true"
              >
                ▾
              </span>
            </button>

            {sortMenuOpen && (
              <div id={sortMenuId} className={styles.sortPopover}>
                <div
                  className={styles.sortGroup}
                  role="group"
                  aria-labelledby={`${sortMenuId}-field-title`}
                >
                  <div id={`${sortMenuId}-field-title`} className={styles.sortGroupTitle}>Sort By</div>
                  {SORT_FIELD_OPTIONS.map(option => (
                    <label key={option.value} className={styles.sortOption}>
                      <input
                        type="radio"
                        name={`${sortMenuId}-field`}
                        value={option.value}
                        checked={sortField === option.value}
                        onChange={() => handleSortFieldChange(option.value)}
                      />
                      <span>{option.label}</span>
                    </label>
                  ))}
                </div>

                {sortField === 'property' && (
                  <div className={styles.propertySortRow}>
                    <label className={styles.propertyLabel} htmlFor={propertyInputId}>Property Key</label>
                    <input
                      id={propertyInputId}
                      className={styles.propertyInput}
                      value={sortPropertyKey}
                      onChange={(event) => setSortPropertyKey(event.target.value)}
                      placeholder="skill"
                      aria-label="Property key to sort by"
                    />
                  </div>
                )}

                <div
                  className={styles.sortGroup}
                  role="group"
                  aria-labelledby={`${sortMenuId}-direction-title`}
                >
                  <div id={`${sortMenuId}-direction-title`} className={styles.sortGroupTitle}>Sort Direction</div>
                  {SORT_DIRECTION_OPTIONS.map(option => (
                    <label key={option.value} className={styles.sortOption}>
                      <input
                        type="radio"
                        name={`${sortMenuId}-direction`}
                        value={option.value}
                        checked={sortDirection === option.value}
                        onChange={() => setSortDirection(option.value)}
                      />
                      <span>{option.label}</span>
                    </label>
                  ))}
                </div>
              </div>
            )}
          </div>
        </div>
      )}

      <div className={styles.itemList}>
        {clipboardCtx.isLoading ? (
          <div className={styles.loading}>Loading…</div>
        ) : clipboardCtx.items.length === 0 ? (
          <ClipboardEmptyState />
        ) : (
          visibleItems.map(item => (
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
