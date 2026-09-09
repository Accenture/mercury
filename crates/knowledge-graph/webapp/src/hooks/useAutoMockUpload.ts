import { useEffect } from 'react';
import { type ProtocolBus } from '../protocol/bus';

export interface UseAutoMockUploadOptions {
  bus:         ProtocolBus;
  onOpenPanel: (uploadPath: string) => void;
}

/**
 * Subscribes to `upload.invitation` events on the ProtocolBus and
 * automatically calls `onOpenPanel` with the extracted POST path.
 */
export function useAutoMockUpload({
  bus,
  onOpenPanel,
}: UseAutoMockUploadOptions): void {
  // Subscribe to upload.invitation events
  useEffect(() => {
    return bus.on('upload.invitation', (event) => {
      onOpenPanel(event.uploadPath);
    });
  }, [bus, onOpenPanel]);
}
