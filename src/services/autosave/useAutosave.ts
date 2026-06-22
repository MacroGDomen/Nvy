import { useEffect, useRef } from "react";

type UseAutosaveOptions<TValue, TResult> = {
  value: TValue;
  enabled: boolean;
  delayMs?: number;
  onSave: (value: TValue) => Promise<TResult>;
  onSaved?: (result: TResult) => void;
  onError?: () => void;
};

export function useAutosave<TValue, TResult>({
  value,
  enabled,
  delayMs = 900,
  onSave,
  onSaved,
  onError,
}: UseAutosaveOptions<TValue, TResult>) {
  const lastSavedValue = useRef("");
  const hasInitialized = useRef(false);

  useEffect(() => {
    const serializedValue = JSON.stringify(value);

    if (!enabled) {
      lastSavedValue.current = serializedValue;
      return;
    }

    if (!hasInitialized.current) {
      hasInitialized.current = true;
      lastSavedValue.current = serializedValue;
      return;
    }

    if (serializedValue === lastSavedValue.current) {
      return;
    }

    const timeoutId = window.setTimeout(() => {
      onSave(value)
        .then((result) => {
          lastSavedValue.current = serializedValue;
          onSaved?.(result);
        })
        .catch(() => {
          onError?.();
        });
    }, delayMs);

    return () => {
      window.clearTimeout(timeoutId);
    };
  }, [delayMs, enabled, onError, onSave, onSaved, value]);
}
