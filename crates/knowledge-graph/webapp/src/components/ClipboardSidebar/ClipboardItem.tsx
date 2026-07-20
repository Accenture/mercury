import type { ClipboardItemRecord } from '../../clipboard/db';
import { writeClipboardItemDragData } from '../../clipboard/dnd';
import CloseIcon from '../../icons/CloseIcon.svg?react';
import { MinigraphNodeBody } from '../GraphView/MinigraphNodeBody';
import { getMinigraphNodeShellStyle } from '../../utils/minigraphNodeTheme';
import { formatRelativeTime } from '../../utils/timeFormat';
import styles from './ClipboardItem.module.css';

interface ClipboardItemProps {
  item: ClipboardItemRecord;
  onRemove: (itemId: string) => void;
  onOpenMenu: (itemId: string, x: number, y: number) => void;
  onCloseMenu: () => void;
}

export function ClipboardItem({
  item,
  onRemove,
  onOpenMenu,
  onCloseMenu,
}: ClipboardItemProps) {
  const { node, clippedAt, sourceLabel } = item;

  const handleDragStart = (event: React.DragEvent<HTMLDivElement>) => {
    onCloseMenu();
    writeClipboardItemDragData(event.dataTransfer, item.id);
  };

  const handleContextMenu = (event: React.MouseEvent<HTMLDivElement>) => {
    event.preventDefault();
    onOpenMenu(item.id, event.clientX, event.clientY);
  };

  const handleKeyDown = (event: React.KeyboardEvent<HTMLDivElement>) => {
    if (event.key === 'ContextMenu' || (event.key === 'F10' && event.shiftKey)) {
      event.preventDefault();
      const rect = event.currentTarget.getBoundingClientRect();
      onOpenMenu(item.id, Math.round(rect.left + 8), Math.round(rect.top + 8));
    }
  };

  return (
    <div className={styles.item}>
      <div className={styles.previewFrame}>
        <button
          type="button"
          className={styles.removeChrome}
          draggable={false}
          aria-label={`Remove node ${node.alias} from clipboard`}
          onClick={(event) => {
            event.stopPropagation();
            onCloseMenu();
            onRemove(item.id);
          }}
        >
          <CloseIcon className={styles.removeIcon} aria-hidden="true" focusable="false" />
        </button>

        <div
          className={styles.preview}
          role="group"
          draggable={true}
          onDragStart={handleDragStart}
          onContextMenu={handleContextMenu}
          onKeyDown={handleKeyDown}
          tabIndex={0}
          aria-label={`Drag node ${node.alias} into the graph to paste`}
        >
          <div
            className={styles.previewShell}
            style={getMinigraphNodeShellStyle(node.types[0] ?? 'unknown')}
          >
            <MinigraphNodeBody
              alias={node.alias}
              nodeType={node.types[0] ?? 'unknown'}
              properties={node.properties}
            />
          </div>
        </div>
      </div>

      <div className={styles.metaBlock}>
        <div className={styles.timestamp}>
          Clipped {formatRelativeTime(clippedAt)} from {sourceLabel}
        </div>
      </div>
    </div>
  );
}
