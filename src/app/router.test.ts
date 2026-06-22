import { describe, expect, it } from "vitest";
import { resolveRoute, type ProtectedRoute } from "./router";

const session = {
  accountId: "account-3",
  username: "User789",
};

describe("resolveRoute", () => {
  const protectedRoutes: ProtectedRoute[] = [
    "home",
    "actresses",
    "videos",
    "settings",
  ];

  it("keeps unauthenticated users on the auth route", () => {
    expect(resolveRoute("auth", null)).toBe("auth");

    for (const route of protectedRoutes) {
      expect(resolveRoute(route, null)).toBe("auth");
    }
  });

  it("sends authenticated users away from the auth route", () => {
    expect(resolveRoute("auth", session)).toBe("home");
  });

  it("allows authenticated users to enter protected routes", () => {
    for (const route of protectedRoutes) {
      expect(resolveRoute(route, session)).toBe(route);
    }
  });
});
