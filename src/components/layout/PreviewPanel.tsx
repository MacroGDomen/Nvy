import type { ReactNode } from "react";

type PreviewPanelProps = {
  children: ReactNode;
};

export function PreviewPanel({ children }: PreviewPanelProps) {
  return (
    <section
      className="grid w-full max-w-[640px] gap-7 rounded-[32px] border border-[var(--color-border)] bg-[var(--color-panel)] p-8 text-center shadow-[var(--shadow-panel)] backdrop-blur"
      aria-label="Nvy application shell"
    >
      {children}
    </section>
  );
}

