import { FormEvent, RefObject, useCallback, useEffect, useRef, useState } from "react";
import {
  applyMetadataCandidate,
  autoTagVideo,
  cancelVideoTranslation,
  cachedAssetUrl,
  cacheLocalImage,
  deleteVideo,
  listActresses,
  listTags,
  listVideoActresses,
  listVideoTags,
  listVideos,
  matchVideoMetadata,
  requestVideoTranslation,
  setVideoActresses,
  setVideoTags,
  updateVideo,
  createVideo,
} from "../services/desktopApi";
import type {
  ActressRecord,
  TagRecord,
  MetadataCandidate,
  VideoInput,
  VideoRecord,
  WorkType,
} from "../services/desktopApi/types";
import { filterVideos, SortMode } from "../domain/videoSearch";
import { requireSession } from "../services/auth/sessionStore";
import { useAutosave } from "../services/autosave/useAutosave";
import { Button } from "../components/ui/Button";
import { ConfirmDialog } from "../components/ui/ConfirmDialog";
import { Dropdown } from "../components/ui/Dropdown";
import { Input } from "../components/ui/Input";
import { useToast } from "../components/ui/Toast";

type SaveStatus = "idle" | "saving" | "saved" | "error";

type VideosPageProps = {
  focusVideoId?: string | null;
  backLabel?: string;
  onBackToLibrary?: () => void;
  onOpenDetail?: (videoId: string) => void;
};

export function VideosPage({
  focusVideoId = null,
  backLabel = "返回影片库",
  onBackToLibrary,
  onOpenDetail,
}: VideosPageProps) {
  const [videos, setVideos] = useState<VideoRecord[]>([]);
  const [selectedVideo, setSelectedVideo] = useState<VideoRecord | null>(null);
  const [coverUrlsById, setCoverUrlsById] = useState<Record<string, string>>({});
  const [code, setCode] = useState("");
  const [title, setTitle] = useState("");
  const [workType, setWorkType] = useState<WorkType>("single");
  const [searchText, setSearchText] = useState("");
  const [workTypeFilter, setWorkTypeFilter] = useState<WorkType | "all">("all");
  const [sortMode, setSortMode] = useState<SortMode>("updatedDesc");
  const [videoTagsById, setVideoTagsById] = useState<Record<string, TagRecord[]>>({});
  const [videoActressesById, setVideoActressesById] = useState<Record<string, ActressRecord[]>>({});
  const [isLoading, setIsLoading] = useState(true);
  const [isSubmitting, setIsSubmitting] = useState(false);
  const { notify } = useToast();

  useEffect(() => {
    let isActive = true;
    const session = requireSession();

    listVideos(session.accountId)
      .then(async (nextVideos) => {
        if (!isActive) {
          return;
        }
        const tagEntries = await Promise.all(
          nextVideos.map(async (video) => [
            video.id,
            await listVideoTags(session.accountId, video.id),
          ] as const),
        );
        const actressEntries = await Promise.all(
          nextVideos.map(async (video) => [
            video.id,
            await listVideoActresses(session.accountId, video.id),
          ] as const),
        );
        if (!isActive) {
          return;
        }
        setVideos(nextVideos);
        setVideoTagsById(Object.fromEntries(tagEntries));
        setVideoActressesById(Object.fromEntries(actressEntries));
        setSelectedVideo(
          nextVideos.find((video) => video.id === focusVideoId) ??
            nextVideos[0] ??
            null,
        );
      })
      .catch(() => {
        if (isActive) {
          notify({
            title: "影片列表加载失败",
            description: "请稍后重试。",
            variant: "error",
          });
        }
      })
      .finally(() => {
        if (isActive) {
          setIsLoading(false);
        }
      });

    return () => {
      isActive = false;
    };
  }, [focusVideoId, notify]);

  useEffect(() => {
    if (!focusVideoId || videos.length === 0) {
      return;
    }

    const focusedVideo = videos.find((video) => video.id === focusVideoId);
    if (focusedVideo) {
      setSelectedVideo(focusedVideo);
    }
  }, [focusVideoId, videos]);

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
        setCoverUrlsById(Object.fromEntries(entries.filter(isImageEntry)));
      }
    });

    return () => {
      isActive = false;
    };
  }, [videos]);

  async function handleCreateVideo(event: FormEvent<HTMLFormElement>) {
    event.preventDefault();
    setIsSubmitting(true);

    try {
      const session = requireSession();
      const created = await createVideo(session.accountId, {
        code,
        title,
        workType,
      });
      setVideos((currentVideos) => [created, ...currentVideos]);
      setVideoTagsById((currentTagsById) => ({
        ...currentTagsById,
        [created.id]: [],
      }));
      setVideoActressesById((currentActressesById) => ({
        ...currentActressesById,
        [created.id]: [],
      }));
      setSelectedVideo(created);
      setCode("");
      setTitle("");
      setWorkType("single");
      notify({ title: "影片已添加", variant: "success" });

      window.setTimeout(() => {
        void applyFirstVideoMetadataCandidate(session.accountId, created)
          .then(async (nextVideo) => {
            const [nextTags, nextActresses] = await Promise.all([
              listVideoTags(session.accountId, nextVideo.id),
              listVideoActresses(session.accountId, nextVideo.id),
            ]);
            setVideos((currentVideos) =>
              currentVideos.map((video) =>
                video.id === nextVideo.id ? nextVideo : video,
              ),
            );
            setSelectedVideo((currentVideo) =>
              currentVideo?.id === nextVideo.id ? nextVideo : currentVideo,
            );
            setVideoTagsById((currentTagsById) => ({
              ...currentTagsById,
              [nextVideo.id]: nextTags,
            }));
            setVideoActressesById((currentActressesById) => ({
              ...currentActressesById,
              [nextVideo.id]: nextActresses,
            }));
          })
          .catch(() => undefined);
      }, 250);
    } catch {
      notify({
        title: "影片添加失败",
        description: "请检查番号和作品类型后重试。",
        variant: "error",
      });
    } finally {
      setIsSubmitting(false);
    }
  }

  function handleVideoSaved(nextVideo: VideoRecord) {
    setVideos((currentVideos) =>
      currentVideos.map((video) => (video.id === nextVideo.id ? nextVideo : video)),
    );
    setSelectedVideo(nextVideo);
  }

  function handleVideoDeleted(videoId: string) {
    setVideos((currentVideos) => {
      const nextVideos = currentVideos.filter((video) => video.id !== videoId);
      setSelectedVideo(nextVideos[0] ?? null);
      return nextVideos;
    });
    onBackToLibrary?.();
  }

  function handleVideoTagsChanged(videoId: string, tags: TagRecord[]) {
    setVideoTagsById((currentTagsById) => ({
      ...currentTagsById,
      [videoId]: tags,
    }));
  }

  function handleVideoActressesChanged(videoId: string, actresses: ActressRecord[]) {
    setVideoActressesById((currentActressesById) => ({
      ...currentActressesById,
      [videoId]: actresses,
    }));
  }

  const filteredVideos = filterVideos(videos, {
    searchText,
    workType: workTypeFilter,
    sortMode,
    tagsByVideoId: videoTagsById,
    actressesByVideoId: videoActressesById,
  });

  const detailVideo = focusVideoId
    ? videos.find((video) => video.id === focusVideoId) ?? selectedVideo
    : null;

  if (focusVideoId) {
    return (
      <main className="h-full bg-[#101014] text-[var(--color-text)]">
        <section className="nvy-page-scroll px-10 py-8">
          <div className="mx-auto grid max-w-[76rem] gap-5">
            <button
              type="button"
              onClick={onBackToLibrary}
              className="w-fit rounded-full border border-[var(--color-border)] bg-[var(--color-surface-soft)] px-4 py-2 text-sm text-[var(--color-muted)] transition hover:bg-[var(--color-surface-hover)] hover:text-[var(--color-text)]"
            >
              ← {backLabel}
            </button>
            {detailVideo ? (
              <VideoDetailForm
                key={detailVideo.id}
                video={detailVideo}
                onSaved={handleVideoSaved}
                onDeleted={handleVideoDeleted}
                onTagsChanged={handleVideoTagsChanged}
                onActressesChanged={handleVideoActressesChanged}
              />
            ) : (
              <EmptyState text={isLoading ? "正在加载影片详情" : "没有找到这部影片"} />
            )}
          </div>
        </section>
      </main>
    );
  }

  return (
    <main className="h-full bg-[#101014] text-[var(--color-text)]">
      <section className="grid h-full grid-rows-[auto_auto_minmax(0,1fr)] px-10 py-7">
        <section className="grid gap-3">
          <div className="grid grid-cols-[minmax(0,1fr)_auto_auto_auto_auto] items-center gap-2">
            <Input
              name="video-search"
              aria-label="搜索影片"
              placeholder="搜索框"
              value={searchText}
              onChange={(event) => setSearchText(event.target.value)}
              className="h-14 rounded-[1.35rem] border-[rgba(255,255,255,0.08)] bg-[rgba(39,39,43,0.96)] px-5 text-base"
            />
            <Button type="button" className="h-14 rounded-[1.2rem] px-6">
              搜索
            </Button>
            <WorkTypeFilterSelect value={workTypeFilter} onChange={setWorkTypeFilter} />
            <SortModeSelect value={sortMode} onChange={setSortMode} />
            <Button type="submit" form="video-create-form" disabled={isSubmitting} className="h-14 rounded-[1.2rem] px-6">
              新增
            </Button>
          </div>
          <form
            id="video-create-form"
            onSubmit={handleCreateVideo}
            className="grid grid-cols-[1fr_1.4fr_12rem] gap-3 rounded-[1.35rem] border border-[var(--color-border)] bg-[rgba(28,27,33,0.7)] p-3"
          >
            <Input
              name="code"
              aria-label="番号"
              placeholder="番号，例如 ABC-001"
              value={code}
              onChange={(event) => setCode(event.target.value)}
            />
            <Input
              name="title"
              aria-label="标题"
              placeholder="标题，可稍后补充"
              value={title}
              onChange={(event) => setTitle(event.target.value)}
            />
            <WorkTypeSelect value={workType} onChange={setWorkType} />
          </form>
        </section>

        <p className="py-3 text-right text-xs text-[var(--color-muted)]">
          {isLoading ? "加载中" : `${videos.length} 条影片`}
        </p>

        <section className="nvy-page-scroll nvy-fade-top pr-1 pt-5">
          <div className="grid grid-cols-3 gap-x-20 gap-y-9 pb-8">
            {filteredVideos.length === 0 ? (
              <EmptyState text={isLoading ? "正在加载影片" : "还没有影片记录"} />
            ) : (
              filteredVideos.map((video) => (
                <VideoCard
                  key={video.id}
                  video={video}
                  coverUrl={coverUrlsById[video.id]}
                  onClick={() => onOpenDetail?.(video.id)}
                />
              ))
            )}
          </div>
        </section>
      </section>
    </main>
  );
}

type VideoDetailFormProps = {
  video: VideoRecord;
  onSaved: (video: VideoRecord) => void;
  onDeleted: (videoId: string) => void;
  onTagsChanged: (videoId: string, tags: TagRecord[]) => void;
  onActressesChanged: (videoId: string, actresses: ActressRecord[]) => void;
};

function VideoDetailForm({
  video,
  onSaved,
  onDeleted,
  onTagsChanged,
  onActressesChanged,
}: VideoDetailFormProps) {
  const [draft, setDraft] = useState<VideoDraft>(() => toVideoDraft(video));
  const [isSaving, setIsSaving] = useState(false);
  const [saveStatus, setSaveStatus] = useState<SaveStatus>("idle");
  const [allActresses, setAllActresses] = useState<ActressRecord[]>([]);
  const [linkedActresses, setLinkedActresses] = useState<ActressRecord[]>([]);
  const [availableTags, setAvailableTags] = useState<TagRecord[]>([]);
  const [selectedTags, setSelectedTags] = useState<TagRecord[]>([]);
  const [timestamp, setTimestamp] = useState({ hour: "", minute: "", second: "" });
  const [isDeleteOpen, setIsDeleteOpen] = useState(false);
  const [coverSourcePath, setCoverSourcePath] = useState("");
  const [coverPreviewUrl, setCoverPreviewUrl] = useState("");
  const [isCachingCover, setIsCachingCover] = useState(false);
  const [metadataCandidates, setMetadataCandidates] = useState<MetadataCandidate[]>([]);
  const [isMatchingMetadata, setIsMatchingMetadata] = useState(false);
  const [isTranslating, setIsTranslating] = useState(false);
  const reviewRef = useRef<HTMLTextAreaElement>(null);
  const { notify } = useToast();

  useEffect(() => {
    setDraft(toVideoDraft(video));
    setSaveStatus("idle");
    setMetadataCandidates([]);
  }, [video]);

  useEffect(() => {
    let isActive = true;
    const session = requireSession();

    Promise.all([
      listActresses(session.accountId),
      listVideoActresses(session.accountId, video.id),
      listTags(session.accountId, "video"),
      listVideoTags(session.accountId, video.id),
    ])
      .then(([nextActresses, nextLinkedActresses, nextTags, nextSelectedTags]) => {
        if (!isActive) {
          return;
        }
        setAllActresses(nextActresses);
        setLinkedActresses(nextLinkedActresses);
        setAvailableTags(nextTags);
        setSelectedTags(nextSelectedTags);
      })
      .catch(() => {
        if (isActive) {
          notify({
            title: "关联资料加载失败",
            description: "请稍后重试。",
            variant: "error",
          });
        }
      });

    return () => {
      isActive = false;
    };
  }, [notify, video.id]);

  useEffect(() => {
    let isActive = true;

    if (!draft.coverPath.trim()) {
      setCoverPreviewUrl("");
      return () => {
        isActive = false;
      };
    }

    cachedAssetUrl(draft.coverPath)
      .then((url) => {
        if (isActive) {
          setCoverPreviewUrl(url);
        }
      })
      .catch(() => {
        if (isActive) {
          setCoverPreviewUrl("");
        }
      });

    return () => {
      isActive = false;
    };
  }, [draft.coverPath]);

  const saveDraft = useCallback(
    async (nextDraft: VideoDraft) => {
      const session = requireSession();
      return updateVideo(session.accountId, video.id, toVideoInput(nextDraft));
    },
    [video.id],
  );

  useAutosave({
    value: draft,
    enabled: Boolean(video.id),
    onSave: async (nextDraft) => {
      setSaveStatus("saving");
      return saveDraft(nextDraft);
    },
    onSaved: (saved) => {
      setSaveStatus("saved");
      onSaved(saved);
    },
    onError: () => {
      setSaveStatus("error");
    },
  });

  async function handleSave(event: FormEvent<HTMLFormElement>) {
    event.preventDefault();
    setIsSaving(true);

    try {
      const saved = await saveDraft(draft);
      onSaved(saved);
      setSaveStatus("saved");
      notify({ title: "影片详情已保存", variant: "success" });
    } catch {
      setSaveStatus("error");
      notify({
        title: "影片详情保存失败",
        description: "请检查必填字段和片长格式。",
        variant: "error",
      });
    } finally {
      setIsSaving(false);
    }
  }

  async function handleCacheCover() {
    const sourcePath = coverSourcePath.trim();

    if (!sourcePath) {
      notify({
        title: "封面路径为空",
        description: "请输入本地图片文件的完整路径。",
        variant: "error",
      });
      return;
    }

    setIsCachingCover(true);

    try {
      const cached = await cacheLocalImage(sourcePath, "cover", video.id);
      setDraft((currentDraft) => ({
        ...currentDraft,
        coverPath: cached.relativePath,
      }));
      setCoverSourcePath("");
      notify({ title: "封面已缓存", variant: "success" });
    } catch {
      notify({
        title: "封面缓存失败",
        description: "请确认文件存在，且格式为 jpg、jpeg、png、webp 或 gif。",
        variant: "error",
      });
    } finally {
      setIsCachingCover(false);
    }
  }

  async function handleMatchMetadata() {
    const query = draft.code.trim();

    if (!query) {
      notify({
        title: "番号为空",
        description: "请输入番号后再匹配资料。",
        variant: "error",
      });
      return;
    }

    setIsMatchingMetadata(true);
    try {
      const session = requireSession();
      const candidates = await matchVideoMetadata(session.accountId, video.id, query);
      setMetadataCandidates(candidates);
      if (candidates.length === 0) {
        notify({
          title: "未找到匹配结果",
          description: "可以继续手动编辑当前影片资料。",
          variant: "info",
        });
      }
    } catch {
      notify({
        title: "影片资料匹配失败",
        description: "请检查网络或稍后重试；当前资料仍可手动编辑。",
        variant: "error",
      });
    } finally {
      setIsMatchingMetadata(false);
    }
  }

  async function handleApplyMetadataCandidate(candidateId: string) {
    try {
      const session = requireSession();
      await applyMetadataCandidate(session.accountId, candidateId);
      const nextVideos = await listVideos(session.accountId);
      const nextVideo = nextVideos.find((next) => next.id === video.id);
      if (nextVideo) {
        setDraft(toVideoDraft(nextVideo));
        onSaved(nextVideo);
      }
      setMetadataCandidates([]);
      notify({ title: "影片资料已写入", variant: "success" });
    } catch {
      notify({
        title: "候选写入失败",
        description: "请继续手动编辑当前影片资料。",
        variant: "error",
      });
    }
  }

  async function handleRequestTranslation() {
    setIsTranslating(true);
    try {
      const session = requireSession();
      await requestVideoTranslation(session.accountId, video.id);
      const nextVideos = await listVideos(session.accountId);
      const nextVideo = nextVideos.find((next) => next.id === video.id);
      if (nextVideo) {
        setDraft(toVideoDraft(nextVideo));
        onSaved(nextVideo);
      }
      notify({ title: "翻译已更新", variant: "success" });
    } catch {
      notify({
        title: "翻译暂不可用",
        description: "请确认已启用翻译、保存 API Key，并检查 Base URL、模型 ID 和网络连接。",
        variant: "error",
      });
    } finally {
      setIsTranslating(false);
    }
  }

  async function handleCancelTranslation() {
    setIsTranslating(true);
    try {
      const session = requireSession();
      await cancelVideoTranslation(session.accountId, video.id);
      const nextVideos = await listVideos(session.accountId);
      const nextVideo = nextVideos.find((next) => next.id === video.id);
      if (nextVideo) {
        setDraft(toVideoDraft(nextVideo));
        onSaved(nextVideo);
      }
      notify({
        title: "已取消当前翻译",
        description: "原始字段会保留；当前中文翻译结果已清空或回退。",
        variant: "success",
      });
    } catch {
      notify({
        title: "取消翻译失败",
        description: "当前影片可能不存在，或本地数据暂时不可写。",
        variant: "error",
      });
    } finally {
      setIsTranslating(false);
    }
  }

  async function handleToggleActress(actressId: string) {
    const isLinked = linkedActresses.some((actress) => actress.id === actressId);
    const nextActressIds = isLinked
      ? linkedActresses
          .filter((actress) => actress.id !== actressId)
          .map((actress) => actress.id)
      : [...linkedActresses.map((actress) => actress.id), actressId];

    try {
      const session = requireSession();
      const nextLinkedActresses = await setVideoActresses(
        session.accountId,
        video.id,
        nextActressIds,
      );
      setLinkedActresses(nextLinkedActresses);
      onActressesChanged(video.id, nextLinkedActresses);
      notify({ title: "影片关联已更新", variant: "success" });
    } catch {
      notify({
        title: "影片关联更新失败",
        description: "单人作品至少需要关联一位女优。",
        variant: "error",
      });
    }
  }

  async function handleToggleTag(tagId: string) {
    const isSelected = selectedTags.some((tag) => tag.id === tagId);
    const nextTagIds = isSelected
      ? selectedTags.filter((tag) => tag.id !== tagId).map((tag) => tag.id)
      : [...selectedTags.map((tag) => tag.id), tagId];

    try {
      const session = requireSession();
      const nextTags = await setVideoTags(session.accountId, video.id, nextTagIds);
      setSelectedTags(nextTags);
      onTagsChanged(video.id, nextTags);
      notify({ title: "影片标签已更新", variant: "success" });
    } catch {
      notify({
        title: "影片标签更新失败",
        description: "请稍后重试。",
        variant: "error",
      });
    }
  }

  async function handleAutoTag() {
    try {
      const session = requireSession();
      const nextTags = await autoTagVideo(session.accountId, video.id);
      setSelectedTags(nextTags);
      onTagsChanged(video.id, nextTags);
      setAvailableTags(await listTags(session.accountId, "video"));
      notify({ title: "自动标签已更新", variant: "success" });
    } catch {
      notify({
        title: "自动标签失败",
        description: "单人作品需要发行日期、关联女优生日或罩杯。",
        variant: "error",
      });
    }
  }

  function handleInsertTimestamp() {
    const hour = Number(timestamp.hour);
    const minute = Number(timestamp.minute);
    const second = Number(timestamp.second);

    if (
      !Number.isInteger(hour) ||
      !Number.isInteger(minute) ||
      !Number.isInteger(second) ||
      hour < 0 ||
      minute < 0 ||
      minute > 59 ||
      second < 0 ||
      second > 59
    ) {
      notify({
        title: "时间戳格式无效",
        description: "请输入有效的时、分、秒。",
        variant: "error",
      });
      return;
    }

    const stamp = `[${hour.toString().padStart(2, "0")}:${minute
      .toString()
      .padStart(2, "0")}:${second.toString().padStart(2, "0")}]`;
    const cursorStart = reviewRef.current?.selectionStart ?? draft.review.length;
    const cursorEnd = reviewRef.current?.selectionEnd ?? cursorStart;
    const nextReview = `${draft.review.slice(0, cursorStart)}${stamp} ${draft.review.slice(
      cursorEnd,
    )}`;

    setDraft({ ...draft, review: nextReview });
    setTimestamp({ hour: "", minute: "", second: "" });

    window.setTimeout(() => {
      const nextCursor = cursorStart + stamp.length + 1;
      reviewRef.current?.focus();
      reviewRef.current?.setSelectionRange(nextCursor, nextCursor);
    }, 0);
  }

  async function handleDeleteVideo() {
    try {
      const session = requireSession();
      await deleteVideo(session.accountId, video.id);
      notify({ title: "影片已删除", variant: "success" });
      onDeleted(video.id);
    } catch {
      notify({
        title: "影片删除失败",
        description: "请稍后重试。",
        variant: "error",
      });
    }
  }

  return (
    <form
      onSubmit={handleSave}
      className="grid gap-4 rounded-[1.6rem] border border-[var(--color-border)] bg-[rgba(28,27,34,0.78)] p-5 shadow-[var(--shadow-panel)]"
    >
      <div className="flex items-start justify-between gap-4">
        <div>
          <p className="text-sm text-[var(--color-muted)]">影片详情</p>
          <h2 className="mt-1 text-xl font-semibold text-[var(--color-text-strong)]">
            {video.title || video.code}
          </h2>
        </div>
        <SaveStatusText status={saveStatus} />
      </div>

      <div className="grid gap-3 sm:grid-cols-2">
        <Input
          name="detail-code"
          label="番号"
          value={draft.code}
          onChange={(event) => setDraft({ ...draft, code: event.target.value })}
        />
        <Input
          name="detail-title"
          label="标题"
          value={draft.title}
          onChange={(event) => setDraft({ ...draft, title: event.target.value })}
        />
        <Input
          name="detail-release-date"
          label="发行日期"
          placeholder="YYYY-MM-DD"
          value={draft.releaseDate}
          onChange={(event) => setDraft({ ...draft, releaseDate: event.target.value })}
        />
        <Input
          name="detail-duration"
          label="片长（分钟）"
          inputMode="numeric"
          value={draft.durationMinutes}
          onChange={(event) =>
            setDraft({ ...draft, durationMinutes: event.target.value })
          }
        />
        <Input
          name="detail-source-url"
          label="来源链接"
          value={draft.sourceUrl}
          onChange={(event) => setDraft({ ...draft, sourceUrl: event.target.value })}
        />
        <WorkTypeSelect
          value={draft.workType}
          onChange={(nextWorkType) => setDraft({ ...draft, workType: nextWorkType })}
        />
      </div>

      <section className="grid gap-3 rounded-2xl border border-[var(--color-border)] bg-[var(--color-surface-soft)] p-4">
        <div className="flex flex-wrap items-center justify-between gap-3">
          <div>
            <h3 className="text-sm font-semibold text-[var(--color-text-strong)]">
              元数据匹配
            </h3>
            <p className="mt-1 text-xs text-[var(--color-muted)]">
              当前版本先走候选确认流程；匹配不到时可以继续手动编辑。
            </p>
          </div>
          <Button
            type="button"
            variant="secondary"
            disabled={isMatchingMetadata}
            onClick={handleMatchMetadata}
          >
            匹配资料
          </Button>
        </div>
        {metadataCandidates.length > 0 ? (
          <div className="grid gap-2">
            {metadataCandidates.map((candidate) => (
              <div
                key={candidate.id}
                className="grid gap-2 rounded-2xl border border-[var(--color-border)] bg-[var(--color-input)] px-3 py-3"
              >
                <div>
                  <p className="text-sm font-semibold text-[var(--color-text-strong)]">
                    {metadataCandidateTitle(candidate)}
                  </p>
                  <p className="text-xs text-[var(--color-muted)]">
                    {candidate.source}
                  </p>
                </div>
                <MetadataCandidateDetails candidate={candidate} />
                <div className="flex flex-wrap justify-end gap-2">
                  <Button
                    type="button"
                    variant="ghost"
                    onClick={() => setMetadataCandidates([])}
                  >
                    跳过
                  </Button>
                  <Button
                    type="button"
                    onClick={() => handleApplyMetadataCandidate(candidate.id)}
                  >
                    使用候选
                  </Button>
                </div>
              </div>
            ))}
          </div>
        ) : null}
      </section>

      <section className="grid gap-3 rounded-2xl border border-[var(--color-border)] bg-[var(--color-surface-soft)] p-4">
        <div className="grid gap-3 sm:grid-cols-[120px_1fr]">
          <div className="grid aspect-[3/4] w-full max-w-36 place-items-center overflow-hidden rounded-2xl border border-[var(--color-border)] bg-[var(--color-input)] text-xs text-[var(--color-muted)]">
            {coverPreviewUrl ? (
              <img
                src={coverPreviewUrl}
                alt=""
                className="h-full w-full object-cover"
              />
            ) : (
              <span>暂无封面</span>
            )}
          </div>
          <div className="grid gap-3">
            <Input
              name="detail-cover-path"
              label="封面缓存路径"
              value={draft.coverPath}
              onChange={(event) =>
                setDraft({ ...draft, coverPath: event.target.value })
              }
            />
            <div className="grid gap-3 sm:grid-cols-[1fr_auto]">
              <Input
                name="detail-cover-source-path"
                label="本地封面文件"
                placeholder="C:\\Images\\cover.png"
                value={coverSourcePath}
                onChange={(event) => setCoverSourcePath(event.target.value)}
              />
              <div className="flex items-end">
                <Button
                  type="button"
                  variant="secondary"
                  disabled={isCachingCover}
                  onClick={handleCacheCover}
                >
                  缓存封面
                </Button>
              </div>
            </div>
          </div>
        </div>
      </section>

      <TextAreaField
        label="演员名单文本"
        value={draft.actorNames}
        onChange={(actorNames) => setDraft({ ...draft, actorNames })}
      />

      <section className="grid gap-3 rounded-2xl border border-[var(--color-border)] bg-[var(--color-surface-soft)] p-4">
        <div>
          <h3 className="text-sm font-semibold text-[var(--color-text-strong)]">
            标题 / 简介翻译
          </h3>
          <p className="mt-1 text-xs text-[var(--color-muted)]">
            重新翻译会覆盖当前中文标题和简介；取消翻译会回到原始字段显示。
          </p>
        </div>
        <div className="flex flex-wrap gap-3">
          <Button
            type="button"
            variant="secondary"
            disabled={isTranslating}
            onClick={handleRequestTranslation}
          >
            重新翻译
          </Button>
          <Button
            type="button"
            variant="ghost"
            disabled={isTranslating}
            onClick={handleCancelTranslation}
          >
            取消翻译
          </Button>
        </div>
      </section>

      <section className="grid gap-3 rounded-2xl border border-[var(--color-border)] bg-[var(--color-surface-soft)] p-4">
        <div>
          <h3 className="text-sm font-semibold text-[var(--color-text-strong)]">
            关联女优
          </h3>
          <p className="mt-1 text-xs text-[var(--color-muted)]">
            单人作品必须至少关联一位女优；多人和素人作品可按需手动关联。
          </p>
        </div>
        {allActresses.length === 0 ? (
          <p className="text-sm text-[var(--color-muted)]">还没有可关联的女优。</p>
        ) : (
          <div className="grid gap-2 sm:grid-cols-2">
            {allActresses.map((actress) => (
              <label
                key={actress.id}
                className="flex items-center gap-2 rounded-2xl border border-[var(--color-border)] bg-[var(--color-input)] px-3 py-2 text-sm text-[var(--color-text)]"
              >
                <input
                  type="checkbox"
                  checked={linkedActresses.some((linked) => linked.id === actress.id)}
                  onChange={() => handleToggleActress(actress.id)}
                />
                <span className="truncate">{actress.name}</span>
              </label>
            ))}
          </div>
        )}
      </section>

      <section className="grid gap-3 rounded-2xl border border-[var(--color-border)] bg-[var(--color-surface-soft)] p-4">
        <div className="flex flex-wrap items-center justify-between gap-3">
          <div>
            <h3 className="text-sm font-semibold text-[var(--color-text-strong)]">
              影片标签
            </h3>
            <p className="mt-1 text-xs text-[var(--color-muted)]">
              自动标签只会补充年龄和 Cup 标签，不会删除手动标签。
            </p>
          </div>
          <Button type="button" variant="secondary" onClick={handleAutoTag}>
            自动标签
          </Button>
        </div>
        {availableTags.length === 0 ? (
          <p className="text-sm text-[var(--color-muted)]">还没有影片标签。</p>
        ) : (
          <div className="grid gap-2 sm:grid-cols-2">
            {availableTags.map((tag) => (
              <label
                key={tag.id}
                className="flex items-center gap-2 rounded-2xl border border-[var(--color-border)] bg-[var(--color-input)] px-3 py-2 text-sm text-[var(--color-text)]"
              >
                <input
                  type="checkbox"
                  checked={selectedTags.some((selectedTag) => selectedTag.id === tag.id)}
                  onChange={() => handleToggleTag(tag.id)}
                />
                <span className="truncate">{tag.canonicalName}</span>
              </label>
            ))}
          </div>
        )}
      </section>

      <TextAreaField
        label="简介"
        value={draft.summary}
        onChange={(summary) => setDraft({ ...draft, summary })}
      />
      <TextAreaField
        label="影评"
        value={draft.review}
        onChange={(review) => setDraft({ ...draft, review })}
        textareaRef={reviewRef}
      />

      <section className="grid gap-3 rounded-2xl border border-[var(--color-border)] bg-[var(--color-surface-soft)] p-4">
        <h3 className="text-sm font-semibold text-[var(--color-text-strong)]">
          插入时间戳
        </h3>
        <div className="grid gap-3 sm:grid-cols-[1fr_1fr_1fr_auto]">
          <Input
            name="timestamp-hour"
            label="时"
            inputMode="numeric"
            value={timestamp.hour}
            onChange={(event) =>
              setTimestamp({ ...timestamp, hour: event.target.value })
            }
          />
          <Input
            name="timestamp-minute"
            label="分"
            inputMode="numeric"
            value={timestamp.minute}
            onChange={(event) =>
              setTimestamp({ ...timestamp, minute: event.target.value })
            }
          />
          <Input
            name="timestamp-second"
            label="秒"
            inputMode="numeric"
            value={timestamp.second}
            onChange={(event) =>
              setTimestamp({ ...timestamp, second: event.target.value })
            }
          />
          <div className="flex items-end">
            <Button type="button" variant="secondary" onClick={handleInsertTimestamp}>
              插入
            </Button>
          </div>
        </div>
      </section>

      <div className="flex flex-wrap justify-between gap-3">
        <Button type="button" variant="danger" onClick={() => setIsDeleteOpen(true)}>
          删除影片
        </Button>
        <Button type="submit" disabled={isSaving}>
          保存详情
        </Button>
      </div>

      <ConfirmDialog
        open={isDeleteOpen}
        title="删除影片"
        description="删除后无法恢复。关联关系会一并移除，但不会删除女优资料。"
        confirmLabel="确认删除"
        variant="danger"
        onOpenChange={setIsDeleteOpen}
        onConfirm={handleDeleteVideo}
      />
    </form>
  );
}

type VideoCardProps = {
  video: VideoRecord;
  coverUrl?: string;
  onClick: () => void;
};

function VideoCard({ video, coverUrl, onClick }: VideoCardProps) {
  const title = `${video.code} ${video.title || ""}`.trim();

  return (
    <button
      type="button"
      onClick={onClick}
      className="grid min-w-0 gap-3 text-left transition hover:opacity-90"
    >
      <span className="grid aspect-[16/10] w-full place-items-center overflow-hidden rounded-[1.35rem] border border-[rgba(255,255,255,0.09)] bg-[var(--color-input)] text-sm font-semibold text-[var(--color-accent-soft)] shadow-[0_18px_44px_rgba(0,0,0,0.28)]">
        {coverUrl ? (
          <img src={coverUrl} alt="" className="h-full w-full object-cover" />
        ) : (
          video.code
        )}
      </span>
      <span className="nvy-line-clamp-2 min-h-10 text-sm font-medium leading-5 text-[var(--color-text-strong)]">
        {title}
      </span>
    </button>
  );
}

type WorkTypeSelectProps = {
  value: WorkType;
  onChange: (workType: WorkType) => void;
};

function WorkTypeSelect({ value, onChange }: WorkTypeSelectProps) {
  const options: Array<{ value: WorkType; label: string; description: string }> = [
    { value: "single", label: "单人作品", description: "至少关联一位女优" },
    { value: "multiple", label: "多人作品", description: "可关联多位女优" },
    { value: "amateur", label: "素人作品", description: "可按需关联女优" },
  ];
  const selectedOption = options.find((option) => option.value === value) ?? options[0];

  return (
    <label className="grid gap-2 text-left text-sm text-[var(--color-muted)]">
      <span>作品类型</span>
      <Dropdown
        label={selectedOption.label}
        align="end"
        triggerClassName="h-11 w-full rounded-2xl"
        items={options.map((option) => ({
          label: option.label,
          description: option.description,
          selected: option.value === value,
          onSelect: () => onChange(option.value),
        }))}
      />
    </label>
  );
}

function WorkTypeFilterSelect({
  value,
  onChange,
}: {
  value: WorkType | "all";
  onChange: (workType: WorkType | "all") => void;
}) {
  const options: Array<{
    value: WorkType | "all";
    label: string;
    description: string;
  }> = [
    { value: "all", label: "全部", description: "显示所有作品类型" },
    { value: "single", label: "单人作品", description: "只显示单人作品" },
    { value: "multiple", label: "多人作品", description: "只显示多人作品" },
    { value: "amateur", label: "素人作品", description: "只显示素人作品" },
  ];
  const selectedOption = options.find((option) => option.value === value) ?? options[0];

  return (
    <label className="grid text-left text-sm text-[var(--color-muted)]">
      <Dropdown
        label={selectedOption.label}
        align="end"
        triggerClassName="h-14 w-full rounded-[1.2rem]"
        items={options.map((option) => ({
          label: option.label,
          description: option.description,
          selected: option.value === value,
          onSelect: () => onChange(option.value),
        }))}
      />
    </label>
  );
}

function SortModeSelect({
  value,
  onChange,
}: {
  value: SortMode;
  onChange: (sortMode: SortMode) => void;
}) {
  const options: Array<{ value: SortMode; label: string; description: string }> = [
    { value: "updatedDesc", label: "最近更新", description: "最近编辑的影片优先" },
    { value: "codeAsc", label: "番号升序", description: "按番号文本升序排列" },
    { value: "releaseDateDesc", label: "发行日期倒序", description: "发行日期新的优先" },
  ];
  const selectedOption = options.find((option) => option.value === value) ?? options[0];

  return (
    <label className="grid text-left text-sm text-[var(--color-muted)]">
      <Dropdown
        label={selectedOption.label}
        align="end"
        triggerClassName="h-14 w-full rounded-[1.2rem]"
        items={options.map((option) => ({
          label: option.label,
          description: option.description,
          selected: option.value === value,
          onSelect: () => onChange(option.value),
        }))}
      />
    </label>
  );
}

type TextAreaFieldProps = {
  label: string;
  value: string;
  onChange: (value: string) => void;
  textareaRef?: RefObject<HTMLTextAreaElement | null>;
};

function TextAreaField({ label, value, onChange, textareaRef }: TextAreaFieldProps) {
  return (
    <label className="grid gap-2 text-left text-sm text-[var(--color-muted)]">
      <span>{label}</span>
      <textarea
        ref={textareaRef}
        value={value}
        onChange={(event) => onChange(event.target.value)}
        rows={5}
        className="resize-y rounded-2xl border border-[var(--color-border)] bg-[var(--color-input)] px-4 py-3 text-[var(--color-text)] outline-none transition placeholder:text-[var(--color-muted-subtle)] focus:border-[var(--color-focus)] focus:ring-2 focus:ring-[var(--color-focus-soft)]"
      />
    </label>
  );
}

function SaveStatusText({ status }: { status: SaveStatus }) {
  const textByStatus: Record<SaveStatus, string> = {
    idle: "自动保存",
    saving: "保存中",
    saved: "已自动保存",
    error: "保存失败",
  };

  return <span className="text-xs text-[var(--color-muted)]">{textByStatus[status]}</span>;
}

function EmptyState({ text }: { text: string }) {
  return (
    <div className="rounded-2xl border border-dashed border-[var(--color-border)] px-4 py-8 text-center text-sm text-[var(--color-muted)]">
      {text}
    </div>
  );
}

function metadataCandidateTitle(candidate: MetadataCandidate) {
  try {
    const payload = JSON.parse(candidate.payloadJson) as {
      code?: string;
      title?: string;
      name?: string;
    };

    return payload.title || payload.code || payload.name || candidate.query;
  } catch {
    return candidate.query;
  }
}

function MetadataCandidateDetails({ candidate }: { candidate: MetadataCandidate }) {
  const details = metadataCandidateDetails(candidate);

  if (details.length === 0) {
    return (
      <p className="text-xs text-[var(--color-muted)]">
        候选中没有可预览的字段；仍可尝试写入已有字段。
      </p>
    );
  }

  return (
    <dl className="grid gap-x-4 gap-y-1 text-xs text-[var(--color-muted)] sm:grid-cols-2">
      {details.map((detail) => (
        <div key={detail.label} className="grid min-w-0 grid-cols-[4.5rem_minmax(0,1fr)] gap-2">
          <dt className="text-[var(--color-muted-subtle)]">{detail.label}</dt>
          <dd className="min-w-0 truncate text-[var(--color-text)]" title={detail.value}>
            {detail.value}
          </dd>
        </div>
      ))}
    </dl>
  );
}

function metadataCandidateDetails(candidate: MetadataCandidate) {
  try {
    const payload = JSON.parse(candidate.payloadJson) as {
      code?: string | null;
      title?: string | null;
      source_url?: string | null;
      summary?: string | null;
      actor_names?: string | null;
      release_date?: string | null;
      duration_minutes?: number | null;
      cover_source_path?: string | null;
      metadata_source?: string | null;
    };
    const details: Array<{ label: string; value: string }> = [];

    addCandidateDetail(details, "番号", payload.code);
    addCandidateDetail(details, "标题", payload.title);
    addCandidateDetail(details, "发行日", payload.release_date);
    addCandidateDetail(
      details,
      "片长",
      typeof payload.duration_minutes === "number"
        ? `${payload.duration_minutes} 分钟`
        : undefined,
    );
    addCandidateDetail(details, "演员", payload.actor_names);
    addCandidateDetail(details, "简介", payload.summary);
    addCandidateDetail(details, "封面", payload.cover_source_path);
    addCandidateDetail(details, "链接", payload.source_url);
    addCandidateDetail(details, "来源", payload.metadata_source);

    return details;
  } catch {
    return [];
  }
}

function addCandidateDetail(
  details: Array<{ label: string; value: string }>,
  label: string,
  value?: string | null,
) {
  const normalized = value?.trim();

  if (normalized) {
    details.push({ label, value: normalized });
  }
}

type VideoDraft = {
  code: string;
  title: string;
  coverPath: string;
  releaseDate: string;
  durationMinutes: string;
  sourceUrl: string;
  summary: string;
  actorNames: string;
  workType: WorkType;
  review: string;
};

function toVideoDraft(video: VideoRecord): VideoDraft {
  return {
    code: video.code,
    title: video.title ?? "",
    coverPath: video.coverPath ?? "",
    releaseDate: video.releaseDate ?? "",
    durationMinutes: video.durationMinutes?.toString() ?? "",
    sourceUrl: video.sourceUrl ?? "",
    summary: video.summary ?? "",
    actorNames: video.actorNames ?? "",
    workType: video.workType,
    review: video.review ?? "",
  };
}

function toVideoInput(draft: VideoDraft): VideoInput {
  const durationMinutes = Number(draft.durationMinutes);

  return {
    code: draft.code,
    title: draft.title,
    coverPath: draft.coverPath,
    releaseDate: draft.releaseDate,
    durationMinutes: draft.durationMinutes.trim() === "" ? undefined : durationMinutes,
    sourceUrl: draft.sourceUrl,
    summary: draft.summary,
    actorNames: draft.actorNames,
    workType: draft.workType,
    review: draft.review,
  };
}

async function applyFirstVideoMetadataCandidate(
  accountId: string,
  video: VideoRecord,
): Promise<VideoRecord> {
  const query = video.code.trim();
  if (!query) {
    return video;
  }

  try {
    const candidates = await matchVideoMetadata(accountId, video.id, query);
    const candidate = candidates[0];
    if (!candidate) {
      return video;
    }

    await applyMetadataCandidate(accountId, candidate.id);
    const nextVideos = await listVideos(accountId);
    return nextVideos.find((nextVideo) => nextVideo.id === video.id) ?? video;
  } catch {
    return video;
  }
}

function isImageEntry(
  entry: readonly [string, string] | null,
): entry is readonly [string, string] {
  return entry !== null;
}
