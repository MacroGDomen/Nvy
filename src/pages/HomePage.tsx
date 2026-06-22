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
import { rotateItems } from "../domain/carousel";

type HomePageProps = {
  onOpenDetail: (target: { route: "actresses"; id: string } | { route: "videos"; id: string }) => void;
};

type ImageMap = Record<string, string>;

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
      setActressOffset((currentOffset) => (currentOffset + 1) % actresses.length);
    }, 1000);

    return () => window.clearInterval(timer);
  }, [actresses.length]);

  useEffect(() => {
    if (videos.length < 2) {
      return;
    }

    const timer = window.setInterval(() => {
      setVideoOffset((currentOffset) => (currentOffset + 1) % videos.length);
    }, 1000);

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
    () => rotateItems(actresses, actressOffset, 7),
    [actresses, actressOffset],
  );
  const visibleVideos = useMemo(
    () => rotateItems(videos, videoOffset, 6),
    [videos, videoOffset],
  );

  return (
    <main className="min-h-screen bg-[var(--color-background)] px-6 py-7 text-[var(--color-text)] lg:px-8">
      <section className="mx-auto grid max-w-7xl gap-6">
        <header className="flex flex-wrap items-end justify-between gap-4 border-b border-[var(--color-border)] pb-5">
          <div>
            <p className="mb-2 text-sm font-medium tracking-normal text-[var(--color-accent-soft)]">
              Home
            </p>
            <h1 className="text-3xl font-semibold tracking-normal text-[var(--color-text-strong)]">
              首页
            </h1>
          </div>
          <p className="text-sm text-[var(--color-muted)]">
            {isLoadingLibrary
              ? "加载中"
              : `${videos.length} 部影片 / ${actresses.length} 位女优`}
          </p>
        </header>

        <div className="grid gap-6 xl:grid-cols-[minmax(0,1fr)_390px]">
          <section className="grid gap-6">
            <HomeActressCarousel
              actresses={visibleActresses}
              imageUrls={actressImages}
              onOpen={(actressId) => onOpenDetail({ route: "actresses", id: actressId })}
            />
            <HomeVideoCarousel
              videos={visibleVideos}
              imageUrls={videoImages}
              onOpen={(videoId) => onOpenDetail({ route: "videos", id: videoId })}
            />
          </section>

          <section className="grid content-start gap-5">
            <HomeRecommendationBox
              preference={preference}
              isSubmitting={isSubmitting}
              onPreferenceChange={setPreference}
              onSubmit={handleSubmit}
            />
            {payload ? <RecommendationPreview payload={payload} /> : null}
          </section>
        </div>
      </section>
    </main>
  );
}

function HomeActressCarousel({
  actresses,
  imageUrls,
  onOpen,
}: {
  actresses: ActressRecord[];
  imageUrls: ImageMap;
  onOpen: (actressId: string) => void;
}) {
  return (
    <section className="grid gap-4">
      <SectionHeader title="女优轮播" subtitle="点击头像进入详情" />
      {actresses.length === 0 ? (
        <EmptyPanel text="还没有女优记录" />
      ) : (
        <div className="grid grid-cols-3 items-center gap-3 overflow-hidden rounded-2xl border border-[var(--color-border)] bg-[var(--color-surface-soft)] p-4 md:grid-cols-5 xl:grid-cols-7">
          {actresses.map((actress, index) => {
            const isCenter = index === Math.floor(actresses.length / 2);

            return (
              <button
                key={actress.id}
                type="button"
                onClick={() => onOpen(actress.id)}
                className={[
                  "grid min-w-0 place-items-center gap-3 rounded-2xl border px-3 py-4 text-center transition hover:border-[var(--color-border-strong)] hover:bg-[var(--color-surface-hover)]",
                  isCenter
                    ? "border-[var(--color-accent)] bg-[rgba(165,140,223,0.16)]"
                    : "border-transparent bg-transparent opacity-78",
                ].join(" ")}
              >
                <span
                  className={[
                    "grid place-items-center overflow-hidden rounded-full border border-[var(--color-border)] bg-[var(--color-input)] text-lg font-semibold text-[var(--color-accent-soft)] transition",
                    isCenter ? "h-28 w-28" : "h-20 w-20",
                  ].join(" ")}
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
                <span className="w-full truncate text-sm font-semibold text-[var(--color-text-strong)]">
                  {actress.simplifiedChineseName || actress.name}
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
  videos: VideoRecord[];
  imageUrls: ImageMap;
  onOpen: (videoId: string) => void;
}) {
  return (
    <section className="grid gap-4">
      <SectionHeader title="影片轮播" subtitle="点击封面进入详情" />
      {videos.length === 0 ? (
        <EmptyPanel text="还没有影片记录" />
      ) : (
        <div className="grid grid-cols-2 gap-3 overflow-hidden rounded-2xl border border-[var(--color-border)] bg-[var(--color-surface-soft)] p-4 md:grid-cols-3 xl:grid-cols-6">
          {videos.map((video, index) => {
            const isCenter = index === Math.floor(videos.length / 2);

            return (
              <button
                key={video.id}
                type="button"
                onClick={() => onOpen(video.id)}
                className={[
                  "grid min-w-0 gap-3 rounded-2xl border p-3 text-left transition hover:border-[var(--color-border-strong)] hover:bg-[var(--color-surface-hover)]",
                  isCenter
                    ? "border-[var(--color-accent)] bg-[rgba(165,140,223,0.14)]"
                    : "border-transparent bg-transparent",
                ].join(" ")}
              >
                <span className="grid aspect-[3/4] w-full place-items-center overflow-hidden rounded-xl border border-[var(--color-border)] bg-[var(--color-input)] text-sm font-semibold text-[var(--color-accent-soft)]">
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
                <span className="line-clamp-2 min-h-10 text-sm font-semibold leading-5 text-[var(--color-text-strong)]">
                  {video.title || video.code}
                </span>
                <span className="truncate text-xs text-[var(--color-muted)]">{video.code}</span>
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
    <section className="grid gap-4 rounded-2xl border border-[var(--color-border)] bg-[var(--color-panel)] p-5 shadow-[var(--shadow-panel)]">
      <div>
        <h2 className="text-xl font-semibold text-[var(--color-text-strong)]">首页推荐</h2>
        <p className="mt-1 text-sm text-[var(--color-muted)]">
          输入偏好后会从本地资料库构造候选；不会发送密码、API Key 或本地绝对路径。
        </p>
      </div>

      <form onSubmit={onSubmit} className="grid gap-3">
        <textarea
          value={preference}
          onChange={(event) => onPreferenceChange(event.target.value)}
          rows={8}
          placeholder="写下今天想看的偏好、关键词或排除条件"
          className="min-h-48 resize-y rounded-2xl border border-[var(--color-border)] bg-[var(--color-input)] px-4 py-4 text-[var(--color-text)] outline-none transition placeholder:text-[var(--color-muted-subtle)] focus:border-[var(--color-focus)] focus:ring-2 focus:ring-[var(--color-focus-soft)]"
        />
        <div className="flex flex-wrap items-center justify-between gap-3">
          <span className="text-xs text-[var(--color-muted)]">真实请求等待 HTTP 客户端接入</span>
          <Button type="submit" disabled={isSubmitting}>
            构造推荐候选
          </Button>
        </div>
      </form>
    </section>
  );
}

function RecommendationPreview({ payload }: { payload: RecommendationPayload }) {
  return (
    <section className="grid gap-4 rounded-2xl border border-[var(--color-border)] bg-[var(--color-surface-soft)] p-5">
      <div>
        <h2 className="text-lg font-semibold text-[var(--color-text-strong)]">候选预览</h2>
        <p className="mt-1 text-sm text-[var(--color-muted)]">
          当前参考上限：{payload.referenceLimit === 0 ? "不限" : payload.referenceLimit}
          ，候选合计 {payload.videos.length + payload.actresses.length} 条。
        </p>
      </div>

      <div className="grid gap-4">
        <CandidateList
          title="影片候选"
          emptyText="暂无影片候选"
          items={payload.videos.map((video) => ({
            id: video.id,
            title: video.title || video.code,
            description: [video.code, video.actorNames, video.workType]
              .filter(Boolean)
              .join(" / "),
          }))}
        />
        <CandidateList
          title="女优候选"
          emptyText="暂无女优候选"
          items={payload.actresses.map((actress) => ({
            id: actress.id,
            title: actress.simplifiedChineseName || actress.name,
            description: [actress.japaneseName, actress.romanizedName, actress.cupSize]
              .filter(Boolean)
              .join(" / "),
          }))}
        />
      </div>
    </section>
  );
}

function CandidateList({
  title,
  emptyText,
  items,
}: {
  title: string;
  emptyText: string;
  items: Array<{ id: string; title: string; description: string }>;
}) {
  return (
    <div className="grid content-start gap-2">
      <h3 className="text-sm font-semibold text-[var(--color-text-strong)]">{title}</h3>
      {items.length === 0 ? (
        <p className="rounded-2xl border border-dashed border-[var(--color-border)] px-4 py-5 text-sm text-[var(--color-muted)]">
          {emptyText}
        </p>
      ) : (
        items.slice(0, 5).map((item) => (
          <div
            key={item.id}
            className="rounded-xl border border-[var(--color-border)] bg-[var(--color-input)] px-4 py-3"
          >
            <p className="truncate text-sm font-semibold text-[var(--color-text-strong)]">
              {item.title}
            </p>
            <p className="mt-1 truncate text-xs text-[var(--color-muted)]">
              {item.description || "无补充字段"}
            </p>
          </div>
        ))
      )}
    </div>
  );
}

function SectionHeader({ title, subtitle }: { title: string; subtitle: string }) {
  return (
    <div className="flex items-end justify-between gap-3">
      <h2 className="text-xl font-semibold text-[var(--color-text-strong)]">{title}</h2>
      <span className="text-xs text-[var(--color-muted)]">{subtitle}</span>
    </div>
  );
}

function EmptyPanel({ text }: { text: string }) {
  return (
    <div className="rounded-2xl border border-dashed border-[var(--color-border)] bg-[var(--color-surface-soft)] px-4 py-12 text-center text-sm text-[var(--color-muted)]">
      {text}
    </div>
  );
}

function isImageEntry(
  entry: readonly [string, string] | null,
): entry is readonly [string, string] {
  return entry !== null;
}
