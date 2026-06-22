import { useState } from "react";
import { resolveRoute, type AppRoute } from "./router";
import { AppShellPreview } from "../pages/AppShellPreview";
import { AuthPage } from "../pages/AuthPage";
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

  if (currentRoute === "auth") {
    return <AuthPage onAuthenticated={handleAuthenticated} />;
  }

  return <AppShellPreview />;
}
