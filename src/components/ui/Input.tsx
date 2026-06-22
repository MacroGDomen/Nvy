import type { InputHTMLAttributes } from "react";
import { cn } from "../../lib/cn";

type InputProps = InputHTMLAttributes<HTMLInputElement> & {
  label?: string;
};

export function Input({ className, id, label, ...props }: InputProps) {
  const inputId = id ?? props.name;

  return (
    <label className="grid gap-2 text-left text-sm text-[var(--color-muted)]">
      {label ? <span>{label}</span> : null}
      <input
        id={inputId}
        className={cn(
          "h-11 rounded-2xl border border-[var(--color-border)] bg-[var(--color-input)] px-4 text-[var(--color-text)] outline-none transition",
          "placeholder:text-[var(--color-muted-subtle)] focus:border-[var(--color-focus)] focus:ring-2 focus:ring-[var(--color-focus-soft)]",
          className
        )}
        {...props}
      />
    </label>
  );
}

