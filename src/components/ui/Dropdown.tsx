import * as DropdownMenu from "@radix-ui/react-dropdown-menu";
import { Button } from "./Button";

type DropdownProps = {
  label: string;
  items: Array<{
    label: string;
    description: string;
  }>;
};

export function Dropdown({ label, items }: DropdownProps) {
  return (
    <DropdownMenu.Root>
      <DropdownMenu.Trigger asChild>
        <Button variant="secondary">{label}</Button>
      </DropdownMenu.Trigger>
      <DropdownMenu.Portal>
        <DropdownMenu.Content
          align="center"
          sideOffset={10}
          className="z-50 grid min-w-64 gap-1 rounded-3xl border border-[var(--color-border)] bg-[var(--color-surface)] p-2 text-left shadow-2xl"
        >
          {items.map((item) => (
            <DropdownMenu.Item
              key={item.label}
              className="grid cursor-default gap-1 rounded-2xl px-4 py-3 outline-none transition hover:bg-[var(--color-surface-hover)] focus:bg-[var(--color-surface-hover)]"
            >
              <span className="text-sm font-medium text-[var(--color-text)]">{item.label}</span>
              <span className="text-xs leading-5 text-[var(--color-muted)]">{item.description}</span>
            </DropdownMenu.Item>
          ))}
        </DropdownMenu.Content>
      </DropdownMenu.Portal>
    </DropdownMenu.Root>
  );
}

