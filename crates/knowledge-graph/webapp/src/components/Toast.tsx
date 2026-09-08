import styles from './Toast.module.css';
import { type Toast } from '../hooks/useToast';

interface ToastContainerProps {
  toasts:   Toast[];
  onRemove: (id: number) => void;
}

export const ToastContainer = ({ toasts, onRemove }: ToastContainerProps) => {
  if (toasts.length === 0) return null;

  return (
    <div className={styles.toastContainer}>
      {toasts.map((toast: Toast) => (
        <div
          key={toast.id}
          className={`${styles.toast} ${styles[toast.type]}`}
          onClick={() => onRemove(toast.id)}
        >
          <span className={styles.toastIcon}>
            {toast.type === 'success' && '✅'}
            {toast.type === 'error' && '❌'}
            {toast.type === 'info' && 'ℹ️'}
          </span>
          <span className={styles.toastMessage}>{toast.message}</span>
          {toast.action && (
            <button
              type="button"
              className={styles.toastAction}
              onClick={(event) => {
                event.stopPropagation();
                toast.action?.onClick();
                onRemove(toast.id);
              }}
            >
              {toast.action.label}
            </button>
          )}
        </div>
      ))}
    </div>
  );
};
