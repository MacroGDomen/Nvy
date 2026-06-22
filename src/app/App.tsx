import { useState } from "react";
import {
  DEFAULT_AUTHENTICATED_ROUTE,
  resolveRoute,
  type AppRoute,
  type ProtectedRoute,
} from "./router";
import { ActressesPage } from "../pages/ActressesPage";
import { AuthPage } from "../pages/AuthPage";
import { HomePage } from "../pages/HomePage";
import { SettingsPage } from "../pages/SettingsPage";
import { VideosPage } from "../pages/VideosPage";
import { AppShell } from "../components/layout/AppShell";
import type { AccountSession } from "../services/desktopApi/types";
import { setSession as setStoredSession } from "../services/auth/sessionStore";

type DetailTarget =
  | { route: "actresses"; id: string }
  | { route: "videos"; id: string };

export function App() {
  const [session, setReactSession] = useState<AccountSession | null>(null);
  const [requestedRoute, setRequestedRoute] = useState<AppRoute>("auth");
  const [detailTarget, setDetailTarget] = useState<DetailTarget | null>(null);
  const currentRoute = resolveRoute(requestedRoute, session);

  function handleAuthenticated(nextSession: AccountSession) {
    setReactSession(nextSession);
    setRequestedRoute(DEFAULT_AUTHENTICATED_ROUTE);
    setStoredSession(nextSession);
  }

  function handleRouteChange(route: ProtectedRoute) {
    setRequestedRoute(route);
  }

  function handleOpenDetail(target: DetailTarget) {
    setDetailTarget(target);
    setRequestedRoute(target.route);
  }

  if (currentRoute === "auth" || !session) {
    return <AuthPage onAuthenticated={handleAuthenticated} />;
  }

  const activeRoute = currentRoute;

  return (
    <AppShell
      session={session}
      activeRoute={activeRoute}
      onRouteChange={handleRouteChange}
    >
      <RouteContent
        route={activeRoute}
        detailTarget={detailTarget}
        onDetailTargetConsumed={() => setDetailTarget(null)}
        onOpenDetail={handleOpenDetail}
      />
    </AppShell>
  );
}

type RouteContentProps = {
  route: ProtectedRoute;
  detailTarget: DetailTarget | null;
  onDetailTargetConsumed: () => void;
  onOpenDetail: (target: DetailTarget) => void;
};

function RouteContent({
  route,
  detailTarget,
  onDetailTargetConsumed,
  onOpenDetail,
}: RouteContentProps) {
  switch (route) {
    case "actresses":
      return (
        <ActressesPage
          focusActressId={detailTarget?.route === "actresses" ? detailTarget.id : null}
          onFocusConsumed={onDetailTargetConsumed}
        />
      );
    case "videos":
      return (
        <VideosPage
          focusVideoId={detailTarget?.route === "videos" ? detailTarget.id : null}
          onFocusConsumed={onDetailTargetConsumed}
        />
      );
    case "settings":
      return <SettingsPage />;
    case "home":
    default:
      return <HomePage onOpenDetail={onOpenDetail} />;
  }
}
