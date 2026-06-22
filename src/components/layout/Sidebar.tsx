import { useState, type ReactNode } from "react";
import type { ProtectedRoute } from "../../app/router";
import { cn } from "../../lib/cn";
import { Tooltip } from "../ui/Tooltip";
import {
  ActressIcon,
  CollapseIcon,
  HomeIcon,
  SettingsIcon,
  VideoIcon,
} from "./icons";

type SidebarItem = {
  id: ProtectedRoute;
  label: string;
  icon: ReactNode;
};

const NAV_ITEMS: SidebarItem[] = [
  { id: "home", label: "首页", icon: <HomeIcon className="h-5 w-5" /> },
  { id: "actresses", label: "女优库", icon: <ActressIcon className="h-5 w-5" /> },
  { id: "videos", label: "影片库", icon: <VideoIcon className="h-5 w-5" /> },
];

const FOOTER_ITEMS: SidebarItem[] = [
  { id: "settings", label: "设置", icon: <SettingsIcon className="h-5 w-5" /> },
];

type SidebarProps = {
  username: string;
  activeRoute: ProtectedRoute;
  onSelect: (route: ProtectedRoute) => void;
};

export function Sidebar({ username, activeRoute, onSelect }: SidebarProps) {
  const [expanded, setExpanded] = useState(false);

  function handleLogoClick() {
    setExpanded((previous) => !previous);
  }

  return (
    <aside
      className={cn(
        "flex h-screen flex-col gap-4 border-r border-[var(--color-border)] bg-[var(--color-surface)]/92 px-2 py-4 backdrop-blur transition-[width] duration-200 ease-out",
        expanded ? "w-56" : "w-16",
      )}
      aria-label="Primary navigation"
    >
      <SidebarLogo expanded={expanded} onClick={handleLogoClick} />

      <nav className="grid flex-1 content-start gap-1.5" aria-label="Main pages">
        {NAV_ITEMS.map((item) => (
          <SidebarButton
            key={item.id}
            item={item}
            expanded={expanded}
            isActive={item.id === activeRoute}
            onSelect={onSelect}
          />
        ))}
      </nav>

      <div className="grid content-end gap-1.5" aria-label="Account area">
        {FOOTER_ITEMS.map((item) => (
          <SidebarButton
            key={item.id}
            item={item}
            expanded={expanded}
            isActive={item.id === activeRoute}
            onSelect={onSelect}
          />
        ))}
        <SidebarAccount expanded={expanded} username={username} />
      </div>
    </aside>
  );
}

type SidebarLogoProps = {
  expanded: boolean;
  onClick: () => void;
};

function SidebarLogo({ expanded, onClick }: SidebarLogoProps) {
  const tooltipLabel = expanded ? "收起状态栏" : "展开状态栏";

  return (
    <Tooltip label={tooltipLabel}>
      <button
        type="button"
        onClick={onClick}
        aria-label={tooltipLabel}
        aria-expanded={expanded}
        className={cn(
          "group flex items-center gap-3 rounded-2xl border border-transparent px-2 py-2 text-left transition",
          "hover:border-[var(--color-border)] hover:bg-[var(--color-surface-hover)]",
          "focus-visible:outline focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-[var(--color-focus)]",
          expanded ? "w-full" : "justify-center",
        )}
      >
        <span className="grid h-8 w-8 shrink-0 place-items-center rounded-xl bg-[linear-gradient(135deg,var(--color-accent),var(--color-accent-pink))] text-sm font-extrabold tracking-normal text-[var(--color-accent-foreground)] shadow-[var(--shadow-glow)]">
          NVY
        </span>
        {expanded ? (
          <span className="flex w-full items-center justify-between">
            <span className="text-sm font-semibold tracking-normal text-[var(--color-text-strong)]">
              Nvy
            </span>
            <CollapseIcon className="h-4 w-4 text-[var(--color-muted)]" />
          </span>
        ) : null}
      </button>
    </Tooltip>
  );
}

type SidebarButtonProps = {
  item: SidebarItem;
  expanded: boolean;
  isActive: boolean;
  onSelect: (route: ProtectedRoute) => void;
};

function SidebarButton({
  item,
  expanded,
  isActive,
  onSelect,
}: SidebarButtonProps) {
  const button = (
    <button
      type="button"
      aria-current={isActive ? "page" : undefined}
      onClick={() => onSelect(item.id)}
      className={cn(
        "flex items-center gap-3 rounded-2xl px-2 py-2 text-sm font-medium tracking-normal transition",
        "focus-visible:outline focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-[var(--color-focus)]",
        expanded ? "w-full" : "justify-center",
        isActive
          ? "bg-[rgba(159,136,219,0.18)] text-[var(--color-text-strong)]"
          : "text-[var(--color-muted)] hover:bg-[var(--color-surface-hover)] hover:text-[var(--color-text)]",
      )}
    >
      <span className="shrink-0">{item.icon}</span>
      {expanded ? <span>{item.label}</span> : null}
    </button>
  );

  if (expanded) {
    return button;
  }

  return <Tooltip label={item.label}>{button}</Tooltip>;
}

type SidebarAccountProps = {
  expanded: boolean;
  username: string;
};

function SidebarAccount({ expanded, username }: SidebarAccountProps) {
  const initial = username.trim().slice(0, 1) || "?";

  if (!expanded) {
    return (
      <Tooltip label={username}>
        <div className="mt-1 flex justify-center">
          <span className="grid h-8 w-8 place-items-center rounded-full bg-[var(--color-surface-strong)] text-xs font-semibold text-[var(--color-accent-soft)]">
            {initial}
          </span>
        </div>
      </Tooltip>
    );
  }

  return (
    <div className="mt-1 flex items-center gap-3 rounded-2xl border border-[var(--color-border)] bg-[var(--color-surface-soft)] px-2 py-2">
      <span className="grid h-8 w-8 shrink-0 place-items-center rounded-full bg-[var(--color-surface-strong)] text-xs font-semibold text-[var(--color-accent-soft)]">
        {initial}
      </span>
      <span className="truncate text-sm font-medium text-[var(--color-text)]">
        {username}
      </span>
    </div>
  );
}
