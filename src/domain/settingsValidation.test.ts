import { describe, expect, it } from "vitest";
import {
  DEFAULT_RECOMMENDATION_REFERENCE_LIMIT,
  formatRecommendationReferenceLimit,
  parseRecommendationReferenceLimit,
} from "./settingsValidation";

describe("settingsValidation", () => {
  it("accepts 0 to 999 recommendation references", () => {
    expect(parseRecommendationReferenceLimit("0")).toBe(0);
    expect(parseRecommendationReferenceLimit("30")).toBe(30);
    expect(parseRecommendationReferenceLimit("999")).toBe(999);
  });

  it("rejects invalid recommendation reference limits", () => {
    expect(() => parseRecommendationReferenceLimit("")).toThrow();
    expect(() => parseRecommendationReferenceLimit("-1")).toThrow();
    expect(() => parseRecommendationReferenceLimit("1000")).toThrow();
    expect(() => parseRecommendationReferenceLimit("abc")).toThrow();
  });

  it("formats the default recommendation reference limit", () => {
    expect(formatRecommendationReferenceLimit()).toBe(
      String(DEFAULT_RECOMMENDATION_REFERENCE_LIMIT),
    );
  });
});
