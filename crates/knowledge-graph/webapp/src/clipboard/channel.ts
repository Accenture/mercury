import type { ClipboardItemRecord } from './db';

const CHANNEL_NAME = 'minigraph-clipboard-sync';

/**
 * Discriminated union of all messages sent over the clipboard BroadcastChannel.
 */
export type ClipboardChannelMessage =
  | { type: 'item-added';    item: ClipboardItemRecord }
  | { type: 'item-replaced'; item: ClipboardItemRecord; previousId: string }
  | { type: 'item-removed';  id: string }
  | { type: 'items-cleared' };

/** Create a BroadcastChannel for clipboard synchronisation. */
export function createClipboardChannel(): BroadcastChannel {
  return new BroadcastChannel(CHANNEL_NAME);
}
