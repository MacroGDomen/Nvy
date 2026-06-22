import * as DialogPrimitive from "@radix-ui/react-dialog";
import { Button } from "./Button";

type ConfirmDialogVariant = "default" | "danger";

type ConfirmDialogProps = {
  open: boolean;
  title: string;
  description: string;
  confirmLabel: string;
  onOpenChange: (open: boolean) => void;
  onConfirm: () => void;
  cancelLabel?: string;
  variant?: ConfirmDialogVariant;
};

export function ConfirmDialog({
  open,
  title,
  description,
  confirmLabel,
  onOpenChange,
  onConfirm,
  cancelLabel = "取消",
  variant = "default",
}: ConfirmDialogProps) {
  function handleConfirm() {
    onConfirm();
    onOpenChange(false);
  }

  return (
    <DialogPrimitive.Root open={open} onOpenChange={onOpenChange}>
      <DialogPrimitive.Portal>
        <DialogPrimitive.Overlay className="fixed inset-0 z-40 bg-black/56 backdrop-blur-sm" />
        <DialogPrimitive.Content className="fixed left-1/2 top-1/2 z-50 grid w-[min(420px,calc(100vw-32px))] -translate-x-1/2 -translate-y-1/2 gap-5 rounded-3xl border border-[var(--color-border)] bg-[var(--color-surface)] p-6 text-left shadow-2xl">
          <div className="grid gap-2">
            <DialogPrimitive.Title className="text-xl font-semibold tracking-normal text-[var(--color-text-strong)]">
              {title}
            </DialogPrimitive.Title>
            <DialogPrimitive.Description className="text-sm leading-6 text-[var(--color-muted)]">
              {description}
            </DialogPrimitive.Description>
          </div>

          <div className="flex justify-end gap-3">
            <DialogPrimitive.Close asChild>
              <Button variant="ghost">{cancelLabel}</Button>
            </DialogPrimitive.Close>
            <Button
              variant={variant === "danger" ? "danger" : "primary"}
              onClick={handleConfirm}
            >
              {confirmLabel}
            </Button>
          </div>
        </DialogPrimitive.Content>
      </DialogPrimitive.Portal>
    </DialogPrimitive.Root>
  );
}
