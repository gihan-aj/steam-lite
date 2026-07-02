import {
  createContext,
  useCallback,
  useContext,
  useEffect,
  useState,
} from "react";
import { AppError, setGlobalErrorHandler } from "../lib/invoke";

interface Toast {
  id: number;
  message: string;
  type: "error" | "warning" | "success" | "info";
  detail?: string;
}

interface ToastContextValue {
  showToast: (message: string, type?: Toast["type"], detail?: string) => void;
  showError: (error: AppError) => void;
  showSuccess: (message: string) => void;
}

const ToastContext = createContext<ToastContextValue | null>(null);

export function useToast() {
  const ctx = useContext(ToastContext);
  if (!ctx) throw new Error("useToast must be used within ToastProvider");
  return ctx;
}

let toastId = 0;

export function ToastProvider({ children }: { children: React.ReactNode }) {
  const [toasts, setToasts] = useState<Toast[]>([]);

  const removeToast = useCallback((id: number) => {
    setToasts((prev) => prev.filter((t) => t.id !== id));
  }, []);

  const showToast = useCallback(
    (message: string, type: Toast["type"] = "info", detail?: string) => {
      const id = ++toastId;
      setToasts((prev) => [...prev, { id, message, type, detail }]);

      // Auto-dismiss after 5 seconds (errors stay longer)
      const duration = type === "error" ? 8000 : 5000;
      setTimeout(() => removeToast(id), duration);
    },
    [removeToast],
  );

  const showError = useCallback(
    (error: AppError) => {
      const type = error.type === "NotFound" ? "warning" : "error";
      showToast(error.message, type, error.detail);
    },
    [showToast],
  );

  const showSuccess = useCallback(
    (message: string) => {
      showToast(message, "success");
    },
    [showToast],
  );

  // Register as the global error handler for invokeCmd
  useEffect(() => {
    setGlobalErrorHandler(showError);
    return () => setGlobalErrorHandler(() => {});
  }, [showError]);

  return (
    <ToastContext.Provider value={{ showToast, showError, showSuccess }}>
      {children}

      {/* Toast container — fixed position, bottom right */}
      <div
        style={{
          position: "fixed",
          bottom: 20,
          right: 20,
          zIndex: 1000,
          display: "flex",
          flexDirection: "column",
          gap: 8,
          maxWidth: 360,
          pointerEvents: "none", // don't block clicks on the app
        }}
      >
        {toasts.map((toast) => (
          <ToastItem
            key={toast.id}
            toast={toast}
            onClose={() => removeToast(toast.id)}
          />
        ))}
      </div>
    </ToastContext.Provider>
  );
}

const TOAST_STYLES = {
  error: { bg: "#2a1515", border: "#7f1d1d", icon: "✕", color: "#fca5a5" },
  warning: { bg: "#292010", border: "#92400e", icon: "⚠", color: "#fbbf24" },
  success: { bg: "#14291e", border: "#166534", icon: "✓", color: "#4ade80" },
  info: { bg: "#0f1e35", border: "#1d4ed8", icon: "ℹ", color: "#60a5fa" },
};

function ToastItem({ toast, onClose }: { toast: Toast; onClose: () => void }) {
  const [visible, setVisible] = useState(false);
  const style = TOAST_STYLES[toast.type];

  // Animate in
  useEffect(() => {
    requestAnimationFrame(() => setVisible(true));
  }, []);

  return (
    <div
      style={{
        background: style.bg,
        border: `1px solid ${style.border}`,
        borderRadius: 8,
        padding: "10px 14px",
        display: "flex",
        alignItems: "flex-start",
        gap: 10,
        pointerEvents: "auto",
        transform: visible ? "translateX(0)" : "translateX(120%)",
        opacity: visible ? 1 : 0,
        transition: "transform 0.25s ease-out, opacity 0.25s ease-out",
        boxShadow: "0 4px 12px rgba(0,0,0,0.4)",
      }}
    >
      {/* Icon */}
      <span
        style={{
          fontSize: 14,
          color: style.color,
          flexShrink: 0,
          marginTop: 1,
        }}
      >
        {style.icon}
      </span>

      {/* Content */}
      <div style={{ flex: 1, minWidth: 0 }}>
        <p
          style={{
            fontSize: 13,
            fontWeight: 500,
            color: style.color,
            margin: 0,
            lineHeight: 1.4,
          }}
        >
          {toast.message}
        </p>
        {toast.detail && toast.detail !== toast.message && (
          <p
            style={{
              fontSize: 11,
              color: style.color,
              opacity: 0.6,
              margin: "3px 0 0",
              lineHeight: 1.4,
              // Truncate very long technical details
              overflow: "hidden",
              textOverflow: "ellipsis",
              display: "-webkit-box",
              WebkitLineClamp: 2,
              WebkitBoxOrient: "vertical" as const,
            }}
          >
            {toast.detail}
          </p>
        )}
      </div>

      {/* Close button */}
      <button
        onClick={onClose}
        style={{
          background: "transparent",
          border: "none",
          color: style.color,
          opacity: 0.5,
          cursor: "pointer",
          padding: 0,
          fontSize: 12,
          flexShrink: 0,
          lineHeight: 1,
        }}
      >
        ✕
      </button>
    </div>
  );
}