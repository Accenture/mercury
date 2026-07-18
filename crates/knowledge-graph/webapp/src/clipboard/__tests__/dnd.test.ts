import {
  hasClipboardItemType,
  MINIGRAPH_CLIPBOARD_ITEM_MIME,
  readClipboardItemId,
  writeClipboardItemDragData,
} from '../dnd';

class DataTransferStub {
  effectAllowed = 'none';
  private data = new Map<string, string>();

  get types(): string[] {
    return Array.from(this.data.keys());
  }

  setData(format: string, value: string): void {
    this.data.set(format, value);
  }

  getData(format: string): string {
    return this.data.get(format) ?? '';
  }
}

describe('clipboard dnd helpers', () => {
  it('detects only the exported clipboard mime type', () => {
    expect(hasClipboardItemType([])).toBe(false);
    expect(hasClipboardItemType(['text/plain'])).toBe(false);
    expect(hasClipboardItemType(['text/plain', MINIGRAPH_CLIPBOARD_ITEM_MIME])).toBe(true);
  });

  it('writes only the clipboard item id under the exported mime type and sets copy semantics', () => {
    const dataTransfer = new DataTransferStub();

    writeClipboardItemDragData(dataTransfer as unknown as DataTransfer, 'item-123');

    expect(dataTransfer.effectAllowed).toBe('copy');
    expect(dataTransfer.types).toEqual([MINIGRAPH_CLIPBOARD_ITEM_MIME]);
    expect(dataTransfer.getData(MINIGRAPH_CLIPBOARD_ITEM_MIME)).toBe('item-123');
    expect(dataTransfer.getData('text/plain')).toBe('');
  });

  it('reads the stored clipboard item id and returns null for missing or empty payloads', () => {
    const present = new DataTransferStub();
    writeClipboardItemDragData(present as unknown as DataTransfer, 'item-456');

    const missing = new DataTransferStub();
    const empty = new DataTransferStub();
    empty.setData(MINIGRAPH_CLIPBOARD_ITEM_MIME, '   ');

    expect(readClipboardItemId(present as unknown as DataTransfer)).toBe('item-456');
    expect(readClipboardItemId(missing as unknown as DataTransfer)).toBeNull();
    expect(readClipboardItemId(empty as unknown as DataTransfer)).toBeNull();
    expect(readClipboardItemId(null)).toBeNull();
  });
});