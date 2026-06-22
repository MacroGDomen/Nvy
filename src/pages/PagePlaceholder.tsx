import type { ReactNode } from "react";

type PagePlaceholderProps = {
  title: string;
  description: string;
  action?: ReactNode;
};

export function PagePlaceholder({
  title,
  description,
  action,
}: PagePlaceholderProps) {
  return (
    <main className="min-h-screen bg-[var(--color-background)] px-8 py-8 text-[var(--color-text)]">
      <section className="mx-auto flex min-h-[calc(100vh-4rem)] max-w-6xl flex-col justify-center border-b border-[var(--color-border)] py-10">
        <div className="max-w-3xl">
          <p className="mb-3 text-sm font-medium tracking-normal text-[var(--color-accent-soft)]">
            Nvy MVP
          </p>
          <h1 className="text-3xl font-semibold tracking-normal text-[var(--color-text-strong)]">
            {title}
          </h1>
          <p className="mt-4 max-w-2xl text-base leading-7 text-[var(--color-muted)]">
            {description}
          </p>
        </div>
        {action ? <div className="mt-8">{action}</div> : null}
      </section>
    </main>
  );
}
