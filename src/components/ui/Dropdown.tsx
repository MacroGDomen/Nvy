import * as DropdownMenu from "@radix-ui/react-dropdown-menu";
import { Button } from "./Button";
import { cn } from "../../lib/cn";

type DropdownProps = {
  label: string;
  items: Array<{
    label: string;
    description: string;
    selected?: boolean;
    onSelect?: () => void;
  }>;
  className?: string;
  align?: "start" | "center" | "end";
  triggerClassName?: string;
};

export function Dropdown({
  label,
  items,
  className,
  align = "center",
  triggerClassName,
}: DropdownProps) {
  return (
    <DropdownMenu.Root>
      <DropdownMenu.Trigger asChild>
        <Button
          variant="secondary"
          className={cn(
            "justify-between border border-[var(--color-border)] bg-[var(--color-input)] px-4 text-left shadow-none data-[state=open]:border-[var(--color-border-strong)] data-[state=open]:bg-[var(--color-surface-strong)]",
            triggerClassName,
          )}
        >
          <span className="truncate">{label}</span>
          <span className="ml-3 text-xs text-[var(--color-muted-subtle)]">v</span>
        </Button>
      </DropdownMenu.Trigger>
      <DropdownMenu.Portal>
        <DropdownMenu.Content
          align={align}
          sideOffset={10}
          className={cn(
            "z-50 grid min-w-64 gap-1 rounded-2xl border border-[var(--color-border-strong)] bg-[var(--color-panel-elevated)] p-2 text-left shadow-2xl backdrop-blur-xl",
            className,
          )}
        >
          {items.map((item) => (
            <DropdownMenu.Item
              key={item.label}
              onSelect={item.onSelect}
              className={cn(
                "grid cursor-default gap-1 rounded-xl px-4 py-3 outline-none transition hover:bg-[var(--color-surface-hover)] focus:bg-[var(--color-surface-hover)]",
                item.selected
                  ? "bg-[rgba(165,140,223,0.18)] text-[var(--color-text-strong)]"
                  : "text-[var(--color-text)]",
              )}
            >
              <span className="flex items-center justify-between gap-3 text-sm font-medium">
                <span>{item.label}</span>
                {item.selected ? (
                  <span className="text-xs text-[var(--color-accent-soft)]">Selected</span>
                ) : null}
              </span>
              <span className="text-xs leading-5 text-[var(--color-muted)]">
                {item.description}
              </span>
            </DropdownMenu.Item>
          ))}
        </DropdownMenu.Content>
      </DropdownMenu.Portal>
    </DropdownMenu.Root>
  );
}
