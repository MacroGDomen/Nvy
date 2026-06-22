import { afterEach, describe, expect, it } from "vitest";
import {
  clearSession,
  getSession,
  hasSession,
  requireSession,
  setSession,
} from "./sessionStore";

afterEach(() => {
  clearSession();
});

describe("sessionStore", () => {
  it("tracks whether an account session exists", () => {
    expect(hasSession()).toBe(false);
    expect(getSession()).toBeNull();

    const session = {
      accountId: "account-2",
      username: "User456",
    };

    setSession(session);

    expect(hasSession()).toBe(true);
    expect(getSession()).toEqual(session);
    expect(requireSession()).toEqual(session);
  });

  it("requires a session before protected routes can read account data", () => {
    expect(() => requireSession()).toThrow("Current account is required.");
  });
});
