import { FormEvent, useEffect, useMemo, useState } from "react";
import {
  buildRecommendationPayload,
  cachedAssetUrl,
  listActresses,
  listVideos,
} from "../services/desktopApi";
import type {
  ActressRecord,
  RecommendationPayload,
  VideoRecord,
} from "../services/desktopApi/types";
import { requireSession } from "../services/auth/sessionStore";
import { Button } from "../components/ui/Button";
import { useToast } from "../components/ui/Toast";

type HomePageProps = {
  onOpenDetail: (
    target:
      | { route: "actresses"; id: string; returnRoute: "home" }
      | { route: "videos"; id: string; returnRoute: "home" },
  ) => void;
};

type ImageMap = Record<string, string>;
const HOME_VIDEO_CAROUSEL_CENTER_X = "59%";
const HOME_ACTRESS_CAROUSEL_CENTER_X = "54%";

type CarouselItem<T> = {
  item: T;
  key: string;
  slot: number;
};

export function HomePage({ onOpenDetail }: HomePageProps) {
  const [preference, setPreference] = useState("");
  const [payload, setPayload] = useState<RecommendationPayload | null>(null);
  const [videos, setVideos] = useState<VideoRecord[]>([]);
  const [actresses, setActresses] = useState<ActressRecord[]>([]);
  const [videoImages, setVideoImages] = useState<ImageMap>({});
  const [actressImages, setActressImages] = useState<ImageMap>({});
  const [videoOffset, setVideoOffset] = useState(0);
  const [actressOffset, setActressOffset] = useState(0);
  const [isSubmitting, setIsSubmitting] = useState(false);
  const [isLoadingLibrary, setIsLoadingLibrary] = useState(true);
  const { notify } = useToast();

  useEffect(() => {
    let isActive = true;
    const session = requireSession();

    Promise.all([listVideos(session.accountId), listActresses(session.accountId)])
      .then(([nextVideos, nextActresses]) => {
        if (!isActive) {
          return;
        }
        setVideos(nextVideos);
        setActresses(nextActresses);
      })
      .catch(() => {
        if (isActive) {
          notify({
            title: "首页资料加载失败",
            description: "影片和女优轮播暂时不可用，可以稍后重试。",
            variant: "error",
          });
        }
      })
      .finally(() => {
        if (isActive) {
          setIsLoadingLibrary(false);
        }
      });

    return () => {
      isActive = false;
    };
  }, [notify]);

  useEffect(() => {
    let isActive = true;

    Promise.all(
      videos.map(async (video) => {
        if (!video.coverPath) {
          return null;
        }
        try {
          return [video.id, await cachedAssetUrl(video.coverPath)] as const;
        } catch {
          return null;
        }
      }),
    ).then((entries) => {
      if (isActive) {
        setVideoImages(Object.fromEntries(entries.filter(isImageEntry)));
      }
    });

    return () => {
      isActive = false;
    };
  }, [videos]);

  useEffect(() => {
    let isActive = true;

    Promise.all(
      actresses.map(async (actress) => {
        if (!actress.avatarPath) {
          return null;
        }
        try {
          return [actress.id, await cachedAssetUrl(actress.avatarPath)] as const;
        } catch {
          return null;
        }
      }),
    ).then((entries) => {
      if (isActive) {
        setActressImages(Object.fromEntries(entries.filter(isImageEntry)));
      }
    });

    return () => {
      isActive = false;
    };
  }, [actresses]);

  useEffect(() => {
    if (actresses.length < 2) {
      return;
    }

    const timer = window.setInterval(() => {
      setActressOffset((currentOffset) => currentOffset - 1);
    }, 3000);

    return () => window.clearInterval(timer);
  }, [actresses.length]);

  useEffect(() => {
    if (videos.length < 2) {
      return;
    }

    const timer = window.setInterval(() => {
      setVideoOffset((currentOffset) => currentOffset - 1);
    }, 3000);

    return () => window.clearInterval(timer);
  }, [videos.length]);

  async function handleSubmit(event: FormEvent<HTMLFormElement>) {
    event.preventDefault();
    setIsSubmitting(true);

    try {
      const session = requireSession();
      const nextPayload = await buildRecommendationPayload(session.accountId, preference);
      setPayload(nextPayload);
      notify({
        title: "推荐候选已构造",
        description: "当前会先预览发送边界；真实模型调用等待 HTTP 客户端接入。",
        variant: "info",
      });
    } catch {
      notify({
        title: "推荐暂不可用",
        description: "请确认已输入偏好文本，并检查大模型设置是否有效。",
        variant: "error",
      });
    } finally {
      setIsSubmitting(false);
    }
  }

  const visibleActresses = useMemo(
    () => buildCarouselWindow(actresses, actressOffset, 7, (actress) => actress.id),
    [actresses, actressOffset],
  );
  const visibleVideos = useMemo(
    () => buildCarouselWindow(videos, videoOffset, 5, (video) => video.id),
    [videos, videoOffset],
  );

  return (
    <main className="relative h-full overflow-hidden bg-[#101014] text-[var(--color-text)]">
      <div className="pointer-events-none absolute inset-0 bg-[radial-gradient(circle_at_54%_55%,rgba(71,84,157,0.28),transparent_31%),radial-gradient(circle_at_75%_22%,rgba(202,139,183,0.12),transparent_24%)]" />
      <section className="relative grid h-full grid-rows-[minmax(0,1fr)_15.5rem] px-14 pb-8 pt-9">
        <div className="relative min-h-0">
          {isLoadingLibrary ? (
            <div className="absolute right-0 top-0 text-xs text-[var(--color-muted)]">
              加载资料中
            </div>
          ) : null}
          <HomeActressCarousel
            actresses={visibleActresses}
            imageUrls={actressImages}
            onOpen={(actressId) =>
              onOpenDetail({ route: "actresses", id: actressId, returnRoute: "home" })
            }
          />
          <HomeVideoCarousel
            videos={visibleVideos}
            imageUrls={videoImages}
            onOpen={(videoId) =>
              onOpenDetail({ route: "videos", id: videoId, returnRoute: "home" })
            }
          />
        </div>

        <section className="grid content-end justify-items-center gap-5">
          <p className="text-center text-3xl font-medium tracking-normal text-[rgba(245,240,250,0.84)]">
            你好，今天你想从谁开始？
          </p>
          <HomeRecommendationBox
            preference={preference}
            isSubmitting={isSubmitting}
            onPreferenceChange={setPreference}
            onSubmit={handleSubmit}
          />
          {payload ? <RecommendationPreview payload={payload} /> : null}
        </section>
      </section>
    </main>
  );
}

function HomeActressCarousel({
  actresses,
  imageUrls,
  onOpen,
}: {
  actresses: Array<CarouselItem<ActressRecord>>;
  imageUrls: ImageMap;
  onOpen: (actressId: string) => void;
}) {
  const centerX = HOME_ACTRESS_CAROUSEL_CENTER_X;
  const arc = [
    { x: "-10%", y: "27%", scale: 0.42, opacity: 0, zIndex: 1 },
    { x: "5%", y: "20%", scale: 0.58, opacity: 0.36, zIndex: 2 },
    { x: "20%", y: "11%", scale: 0.74, opacity: 0.58, zIndex: 3 },
    { x: "37%", y: "5%", scale: 0.9, opacity: 0.78, zIndex: 4 },
    { x: centerX, y: "0%", scale: 1.18, opacity: 1, zIndex: 6 },
    { x: "71%", y: "5%", scale: 0.9, opacity: 0.78, zIndex: 4 },
    { x: "88%", y: "11%", scale: 0.74, opacity: 0.58, zIndex: 3 },
    { x: "103%", y: "20%", scale: 0.58, opacity: 0.32, zIndex: 2 },
    { x: "117%", y: "27%", scale: 0.42, opacity: 0, zIndex: 1 },
  ];

  return (
    <section className="absolute inset-x-0 top-0 h-[18.5rem]">
      {actresses.length === 0 ? (
        <EmptyPanel className="mx-auto mt-14 w-80" text="还没有女优记录" />
      ) : (
        <div className="relative h-full">
          {actresses.map(({ item: actress, key, slot }) => {
            const position = arc[slot] ?? arc[0];
            const displayName = actress.simplifiedChineseName || actress.name;

            return (
              <button
                key={key}
                type="button"
                onClick={() => onOpen(actress.id)}
                className="absolute grid -translate-x-1/2 place-items-center gap-2 text-center transition-[left,top,opacity,transform] duration-[900ms] ease-in-out hover:opacity-100"
                style={{
                  left: position.x,
                  top: position.y,
                  opacity: position.opacity,
                  transform: `translateX(-50%) scale(${position.scale})`,
                  zIndex: position.zIndex,
                }}
              >
                <span
                  className="grid h-28 w-28 place-items-center overflow-hidden rounded-full border border-[rgba(255,255,255,0.12)] bg-[var(--color-input)] text-2xl font-semibold text-[var(--color-accent-soft)] shadow-[0_18px_48px_rgba(0,0,0,0.36)]"
                >
                  {imageUrls[actress.id] ? (
                    <img
                      src={imageUrls[actress.id]}
                      alt=""
                      className="h-full w-full object-cover"
                    />
                  ) : (
                    actress.name.trim().slice(0, 1) || "?"
                  )}
                </span>
                <span className="w-32 truncate text-sm font-medium text-[rgba(245,240,250,0.86)]">
                  {displayName}
                </span>
              </button>
            );
          })}
        </div>
      )}
    </section>
  );
}

function HomeVideoCarousel({
  videos,
  imageUrls,
  onOpen,
}: {
  videos: Array<CarouselItem<VideoRecord>>;
  imageUrls: ImageMap;
  onOpen: (videoId: string) => void;
}) {
  const centerX = HOME_VIDEO_CAROUSEL_CENTER_X;
  const arc = [
    { x: "3%", y: "58%", scale: 0.5, opacity: 0, zIndex: 1 },
    { x: "21%", y: "49%", scale: 0.72, opacity: 0.55, zIndex: 2 },
    { x: "40%", y: "39%", scale: 0.9, opacity: 0.78, zIndex: 4 },
    { x: centerX, y: "31%", scale: 1.2, opacity: 1, zIndex: 7 },
    { x: "78%", y: "39%", scale: 0.9, opacity: 0.78, zIndex: 3 },
    { x: "97%", y: "49%", scale: 0.72, opacity: 0.55, zIndex: 2 },
    { x: "115%", y: "58%", scale: 0.5, opacity: 0, zIndex: 1 },
  ];

  return (
    <section className="absolute inset-x-0 top-20 h-[24rem]">
      {videos.length === 0 ? (
        <EmptyPanel className="mx-auto mt-32 w-96" text="还没有影片记录" />
      ) : (
        <div className="relative h-full">
          {videos.map(({ item: video, key, slot }) => {
            const position = arc[slot] ?? arc[0];
            const title = `${video.code} ${video.title || ""}`.trim();

            return (
              <button
                key={key}
                type="button"
                onClick={() => onOpen(video.id)}
                className="absolute grid -translate-x-1/2 gap-2 text-left transition-[left,top,opacity,transform] duration-[900ms] ease-in-out hover:opacity-100"
                style={{
                  left: position.x,
                  top: position.y,
                  opacity: position.opacity,
                  transform: `translateX(-50%) scale(${position.scale})`,
                  zIndex: position.zIndex,
                }}
              >
                <span className="grid h-36 w-56 place-items-center overflow-hidden rounded-[1.25rem] border border-[rgba(255,255,255,0.12)] bg-[var(--color-input)] text-sm font-semibold text-[var(--color-accent-soft)] shadow-[0_18px_56px_rgba(0,0,0,0.44)]">
                  {imageUrls[video.id] ? (
                    <img
                      src={imageUrls[video.id]}
                      alt=""
                      className="h-full w-full object-cover"
                    />
                  ) : (
                    video.code
                  )}
                </span>
                <span className="nvy-line-clamp-2 w-56 text-sm font-medium leading-5 text-[rgba(245,240,250,0.86)]">
                  {title}
                </span>
              </button>
            );
          })}
        </div>
      )}
    </section>
  );
}

function HomeRecommendationBox({
  preference,
  isSubmitting,
  onPreferenceChange,
  onSubmit,
}: {
  preference: string;
  isSubmitting: boolean;
  onPreferenceChange: (value: string) => void;
  onSubmit: (event: FormEvent<HTMLFormElement>) => void;
}) {
  return (
      <form
        onSubmit={onSubmit}
        className="grid w-[min(61rem,78vw)] gap-3 rounded-[1.45rem] border border-[rgba(255,255,255,0.08)] bg-[rgba(35,35,38,0.94)] px-4 py-4 shadow-[0_28px_80px_rgba(0,0,0,0.38)]"
      >
        <textarea
          value={preference}
          onChange={(event) => onPreferenceChange(event.target.value)}
          rows={3}
          placeholder="写下今天想看的偏好、关键词或排除条件"
          className="min-h-20 resize-none rounded-2xl border-0 bg-transparent px-1 py-1 text-[var(--color-text)] outline-none placeholder:text-[var(--color-muted-subtle)]"
        />
        <div className="flex flex-wrap items-center justify-between gap-3">
          <span className="text-xs text-[var(--color-muted)]">本地候选 + 首页推荐</span>
          <Button type="submit" disabled={isSubmitting} className="h-9 px-4">
            发送
          </Button>
        </div>
      </form>
  );
}

function RecommendationPreview({ payload }: { payload: RecommendationPayload }) {
  return (
    <section className="max-w-[52rem] text-center text-xs text-[var(--color-muted)]">
      当前参考上限：{payload.referenceLimit === 0 ? "不限" : payload.referenceLimit}
      ，候选合计 {payload.videos.length + payload.actresses.length} 条。
    </section>
  );
}

function EmptyPanel({ text, className }: { text: string; className?: string }) {
  return (
    <div className={`rounded-2xl border border-dashed border-[var(--color-border)] bg-[var(--color-surface-soft)] px-4 py-10 text-center text-sm text-[var(--color-muted)] ${className ?? ""}`}>
      {text}
    </div>
  );
}

function isImageEntry(
  entry: readonly [string, string] | null,
): entry is readonly [string, string] {
  return entry !== null;
}

function buildCarouselWindow<T>(
  items: readonly T[],
  firstVisibleOffset: number,
  visibleCount: number,
  getId: (item: T) => string,
): Array<CarouselItem<T>> {
  if (items.length === 0 || visibleCount <= 0) {
    return [];
  }

  const windowCount = Math.min(visibleCount + 2, items.length + 2);
  const start = firstVisibleOffset - 1;

  return Array.from({ length: windowCount }, (_, slot) => {
    const absoluteIndex = start + slot;
    const item = items[positiveModulo(absoluteIndex, items.length)];

    return {
      item,
      key: `${getId(item)}:${absoluteIndex}`,
      slot,
    };
  });
}

function positiveModulo(value: number, divisor: number) {
  return ((value % divisor) + divisor) % divisor;
}
