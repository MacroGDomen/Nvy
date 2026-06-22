import { describe, expect, it } from "vitest";
import { resolveRoute } from "./router";

const session = {
  accountId: "account-3",
  username: "User789",
};

describe("resolveRoute", () => {
  it("keeps unauthenticated users on the auth route", () => {
    expect(resolveRoute("auth", null)).toBe("auth");
    expect(resolveRoute("home", null)).toBe("auth");
  });

  it("sends authenticated users away from the auth route", () => {
    expect(resolveRoute("auth", session)).toBe("home");
  });

  it("allows authenticated users to enter protected routes", () => {
    expect(resolveRoute("home", session)).toBe("home");
  });
});
