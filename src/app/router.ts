import type { AccountSession } from "../services/desktopApi/types";

export type ProtectedRoute = "home" | "actresses" | "videos" | "settings";

export type AppRoute = "auth" | ProtectedRoute;

export const DEFAULT_AUTHENTICATED_ROUTE: ProtectedRoute = "home";

export function resolveRoute(
  requestedRoute: AppRoute,
  session: AccountSession | null,
): AppRoute {
  if (!session) {
    return "auth";
  }

  if (requestedRoute === "auth") {
    return DEFAULT_AUTHENTICATED_ROUTE;
  }

  return requestedRoute;
}
