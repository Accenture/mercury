import { Component, type ReactNode, type ErrorInfo } from 'react';
import styles from './GraphView.module.css';

interface Props {
  onRenderError?: (message: string) => void;
  children:       ReactNode;
}

interface State {
  caughtError: string | null;
}

/**
 * Class-based error boundary that wraps <ReactFlow>.
 *
 * React's render-phase exceptions (thrown inside third-party components like
 * ReactFlow) cannot be caught by try/catch in useMemo or useEffect — only a
 * class component implementing componentDidCatch can intercept them.
 *
 * On error we:
 *  1. Record the message in local state so the fallback UI is shown.
 *  2. Fire onRenderError (→ addToast in Playground) so the toast appears.
 *  3. Reset caughtError when new graph data arrives (via getDerivedStateFromError
 *     lifecycle + key prop on the boundary at the call-site).
 */
export class GraphViewErrorBoundary extends Component<Props, State> {
  state: State = { caughtError: null };

  static getDerivedStateFromError(error: unknown): State {
    const caughtError = error instanceof Error ? error.message : String(error);
    return { caughtError };
  }

  componentDidCatch(error: unknown, info: ErrorInfo): void {
    const message = error instanceof Error ? error.message : String(error);
    console.error('[GraphView] Render error:', message, info.componentStack);
    this.props.onRenderError?.(`Graph render failed: ${message}`);
  }

  render() {
    if (this.state.caughtError) {
      return (
        <div className={styles.empty}>
          <span className={styles.emptyIcon}>⚠️</span>
          <span>Graph could not be rendered.</span>
          <span>{this.state.caughtError}</span>
        </div>
      );
    }
    return this.props.children;
  }
}
