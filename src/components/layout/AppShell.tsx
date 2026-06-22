import type { ReactNode } from "react";
import type { ProtectedRoute } from "../../app/router";
import type { AccountSession } from "../../services/desktopApi/types";
import { Sidebar } from "./Sidebar";

type AppShellProps = {
  session: AccountSession;
  activeRoute: ProtectedRoute;
  onRouteChange: (route: ProtectedRoute) => void;
  children: ReactNode;
};

// AppShell is the shared layout for authenticated pages. It places the global
// collapsible sidebar on the left and the active page content on the right.
export function AppShell({
  session,
  activeRoute,
  onRouteChange,
  children,
}: AppShellProps) {
  return (
    <div className="flex min-h-screen">
      <Sidebar
        username={session.username}
        activeRoute={activeRoute}
        onSelect={onRouteChange}
      />
      <div className="min-w-0 flex-1">{children}</div>
    </div>
  );
}
