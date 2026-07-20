import { useState, useRef, useCallback } from 'react';
import { type ProtocolBus } from '../protocol/bus';
import { type ToastType } from './useToast';
import { useAutoMockUpload } from './useAutoMockUpload';

export interface UseMockUploadModalOptions {
  bus:      ProtocolBus;
  addToast: (message: string, type?: ToastType) => void;
}

export interface UseMockUploadModalReturn {
  /** null = modal closed; non-null = modal open for that endpoint path. */
  modalUploadPath:        string | null;
  /** Set of POST paths that have been successfully uploaded (drives badge). */
  successfulUploadPaths:  Set<string>;
  /** Open the upload modal for a specific endpoint path. */
  handleOpenUploadModal:  (path: string) => void;
  /** Close the modal and restore focus. */
  handleCloseUploadModal: () => void;
  /** Mark a path as successfully uploaded, close modal, restore focus, toast. */
  handleUploadSuccess:    (responseBody: string) => void;
  /** Show an error toast (modal stays open). */
  handleUploadError:      (errorMessage: string) => void;
  /** Reset successful paths (called by handleClearMessages). */
  resetSuccessfulPaths:   () => void;
}

/**
 * Manages the mock-upload modal lifecycle: open/close state, focus
 * restoration, success/error handling, and the auto-open subscription
 * via the ProtocolBus.
 *
 * Composes `useAutoMockUpload` internally — callers do not need to
 * invoke that hook separately.
 */
export function useMockUploadModal({
  bus,
  addToast,
}: UseMockUploadModalOptions): UseMockUploadModalReturn {
  // Path extracted from the server's upload invitation.
  // null = modal closed; non-null = modal open for that specific endpoint.
  const [modalUploadPath, setModalUploadPath] = useState<string | null>(null);

  // Capture the element that triggered the modal so focus can be restored on close.
  const modalTriggerRef = useRef<HTMLElement | null>(null);

  // POST paths of invitations that have been successfully fulfilled — drives badge.
  const [successfulUploadPaths, setSuccessfulUploadPaths] = useState<Set<string>>(new Set());

  const handleOpenUploadModal = useCallback((path: string) => {
    // Capture the focused element before opening so we can restore focus on close.
    modalTriggerRef.current = document.activeElement as HTMLElement;
    setModalUploadPath(path);
  }, []);

  const handleCloseUploadModal = useCallback(() => {
    setModalUploadPath(null);
    // Restore focus to the element that triggered the modal open.
    // setTimeout ensures the dialog is fully unmounted before focus() runs.
    setTimeout(() => modalTriggerRef.current?.focus(), 0);
  }, []);

  const handleUploadSuccess = useCallback((_responseBody: string) => {
    // _responseBody is available but intentionally not surfaced per spec §2.
    setSuccessfulUploadPaths(prev => new Set([...prev, modalUploadPath!]));
    setModalUploadPath(null);
    setTimeout(() => modalTriggerRef.current?.focus(), 0);
    addToast('Mock data uploaded successfully ✓', 'success');
  }, [modalUploadPath, addToast]);

  const handleUploadError = useCallback((errorMessage: string) => {
    // Modal stays open — error is displayed inline inside the modal.
    addToast(`Upload failed: ${errorMessage}`, 'error');
  }, [addToast]);

  const resetSuccessfulPaths = useCallback(() => {
    setSuccessfulUploadPaths(new Set());
  }, []);

  // Auto-open modal when server sends upload invitation.
  useAutoMockUpload({
    bus,
    onOpenModal: handleOpenUploadModal,
    modalOpen:   modalUploadPath !== null,
  });

  return {
    modalUploadPath,
    successfulUploadPaths,
    handleOpenUploadModal,
    handleCloseUploadModal,
    handleUploadSuccess,
    handleUploadError,
    resetSuccessfulPaths,
  };
}
