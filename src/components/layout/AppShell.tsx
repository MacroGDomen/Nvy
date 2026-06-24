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

export function AppShell({
  session,
  activeRoute,
  onRouteChange,
  children,
}: AppShellProps) {
  return (
    <div className="nvy-viewport">
      <div className="nvy-stage">
        <Sidebar
          username={session.username}
          activeRoute={activeRoute}
          onSelect={onRouteChange}
        />
        <div className="nvy-page">{children}</div>
      </div>
    </div>
  );
}
