import { useState, useRef, useCallback } from 'react';

export interface UseMockUploadOptions {
  /** The POST path extracted from the server message, e.g. "/api/mock/ws-417669-24" */
  uploadPath: string;
  /** The current textarea value — the JSON string to POST. */
  json: string;
  /** Called with the response body text on a successful (2xx) upload. */
  onSuccess: (responseBody: string) => void;
  /** Called with a human-readable error message on failure. */
  onError:   (message: string) => void;
}

export interface UseMockUploadReturn {
  /** True while a fetch is in-flight. */
  isUploading: boolean;
  /** Initiate the POST request with the current `json` value. */
  upload: () => void;
  /** Abort any in-flight request. */
  cancel: () => void;
}

/**
 * Owns the fetch lifecycle for a mock-data upload POST request.
 *
 * Keeping this logic out of the modal component means MockUploadModal is a
 * pure renderer and this hook is independently testable.
 *
 * Design decisions:
 *  - AbortController per upload attempt — a new controller is created each
 *    time `upload()` is called, cancelling the previous one if still running.
 *  - On success (2xx): calls `onSuccess` with the drained response body.
 *    The body is available to the caller but the spec mandates it is not
 *    surfaced in the console or toast — that decision belongs to Playground.tsx.
 *  - On failure (non-2xx or network error): calls `onError` with a message
 *    of the form "HTTP 400 — <server response body>" or the caught error message.
 *  - `isUploading` is set false before calling callbacks so the modal can
 *    re-enable its button immediately on error (for retry) or unmount on success.
 */
export function useMockUpload({
  uploadPath,
  json,
  onSuccess,
  onError,
}: UseMockUploadOptions): UseMockUploadReturn {

  const [isUploading, setIsUploading] = useState(false);
  const abortRef = useRef<AbortController | null>(null);

  const cancel = useCallback(() => {
    abortRef.current?.abort();
    abortRef.current = null;
    setIsUploading(false);
  }, []);

  const upload = useCallback(async () => {
    // Abort any previous in-flight request before starting a new one.
    abortRef.current?.abort();
    const controller = new AbortController();
    abortRef.current = controller;

    setIsUploading(true);

    try {
      const response = await fetch(uploadPath, {
        method:  'POST',
        headers: { 'Content-Type': 'application/json' },
        body:    json,
        signal:  controller.signal,
      });

      const body = await response.text();

      if (!response.ok) {
        setIsUploading(false);
        onError(`HTTP ${response.status} — ${body}`);
        return;
      }

      setIsUploading(false);
      onSuccess(body);
    } catch (err) {
      if ((err as Error).name === 'AbortError') {
        // Cancelled by the caller — do not call onError.
        setIsUploading(false);
        return;
      }
      setIsUploading(false);
      onError((err as Error).message ?? 'Network error');
    }
  }, [uploadPath, json, onSuccess, onError]);

  return { isUploading, upload, cancel };
}
