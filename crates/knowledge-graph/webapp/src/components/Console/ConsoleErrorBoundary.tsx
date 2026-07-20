import React from 'react';

interface ConsoleErrorBoundaryProps {
  fallback: string;
  children: React.ReactNode;
}

interface ConsoleErrorBoundaryState {
  hasError: boolean;
}

export class ConsoleErrorBoundary extends React.Component<
  ConsoleErrorBoundaryProps,
  ConsoleErrorBoundaryState
> {
  state: ConsoleErrorBoundaryState = { hasError: false };

  static getDerivedStateFromError(): ConsoleErrorBoundaryState {
    return { hasError: true };
  }

  componentDidCatch(error: unknown, info: React.ErrorInfo): void {
    // Surface render errors in the browser console so they remain visible in
    // production builds where the React error overlay is absent.
    console.error('[ConsoleErrorBoundary] Failed to render message:', error, info.componentStack);
  }

  render() {
    if (this.state.hasError) {
      return <span>{this.props.fallback}</span>;
    }
    return this.props.children;
  }
}
