import { describe, expect, it } from "vitest";
import { actressMatchesQuery, displayActressName, normalizeActressName } from "./actressName";
import { filterActresses, normalizeCupSize } from "./actressFilter";
import { parseAgeRange, parseVideoSearch } from "./videoSearch";
import type { ActressRecord } from "../services/desktopApi/types";

const baseActress: ActressRecord = {
  id: "actress-1",
  accountId: "account-1",
  name: "Alice",
  simplifiedChineseName: "爱丽丝",
  formerChineseNames: "旧名、旧称",
  traditionalChineseName: "愛麗絲",
  japaneseName: "アリス",
  romanizedName: "Alice",
  cupSize: "F",
  createdAt: "2026-01-01",
  updatedAt: "2026-01-01",
};

describe("stage 5 frontend rules", () => {
  it("normalizes actress names across width, case, spaces and separators", () => {
    expect(normalizeActressName("Ａlice ・ Test")).toBe("alicetest");
  });

  it("searches multi-language actress names", () => {
    expect(actressMatchesQuery(baseActress, "愛 麗 絲")).toBe(true);
    expect(actressMatchesQuery(baseActress, "旧称")).toBe(true);
    expect(actressMatchesQuery(baseActress, "unknown")).toBe(false);
  });

  it("falls back display names by preferred type", () => {
    expect(displayActressName(baseActress, "traditionalChinese")).toBe("愛麗絲");
    expect(displayActressName({ ...baseActress, traditionalChineseName: undefined }, "traditionalChinese")).toBe("爱丽丝");
  });

  it("parses video search tokens and age ranges", () => {
    expect(parseVideoSearch("abc + #按摩 + #[22-33]")).toEqual({
      keywords: ["abc"],
      tagQueries: ["#按摩"],
      ageRanges: [{ min: 22, max: 33 }],
    });
    expect(parseAgeRange("#[22-]")).toEqual({ min: 22, max: undefined });
    expect(parseAgeRange("#[-25]")).toEqual({ min: undefined, max: 25 });
    expect(parseAgeRange("#[25]")).toEqual({ min: 25, max: 25 });
  });

  it("filters actresses by valid cup sizes and excludes I/J", () => {
    expect(normalizeCupSize("I")).toBeUndefined();
    expect(normalizeCupSize("J")).toBeUndefined();
    expect(normalizeCupSize("K")).toBe("K");
    expect(
      filterActresses([baseActress], {
        searchText: "alice",
        selectedCupSizes: ["F"],
      }),
    ).toHaveLength(1);
  });
});
