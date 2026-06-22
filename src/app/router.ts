import type { AccountSession } from "../services/desktopApi/types";

export type AppRoute = "auth" | "home";

export function resolveRoute(
  requestedRoute: AppRoute,
  session: AccountSession | null,
): AppRoute {
  if (!session) {
    return "auth";
  }

  if (requestedRoute === "auth") {
    return "home";
  }

  return requestedRoute;
}
