import { PreviewPanel } from "../components/layout/PreviewPanel";
import { Button } from "../components/ui/Button";
import { Dialog } from "../components/ui/Dialog";
import { Dropdown } from "../components/ui/Dropdown";
import { Input } from "../components/ui/Input";
import { Tooltip } from "../components/ui/Tooltip";

export function AppShellPreview() {
  return (
    <main className="grid min-h-screen place-items-center px-6 py-12">
      <PreviewPanel>
        <div className="mx-auto grid min-h-14 min-w-24 place-items-center rounded-[20px] bg-[linear-gradient(135deg,var(--color-accent),var(--color-accent-pink))] text-2xl font-extrabold tracking-normal text-[var(--color-accent-foreground)]">
          NVY
        </div>
        <div className="grid gap-3">
          <p className="text-sm font-semibold uppercase tracking-normal text-[var(--color-accent-soft)]">
            MVP shell
          </p>
          <h1 className="m-0 text-6xl font-semibold leading-none tracking-normal text-[var(--color-text-strong)]">
            Nvy
          </h1>
          <p className="mx-auto max-w-[440px] text-base leading-7 text-[var(--color-muted)]">
            Windows local desktop app skeleton is ready for the next MVP task.
          </p>
        </div>
        <div className="grid gap-4 rounded-[28px] border border-[var(--color-border)] bg-[var(--color-surface-soft)] p-5">
          <Input label="Theme preview" placeholder="Dark input field" />
          <div className="flex flex-wrap items-center justify-center gap-3">
            <Tooltip label="Primary action token">
              <Button>Primary</Button>
            </Tooltip>
            <Dropdown
              label="Dropdown"
              items={[
                {
                  label: "Material dark",
                  description: "Baseline for Nvy controls"
                },
                {
                  label: "Gemini mood",
                  description: "Soft purple and pink accents"
                }
              ]}
            />
            <Dialog
              triggerLabel="Dialog"
              title="UI primitives ready"
              description="Radix Dialog, Dropdown Menu, Tooltip, Button, Input, Tailwind CSS, and CSS variables are connected."
            />
          </div>
        </div>
      </PreviewPanel>
    </main>
  );
}

