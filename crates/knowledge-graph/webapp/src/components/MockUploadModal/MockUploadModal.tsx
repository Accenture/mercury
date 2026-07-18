import { useState, useEffect, useRef, useCallback } from 'react';
import styles from './MockUploadModal.module.css';
import { tryParseJSON } from '../../utils/messageParser';
import { formatJSON } from '../../utils/validators';
import { useMockUpload } from '../../hooks/useMockUpload';

interface MockUploadModalProps {
  /** The POST path extracted from the server message, e.g. "/api/mock/ws-417669-24" */
  uploadPath: string;
  /** Called (with the drained response body) on a 2xx response. */
  onSuccess: (responseBody: string) => void;
  /** Called to close the modal. Playground restores focus. */
  onClose: () => void;
  /** Called with a human-readable error string on failure. */
  onError: (errorMessage: string) => void;
}

// Derive macOS status once — no hook needed; navigator APIs are synchronous.
// navigator.userAgentData?.platform is preferred (Chromium-based browsers);
// navigator.platform is the deprecated-but-universal fallback (Firefox, Safari).
const isMac =
  ((navigator as Navigator & { userAgentData?: { platform: string } }).userAgentData?.platform
    ?? navigator.platform)
    .toLowerCase()
    .includes('mac');

/** Read a File as text, resolving with the string or rejecting with an Error. */
function readFileAsText(file: File): Promise<string> {
  return new Promise((resolve, reject) => {
    const reader = new FileReader();
    reader.onload  = () => resolve(reader.result as string);
    reader.onerror = () => reject(new Error(`Could not read file "${file.name}"`));
    reader.readAsText(file, 'utf-8');
  });
}

/**
 * Validate that a dropped / selected file is acceptable:
 *  - Must have a `.json` extension OR a `application/json` MIME type.
 * Returns null on success, or a human-readable error string on failure.
 */
function validateFileType(file: File): string | null {
  const hasJsonExt  = file.name.toLowerCase().endsWith('.json');
  const hasJsonMime = file.type === 'application/json' || file.type === 'text/plain';
  if (!hasJsonExt && !hasJsonMime) {
    return `"${file.name}" does not appear to be a JSON file. Only .json files are accepted.`;
  }
  return null;
}

export function MockUploadModal({ uploadPath, onSuccess, onClose, onError }: MockUploadModalProps) {
  const [json,        setJson]        = useState('');
  const [uploadError, setUploadError] = useState<string | null>(null);
  const [fileError,   setFileError]   = useState<string | null>(null);
  const [isDragOver,  setIsDragOver]  = useState(false);

  const dialogRef   = useRef<HTMLDialogElement>(null);
  const textareaRef = useRef<HTMLTextAreaElement>(null);
  const fileInputRef = useRef<HTMLInputElement>(null);

  // ── Validation ──────────────────────────────────────────────────────────
  // Uses tryParseJSON directly — not validatePayload — because the mock endpoint
  // is JSON-only. tryParseJSON returns isJSON: false for JSON primitives, which
  // is intentional: the endpoint expects an object or array.
  const jsonResult  = tryParseJSON(json);
  const isValidJson = jsonResult.isJSON;
  const canSubmit   = isValidJson && json.trim() !== '';

  // ── useMockUpload ────────────────────────────────────────────────────────
  const { isUploading, upload, cancel } = useMockUpload({
    uploadPath,
    json,
    onSuccess,
    onError: (msg) => {
      setUploadError(msg);   // inline error banner
      onError(msg);          // Playground fires a toast
    },
  });

  // ── Open the native dialog on mount ─────────────────────────────────────
  useEffect(() => {
    const dialog = dialogRef.current;
    if (!dialog) return;
    if (!dialog.open) {
      dialog.showModal();
    }
    // Focus the textarea after the dialog is shown
    textareaRef.current?.focus();

    return () => {
      // Gracefully close the dialog on unmount to avoid "dialog already closed" warnings.
      if (dialog.open) {
        dialog.close();
      }
    };
  }, []);

  // ── Close handling ───────────────────────────────────────────────────────
  const handleClose = useCallback(() => {
    cancel();
    onClose(); // Playground unmounts the component and restores focus.
  }, [cancel, onClose]);

  // Distinguish backdrop click (target === dialog element) from inner content.
  const handleBackdropClick = useCallback((e: React.MouseEvent<HTMLDialogElement>) => {
    if (e.target === dialogRef.current) {
      handleClose();
    }
  }, [handleClose]);

  // Browser fires `cancel` event on Escape — delegate to our close handler.
  const handleCancel = useCallback((e: React.SyntheticEvent<HTMLDialogElement>) => {
    e.preventDefault(); // prevent automatic dialog.close() — we control unmounting
    handleClose();
  }, [handleClose]);

  // ── Upload handling ──────────────────────────────────────────────────────
  const handleUpload = useCallback(() => {
    setUploadError(null); // clear stale error from previous attempt
    upload();
  }, [upload]);

  const handleTextareaKeyDown = useCallback((e: React.KeyboardEvent<HTMLTextAreaElement>) => {
    if (e.key === 'Enter' && (e.ctrlKey || e.metaKey)) {
      e.preventDefault();
      if (canSubmit && !isUploading) {
        handleUpload();
      }
    }
  }, [canSubmit, isUploading, handleUpload]);

  const handleFormat = useCallback(() => {
    if (!isValidJson) return;
    setJson(formatJSON(json));
  }, [isValidJson, json]);

  // ── File loading (shared by drop and file-picker) ────────────────────────
  const loadFile = useCallback(async (file: File) => {
    setFileError(null);
    setUploadError(null);

    const typeError = validateFileType(file);
    if (typeError) {
      setFileError(typeError);
      return;
    }

    try {
      const text = await readFileAsText(file);
      // Validate it parses as a JSON object/array before loading into textarea.
      const result = tryParseJSON(text);
      if (!result.isJSON) {
        setFileError(`"${file.name}" contains invalid JSON.`);
        return;
      }
      setJson(formatJSON(text));   // pretty-print on load
      textareaRef.current?.focus();
    } catch (err) {
      setFileError((err as Error).message);
    }
  }, []);

  // ── Drag-and-drop handlers ────────────────────────────────────────────────
  const handleDragOver = useCallback((e: React.DragEvent<HTMLDivElement>) => {
    e.preventDefault();
    e.stopPropagation();
    if (!isDragOver) setIsDragOver(true);
  }, [isDragOver]);

  const handleDragLeave = useCallback((e: React.DragEvent<HTMLDivElement>) => {
    e.preventDefault();
    e.stopPropagation();
    // Only clear if leaving the drop zone itself, not a child element.
    if (e.currentTarget === e.target || !e.currentTarget.contains(e.relatedTarget as Node)) {
      setIsDragOver(false);
    }
  }, []);

  const handleDrop = useCallback((e: React.DragEvent<HTMLDivElement>) => {
    e.preventDefault();
    e.stopPropagation();
    setIsDragOver(false);

    const file = e.dataTransfer.files[0];
    if (!file) return;
    loadFile(file);
  }, [loadFile]);

  // ── File-picker handler ───────────────────────────────────────────────────
  const handleFileInputChange = useCallback((e: React.ChangeEvent<HTMLInputElement>) => {
    const file = e.target.files?.[0];
    if (!file) return;
    loadFile(file);
    // Reset the input so the same file can be re-selected after a fix.
    e.target.value = '';
  }, [loadFile]);

  const showValidationError = !isValidJson && json.trim() !== '';

  return (
    <dialog
      ref={dialogRef}
      className={styles.dialog}
      aria-modal="true"
      aria-labelledby="mock-upload-modal-title"
      onClick={handleBackdropClick}
      onCancel={handleCancel}
    >
      <div className={styles.modalInner} onClick={(e) => e.stopPropagation()}>

        {/* ── Header ─────────────────────────────────────────────────── */}
        <div className={styles.modalHeader}>
          <div className={styles.modalTitleGroup}>
            <span id="mock-upload-modal-title" className={styles.modalTitle}>
              ⬆️ Upload Mock Data
            </span>
            <span className={styles.modalPath}>{uploadPath}</span>
          </div>
          <button
            className={styles.closeButton}
            onClick={handleClose}
            aria-label="Close upload modal"
            title="Close"
            disabled={isUploading}
          >
            ✕
          </button>
        </div>

        {/* ── Body ───────────────────────────────────────────────────── */}
        <div className={styles.modalBody}>

          {/* ── Drop zone ──────────────────────────────────────────── */}
          <div
            className={`${styles.dropZone} ${isDragOver ? styles.dropZoneActive : ''}`}
            onDragOver={handleDragOver}
            onDragLeave={handleDragLeave}
            onDrop={handleDrop}
            aria-label="Drop a JSON file here"
          >
            <span className={styles.dropZoneIcon}>📂</span>
            <span className={styles.dropZoneText}>
              Drop a <code>.json</code> file here
            </span>
            <span className={styles.dropZoneOr}>— or —</span>
            {/* Hidden file input, triggered by the visible button below */}
            <input
              ref={fileInputRef}
              type="file"
              accept=".json,application/json"
              className={styles.fileInputHidden}
              aria-hidden="true"
              tabIndex={-1}
              onChange={handleFileInputChange}
            />
            <button
              type="button"
              className={styles.browseButton}
              onClick={() => fileInputRef.current?.click()}
              disabled={isUploading}
              aria-label="Browse for a JSON file"
            >
              Browse file…
            </button>
          </div>

          {fileError && (
            <span className={styles.fileError} role="alert">
              ⚠️ {fileError}
            </span>
          )}

          <label htmlFor="mock-upload-textarea" className={styles.textareaLabel}>
            JSON Payload
          </label>
          <textarea
            id="mock-upload-textarea"
            ref={textareaRef}
            className={styles.textarea}
            value={json}
            onChange={(e) => { setJson(e.target.value); setFileError(null); }}
            onKeyDown={handleTextareaKeyDown}
            placeholder='Paste JSON here, or drop / browse a .json file above'
            rows={10}
            spellCheck={false}
            aria-describedby={showValidationError ? 'mock-upload-validation' : undefined}
          />
          {showValidationError && (
            <span
              id="mock-upload-validation"
              className={styles.validationError}
              role="status"
            >
              ⚠️ Invalid JSON — check syntax
            </span>
          )}
          <span className={styles.keyboardHint}>
            {isMac ? '⌘+Enter to upload' : 'Ctrl+Enter to upload'}
          </span>
          {uploadError && (
            <div className={styles.errorBanner} role="alert">
              ❌ Upload failed: {uploadError}
            </div>
          )}
        </div>

        {/* ── Footer ─────────────────────────────────────────────────── */}
        <div className={styles.modalFooter}>
          <button
            className={styles.formatButton}
            onClick={handleFormat}
            disabled={!isValidJson || isUploading}
            title="Format JSON"
            aria-label="Format JSON"
          >
            Format
          </button>
          <div className={styles.footerActions}>
            <button
              className={styles.cancelButton}
              onClick={handleClose}
              disabled={isUploading}
            >
              Cancel
            </button>
            <button
              className={styles.uploadButton}
              onClick={handleUpload}
              disabled={!canSubmit || isUploading}
              aria-busy={isUploading}
            >
              {isUploading ? (
                <><span className={styles.spinner} aria-hidden="true" /> Uploading…</>
              ) : (
                'Upload ▶'
              )}
            </button>
          </div>
        </div>

      </div>
    </dialog>
  );
}

