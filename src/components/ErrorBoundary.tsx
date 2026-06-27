import { Component, ErrorInfo, ReactNode } from "react";

interface Props {
    children: ReactNode;
    fallback?: ReactNode; // Optional custom fallback UI
}

interface State {
    hasError: boolean;
    error: Error | null;
}

export class ErrorBoundary extends Component<Props, State> {
    state: State = { hasError: false, error: null };

    static getDerivedStateFromError(error: Error): State {
        return { hasError: true, error };
    }

    componentDidCatch(error: Error, errorInfo: ErrorInfo): void {
        console.error('[ErrorBoundary] Caught Error:', error, errorInfo.componentStack);
    }

    render(): ReactNode {
        if(this.state.hasError) {
            if (this.props.fallback) return this.props.fallback;

            return (
              <div
                style={{
                  display: "flex",
                  flexDirection: "column",
                  alignItems: "center",
                  justifyContent: "center",
                  height: "100%",
                  gap: 12,
                  padding: 24,
                }}
              >
                <span style={{ fontSize: 32 }}>⚠️</span>
                <h2
                  style={{
                    fontSize: 16,
                    color: "#e0e2e8",
                    fontWeight: 600,
                    margin: 0,
                  }}
                >
                  Something went wrong
                </h2>
                <p
                  style={{
                    fontSize: 12,
                    color: "#5a5f72",
                    textAlign: "center",
                    maxWidth: 320,
                  }}
                >
                  {this.state.error?.message ?? "An unexpected error occurred"}
                </p>
                <button
                  onClick={() =>
                    this.setState({ hasError: false, error: null })
                  }
                  style={{
                    background: "#3d6ef8",
                    color: "#fff",
                    border: "none",
                    borderRadius: 6,
                    padding: "7px 16px",
                    fontSize: 13,
                    cursor: "pointer",
                  }}
                >
                  Try again
                </button>
              </div>
            );
        }

        return this.props.children;
    }
}