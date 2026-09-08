import { useState, useCallback, useRef, useEffect } from 'react';

export type ToastType = 'info' | 'success' | 'error';

export interface ToastAction {
  label:   string;
  onClick: () => void;
}

export interface ToastOptions {
  /** Optional action button (e.g. "Undo") rendered inside the toast. */
  action?:     ToastAction;
  /** Auto-dismiss delay; defaults to 3000ms (action toasts usually want longer). */
  durationMs?: number;
}

export interface Toast {
  id:      number;
  message: string;
  type:    ToastType;
  action?: ToastAction;
}

export interface UseToastReturn {
  toasts:      Toast[];
  addToast:    (message: string, type?: ToastType, options?: ToastOptions) => void;
  removeToast: (id: number) => void;
}

export const useToast = (): UseToastReturn => {
  const [toasts, setToasts] = useState<Toast[]>([]);
  const nextId   = useRef(0);
  // Track all pending auto-dismiss timers so they can be cleared on unmount.
  const timers   = useRef<Set<ReturnType<typeof setTimeout>>>(new Set());

  // Clear every pending timer when the provider unmounts.
  useEffect(() => {
    return () => { timers.current.forEach(clearTimeout); };
  }, []);

  const addToast = useCallback((message: string, type: ToastType = 'info', options?: ToastOptions): void => {
    const id = ++nextId.current;
    setToasts(prev => [...prev, { id, message, type, action: options?.action }]);

    const timer = setTimeout(() => {
      timers.current.delete(timer);
      setToasts(prev => prev.filter(t => t.id !== id));
    }, options?.durationMs ?? 3000);
    timers.current.add(timer);
  }, []);

  const removeToast = useCallback((id: number): void => {
    setToasts(prev => prev.filter(t => t.id !== id));
  }, []);

  return { toasts, addToast, removeToast };
};
