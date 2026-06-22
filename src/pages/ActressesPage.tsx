import { FormEvent, useCallback, useEffect, useState } from "react";
import {
  addVideoActress,
  applyMetadataCandidate,
  cachedAssetUrl,
  cacheLocalImage,
  createActress,
  deleteActress,
  listActressTags,
  listActressVideos,
  listActresses,
  listAssociationSuggestions,
  listTags,
  openExternalUrl,
  matchActressMetadata,
  setActressTags,
  updateActress,
} from "../services/desktopApi";
import type {
  ActressInput,
  ActressRecord,
  AssociationSuggestion,
  MetadataCandidate,
  TagRecord,
  VideoRecord,
} from "../services/desktopApi/types";
import { DisplayNameType, displayActressName } from "../domain/actressName";
import { allowedCupSizes, CupSize, filterActresses } from "../domain/actressFilter";
import { requireSession } from "../services/auth/sessionStore";
import { useAutosave } from "../services/autosave/useAutosave";
import { Button } from "../components/ui/Button";
import { ConfirmDialog } from "../components/ui/ConfirmDialog";
import { Dropdown } from "../components/ui/Dropdown";
import { Input } from "../components/ui/Input";
import { useToast } from "../components/ui/Toast";

type SaveStatus = "idle" | "saving" | "saved" | "error";
type DisplayMode = DisplayNameType | "recordDefault";

type ActressesPageProps = {
  focusActressId?: string | null;
  onFocusConsumed?: () => void;
};

export function ActressesPage({
  focusActressId = null,
  onFocusConsumed,
}: ActressesPageProps) {
  const [actresses, setActresses] = useState<ActressRecord[]>([]);
  const [selectedActress, setSelectedActress] = useState<ActressRecord | null>(
    null,
  );
  const [name, setName] = useState("");
  const [avatarPath, setAvatarPath] = useState("");
  const [searchText, setSearchText] = useState("");
  const [displayNameType, setDisplayNameType] = useState<DisplayMode>("recordDefault");
  const [selectedCupSizes, setSelectedCupSizes] = useState<CupSize[]>([]);
  const [actressTagsById, setActressTagsById] = useState<Record<string, TagRecord[]>>({});
  const [pendingSuggestion, setPendingSuggestion] =
    useState<AssociationSuggestion | null>(null);
  const [isLoading, setIsLoading] = useState(true);
  const [isSubmitting, setIsSubmitting] = useState(false);
  const { notify } = useToast();

  useEffect(() => {
    let isActive = true;
    const session = requireSession();

    listActresses(session.accountId)
      .then(async (nextActresses) => {
        if (!isActive) {
          return;
        }
        const tagEntries = await Promise.all(
          nextActresses.map(async (actress) => [
            actress.id,
            await listActressTags(session.accountId, actress.id),
          ] as const),
        );
        if (!isActive) {
          return;
        }
        setActresses(nextActresses);
        setActressTagsById(Object.fromEntries(tagEntries));
        setSelectedActress(
          nextActresses.find((actress) => actress.id === focusActressId) ??
            nextActresses[0] ??
            null,
        );
        if (focusActressId) {
          onFocusConsumed?.();
        }
      })
      .catch(() => {
        if (isActive) {
          notify({
            title: "女优列表加载失败",
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
  }, [focusActressId, notify, onFocusConsumed]);

  useEffect(() => {
    if (!focusActressId || actresses.length === 0) {
      return;
    }

    const focusedActress = actresses.find((actress) => actress.id === focusActressId);
    if (focusedActress) {
      setSelectedActress(focusedActress);
      onFocusConsumed?.();
    }
  }, [actresses, focusActressId, onFocusConsumed]);

  async function handleCreateActress(event: FormEvent<HTMLFormElement>) {
    event.preventDefault();
    setIsSubmitting(true);

    try {
      const session = requireSession();
      const created = await createActress(session.accountId, {
        name,
        avatarPath,
      });
      setActresses((currentActresses) => [created, ...currentActresses]);
      setSelectedActress(created);
      setName("");
      setAvatarPath("");
      notify({ title: "女优已添加", variant: "success" });

      const suggestions = await listAssociationSuggestions(
        session.accountId,
        created.id,
      );
      setPendingSuggestion(suggestions[0] ?? null);
    } catch {
      notify({
        title: "女优添加失败",
        description: "请检查姓名后重试。",
        variant: "error",
      });
    } finally {
      setIsSubmitting(false);
    }
  }

  function handleActressSaved(nextActress: ActressRecord) {
    setActresses((currentActresses) =>
      currentActresses.map((actress) =>
        actress.id === nextActress.id ? nextActress : actress,
      ),
    );
    setSelectedActress(nextActress);
  }

  function handleActressDeleted(actressId: string) {
    setActresses((currentActresses) => {
      const nextActresses = currentActresses.filter(
        (actress) => actress.id !== actressId,
      );
      setSelectedActress(nextActresses[0] ?? null);
      return nextActresses;
    });
  }

  function handleActressTagsChanged(actressId: string, tags: TagRecord[]) {
    setActressTagsById((currentTagsById) => ({
      ...currentTagsById,
      [actressId]: tags,
    }));
  }

  async function handleConfirmSuggestion() {
    if (!pendingSuggestion) {
      return;
    }

    try {
      const session = requireSession();
      await addVideoActress(
        session.accountId,
        pendingSuggestion.video.id,
        pendingSuggestion.actress.id,
      );
      notify({ title: "已补充影片关联", variant: "success" });
    } catch {
      notify({
        title: "补充关联失败",
        description: "请稍后在影片详情中手动关联。",
        variant: "error",
      });
    } finally {
      setPendingSuggestion(null);
    }
  }

  function handleToggleCupSize(cupSize: CupSize) {
    setSelectedCupSizes((currentCupSizes) =>
      currentCupSizes.includes(cupSize)
        ? currentCupSizes.filter((currentCupSize) => currentCupSize !== cupSize)
        : [...currentCupSizes, cupSize],
    );
  }

  const filteredActresses = filterActresses(actresses, {
    searchText,
    selectedCupSizes,
    tagsByActressId: actressTagsById,
  });

  return (
    <main className="min-h-screen bg-[var(--color-background)] px-6 py-7 text-[var(--color-text)] lg:px-8">
      <section className="mx-auto grid max-w-7xl gap-6">
        <header className="flex flex-wrap items-end justify-between gap-4 border-b border-[var(--color-border)] pb-5">
          <div>
            <p className="mb-2 text-sm font-medium tracking-normal text-[var(--color-accent-soft)]">
              Library
            </p>
            <h1 className="text-3xl font-semibold tracking-normal text-[var(--color-text-strong)]">
              女优库
            </h1>
          </div>
          <p className="text-sm text-[var(--color-muted)]">
            {isLoading ? "加载中" : `${actresses.length} 条女优`}
          </p>
        </header>

        <form
          onSubmit={handleCreateActress}
          className="grid gap-3 border-b border-[var(--color-border)] pb-6 md:grid-cols-[1fr_1.3fr_auto]"
        >
          <Input
            name="name"
            label="姓名"
            placeholder="输入女优姓名"
            value={name}
            onChange={(event) => setName(event.target.value)}
          />
          <Input
            name="avatarPath"
            label="头像路径"
            placeholder="可稍后由匹配流程补充"
            value={avatarPath}
            onChange={(event) => setAvatarPath(event.target.value)}
          />
          <div className="flex items-end">
            <Button type="submit" disabled={isSubmitting}>
              添加
            </Button>
          </div>
        </form>

        <section className="sticky top-0 z-20 grid gap-3 border-b border-[var(--color-border)] bg-[linear-gradient(180deg,var(--color-background)_0%,rgba(17,17,20,0.94)_100%)] py-4 backdrop-blur-xl">
          <div className="grid gap-3 md:grid-cols-[1.4fr_220px]">
            <Input
              name="actress-search"
              label="搜索"
              placeholder="姓名 / 别名 / #标签"
              value={searchText}
              onChange={(event) => setSearchText(event.target.value)}
            />
            <DisplayNameSelect
              value={displayNameType}
              includeRecordDefault
              onChange={setDisplayNameType}
            />
          </div>
          <div className="flex flex-wrap gap-2">
            {allowedCupSizes.map((cupSize) => (
              <button
                key={cupSize}
                type="button"
                onClick={() => handleToggleCupSize(cupSize)}
                className={[
                  "rounded-full border px-3 py-1 text-xs transition",
                  selectedCupSizes.includes(cupSize)
                    ? "border-[var(--color-accent)] bg-[rgba(159,136,219,0.18)] text-[var(--color-text-strong)]"
                    : "border-[var(--color-border)] bg-[var(--color-surface-soft)] text-[var(--color-muted)]",
                ].join(" ")}
              >
                {cupSize}
              </button>
            ))}
          </div>
        </section>

        <div className="grid gap-6 lg:grid-cols-[minmax(0,0.95fr)_minmax(420px,1.05fr)]">
          <section className="relative grid max-h-[calc(100vh-220px)] content-start gap-3 overflow-y-auto pr-1 sm:grid-cols-2 xl:grid-cols-3">
            <div className="pointer-events-none sticky top-0 z-10 col-span-full h-6 bg-gradient-to-b from-[var(--color-background)] to-transparent" />
            {filteredActresses.length === 0 ? (
              <EmptyState text={isLoading ? "正在加载女优" : "还没有女优记录"} />
            ) : (
              filteredActresses.map((actress) => (
                <ActressCard
                  key={actress.id}
                  actress={actress}
                  displayNameType={displayNameType}
                  isSelected={selectedActress?.id === actress.id}
                  onClick={() => setSelectedActress(actress)}
                />
              ))
            )}
          </section>

          <aside className="min-w-0 border-l border-[var(--color-border)] pl-6">
            {selectedActress ? (
              <ActressDetailForm
                key={selectedActress.id}
                actress={selectedActress}
                onSaved={handleActressSaved}
                onDeleted={handleActressDeleted}
                onTagsChanged={handleActressTagsChanged}
              />
            ) : (
              <p className="text-sm text-[var(--color-muted)]">
                选择一条女优查看和编辑详情。
              </p>
            )}
          </aside>
        </div>

        <ConfirmDialog
          open={Boolean(pendingSuggestion)}
          title="发现可补充关联"
          description={
            pendingSuggestion
              ? `${pendingSuggestion.actress.name} 出现在多人作品 ${pendingSuggestion.video.code} 的演员名单中，是否补充关联？`
              : ""
          }
          confirmLabel="补充关联"
          onOpenChange={(open) => {
            if (!open) {
              setPendingSuggestion(null);
            }
          }}
          onConfirm={handleConfirmSuggestion}
        />
      </section>
    </main>
  );
}

type ActressDetailFormProps = {
  actress: ActressRecord;
  onSaved: (actress: ActressRecord) => void;
  onDeleted: (actressId: string) => void;
  onTagsChanged: (actressId: string, tags: TagRecord[]) => void;
};

function ActressDetailForm({
  actress,
  onSaved,
  onDeleted,
  onTagsChanged,
}: ActressDetailFormProps) {
  const [draft, setDraft] = useState<ActressDraft>(() => toActressDraft(actress));
  const [isSaving, setIsSaving] = useState(false);
  const [saveStatus, setSaveStatus] = useState<SaveStatus>("idle");
  const [relatedVideos, setRelatedVideos] = useState<VideoRecord[]>([]);
  const [availableTags, setAvailableTags] = useState<TagRecord[]>([]);
  const [selectedTags, setSelectedTags] = useState<TagRecord[]>([]);
  const [isDeleteOpen, setIsDeleteOpen] = useState(false);
  const [avatarSourcePath, setAvatarSourcePath] = useState("");
  const [avatarPreviewUrl, setAvatarPreviewUrl] = useState("");
  const [isCachingAvatar, setIsCachingAvatar] = useState(false);
  const [metadataCandidates, setMetadataCandidates] = useState<MetadataCandidate[]>([]);
  const [isMatchingMetadata, setIsMatchingMetadata] = useState(false);
  const { notify } = useToast();

  useEffect(() => {
    setDraft(toActressDraft(actress));
    setSaveStatus("idle");
    setMetadataCandidates([]);
  }, [actress]);

  useEffect(() => {
    let isActive = true;
    const session = requireSession();

    Promise.all([
      listActressVideos(session.accountId, actress.id),
      listTags(session.accountId, "actress"),
      listActressTags(session.accountId, actress.id),
    ])
      .then(([videos, tags, actressTags]) => {
        if (isActive) {
          setRelatedVideos(videos);
          setAvailableTags(tags);
          setSelectedTags(actressTags);
        }
      })
      .catch(() => {
        if (isActive) {
          notify({
            title: "相关影片加载失败",
            description: "请稍后重试。",
            variant: "error",
          });
        }
      });

    return () => {
      isActive = false;
    };
  }, [actress.id, notify]);

  useEffect(() => {
    let isActive = true;

    if (!draft.avatarPath.trim()) {
      setAvatarPreviewUrl("");
      return () => {
        isActive = false;
      };
    }

    cachedAssetUrl(draft.avatarPath)
      .then((url) => {
        if (isActive) {
          setAvatarPreviewUrl(url);
        }
      })
      .catch(() => {
        if (isActive) {
          setAvatarPreviewUrl("");
        }
      });

    return () => {
      isActive = false;
    };
  }, [draft.avatarPath]);

  const saveDraft = useCallback(
    async (nextDraft: ActressDraft) => {
      const session = requireSession();
      return updateActress(session.accountId, actress.id, toActressInput(nextDraft));
    },
    [actress.id],
  );

  useAutosave({
    value: draft,
    enabled: Boolean(actress.id),
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
      notify({ title: "女优详情已保存", variant: "success" });
    } catch {
      setSaveStatus("error");
      notify({
        title: "女优详情保存失败",
        description: "请检查姓名和身高格式。",
        variant: "error",
      });
    } finally {
      setIsSaving(false);
    }
  }

  async function handleCacheAvatar() {
    const sourcePath = avatarSourcePath.trim();

    if (!sourcePath) {
      notify({
        title: "头像路径为空",
        description: "请输入本地图片文件的完整路径。",
        variant: "error",
      });
      return;
    }

    setIsCachingAvatar(true);

    try {
      const cached = await cacheLocalImage(sourcePath, "actress", actress.id);
      setDraft((currentDraft) => ({
        ...currentDraft,
        avatarPath: cached.relativePath,
      }));
      setAvatarSourcePath("");
      notify({ title: "头像已缓存", variant: "success" });
    } catch {
      notify({
        title: "头像缓存失败",
        description: "请确认文件存在，且格式为 jpg、jpeg、png、webp 或 gif。",
        variant: "error",
      });
    } finally {
      setIsCachingAvatar(false);
    }
  }

  async function handleMatchMetadata() {
    const query = draft.name.trim();

    if (!query) {
      notify({
        title: "姓名为空",
        description: "请输入女优姓名后再匹配资料。",
        variant: "error",
      });
      return;
    }

    setIsMatchingMetadata(true);
    try {
      const session = requireSession();
      const candidates = await matchActressMetadata(
        session.accountId,
        actress.id,
        query,
      );
      setMetadataCandidates(candidates);
      if (candidates.length === 0) {
        notify({
          title: "未找到匹配结果",
          description: "可以继续手动编辑当前女优资料。",
          variant: "info",
        });
      }
    } catch {
      notify({
        title: "女优资料匹配失败",
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
      const nextActresses = await listActresses(session.accountId);
      const nextActress = nextActresses.find((next) => next.id === actress.id);
      if (nextActress) {
        setDraft(toActressDraft(nextActress));
        onSaved(nextActress);
      }
      setMetadataCandidates([]);
      notify({ title: "女优资料已写入", variant: "success" });
    } catch {
      notify({
        title: "候选写入失败",
        description: "请继续手动编辑当前女优资料。",
        variant: "error",
      });
    }
  }

  async function handleOpenWikipedia() {
    const url = draft.wikipediaZhUrl.trim();

    if (!url) {
      return;
    }

    try {
      await openExternalUrl(url);
    } catch {
      notify({
        title: "维基百科链接打开失败",
        description: "请检查链接是否以 http:// 或 https:// 开头。",
        variant: "error",
      });
    }
  }

  async function handleDeleteActress() {
    try {
      const session = requireSession();
      await deleteActress(session.accountId, actress.id);
      notify({ title: "女优已删除", variant: "success" });
      onDeleted(actress.id);
    } catch {
      notify({
        title: "女优删除失败",
        description: "请稍后重试。",
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
      const nextTags = await setActressTags(session.accountId, actress.id, nextTagIds);
      setSelectedTags(nextTags);
      onTagsChanged(actress.id, nextTags);
      notify({ title: "女优标签已更新", variant: "success" });
    } catch {
      notify({
        title: "女优标签更新失败",
        description: "请稍后重试。",
        variant: "error",
      });
    }
  }

  return (
    <form
      onSubmit={handleSave}
      className="sticky top-7 grid max-h-[calc(100vh-56px)] gap-4 overflow-y-auto rounded-2xl border border-[var(--color-border)] bg-[var(--color-panel)] p-5 shadow-[var(--shadow-panel)]"
    >
      <div className="flex items-start justify-between gap-4">
        <div>
          <p className="text-sm text-[var(--color-muted)]">女优详情</p>
          <h2 className="mt-1 text-xl font-semibold text-[var(--color-text-strong)]">
            {actress.name}
          </h2>
        </div>
        <SaveStatusText status={saveStatus} />
      </div>

      <section className="grid gap-3 rounded-2xl border border-[var(--color-border)] bg-[var(--color-surface-soft)] p-4">
        <div className="grid gap-3 sm:grid-cols-[120px_1fr]">
          <div className="grid h-32 w-32 place-items-center overflow-hidden rounded-full border border-[var(--color-border)] bg-[var(--color-input)] text-xs text-[var(--color-muted)]">
            {avatarPreviewUrl ? (
              <img
                src={avatarPreviewUrl}
                alt=""
                className="h-full w-full object-cover"
              />
            ) : (
              <span>暂无头像</span>
            )}
          </div>
          <div className="grid gap-3">
            <Input
              name="detail-avatar-path"
              label="头像缓存路径"
              value={draft.avatarPath}
              onChange={(event) =>
                setDraft({ ...draft, avatarPath: event.target.value })
              }
            />
            <div className="grid gap-3 sm:grid-cols-[1fr_auto]">
              <Input
                name="detail-avatar-source-path"
                label="本地头像文件"
                placeholder="C:\\Images\\avatar.png"
                value={avatarSourcePath}
                onChange={(event) => setAvatarSourcePath(event.target.value)}
              />
              <div className="flex items-end">
                <Button
                  type="button"
                  variant="secondary"
                  disabled={isCachingAvatar}
                  onClick={handleCacheAvatar}
                >
                  缓存头像
                </Button>
              </div>
            </div>
          </div>
        </div>
      </section>

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

      <div className="grid gap-3 sm:grid-cols-2">
        <Input
          name="detail-name"
          label="姓名"
          value={draft.name}
          onChange={(event) => setDraft({ ...draft, name: event.target.value })}
        />
        <Input
          name="detail-simplified-name"
          label="简体中文名"
          value={draft.simplifiedChineseName}
          onChange={(event) =>
            setDraft({ ...draft, simplifiedChineseName: event.target.value })
          }
        />
        <Input
          name="detail-former-names"
          label="曾用中文名"
          value={draft.formerChineseNames}
          onChange={(event) =>
            setDraft({ ...draft, formerChineseNames: event.target.value })
          }
        />
        <Input
          name="detail-traditional-name"
          label="繁体中文名"
          value={draft.traditionalChineseName}
          onChange={(event) =>
            setDraft({ ...draft, traditionalChineseName: event.target.value })
          }
        />
        <Input
          name="detail-japanese-name"
          label="日文名"
          value={draft.japaneseName}
          onChange={(event) =>
            setDraft({ ...draft, japaneseName: event.target.value })
          }
        />
        <Input
          name="detail-romanized-name"
          label="罗马音"
          value={draft.romanizedName}
          onChange={(event) =>
            setDraft({ ...draft, romanizedName: event.target.value })
          }
        />
        <DisplayNameSelect
          value={draft.defaultDisplayNameType}
          onChange={(defaultDisplayNameType) =>
            setDraft({
              ...draft,
              defaultDisplayNameType:
                defaultDisplayNameType === "recordDefault"
                  ? "simplifiedChinese"
                  : defaultDisplayNameType,
            })
          }
        />
        <Input
          name="detail-measurements"
          label="三围"
          value={draft.measurements}
          onChange={(event) =>
            setDraft({ ...draft, measurements: event.target.value })
          }
        />
        <Input
          name="detail-cup-size"
          label="罩杯"
          value={draft.cupSize}
          onChange={(event) => setDraft({ ...draft, cupSize: event.target.value })}
        />
        <Input
          name="detail-birthday"
          label="生日"
          placeholder="YYYY-MM-DD"
          value={draft.birthday}
          onChange={(event) => setDraft({ ...draft, birthday: event.target.value })}
        />
        <Input
          name="detail-height"
          label="身高（cm）"
          inputMode="numeric"
          value={draft.heightCm}
          onChange={(event) => setDraft({ ...draft, heightCm: event.target.value })}
        />
        <Input
          name="detail-debut-date"
          label="出道日期"
          placeholder="YYYY-MM-DD"
          value={draft.debutDate}
          onChange={(event) => setDraft({ ...draft, debutDate: event.target.value })}
        />
        <Input
          name="detail-wikipedia"
          label="中文维基链接"
          value={draft.wikipediaZhUrl}
          onChange={(event) =>
            setDraft({ ...draft, wikipediaZhUrl: event.target.value })
          }
        />
      </div>

      <TextAreaField
        label="个人评价"
        value={draft.note}
        onChange={(note) => setDraft({ ...draft, note })}
      />

      <section className="grid gap-3 rounded-2xl border border-[var(--color-border)] bg-[var(--color-surface-soft)] p-4">
        <h3 className="text-sm font-semibold text-[var(--color-text-strong)]">
          女优标签
        </h3>
        {availableTags.length === 0 ? (
          <p className="text-sm text-[var(--color-muted)]">还没有女优标签。</p>
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

      <section className="grid gap-3 rounded-2xl border border-[var(--color-border)] bg-[var(--color-surface-soft)] p-4">
        <h3 className="text-sm font-semibold text-[var(--color-text-strong)]">
          相关影片
        </h3>
        {relatedVideos.length === 0 ? (
          <p className="text-sm text-[var(--color-muted)]">暂无关联影片。</p>
        ) : (
          <div className="grid gap-2 sm:grid-cols-3">
            {relatedVideos.map((video) => (
              <div
                key={video.id}
                className="grid min-h-24 content-between rounded-2xl border border-[var(--color-border)] bg-[var(--color-input)] px-3 py-3"
              >
                <p className="line-clamp-2 text-sm font-semibold leading-5 text-[var(--color-text-strong)]">
                  {video.title || video.code}
                </p>
                <p className="text-xs text-[var(--color-muted)]">{video.code}</p>
              </div>
            ))}
          </div>
        )}
      </section>

      <div className="flex flex-wrap justify-between gap-3">
        <Button type="button" variant="danger" onClick={() => setIsDeleteOpen(true)}>
          删除女优
        </Button>
        <div className="flex flex-wrap justify-end gap-3">
        <Button
          type="button"
          variant="secondary"
          disabled={!draft.wikipediaZhUrl.trim()}
          onClick={handleOpenWikipedia}
        >
          打开维基
        </Button>
        <Button type="submit" disabled={isSaving}>
          保存详情
        </Button>
        </div>
      </div>

      <ConfirmDialog
        open={isDeleteOpen}
        title="删除女优"
        description="删除后无法恢复。相关影片会保留，只会移除此女优的关联关系。"
        confirmLabel="确认删除"
        variant="danger"
        onOpenChange={setIsDeleteOpen}
        onConfirm={handleDeleteActress}
      />
    </form>
  );
}

type ActressCardProps = {
  actress: ActressRecord;
  displayNameType: DisplayMode;
  isSelected: boolean;
  onClick: () => void;
};

function ActressCard({ actress, displayNameType, isSelected, onClick }: ActressCardProps) {
  const displayName = displayActressName(
    actress,
    displayNameType === "recordDefault" ? undefined : displayNameType,
  );

  return (
    <button
      type="button"
      onClick={onClick}
      className={[
        "grid min-w-0 place-items-center gap-3 rounded-2xl border px-3 py-4 text-center transition",
        isSelected
          ? "border-[var(--color-accent)] bg-[rgba(165,140,223,0.16)]"
          : "border-[var(--color-border)] bg-[var(--color-surface-soft)] hover:border-[var(--color-border-strong)] hover:bg-[var(--color-surface-hover)]",
      ].join(" ")}
    >
      <span className="grid h-24 w-24 shrink-0 place-items-center rounded-full border border-[var(--color-border)] bg-[var(--color-surface-strong)] text-2xl font-semibold text-[var(--color-accent-soft)]">
        {actress.name.trim().slice(0, 1) || "?"}
      </span>
      <span className="grid w-full min-w-0 gap-1">
        <span className="block truncate text-sm font-semibold text-[var(--color-text-strong)]">
          {displayName}
        </span>
        <span className="block truncate text-xs text-[var(--color-muted)]">
          {actress.avatarPath || actress.simplifiedChineseName || "暂无头像"}
        </span>
      </span>
    </button>
  );
}

function DisplayNameSelect({
  value,
  onChange,
  includeRecordDefault = false,
}: {
  value: DisplayMode;
  onChange: (value: DisplayMode) => void;
  includeRecordDefault?: boolean;
}) {
  const options: Array<{ value: DisplayMode; label: string; description: string }> = [
    ...(includeRecordDefault
      ? [
          {
            value: "recordDefault" as const,
            label: "单个默认",
            description: "使用每位女优自己的默认显示名",
          },
        ]
      : []),
    { value: "simplifiedChinese", label: "简体中文", description: "优先显示简体中文名" },
    { value: "traditionalChinese", label: "繁体中文", description: "优先显示繁体中文名" },
    { value: "japanese", label: "日文", description: "优先显示日文名" },
    { value: "romanized", label: "罗马音", description: "优先显示罗马音" },
    { value: "name", label: "原始姓名", description: "显示录入时的原始姓名" },
  ];
  const selectedOption = options.find((option) => option.value === value) ?? options[0];

  return (
    <label className="grid gap-2 text-left text-sm text-[var(--color-muted)]">
      <span>显示名</span>
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

type TextAreaFieldProps = {
  label: string;
  value: string;
  onChange: (value: string) => void;
};

function TextAreaField({ label, value, onChange }: TextAreaFieldProps) {
  return (
    <label className="grid gap-2 text-left text-sm text-[var(--color-muted)]">
      <span>{label}</span>
      <textarea
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
    <div className="rounded-2xl border border-dashed border-[var(--color-border)] px-4 py-8 text-center text-sm text-[var(--color-muted)] sm:col-span-2">
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
      simplified_chinese_name?: string;
    };

    return (
      payload.simplified_chinese_name ||
      payload.name ||
      payload.title ||
      payload.code ||
      candidate.query
    );
  } catch {
    return candidate.query;
  }
}

type ActressDraft = {
  name: string;
  avatarPath: string;
  simplifiedChineseName: string;
  formerChineseNames: string;
  traditionalChineseName: string;
  japaneseName: string;
  romanizedName: string;
  defaultDisplayNameType: DisplayNameType;
  measurements: string;
  cupSize: string;
  birthday: string;
  heightCm: string;
  debutDate: string;
  wikipediaZhUrl: string;
  note: string;
};

function toActressDraft(actress: ActressRecord): ActressDraft {
  return {
    name: actress.name,
    avatarPath: actress.avatarPath ?? "",
    simplifiedChineseName: actress.simplifiedChineseName ?? "",
    formerChineseNames: actress.formerChineseNames ?? "",
    traditionalChineseName: actress.traditionalChineseName ?? "",
    japaneseName: actress.japaneseName ?? "",
    romanizedName: actress.romanizedName ?? "",
    defaultDisplayNameType:
      (actress.defaultDisplayNameType as DisplayNameType | undefined) ?? "simplifiedChinese",
    measurements: actress.measurements ?? "",
    cupSize: actress.cupSize ?? "",
    birthday: actress.birthday ?? "",
    heightCm: actress.heightCm?.toString() ?? "",
    debutDate: actress.debutDate ?? "",
    wikipediaZhUrl: actress.wikipediaZhUrl ?? "",
    note: actress.note ?? "",
  };
}

function toActressInput(draft: ActressDraft): ActressInput {
  const heightCm = Number(draft.heightCm);

  return {
    name: draft.name,
    avatarPath: draft.avatarPath,
    simplifiedChineseName: draft.simplifiedChineseName,
    formerChineseNames: draft.formerChineseNames,
    traditionalChineseName: draft.traditionalChineseName,
    japaneseName: draft.japaneseName,
    romanizedName: draft.romanizedName,
    defaultDisplayNameType: draft.defaultDisplayNameType,
    measurements: draft.measurements,
    cupSize: draft.cupSize,
    birthday: draft.birthday,
    heightCm: draft.heightCm.trim() === "" ? undefined : heightCm,
    debutDate: draft.debutDate,
    wikipediaZhUrl: draft.wikipediaZhUrl,
    note: draft.note,
  };
}
