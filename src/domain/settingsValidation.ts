export const DEFAULT_RECOMMENDATION_REFERENCE_LIMIT = 30;

export function parseRecommendationReferenceLimit(value: string): number {
  const trimmed = value.trim();

  if (!/^\d+$/.test(trimmed)) {
    throw new Error("Recommendation reference limit must be a number.");
  }

  const parsed = Number(trimmed);
  if (!Number.isInteger(parsed) || parsed < 0 || parsed > 999) {
    throw new Error("Recommendation reference limit must be between 0 and 999.");
  }

  return parsed;
}

export function formatRecommendationReferenceLimit(value?: number): string {
  return String(value ?? DEFAULT_RECOMMENDATION_REFERENCE_LIMIT);
}
