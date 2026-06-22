import type { ActressRecord } from "../services/desktopApi/types";

export type DisplayNameType =
  | "simplifiedChinese"
  | "traditionalChinese"
  | "japanese"
  | "romanized"
  | "name";

const fullWidthOffset = 0xfee0;
const fallbackOrder: DisplayNameType[] = [
  "simplifiedChinese",
  "traditionalChinese",
  "japanese",
  "romanized",
  "name",
];

export function normalizeActressName(value: string): string {
  return value
    .normalize("NFKC")
    .split("")
    .map((character) => {
      const code = character.charCodeAt(0);
      if (code >= 0xff01 && code <= 0xff5e) {
        return String.fromCharCode(code - fullWidthOffset);
      }
      return character;
    })
    .join("")
    .toLocaleLowerCase()
    .replace(/[\s　·・.\-_／/\\|,，、;；]+/g, "");
}

export function actressSearchNames(actress: ActressRecord): string[] {
  return [
    actress.name,
    actress.simplifiedChineseName,
    actress.formerChineseNames,
    actress.traditionalChineseName,
    actress.japaneseName,
    actress.romanizedName,
  ]
    .flatMap((value) => splitNameList(value))
    .map(normalizeActressName)
    .filter(Boolean)
    .filter((value, index, names) => names.indexOf(value) === index);
}

export function actressMatchesQuery(actress: ActressRecord, query: string): boolean {
  const normalizedQuery = normalizeActressName(query);
  if (!normalizedQuery) {
    return true;
  }

  return actressSearchNames(actress).some((name) => name.includes(normalizedQuery));
}

export function displayActressName(
  actress: ActressRecord,
  preferredType?: DisplayNameType,
): string {
  const recordPreferredType = isDisplayNameType(actress.defaultDisplayNameType)
    ? actress.defaultDisplayNameType
    : undefined;
  const effectivePreferredType = preferredType ?? recordPreferredType;
  const orderedTypes = effectivePreferredType
    ? [
        effectivePreferredType,
        ...fallbackOrder.filter((type) => type !== effectivePreferredType),
      ]
    : fallbackOrder;

  for (const type of orderedTypes) {
    const value = nameByType(actress, type);
    if (value?.trim()) {
      return value.trim();
    }
  }

  return actress.name;
}

function isDisplayNameType(value?: string): value is DisplayNameType {
  return fallbackOrder.includes(value as DisplayNameType);
}

function nameByType(actress: ActressRecord, type: DisplayNameType) {
  const valueByType: Record<DisplayNameType, string | undefined> = {
    simplifiedChinese: actress.simplifiedChineseName,
    traditionalChinese: actress.traditionalChineseName,
    japanese: actress.japaneseName,
    romanized: actress.romanizedName,
    name: actress.name,
  };

  return valueByType[type];
}

function splitNameList(value?: string): string[] {
  return (
    value
      ?.split(/[,，、;；\n\r/／]+/)
      .map((name) => name.trim())
      .filter(Boolean) ?? []
  );
}
