export type ToastVariant = "info" | "success" | "error";

export type ToastMessage = {
  id: string;
  title: string;
  description?: string;
  variant: ToastVariant;
};

export type ToastInput = {
  title: string;
  description?: string;
  variant?: ToastVariant;
  durationMs?: number;
};

export const DEFAULT_TOAST_DURATION_MS = 4200;
export const MAX_VISIBLE_TOASTS = 4;

export function createToastMessage(
  id: string,
  input: ToastInput,
): ToastMessage {
  return {
    id,
    title: input.title,
    description: input.description,
    variant: input.variant ?? "info",
  };
}

export function addToastMessage(
  messages: ToastMessage[],
  message: ToastMessage,
  maxVisible = MAX_VISIBLE_TOASTS,
): ToastMessage[] {
  return [message, ...messages].slice(0, maxVisible);
}

export function dismissToastMessage(
  messages: ToastMessage[],
  id: string,
): ToastMessage[] {
  return messages.filter((message) => message.id !== id);
}
