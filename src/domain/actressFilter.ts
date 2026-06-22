import type { ActressRecord, TagRecord } from "../services/desktopApi/types";
import { actressMatchesQuery } from "./actressName";

export const allowedCupSizes = ["A", "B", "C", "D", "E", "F", "G", "H", "K"] as const;

export type CupSize = (typeof allowedCupSizes)[number];

export function normalizeCupSize(value?: string): CupSize | undefined {
  const upper = value?.trim().toLocaleUpperCase();
  return allowedCupSizes.find((cupSize) => cupSize === upper);
}

export function filterActresses(
  actresses: ActressRecord[],
  options: {
    searchText: string;
    selectedCupSizes?: CupSize[];
    tagsByActressId?: Record<string, TagRecord[]>;
  },
) {
  const selectedCupSizes = options.selectedCupSizes ?? [];
  const tagQuery = options.searchText.trim().startsWith("#")
    ? options.searchText.trim()
    : undefined;

  return actresses.filter((actress) => {
    const cupSize = normalizeCupSize(actress.cupSize);
    const cupMatches =
      selectedCupSizes.length === 0 || (cupSize ? selectedCupSizes.includes(cupSize) : false);
    const textMatches = tagQuery
      ? (options.tagsByActressId?.[actress.id] ?? []).some((tag) =>
          tagMatches(tag, tagQuery),
        )
      : actressMatchesQuery(actress, options.searchText);

    return cupMatches && textMatches;
  });
}

function tagMatches(tag: TagRecord, query: string) {
  const normalizedQuery = normalizeTag(query);
  return [tag.canonicalName, tag.aliases, tag.relatedTags]
    .flatMap((value) => value?.split(",") ?? [])
    .map((value) => normalizeTag(value))
    .includes(normalizedQuery);
}

function normalizeTag(value: string) {
  return value.trim().replace(/^#/, "").toLocaleLowerCase();
}
