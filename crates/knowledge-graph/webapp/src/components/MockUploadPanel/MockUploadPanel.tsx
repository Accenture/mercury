import { useState, useEffect, useRef, useCallback } from 'react';
import styles from './MockUploadPanel.module.css';
import { tryParseJSON } from '../../utils/messageParser';
import { formatJSON } from '../../utils/validators';
import { useMockUpload } from '../../hooks/useMockUpload';
import CloseIcon from '../../icons/CloseIcon.svg?react';

interface MockUploadPanelProps {
  /** The POST path extracted from the server message, e.g. "/api/mock/ws-417669-24" */
  uploadPath: string;
  /** Called with the drained response body and owning path on a 2xx response. */
  onSuccess: (responseBody: string, uploadPath: string) => void;
  /** Called with the owning path to close the panel. Playground restores focus. */
  onClose: (uploadPath: string) => void;
  /** Called with a human-readable error string on failure. */
  onError: (errorMessage: string) => void;
  /** Optional workflow-specific title. Defaults to the manual mock-upload title. */
  title?: string;
  /** Optional context shown above the JSON editor. */
  description?: string;
  /** Derived graph paths shown as non-authoritative input hints. */
  inputPathHints?: string[];
  /** Submit action label. Defaults to the existing Upload action. */
  submitLabel?: string;
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

/**
 * Mock-data upload form rendered in the left panel slot (the console's
 * space) instead of a modal — the same in-place pattern as NodeEditPanel:
 * Esc / Cancel / a successful upload closes the session and the slot
 * returns to whatever it held before (console back if it was open,
 * full-width graph if hidden). consoleOpen itself is never touched.
 *
 * Serves both entry points: the manual re-open button on a console
 * invitation row, and the graph-run workflow's "Mock Graph Input" step
 * (title / description / hints / submit label come in as props).
 */
export default function MockUploadPanel({
  uploadPath,
  onSuccess,
  onClose,
  onError,
  title = '⬆️ Upload Mock Data',
  description,
  inputPathHints = [],
  submitLabel = 'Upload',
}: MockUploadPanelProps) {
  const [json,        setJson]        = useState('');
  const [uploadError, setUploadError] = useState<string | null>(null);
  const [fileError,   setFileError]   = useState<string | null>(null);
  const [isDragOver,  setIsDragOver]  = useState(false);

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
    onSuccess: (responseBody) => onSuccess(responseBody, uploadPath),
    onError: (msg) => {
      setUploadError(msg);   // inline error banner
      onError(msg);          // Playground fires a toast
    },
  });

  // ── Close handling ───────────────────────────────────────────────────────
  const handleClose = useCallback(() => {
    cancel();
    onClose(uploadPath); // Playground unmounts the component and restores focus.
  }, [cancel, onClose, uploadPath]);

  // Focus the editor on mount; Escape closes the panel and restores the
  // previous left-panel content (same contract as NodeEditPanel). An upload
  // in flight blocks Esc, exactly like the disabled Cancel button.
  const isUploadingRef = useRef(isUploading);
  useEffect(() => { isUploadingRef.current = isUploading; }, [isUploading]);
  useEffect(() => {
    textareaRef.current?.focus();
    const handleKeyDown = (event: KeyboardEvent) => {
      if (event.key !== 'Escape') return;
      event.preventDefault();
      if (!isUploadingRef.current) handleClose();
    };
    document.addEventListener('keydown', handleKeyDown);
    return () => document.removeEventListener('keydown', handleKeyDown);
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
    <div className={styles.root}>
      <section className={styles.card} aria-label={title}>

        {/* ── Header ribbon ──────────────────────────────────────────── */}
        <header className={styles.ribbon}>
          <div className={styles.titleGroup}>
            <span className={styles.title}>{title}</span>
            <span className={styles.path}>{uploadPath}</span>
          </div>
          <button
            className={styles.ribbonClose}
            onClick={handleClose}
            aria-label="Close upload panel"
            title="Close (Esc)"
            disabled={isUploading}
          >
            <CloseIcon className={styles.ribbonCloseIcon} aria-hidden="true" focusable="false" />
          </button>
        </header>

        {/* ── Body ───────────────────────────────────────────────────── */}
        <div className={styles.body}>

          {description && <p className={styles.description}>{description}</p>}

          {inputPathHints.length > 0 && (
            <div className={styles.inputHints} aria-label="Referenced graph input paths">
              <span className={styles.inputHintsLabel}>Referenced input paths</span>
              <div className={styles.inputHintList}>
                {inputPathHints.slice(0, 6).map(path => <code key={path}>{path}</code>)}
                {inputPathHints.length > 6 && (
                  <span className={styles.moreHints}>+{inputPathHints.length - 6} more</span>
                )}
              </div>
              <span className={styles.inputHintsNote}>Hints are derived from graph references.</span>
            </div>
          )}

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
            {isMac ? `⌘+Enter to ${submitLabel.toLowerCase()}` : `Ctrl+Enter to ${submitLabel.toLowerCase()}`}
          </span>
          {uploadError && (
            <div className={styles.errorBanner} role="alert">
              ❌ Upload failed: {uploadError}
            </div>
          )}
        </div>

        {/* ── Footer ─────────────────────────────────────────────────── */}
        <footer className={styles.footer}>
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
                submitLabel === 'Upload' ? 'Upload ▶' : submitLabel
              )}
            </button>
          </div>
        </footer>

      </section>
    </div>
  );
}
