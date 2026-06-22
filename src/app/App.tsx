import { useState } from "react";
import { resolveRoute, type AppRoute } from "./router";
import { AppShellPreview } from "../pages/AppShellPreview";
import { AuthPage } from "../pages/AuthPage";
import { AppShell } from "../components/layout/AppShell";
import type { AccountSession } from "../services/desktopApi/types";
import { setSession as setStoredSession } from "../services/auth/sessionStore";

export function App() {
  const [session, setReactSession] = useState<AccountSession | null>(null);
  const [requestedRoute, setRequestedRoute] = useState<AppRoute>("auth");
  const currentRoute = resolveRoute(requestedRoute, session);

  function handleAuthenticated(nextSession: AccountSession) {
    setReactSession(nextSession);
    setRequestedRoute("home");
    setStoredSession(nextSession);
  }

  if (currentRoute === "auth" || !session) {
    return <AuthPage onAuthenticated={handleAuthenticated} />;
  }

  // After login the authenticated app is rendered inside the shared AppShell,
  // which owns the collapsible sidebar. Task 2.3 keeps the home page as a
  // placeholder; task 2.4 will switch this body based on the active route.
  return (
    <AppShell session={session}>
      <AppShellPreview />
    </AppShell>
  );
}
