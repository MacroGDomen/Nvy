import type {
  ActressRecord,
  RecommendationPayload,
  TagRecord,
  VideoRecord,
  WorkType,
} from "../services/desktopApi/types";

export type Stage10ScaleDataset = {
  accountId: string;
  videos: VideoRecord[];
  actresses: ActressRecord[];
  tagsByVideoId: Record<string, TagRecord[]>;
  tagsByActressId: Record<string, TagRecord[]>;
  actressesByVideoId: Record<string, ActressRecord[]>;
};

const accountId = "stage10-scale-account";
const updatedAt = "2026-06-22T08:00:00.000Z";
const workTypes: WorkType[] = ["single", "multiple", "amateur"];
const cupSizes = ["A", "B", "C", "D", "E", "F", "G", "H", "K"];

export function buildStage10ScaleDataset(
  videoCount = 1400,
  actressCount = 599,
): Stage10ScaleDataset {
  const actresses = Array.from({ length: actressCount }, (_, index) => {
    const serial = index + 1;
    const cupSize = cupSizes[index % cupSizes.length];

    return {
      id: `scale-actress-${serial}`,
      accountId,
      name: `Scale Actress ${serial}`,
      simplifiedChineseName: `Scale Actress CN ${serial}`,
      formerChineseNames: serial % 7 === 0 ? `Former Scale ${serial}` : undefined,
      traditionalChineseName: `Scale Actress TW ${serial}`,
      japaneseName: `Scale Actress JP ${serial}`,
      romanizedName: `scale-actress-${serial}`,
      defaultDisplayNameType: "simplifiedChinese",
      avatarPath: `actresses/scale-actress-${serial}.jpg`,
      measurements: `${80 + (index % 18)}-${55 + (index % 9)}-${82 + (index % 16)}`,
      cupSize,
      birthday: `199${index % 10}-0${(index % 9) + 1}-15`,
      heightCm: 150 + (index % 25),
      note: `scale note ${serial}`,
      createdAt: "2026-01-01T00:00:00.000Z",
      updatedAt,
    } satisfies ActressRecord;
  });

  const videos = Array.from({ length: videoCount }, (_, index) => {
    const serial = index + 1;
    const actress = actresses[index % actresses.length];

    return {
      id: `scale-video-${serial}`,
      accountId,
      code: `SCL-${String(serial).padStart(4, "0")}`,
      title: `Scale Video ${serial}`,
      coverPath: `covers/scale-video-${serial}.jpg`,
      releaseDate: `2026-${String((index % 12) + 1).padStart(2, "0")}-10`,
      durationMinutes: 75 + (index % 80),
      sourceUrl: `https://example.invalid/videos/${serial}`,
      summary: `scale summary ${serial} massage searchable local metadata`,
      actorNames: actress.simplifiedChineseName,
      workType: workTypes[index % workTypes.length],
      review: `scale review ${serial}`,
      createdAt: "2026-01-01T00:00:00.000Z",
      updatedAt: new Date(Date.UTC(2026, 5, 22, 8, 0, index % 60)).toISOString(),
    } satisfies VideoRecord;
  });

  const tagsByVideoId = Object.fromEntries(
    videos.map((video, index) => [
      video.id,
      [
        buildTag("video", `scale-video-tag-${index}-massage`, "#massage", "#按摩,#推拿"),
        buildTag("video", `scale-video-tag-${index}-age`, `#${22 + (index % 18)}岁`),
        buildTag("video", `scale-video-tag-${index}-cup`, `#${cupSizes[index % cupSizes.length]}Cup`),
      ],
    ]),
  );

  const tagsByActressId = Object.fromEntries(
    actresses.map((actress, index) => [
      actress.id,
      [
        buildTag("actress", `scale-actress-tag-${index}-style`, "#gentle", "#温柔,#治愈"),
        buildTag("actress", `scale-actress-tag-${index}-cup`, `#${cupSizes[index % cupSizes.length]}Cup`),
      ],
    ]),
  );

  const actressesByVideoId = Object.fromEntries(
    videos.map((video, index) => [video.id, [actresses[index % actresses.length]]]),
  );

  return {
    accountId,
    videos,
    actresses,
    tagsByVideoId,
    tagsByActressId,
    actressesByVideoId,
  };
}

export function buildStage10RecommendationPayload(
  dataset: Stage10ScaleDataset,
  userText = "prefer local searchable metadata",
  referenceLimit = 30,
): RecommendationPayload {
  const videos = dataset.videos.slice(0, referenceLimit).map((video) => ({
    id: video.id,
    code: video.code,
    title: video.title,
    summary: video.summary,
    actorNames: video.actorNames,
    workType: video.workType,
    review: video.review,
  }));
  const actresses = dataset.actresses.slice(0, referenceLimit).map((actress) => ({
    id: actress.id,
    name: actress.name,
    simplifiedChineseName: actress.simplifiedChineseName,
    japaneseName: actress.japaneseName,
    romanizedName: actress.romanizedName,
    measurements: actress.measurements,
    cupSize: actress.cupSize,
    birthday: actress.birthday,
    note: actress.note,
  }));

  return {
    userText,
    prompt: "Return local recommendations only.",
    referenceLimit,
    videos,
    actresses,
  };
}

function buildTag(
  scope: "video" | "actress",
  id: string,
  canonicalName: string,
  aliases?: string,
): TagRecord {
  return {
    id,
    accountId,
    scope,
    canonicalName,
    aliases,
    relatedTags: aliases,
    isPreset: false,
    createdAt: "2026-01-01T00:00:00.000Z",
    updatedAt,
  };
}
