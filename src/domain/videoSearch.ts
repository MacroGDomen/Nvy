import type {
  ActressRecord,
  TagRecord,
  VideoRecord,
  WorkType,
} from "../services/desktopApi/types";
import { actressMatchesQuery } from "./actressName";

export type SortMode = "updatedDesc" | "codeAsc" | "releaseDateDesc";

export type AgeRange = {
  min?: number;
  max?: number;
};

export type VideoSearchQuery = {
  keywords: string[];
  tagQueries: string[];
  ageRanges: AgeRange[];
};

export function parseVideoSearch(input: string): VideoSearchQuery {
  const tokens = input
    .split("+")
    .map((token) => token.trim())
    .filter(Boolean);

  const query: VideoSearchQuery = {
    keywords: [],
    tagQueries: [],
    ageRanges: [],
  };

  for (const token of tokens) {
    const ageRange = parseAgeRange(token);
    if (ageRange) {
      query.ageRanges.push(ageRange);
    } else if (token.startsWith("#")) {
      query.tagQueries.push(token);
    } else {
      query.keywords.push(token);
    }
  }

  return query;
}

export function parseAgeRange(token: string): AgeRange | null {
  const match = token.match(/^#\[(\d*)-?(\d*)\]$/);
  if (!match) {
    return null;
  }

  const min = match[1] ? Number(match[1]) : undefined;
  const max = match[2] ? Number(match[2]) : undefined;
  if (min === undefined && max === undefined) {
    return null;
  }

  return { min, max: max ?? (token.includes("-") ? undefined : min) };
}

export function filterVideos(
  videos: VideoRecord[],
  options: {
    searchText: string;
    workType?: WorkType | "all";
    sortMode?: SortMode;
    tagsByVideoId?: Record<string, TagRecord[]>;
    actressesByVideoId?: Record<string, ActressRecord[]>;
  },
): VideoRecord[] {
  const parsed = parseVideoSearch(options.searchText);
  const tagsByVideoId = options.tagsByVideoId ?? {};
  const actressesByVideoId = options.actressesByVideoId ?? {};

  return videos
    .filter((video) => {
      if (options.workType && options.workType !== "all" && video.workType !== options.workType) {
        return false;
      }

      const tags = tagsByVideoId[video.id] ?? [];
      const actresses = actressesByVideoId[video.id] ?? [];
      return (
        parsed.keywords.every((keyword) => videoMatchesKeyword(video, actresses, keyword)) &&
        parsed.tagQueries.every((tagQuery) => tags.some((tag) => tagMatches(tag, tagQuery))) &&
        parsed.ageRanges.every((range) => tags.some((tag) => ageTagMatches(tag, range)))
      );
    })
    .sort((first, second) => compareVideos(first, second, options.sortMode ?? "updatedDesc"));
}

function videoMatchesKeyword(
  video: VideoRecord,
  actresses: ActressRecord[],
  keyword: string,
) {
  const normalizedKeyword = keyword.toLocaleLowerCase();
  return (
    video.code.toLocaleLowerCase().includes(normalizedKeyword) ||
    video.title?.toLocaleLowerCase().includes(normalizedKeyword) ||
    video.summary?.toLocaleLowerCase().includes(normalizedKeyword) ||
    video.review?.toLocaleLowerCase().includes(normalizedKeyword) ||
    video.actorNames?.toLocaleLowerCase().includes(normalizedKeyword) ||
    actresses.some((actress) => actressMatchesQuery(actress, keyword))
  );
}

function tagMatches(tag: TagRecord, tagQuery: string) {
  const normalizedQuery = normalizeTag(tagQuery);
  return [tag.canonicalName, tag.aliases, tag.relatedTags]
    .flatMap((value) => splitTagList(value))
    .some((tagName) => normalizeTag(tagName) === normalizedQuery);
}

function ageTagMatches(tag: TagRecord, range: AgeRange) {
  const match = tag.canonicalName.match(/^#(\d+)岁$/);
  if (!match) {
    return false;
  }

  const age = Number(match[1]);
  return (range.min === undefined || age >= range.min) &&
    (range.max === undefined || age <= range.max);
}

function compareVideos(first: VideoRecord, second: VideoRecord, sortMode: SortMode) {
  if (sortMode === "codeAsc") {
    return first.code.localeCompare(second.code);
  }
  if (sortMode === "releaseDateDesc") {
    return (second.releaseDate ?? "").localeCompare(first.releaseDate ?? "");
  }

  return second.updatedAt.localeCompare(first.updatedAt);
}

function splitTagList(value?: string): string[] {
  return value?.split(",").map((tag) => tag.trim()).filter(Boolean) ?? [];
}

function normalizeTag(value: string) {
  return value.trim().trimStart().replace(/^#/, "").toLocaleLowerCase();
}
