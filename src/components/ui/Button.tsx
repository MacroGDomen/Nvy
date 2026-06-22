import type { ButtonHTMLAttributes } from "react";
import { cn } from "../../lib/cn";

type ButtonVariant = "primary" | "secondary" | "ghost" | "danger";

type ButtonProps = ButtonHTMLAttributes<HTMLButtonElement> & {
  variant?: ButtonVariant;
};

const variantClassName: Record<ButtonVariant, string> = {
  primary:
    "bg-[var(--color-accent)] text-[var(--color-accent-foreground)] shadow-[var(--shadow-glow)] hover:bg-[var(--color-accent-hover)]",
  secondary:
    "bg-[var(--color-surface-strong)] text-[var(--color-text)] hover:bg-[var(--color-surface-hover)]",
  ghost:
    "bg-transparent text-[var(--color-muted)] hover:bg-[var(--color-surface)] hover:text-[var(--color-text)]",
  danger:
    "bg-[var(--color-danger)] text-[var(--color-danger-foreground)] hover:bg-[var(--color-danger-hover)]"
};

export function Button({
  className,
  type = "button",
  variant = "primary",
  ...props
}: ButtonProps) {
  return (
    <button
      type={type}
      className={cn(
        "inline-flex h-10 items-center justify-center rounded-full px-5 text-sm font-medium tracking-normal transition duration-150",
        "focus-visible:outline focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-[var(--color-focus)]",
        "disabled:pointer-events-none disabled:opacity-50",
        variantClassName[variant],
        className
      )}
      {...props}
    />
  );
}

