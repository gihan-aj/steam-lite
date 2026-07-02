import { invoke } from "@tauri-apps/api/core";

export interface BackendError {
  type: string; // "Api" | "Database" | "NotFound" | "Parse" | "Internal"
  message: string; // user-friendly message
  detail: string; // technical detail for logs
}

export class AppError extends Error {
  public readonly type: string;
  public readonly message: string;
  public readonly detail: string;

  constructor(error: BackendError) {
    super(error.message);
    this.type = error.type;
    this.message = error.message;
    this.detail = error.detail;
    this.name = "AppError";
  }

  // True for errors the user should see
  get isUserVisible(): boolean {
    return this.type !== "Parse";
  }

  // True for errors caused by misconfiguration the user can fix
  get isConfigError(): boolean {
    return (
      this.type === "NotFound" ||
      this.message.includes("not set") ||
      this.message.includes("API key")
    );
  }
}

// Global error handler — set this from your toast context
let globalErrorHandler: ((error: AppError) => void) | null = null;

export function setGlobalErrorHandler(handler: (error: AppError) => void) {
  globalErrorHandler = handler;
}

// The main wrapper — use this instead of invoke() everywhere
export async function invokeCmd<T>(
  command: string,
  args?: Record<string, unknown>,
  options?: {
    silent?: boolean; // true = don't show toast, just throw
    fallback?: T; // return this value instead of throwing
  },
): Promise<T> {
  try {
    return await invoke<T>(command, args);
  } catch (raw) {
    const backendError = parseError(raw);
    const appError = new AppError(backendError);

    // Always log to console
    console.error(`[${command}] ${appError.type}: ${appError.detail}`);

    if (!options?.silent && appError.isUserVisible && globalErrorHandler) {
      globalErrorHandler(appError);
    }

    // Return fallback if provided, otherwise throw
    if (options?.fallback !== undefined) {
      return options.fallback;
    }

    throw appError;
  }
}

function parseError(raw: unknown): BackendError {
  // Tauri sends our serialized AppError as an object
  if (raw && typeof raw === "object" && "message" in raw) {
    return raw as BackendError;
  }
  // String error (network error, etc.)
  if (typeof raw === "string") {
    return {
      type: "Internal",
      message: "An unexpected error occurred.",
      detail: raw,
    };
  }
  return {
    type: "Internal",
    message: "An unexpected error occurred.",
    detail: JSON.stringify(raw),
  };
}