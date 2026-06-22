import {
  createContext,
  useCallback,
  useContext,
  useMemo,
  useRef,
  useState,
  type ReactNode,
} from "react";
import { cn } from "../../lib/cn";
import {
  addToastMessage,
  createToastMessage,
  DEFAULT_TOAST_DURATION_MS,
  dismissToastMessage,
  type ToastInput,
  type ToastMessage,
  type ToastVariant,
} from "./toastState";

type ToastContextValue = {
  notify: (input: ToastInput) => string;
  dismiss: (id: string) => void;
};

const ToastContext = createContext<ToastContextValue | null>(null);

type ToastProviderProps = {
  children: ReactNode;
};

export function ToastProvider({ children }: ToastProviderProps) {
  const [messages, setMessages] = useState<ToastMessage[]>([]);
  const idSeed = useRef(0);

  const dismiss = useCallback((id: string) => {
    setMessages((currentMessages) => dismissToastMessage(currentMessages, id));
  }, []);

  const notify = useCallback(
    (input: ToastInput) => {
      idSeed.current += 1;
      const id = `toast-${Date.now()}-${idSeed.current}`;
      const message = createToastMessage(id, input);
      const durationMs = input.durationMs ?? DEFAULT_TOAST_DURATION_MS;

      setMessages((currentMessages) => addToastMessage(currentMessages, message));

      if (durationMs > 0) {
        window.setTimeout(() => dismiss(id), durationMs);
      }

      return id;
    },
    [dismiss],
  );

  const value = useMemo(() => ({ notify, dismiss }), [dismiss, notify]);

  return (
    <ToastContext.Provider value={value}>
      {children}
      <ToastViewport messages={messages} onDismiss={dismiss} />
    </ToastContext.Provider>
  );
}

export function useToast() {
  const context = useContext(ToastContext);

  if (!context) {
    throw new Error("useToast must be used within ToastProvider.");
  }

  return context;
}

type ToastViewportProps = {
  messages: ToastMessage[];
  onDismiss: (id: string) => void;
};

function ToastViewport({ messages, onDismiss }: ToastViewportProps) {
  if (messages.length === 0) {
    return null;
  }

  return (
    <div
      className="fixed right-4 top-4 z-[70] grid w-[min(360px,calc(100vw-32px))] gap-3"
      role="region"
      aria-label="Notifications"
    >
      {messages.map((message) => (
        <ToastCard
          key={message.id}
          message={message}
          onDismiss={() => onDismiss(message.id)}
        />
      ))}
    </div>
  );
}

type ToastCardProps = {
  message: ToastMessage;
  onDismiss: () => void;
};

function ToastCard({ message, onDismiss }: ToastCardProps) {
  return (
    <section
      className={cn(
        "grid gap-2 rounded-2xl border bg-[var(--color-surface)] px-4 py-3 text-left shadow-[var(--shadow-panel)]",
        variantClassName[message.variant],
      )}
      role={message.variant === "error" ? "alert" : "status"}
    >
      <div className="flex items-start gap-3">
        <span
          className={cn(
            "mt-1 h-2.5 w-2.5 shrink-0 rounded-full",
            markerClassName[message.variant],
          )}
        />
        <div className="min-w-0 flex-1">
          <p className="m-0 text-sm font-semibold text-[var(--color-text-strong)]">
            {message.title}
          </p>
          {message.description ? (
            <p className="m-0 mt-1 text-sm leading-5 text-[var(--color-muted)]">
              {message.description}
            </p>
          ) : null}
        </div>
        <button
          type="button"
          aria-label="Close notification"
          onClick={onDismiss}
          className="grid h-7 w-7 shrink-0 place-items-center rounded-full text-lg leading-none text-[var(--color-muted)] transition hover:bg-[var(--color-surface-hover)] hover:text-[var(--color-text)] focus-visible:outline focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-[var(--color-focus)]"
        >
          x
        </button>
      </div>
    </section>
  );
}

const variantClassName: Record<ToastVariant, string> = {
  info: "border-[var(--color-border)]",
  success: "border-[rgba(139,199,163,0.34)]",
  error: "border-[rgba(183,91,116,0.42)] bg-[rgba(52,31,41,0.96)]",
};

const markerClassName: Record<ToastVariant, string> = {
  info: "bg-[var(--color-accent-soft)]",
  success: "bg-[#8bc7a3]",
  error: "bg-[var(--color-danger-hover)]",
};
