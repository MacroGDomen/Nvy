export function rotateItems<T>(
  items: readonly T[],
  offset: number,
  limit: number,
): T[] {
  if (items.length === 0 || limit <= 0) {
    return [];
  }

  const start = ((offset % items.length) + items.length) % items.length;
  const count = Math.min(limit, items.length);

  return Array.from({ length: count }, (_, index) => {
    return items[(start + index) % items.length];
  });
}
