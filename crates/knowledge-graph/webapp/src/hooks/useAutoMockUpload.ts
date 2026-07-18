import { useEffect, useRef } from 'react';
import { type ProtocolBus } from '../protocol/bus';

export interface UseAutoMockUploadOptions {
  bus:         ProtocolBus;
  onOpenModal: (uploadPath: string) => void;
  modalOpen:   boolean;
}

/**
 * Subscribes to `upload.invitation` events on the ProtocolBus and
 * automatically calls `onOpenModal` with the extracted POST path.
 */
export function useAutoMockUpload({
  bus,
  onOpenModal,
  modalOpen,
}: UseAutoMockUploadOptions): void {

  const pendingModalRef = useRef(false);

  // Clear the guard when the modal closes
  useEffect(() => {
    if (!modalOpen) pendingModalRef.current = false;
  }, [modalOpen]);

  // Subscribe to upload.invitation events
  useEffect(() => {
    return bus.on('upload.invitation', (event) => {
      if (pendingModalRef.current) return;
      pendingModalRef.current = true;
      onOpenModal(event.uploadPath);
    });
  }, [bus, onOpenModal]);
}
