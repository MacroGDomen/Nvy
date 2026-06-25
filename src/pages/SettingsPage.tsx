import { FormEvent, useEffect, useState } from "react";
import {
  clearLlmApiKey,
  createTag,
  deleteTag,
  exportEncryptedBackup,
  exportPlainBackup,
  getLlmSettings,
  getStoredLlmApiKeyStatus,
  getAppDataPaths,
  importEncryptedBackup,
  importPlainBackup,
  listTags,
  saveLlmApiKey,
  saveLlmSettings,
  updateTag,
} from "../services/desktopApi";
import type {
  AppDataPaths,
  LlmApiType,
  LlmSettings,
  LlmSettingsInput,
  TagInput,
  TagRecord,
  TagScope,
} from "../services/desktopApi/types";
import {
  formatRecommendationReferenceLimit,
  parseRecommendationReferenceLimit,
} from "../domain/settingsValidation";
import { requireSession } from "../services/auth/sessionStore";
import { Button } from "../components/ui/Button";
import { ConfirmDialog } from "../components/ui/ConfirmDialog";
import { Dropdown } from "../components/ui/Dropdown";
import { Input } from "../components/ui/Input";
import { useToast } from "../components/ui/Toast";

type TagDraft = {
  canonicalName: string;
  aliases: string;
  relatedTags: string;
};

const emptyDraft: TagDraft = {
  canonicalName: "",
  aliases: "",
  relatedTags: "",
};

type LlmSettingsDraft = {
  apiType: LlmApiType;
  baseUrl: string;
  providerName: string;
  model: string;
  temperature: string;
  maxTokens: string;
  recommendationReferenceLimit: string;
  translationPrompt: string;
  recommendationPrompt: string;
  enableLlmTranslation: boolean;
  recommendationDefaultEnabled: boolean;
  metadataAllowBrowserCookies: boolean;
};

export function SettingsPage() {
  return (
    <main className="nvy-page-scroll bg-[var(--color-background)] px-6 py-7 text-[var(--color-text)] lg:px-8">
      <section className="mx-auto grid max-w-7xl gap-6">
        <header className="border-b border-[var(--color-border)] pb-5">
          <p className="mb-2 text-sm font-medium tracking-normal text-[var(--color-accent-soft)]">
            Settings
          </p>
          <h1 className="text-3xl font-semibold tracking-normal text-[var(--color-text-strong)]">
            设置
          </h1>
        </header>

        <div className="grid items-start gap-6 xl:grid-cols-[minmax(0,1.25fr)_minmax(360px,0.75fr)]">
          <div className="grid content-start gap-6">
            <LlmSettingsSection />
            <StorageSection />
          </div>
          <ImportExportSection />
        </div>

        <div className="grid gap-6 xl:grid-cols-2">
          <TagSection scope="video" title="影片标签库" />
          <TagSection scope="actress" title="女优标签库" />
        </div>
      </section>
    </main>
  );
}

function LlmSettingsSection() {
  const [draft, setDraft] = useState<LlmSettingsDraft>(() => toLlmSettingsDraft(null));
  const [apiKey, setApiKey] = useState("");
  const [hasApiKey, setHasApiKey] = useState(false);
  const [isLoading, setIsLoading] = useState(true);
  const [isSaving, setIsSaving] = useState(false);
  const { notify } = useToast();
  const apiTypeOptions: Array<{
    value: LlmApiType;
    label: string;
    description: string;
  }> = [
    {
      value: "responses",
      label: "OpenAI Responses API",
      description: "OpenAI 新版响应接口",
    },
    {
      value: "chat_completions",
      label: "OpenAI 兼容 Chat Completions",
      description: "兼容 DeepSeek 等 OpenAI 风格接口",
    },
    {
      value: "custom",
      label: "自定义接口",
      description: "为后续自定义请求格式预留",
    },
  ];
  const selectedApiType =
    apiTypeOptions.find((option) => option.value === draft.apiType) ?? apiTypeOptions[0];

  useEffect(() => {
    let isActive = true;
    const session = requireSession();

    getLlmSettings(session.accountId)
      .then((settings) => {
        if (!isActive) {
          return;
        }
        setDraft(toLlmSettingsDraft(settings));
        setHasApiKey(settings.hasApiKey);
      })
      .catch(() => {
        if (isActive) {
          notify({
            title: "大模型设置加载失败",
            description: "可以稍后重试，当前不会影响本地资料编辑。",
            variant: "error",
          });
        }
      })
      .finally(() => {
        if (isActive) {
          setIsLoading(false);
        }
      });

    getStoredLlmApiKeyStatus(session.accountId)
      .then((status) => {
        if (isActive) {
          setHasApiKey(status.hasApiKey);
        }
      })
      .catch(() => undefined);

    return () => {
      isActive = false;
    };
  }, [notify]);

  async function handleSaveSettings(event: FormEvent<HTMLFormElement>) {
    event.preventDefault();
    setIsSaving(true);

    try {
      const session = requireSession();
      const saved = await saveLlmSettings(session.accountId, toLlmSettingsInput(draft));
      setDraft(toLlmSettingsDraft(saved));
      setHasApiKey(saved.hasApiKey);
      notify({ title: "大模型设置已保存", variant: "success" });
    } catch {
      notify({
        title: "大模型设置保存失败",
        description: "请确认推荐参考总数为 0-999，温度为 0-2，最大输出长度为正整数。",
        variant: "error",
      });
    } finally {
      setIsSaving(false);
    }
  }

  async function handleSaveApiKey() {
    if (!apiKey.trim()) {
      notify({
        title: "API Key 为空",
        description: "请输入 API Key 后再保存。",
        variant: "error",
      });
      return;
    }

    try {
      const session = requireSession();
      const status = await saveLlmApiKey(session.accountId, apiKey);
      setHasApiKey(status.hasApiKey);
      setApiKey("");
      notify({ title: "API Key 已保存", variant: "success" });
    } catch {
      notify({
        title: "API Key 保存失败",
        description: "请稍后重试；API Key 不会写入 SQLite。",
        variant: "error",
      });
    }
  }

  async function handleClearApiKey() {
    try {
      const session = requireSession();
      const status = await clearLlmApiKey(session.accountId);
      setHasApiKey(status.hasApiKey);
      setApiKey("");
      notify({ title: "API Key 已清除", variant: "success" });
    } catch {
      notify({
        title: "API Key 清除失败",
        description: "请稍后重试。",
        variant: "error",
      });
    }
  }

  return (
    <section className="grid content-start gap-5 rounded-2xl border border-[var(--color-border)] bg-[var(--color-panel)] p-5 shadow-[var(--shadow-panel)]">
      <div>
        <h2 className="text-xl font-semibold text-[var(--color-text-strong)]">
          大模型设置
        </h2>
        <p className="mt-1 text-sm text-[var(--color-muted)]">
          保存非敏感模型配置和提示词；API Key 只进入本地 Stronghold 加密存储。
        </p>
      </div>

      <form onSubmit={handleSaveSettings} className="grid content-start gap-4">
        <div className="grid min-w-0 gap-3 lg:grid-cols-3">
          <label className="grid min-w-0 gap-2 text-left text-sm text-[var(--color-muted)]">
            <span>接口类型</span>
            <Dropdown
              label={selectedApiType.label}
              align="end"
              triggerClassName="h-11 w-full rounded-2xl"
              items={apiTypeOptions.map((option) => ({
                label: option.label,
                description: option.description,
                selected: option.value === draft.apiType,
                onSelect: () => setDraft({ ...draft, apiType: option.value }),
              }))}
            />
          </label>
          <Input
            name="llm-base-url"
            label="Base URL"
            placeholder="https://api.openai.com/v1"
            value={draft.baseUrl}
            onChange={(event) => setDraft({ ...draft, baseUrl: event.target.value })}
          />
          <Input
            name="llm-model"
            label="模型型号 / 模型 ID"
            placeholder="gpt-4.1-mini"
            value={draft.model}
            onChange={(event) => setDraft({ ...draft, model: event.target.value })}
          />
          <Input
            name="llm-provider"
            label="供应商备注"
            value={draft.providerName}
            onChange={(event) => setDraft({ ...draft, providerName: event.target.value })}
          />
          <Input
            name="llm-temperature"
            label="温度"
            inputMode="decimal"
            placeholder="0.7"
            value={draft.temperature}
            onChange={(event) => setDraft({ ...draft, temperature: event.target.value })}
          />
          <Input
            name="llm-max-tokens"
            label="最大输出长度"
            inputMode="numeric"
            placeholder="1000"
            value={draft.maxTokens}
            onChange={(event) => setDraft({ ...draft, maxTokens: event.target.value })}
          />
          <Input
            name="llm-reference-limit"
            label="推荐参考总数上限"
            inputMode="numeric"
            value={draft.recommendationReferenceLimit}
            onChange={(event) =>
              setDraft({ ...draft, recommendationReferenceLimit: event.target.value })
            }
          />
        </div>

        <div className="grid gap-3 lg:grid-cols-2">
          <TextAreaSetting
            label="翻译提示词"
            value={draft.translationPrompt}
            onChange={(translationPrompt) => setDraft({ ...draft, translationPrompt })}
          />
          <TextAreaSetting
            label="首页推荐提示词"
            value={draft.recommendationPrompt}
            onChange={(recommendationPrompt) => setDraft({ ...draft, recommendationPrompt })}
          />
        </div>

        <div className="flex flex-wrap gap-4 text-sm text-[var(--color-muted)]">
          <label className="flex items-center gap-2">
            <input
              type="checkbox"
              checked={draft.enableLlmTranslation}
              onChange={(event) =>
                setDraft({ ...draft, enableLlmTranslation: event.target.checked })
              }
            />
            启用标题 / 简介自动翻译
          </label>
          <label className="flex items-center gap-2">
            <input
              type="checkbox"
              checked={draft.recommendationDefaultEnabled}
              onChange={(event) =>
                setDraft({ ...draft, recommendationDefaultEnabled: event.target.checked })
              }
            />
            首页推荐默认启用
          </label>
        </div>

        <section className="grid min-w-0 gap-3 rounded-2xl border border-[var(--color-border)] bg-[var(--color-input)] p-4">
          <label className="flex items-start gap-3 text-left text-sm text-[var(--color-muted)]">
            <input
              type="checkbox"
              className="mt-1"
              checked={draft.metadataAllowBrowserCookies}
              onChange={(event) =>
                setDraft({ ...draft, metadataAllowBrowserCookies: event.target.checked })
              }
            />
            <span className="grid gap-1">
              <span className="font-medium text-[var(--color-text-strong)]">
                允许元数据来源读取浏览器 Cookie
              </span>
              <span>
                默认关闭。读取 Cookie 可能接触本机浏览器登录凭据，存在隐私风险；仅在番号元数据获取失败、且你明确接受风险时再打开。
              </span>
            </span>
          </label>
        </section>

        <div className="flex flex-wrap items-end gap-3">
          <Input
            name="llm-api-key"
            label={`API Key${hasApiKey ? "（已保存）" : ""}`}
            type="password"
            value={apiKey}
            onChange={(event) => setApiKey(event.target.value)}
          />
          <Button type="button" variant="secondary" onClick={handleSaveApiKey}>
            保存 API Key
          </Button>
          <Button
            type="button"
            variant="ghost"
            disabled={!hasApiKey}
            onClick={handleClearApiKey}
          >
            清除 API Key
          </Button>
          <Button type="submit" disabled={isSaving || isLoading}>
            保存大模型设置
          </Button>
        </div>
      </form>
    </section>
  );
}

function StorageSection() {
  const [paths, setPaths] = useState<AppDataPaths | null>(null);
  const { notify } = useToast();

  useEffect(() => {
    let isActive = true;

    getAppDataPaths()
      .then((nextPaths) => {
        if (isActive) {
          setPaths(nextPaths);
        }
      })
      .catch(() => {
        if (isActive) {
          notify({
            title: "存储路径加载失败",
            description: "请稍后重试。",
            variant: "error",
          });
        }
      });

    return () => {
      isActive = false;
    };
  }, [notify]);

  return (
    <section className="grid gap-4 rounded-2xl border border-[var(--color-border)] bg-[var(--color-surface-soft)] p-5">
      <div>
        <h2 className="text-xl font-semibold text-[var(--color-text-strong)]">
          本地存储
        </h2>
        <p className="mt-1 text-sm text-[var(--color-muted)]">
          数据库和图片缓存保存在当前 Windows 用户的应用数据目录。
        </p>
      </div>
      <div className="grid gap-2">
        <PathRow label="应用数据目录" value={paths?.appDataDir} />
        <PathRow label="数据库文件" value={paths?.databasePath} />
        <PathRow label="图片缓存目录" value={paths?.cacheDir} />
        <PathRow label="影片封面目录" value={paths?.coversDir} />
        <PathRow label="女优头像目录" value={paths?.actressesDir} />
      </div>
    </section>
  );
}

function PathRow({ label, value }: { label: string; value?: string }) {
  return (
    <div className="grid gap-1 rounded-2xl border border-[var(--color-border)] bg-[var(--color-input)] px-4 py-3">
      <span className="text-xs text-[var(--color-muted)]">{label}</span>
      <span className="break-all text-sm text-[var(--color-text-strong)]">
        {value ?? "加载中"}
      </span>
    </div>
  );
}

function TextAreaSetting({
  label,
  value,
  onChange,
}: {
  label: string;
  value: string;
  onChange: (value: string) => void;
}) {
  return (
    <label className="grid gap-2 text-left text-sm text-[var(--color-muted)]">
      <span>{label}</span>
      <textarea
        value={value}
        onChange={(event) => onChange(event.target.value)}
        rows={8}
        className="resize-y rounded-2xl border border-[var(--color-border)] bg-[var(--color-input)] px-4 py-3 text-[var(--color-text)] outline-none transition placeholder:text-[var(--color-muted-subtle)] focus:border-[var(--color-focus)] focus:ring-2 focus:ring-[var(--color-focus-soft)]"
      />
    </label>
  );
}

function toLlmSettingsDraft(settings: LlmSettings | null): LlmSettingsDraft {
  return {
    apiType: settings?.apiType ?? "responses",
    baseUrl: settings?.baseUrl ?? "",
    providerName: settings?.providerName ?? "",
    model: settings?.model ?? "",
    temperature: settings?.temperature?.toString() ?? "",
    maxTokens: settings?.maxTokens?.toString() ?? "",
    recommendationReferenceLimit: formatRecommendationReferenceLimit(
      settings?.recommendationReferenceLimit,
    ),
    translationPrompt: settings?.translationPrompt ?? "",
    recommendationPrompt: settings?.recommendationPrompt ?? "",
    enableLlmTranslation: settings?.enableLlmTranslation ?? false,
    recommendationDefaultEnabled: settings?.recommendationDefaultEnabled ?? true,
    metadataAllowBrowserCookies: settings?.metadataAllowBrowserCookies ?? false,
  };
}

function toLlmSettingsInput(draft: LlmSettingsDraft): LlmSettingsInput {
  const temperature = draft.temperature.trim();
  const maxTokens = draft.maxTokens.trim();

  return {
    apiType: draft.apiType,
    baseUrl: draft.baseUrl,
    providerName: draft.providerName,
    model: draft.model,
    temperature: temperature ? Number(temperature) : undefined,
    maxTokens: maxTokens ? Number(maxTokens) : undefined,
    recommendationReferenceLimit: parseRecommendationReferenceLimit(
      draft.recommendationReferenceLimit,
    ),
    translationPrompt: draft.translationPrompt,
    recommendationPrompt: draft.recommendationPrompt,
    enableLlmTranslation: draft.enableLlmTranslation,
    recommendationDefaultEnabled: draft.recommendationDefaultEnabled,
    metadataAllowBrowserCookies: draft.metadataAllowBrowserCookies,
  };
}

function ImportExportSection() {
  const [plainExportPath, setPlainExportPath] = useState("");
  const [encryptedExportPath, setEncryptedExportPath] = useState("");
  const [plainImportPath, setPlainImportPath] = useState("");
  const [encryptedImportPath, setEncryptedImportPath] = useState("");
  const [exportPassword, setExportPassword] = useState("");
  const [importPassword, setImportPassword] = useState("");
  const [pendingImport, setPendingImport] = useState<"plain" | "encrypted" | null>(null);
  const [isWorking, setIsWorking] = useState(false);
  const { notify } = useToast();

  async function handlePlainExport() {
    const outputPath = plainExportPath.trim();
    if (!outputPath) {
      notify({
        title: "导出路径为空",
        description: "请输入 .nvyzip 导出文件路径。",
        variant: "error",
      });
      return;
    }

    setIsWorking(true);
    try {
      const session = requireSession();
      const result = await exportPlainBackup(session.accountId, outputPath);
      notify({
        title: "普通导出完成",
        description: `${result.path}，共 ${result.entryCount} 个条目。`,
        variant: "success",
      });
    } catch {
      notify({
        title: "普通导出失败",
        description: "请检查导出路径是否可写。",
        variant: "error",
      });
    } finally {
      setIsWorking(false);
    }
  }

  async function handleEncryptedExport() {
    const outputPath = encryptedExportPath.trim();
    if (!outputPath || exportPassword.length < 6) {
      notify({
        title: "加密导出信息不完整",
        description: "请输入 .nvyenc 导出路径和至少 6 位密码。",
        variant: "error",
      });
      return;
    }

    setIsWorking(true);
    try {
      const session = requireSession();
      const result = await exportEncryptedBackup(
        session.accountId,
        outputPath,
        exportPassword,
      );
      notify({
        title: "加密导出完成",
        description: `${result.path}，共 ${result.entryCount} 个条目。`,
        variant: "success",
      });
    } catch {
      notify({
        title: "加密导出失败",
        description: "请检查导出路径和加密密码。",
        variant: "error",
      });
    } finally {
      setIsWorking(false);
    }
  }

  async function handleConfirmImport() {
    if (!pendingImport) {
      return;
    }

    const isEncrypted = pendingImport === "encrypted";
    const inputPath = (isEncrypted ? encryptedImportPath : plainImportPath).trim();
    if (!inputPath || (isEncrypted && importPassword.length < 6)) {
      notify({
        title: "导入信息不完整",
        description: isEncrypted
          ? "请输入 .nvyenc 文件路径和至少 6 位密码。"
          : "请输入 .nvyzip 文件路径。",
        variant: "error",
      });
      setPendingImport(null);
      return;
    }

    setIsWorking(true);
    try {
      const session = requireSession();
      const result = isEncrypted
        ? await importEncryptedBackup(session.accountId, inputPath, importPassword)
        : await importPlainBackup(session.accountId, inputPath);
      notify({
        title: "导入完成",
        description: `恢复 ${result.videoCount} 部影片、${result.actressCount} 位女优、${result.tagCount} 个标签和 ${result.assetCount} 个资源。`,
        variant: "success",
      });
    } catch {
      notify({
        title: "导入失败",
        description: "文件无效、校验失败或密码不正确；当前数据未被写入。",
        variant: "error",
      });
    } finally {
      setIsWorking(false);
      setPendingImport(null);
    }
  }

  return (
    <section className="grid gap-5 rounded-2xl border border-[var(--color-border)] bg-[var(--color-surface-soft)] p-5">
      <div>
        <h2 className="text-xl font-semibold text-[var(--color-text-strong)]">
          数据导入导出
        </h2>
        <p className="mt-1 text-sm text-[var(--color-muted)]">
          普通导出为 .nvyzip；加密导出为 .nvyenc。导入会替换当前账号的数据。
        </p>
      </div>

      <div className="grid min-w-0 gap-4">
        <section className="grid min-w-0 gap-3 rounded-2xl border border-[var(--color-border)] bg-[var(--color-input)] p-4">
          <h3 className="text-sm font-semibold text-[var(--color-text-strong)]">
            普通导出
          </h3>
          <Input
            name="plain-export-path"
            label="导出路径"
            placeholder="D:\\Backup\\NvyExport.nvyzip"
            value={plainExportPath}
            onChange={(event) => setPlainExportPath(event.target.value)}
          />
          <Button type="button" disabled={isWorking} onClick={handlePlainExport}>
            导出 .nvyzip
          </Button>
        </section>

        <section className="grid min-w-0 gap-3 rounded-2xl border border-[var(--color-border)] bg-[var(--color-input)] p-4">
          <h3 className="text-sm font-semibold text-[var(--color-text-strong)]">
            加密导出
          </h3>
          <Input
            name="encrypted-export-path"
            label="导出路径"
            placeholder="D:\\Backup\\NvyExport.nvyenc"
            value={encryptedExportPath}
            onChange={(event) => setEncryptedExportPath(event.target.value)}
          />
          <Input
            name="export-password"
            label="导出密码"
            type="password"
            value={exportPassword}
            onChange={(event) => setExportPassword(event.target.value)}
          />
          <Button type="button" disabled={isWorking} onClick={handleEncryptedExport}>
            导出 .nvyenc
          </Button>
        </section>

        <section className="grid min-w-0 gap-3 rounded-2xl border border-[var(--color-border)] bg-[var(--color-input)] p-4">
          <h3 className="text-sm font-semibold text-[var(--color-text-strong)]">
            普通导入
          </h3>
          <Input
            name="plain-import-path"
            label="导入文件"
            placeholder="D:\\Backup\\NvyExport.nvyzip"
            value={plainImportPath}
            onChange={(event) => setPlainImportPath(event.target.value)}
          />
          <Button
            type="button"
            variant="secondary"
            disabled={isWorking}
            onClick={() => setPendingImport("plain")}
          >
            导入 .nvyzip
          </Button>
        </section>

        <section className="grid gap-3 rounded-2xl border border-[var(--color-border)] bg-[var(--color-input)] p-4">
          <h3 className="text-sm font-semibold text-[var(--color-text-strong)]">
            加密导入
          </h3>
          <Input
            name="encrypted-import-path"
            label="导入文件"
            placeholder="D:\\Backup\\NvyExport.nvyenc"
            value={encryptedImportPath}
            onChange={(event) => setEncryptedImportPath(event.target.value)}
          />
          <Input
            name="import-password"
            label="导入密码"
            type="password"
            value={importPassword}
            onChange={(event) => setImportPassword(event.target.value)}
          />
          <Button
            type="button"
            variant="secondary"
            disabled={isWorking}
            onClick={() => setPendingImport("encrypted")}
          >
            导入 .nvyenc
          </Button>
        </section>
      </div>

      <ConfirmDialog
        open={Boolean(pendingImport)}
        title="确认导入数据"
        description="导入会替换当前登录账号下的影片、女优、标签、关联和设置。文件校验失败或密码错误时不会写入数据库。"
        confirmLabel="确认导入"
        variant="danger"
        onOpenChange={(open) => {
          if (!open) {
            setPendingImport(null);
          }
        }}
        onConfirm={handleConfirmImport}
      />
    </section>
  );
}

function TagSection({ scope, title }: { scope: TagScope; title: string }) {
  const [tags, setTags] = useState<TagRecord[]>([]);
  const [draft, setDraft] = useState<TagDraft>(emptyDraft);
  const [editingTagId, setEditingTagId] = useState<string | null>(null);
  const [deletingTag, setDeletingTag] = useState<TagRecord | null>(null);
  const [isSubmitting, setIsSubmitting] = useState(false);
  const { notify } = useToast();

  useEffect(() => {
    let isActive = true;
    const session = requireSession();

    listTags(session.accountId, scope)
      .then((nextTags) => {
        if (isActive) {
          setTags(nextTags);
        }
      })
      .catch(() => {
        if (isActive) {
          notify({
            title: "标签库加载失败",
            description: "请稍后重试。",
            variant: "error",
          });
        }
      });

    return () => {
      isActive = false;
    };
  }, [notify, scope]);

  async function handleSubmit(event: FormEvent<HTMLFormElement>) {
    event.preventDefault();
    setIsSubmitting(true);

    try {
      const session = requireSession();
      const input: TagInput = {
        scope,
        canonicalName: draft.canonicalName,
        aliases: draft.aliases,
        relatedTags: draft.relatedTags,
      };
      const saved = editingTagId
        ? await updateTag(session.accountId, editingTagId, input)
        : await createTag(session.accountId, input);

      setTags((currentTags) =>
        editingTagId
          ? currentTags.map((tag) => (tag.id === saved.id ? saved : tag))
          : [saved, ...currentTags],
      );
      setDraft(emptyDraft);
      setEditingTagId(null);
      notify({ title: "标签已保存", variant: "success" });
    } catch {
      notify({
        title: "标签保存失败",
        description: "标签需要中文，#NTR 例外；未成年人相关标签会被拒绝。",
        variant: "error",
      });
    } finally {
      setIsSubmitting(false);
    }
  }

  function handleEdit(tag: TagRecord) {
    setEditingTagId(tag.id);
    setDraft({
      canonicalName: tag.canonicalName,
      aliases: tag.aliases ?? "",
      relatedTags: tag.relatedTags ?? "",
    });
  }

  async function handleDelete() {
    if (!deletingTag) {
      return;
    }

    try {
      const session = requireSession();
      await deleteTag(session.accountId, deletingTag.id);
      setTags((currentTags) => currentTags.filter((tag) => tag.id !== deletingTag.id));
      notify({ title: "标签已删除", variant: "success" });
    } catch {
      notify({
        title: "标签删除失败",
        description: "请稍后重试。",
        variant: "error",
      });
    } finally {
      setDeletingTag(null);
    }
  }

  return (
    <section className="grid content-start gap-4 rounded-2xl border border-[var(--color-border)] bg-[var(--color-surface-soft)] p-5">
      <div>
        <h2 className="text-xl font-semibold text-[var(--color-text-strong)]">{title}</h2>
        <p className="mt-1 text-sm text-[var(--color-muted)]">
          别名和关联词用逗号分隔；搜索时会命中规范标签、别名和关联词。
        </p>
      </div>

      <form onSubmit={handleSubmit} className="grid gap-3">
        <Input
          name={`${scope}-canonical-name`}
          label="规范标签"
          placeholder="#标签"
          value={draft.canonicalName}
          onChange={(event) => setDraft({ ...draft, canonicalName: event.target.value })}
        />
        <Input
          name={`${scope}-aliases`}
          label="同义词"
          placeholder="#繁体, #别名"
          value={draft.aliases}
          onChange={(event) => setDraft({ ...draft, aliases: event.target.value })}
        />
        <Input
          name={`${scope}-related-tags`}
          label="关联词"
          placeholder="#关联标签"
          value={draft.relatedTags}
          onChange={(event) => setDraft({ ...draft, relatedTags: event.target.value })}
        />
        <div className="flex flex-wrap justify-end gap-3">
          {editingTagId ? (
            <Button
              type="button"
              variant="ghost"
              onClick={() => {
                setEditingTagId(null);
                setDraft(emptyDraft);
              }}
            >
              取消编辑
            </Button>
          ) : null}
          <Button type="submit" disabled={isSubmitting}>
            {editingTagId ? "保存修改" : "新增标签"}
          </Button>
        </div>
      </form>

      <div className="grid gap-2">
        {tags.map((tag) => (
          <div
            key={tag.id}
            className="grid gap-2 rounded-2xl border border-[var(--color-border)] bg-[var(--color-input)] px-4 py-3"
          >
            <div className="flex flex-wrap items-center justify-between gap-3">
              <div>
                <p className="text-sm font-semibold text-[var(--color-text-strong)]">
                  {tag.canonicalName}
                </p>
                <p className="text-xs text-[var(--color-muted)]">
                  {tag.isPreset ? "预置标签" : "自定义标签"}
                </p>
              </div>
              <div className="flex gap-2">
                <Button type="button" variant="secondary" onClick={() => handleEdit(tag)}>
                  编辑
                </Button>
                <Button type="button" variant="danger" onClick={() => setDeletingTag(tag)}>
                  删除
                </Button>
              </div>
            </div>
            {tag.aliases || tag.relatedTags ? (
              <p className="text-xs text-[var(--color-muted)]">
                {tag.aliases ? `同义词：${tag.aliases}` : ""}
                {tag.aliases && tag.relatedTags ? " / " : ""}
                {tag.relatedTags ? `关联词：${tag.relatedTags}` : ""}
              </p>
            ) : null}
          </div>
        ))}
      </div>

      <ConfirmDialog
        open={Boolean(deletingTag)}
        title="删除标签"
        description={
          deletingTag
            ? `确认删除 ${deletingTag.canonicalName}？已有记录上的该标签也会移除。`
            : ""
        }
        confirmLabel="确认删除"
        variant="danger"
        onOpenChange={(open) => {
          if (!open) {
            setDeletingTag(null);
          }
        }}
        onConfirm={handleDelete}
      />
    </section>
  );
}
