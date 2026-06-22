import * as DialogPrimitive from "@radix-ui/react-dialog";
import type { ReactNode } from "react";
import { Button } from "./Button";

type DialogProps = {
  triggerLabel: string;
  title: string;
  description: string;
  children?: ReactNode;
};

export function Dialog({ triggerLabel, title, description, children }: DialogProps) {
  return (
    <DialogPrimitive.Root>
      <DialogPrimitive.Trigger asChild>
        <Button variant="secondary">{triggerLabel}</Button>
      </DialogPrimitive.Trigger>
      <DialogPrimitive.Portal>
        <DialogPrimitive.Overlay className="fixed inset-0 z-40 bg-black/50 backdrop-blur-sm" />
        <DialogPrimitive.Content className="fixed left-1/2 top-1/2 z-50 grid w-[min(420px,calc(100vw-32px))] -translate-x-1/2 -translate-y-1/2 gap-5 rounded-[28px] border border-[var(--color-border)] bg-[var(--color-surface)] p-6 text-left shadow-2xl">
          <div className="grid gap-2">
            <DialogPrimitive.Title className="text-xl font-semibold text-[var(--color-text-strong)]">
              {title}
            </DialogPrimitive.Title>
            <DialogPrimitive.Description className="text-sm leading-6 text-[var(--color-muted)]">
              {description}
            </DialogPrimitive.Description>
          </div>
          {children}
          <div className="flex justify-end">
            <DialogPrimitive.Close asChild>
              <Button variant="ghost">Close</Button>
            </DialogPrimitive.Close>
          </div>
        </DialogPrimitive.Content>
      </DialogPrimitive.Portal>
    </DialogPrimitive.Root>
  );
}

