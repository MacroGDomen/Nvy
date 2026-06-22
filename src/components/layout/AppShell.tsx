import { useState, type ReactNode } from "react";
import type { AccountSession } from "../../services/desktopApi/types";
import { Sidebar, type SidebarItemId } from "./Sidebar";

type AppShellProps = {
  session: AccountSession;
  children: ReactNode;
};

// AppShell is the shared layout for authenticated pages. It places the global
// collapsible sidebar on the left and the active page content on the right.
// Task 2.3 only wires the shell + sidebar; full page routing arrives in 2.4.
export function AppShell({ session, children }: AppShellProps) {
  // The shell keeps the active sidebar entry in its own state for now so the
  // sidebar is interactive even though page routing is not implemented yet.
  // Task 2.4 will replace this with a real route store.
  const [activeId, setActiveId] = useState<SidebarItemId>("home");

  return (
    <div className="flex min-h-screen">
      <Sidebar
        username={session.username}
        activeId={activeId}
        onSelect={setActiveId}
      />
      <div className="min-w-0 flex-1">{children}</div>
    </div>
  );
}
