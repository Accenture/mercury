export const MINIGRAPH_CLIPBOARD_ITEM_MIME = 'application/x-minigraph-clipboard-item';

export function hasClipboardItemType(types: readonly string[]): boolean {
  return types.includes(MINIGRAPH_CLIPBOARD_ITEM_MIME);
}

export function writeClipboardItemDragData(
  dataTransfer: DataTransfer,
  itemId: string,
): void {
  dataTransfer.effectAllowed = 'copy';
  dataTransfer.setData(MINIGRAPH_CLIPBOARD_ITEM_MIME, itemId);
}

export function readClipboardItemId(
  dataTransfer: DataTransfer | null,
): string | null {
  const itemId = dataTransfer?.getData(MINIGRAPH_CLIPBOARD_ITEM_MIME) ?? '';
  return itemId.trim() ? itemId : null;
}