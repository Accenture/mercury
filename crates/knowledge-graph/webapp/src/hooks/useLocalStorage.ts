import { useState, useEffect, useCallback } from 'react';

export const useLocalStorage = <T>(
  key: string,
  initialValue: T
): [T, React.Dispatch<React.SetStateAction<T>>] => {
  // ---------------------------------------------------------------------------
  // Read helper — centralises the parse logic used in multiple places.
  // ---------------------------------------------------------------------------
  const readFromStorage = useCallback((): T => {
    try {
      const item = window.localStorage.getItem(key);
      return item ? (JSON.parse(item) as T) : initialValue;
    } catch {
      return initialValue;
    }
  // initialValue is intentionally excluded from deps (same reason as the
  // useEffect below — callers often pass referentially-unstable literals).
  // eslint-disable-next-line react-hooks/exhaustive-deps
  }, [key]);

  const [value, setValue] = useState<T>(readFromStorage);

  // ── Re-sync when the key prop changes ──────────────────────────────────
  // (e.g. Playground unmounts and remounts with a different storageKey)
  useEffect(() => {
    setValue(readFromStorage());
  // eslint-disable-next-line react-hooks/exhaustive-deps
  }, [key]);

  // ── Persist to localStorage whenever React state changes ───────────────
  useEffect(() => {
    try {
      window.localStorage.setItem(key, JSON.stringify(value));
    } catch (error) {
      console.error(`Error setting localStorage key "${key}":`, error);
    }
  }, [key, value]);

  // ── Re-sync when another tab writes (or clears) the same key ──────────
  // The native `storage` event only fires in tabs *other than* the one that
  // made the change, so this covers multi-tab scenarios.
  useEffect(() => {
    const handleStorageEvent = (e: StorageEvent) => {
      // e.key is null when localStorage.clear() is called.
      if (e.key === key || e.key === null) {
        setValue(readFromStorage());
      }
    };
    window.addEventListener('storage', handleStorageEvent);
    return () => window.removeEventListener('storage', handleStorageEvent);
  }, [key, readFromStorage]);

  // ── Re-sync when the user returns to this tab ──────────────────────────
  // DevTools `localStorage.clear()` on the *same* tab does NOT fire the
  // `storage` event.  Checking on `visibilitychange` (tab regains focus)
  // and `focus` (window regains focus) catches that case and any other
  // out-of-band mutation that happened while the tab was backgrounded.
  useEffect(() => {
    const handleFocus = () => setValue(readFromStorage());
    window.addEventListener('focus', handleFocus);
    document.addEventListener('visibilitychange', handleFocus);
    return () => {
      window.removeEventListener('focus', handleFocus);
      document.removeEventListener('visibilitychange', handleFocus);
    };
  }, [readFromStorage]);

  return [value, setValue];
};
