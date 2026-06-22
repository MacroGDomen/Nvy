import { afterEach, describe, expect, it } from "vitest";
import {
  clearCurrentAccount,
  getCurrentAccount,
  requireCurrentAccount,
  setCurrentAccount,
} from "./currentAccount";

afterEach(() => {
  clearCurrentAccount();
});

describe("currentAccount", () => {
  it("stores and clears the current account in memory", () => {
    const session = {
      accountId: "account-1",
      username: "User123",
    };

    setCurrentAccount(session);

    expect(getCurrentAccount()).toEqual(session);
    expect(requireCurrentAccount()).toEqual(session);

    clearCurrentAccount();

    expect(getCurrentAccount()).toBeNull();
  });

  it("rejects account access before login", () => {
    expect(() => requireCurrentAccount()).toThrow("Current account is required.");
  });
});
