import { describe, expect, it } from "vitest";
import { formatAppTitle } from "./appMetadata";

describe("formatAppTitle", () => {
  it("includes the version when one is present", () => {
    expect(formatAppTitle({ name: "Nvy", version: "0.1.0" })).toBe("Nvy 0.1.0");
  });

  it("omits an empty version", () => {
    expect(formatAppTitle({ name: "Nvy", version: "   " })).toBe("Nvy");
  });
});

