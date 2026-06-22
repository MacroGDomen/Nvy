import { convertFileSrc, invoke } from "@tauri-apps/api/core";
import {
  clearLlmApiKeySecret,
  getLlmApiKeyStatus,
  saveLlmApiKeySecret,
} from "../stronghold/llmSecrets";
import type {
  AccountDataSummary,
  AccountDefaults,
  AccountSession,
  ActressInput,
  ActressRecord,
  AppDataPaths,
  AppInfo,
  AssociationSuggestion,
  BackupResult,
  CachedImage,
  DatabaseStatus,
  ImageCacheKind,
  ImportResult,
  LlmSettings,
  LlmSettingsInput,
  MetadataCandidate,
  RecommendationPayload,
  SecretStatus,
  TagInput,
  TagMatch,
  TagRecord,
  TagScope,
  TranslationState,
  VideoInput,
  VideoRecord,
} from "./types";

type RawDatabaseStatus = {
  path: string;
  schema_version: number;
};

type RawAccountSession = {
  account_id: string;
  username: string;
};

type RawAccountDataSummary = {
  account_id: string;
  video_count: number;
  actress_count: number;
};

type RawAccountDefaults = {
  account_id: string;
  recommendation_reference_limit: number;
  video_tag_count: number;
  actress_tag_count: number;
};

type RawAppDataPaths = {
  app_data_dir: string;
  database_path: string;
  cache_dir: string;
  covers_dir: string;
  actresses_dir: string;
};

type RawCachedImage = {
  relative_path: string;
  absolute_path: string;
};

type RawBackupResult = {
  path: string;
  format: string;
  entry_count: number;
};

type RawImportResult = {
  account_id: string;
  video_count: number;
  actress_count: number;
  tag_count: number;
  asset_count: number;
};

type RawVideoRecord = {
  id: string;
  account_id: string;
  code: string;
  title?: string | null;
  cover_path?: string | null;
  release_date?: string | null;
  duration_minutes?: number | null;
  source_url?: string | null;
  summary?: string | null;
  actor_names?: string | null;
  work_type: VideoRecord["workType"];
  review?: string | null;
  created_at: string;
  updated_at: string;
};

type RawMetadataCandidate = {
  id: string;
  account_id: string;
  target_type: "video" | "actress";
  query: string;
  source: string;
  payload_json: string;
  created_at: string;
};

type RawLlmSettings = {
  account_id: string;
  api_type?: LlmSettings["apiType"] | null;
  base_url?: string | null;
  provider_name?: string | null;
  model?: string | null;
  temperature?: number | null;
  max_tokens?: number | null;
  recommendation_reference_limit: number;
  translation_prompt: string;
  recommendation_prompt: string;
  enable_llm_translation: boolean;
  recommendation_default_enabled: boolean;
  metadata_allow_browser_cookies: boolean;
  has_api_key: boolean;
};

type RawSecretStatus = {
  has_api_key: boolean;
  storage: string;
};

type RawRecommendationPayload = {
  user_text: string;
  prompt: string;
  reference_limit: number;
  videos: Array<{
    id: string;
    code: string;
    title?: string | null;
    summary?: string | null;
    actor_names?: string | null;
    work_type: VideoRecord["workType"];
    review?: string | null;
  }>;
  actresses: Array<{
    id: string;
    name: string;
    simplified_chinese_name?: string | null;
    japanese_name?: string | null;
    romanized_name?: string | null;
    measurements?: string | null;
    cup_size?: string | null;
    birthday?: string | null;
    note?: string | null;
  }>;
};

type RawTranslationState = {
  video_id: string;
  translation_status?: string | null;
};

type RawAssociationSuggestion = {
  video: RawVideoRecord;
  actress: RawActressRecord;
};

type RawTagRecord = {
  id: string;
  account_id: string;
  scope: TagScope;
  canonical_name: string;
  aliases?: string | null;
  related_tags?: string | null;
  is_preset: boolean;
  created_at: string;
  updated_at: string;
};

type RawTagMatch = {
  tag: RawTagRecord;
  matched_by: string;
};

type RawActressRecord = {
  id: string;
  account_id: string;
  name: string;
  simplified_chinese_name?: string | null;
  former_chinese_names?: string | null;
  traditional_chinese_name?: string | null;
  japanese_name?: string | null;
  romanized_name?: string | null;
  default_display_name_type?: string | null;
  avatar_path?: string | null;
  measurements?: string | null;
  cup_size?: string | null;
  birthday?: string | null;
  height_cm?: number | null;
  debut_date?: string | null;
  wikipedia_zh_url?: string | null;
  note?: string | null;
  created_at: string;
  updated_at: string;
};

export async function getAppInfo() {
  return invoke<AppInfo>("app_info");
}

export async function initializeDatabase(): Promise<DatabaseStatus> {
  const status = await invoke<RawDatabaseStatus>("initialize_database");

  return {
    path: status.path,
    schemaVersion: status.schema_version,
  };
}

export async function getAppDataPaths(): Promise<AppDataPaths> {
  const paths = await invoke<RawAppDataPaths>("app_data_paths");

  return {
    appDataDir: paths.app_data_dir,
    databasePath: paths.database_path,
    cacheDir: paths.cache_dir,
    coversDir: paths.covers_dir,
    actressesDir: paths.actresses_dir,
  };
}

export async function cacheLocalImage(
  sourcePath: string,
  kind: ImageCacheKind,
  ownerId: string,
): Promise<CachedImage> {
  const cached = await invoke<RawCachedImage>("cache_local_image", {
    sourcePath,
    kind,
    ownerId,
  });

  return {
    relativePath: cached.relative_path,
    absolutePath: cached.absolute_path,
  };
}

export async function resolveCachedAsset(relativePath: string): Promise<string> {
  return invoke<string>("resolve_cached_asset", { relativePath });
}

export async function cachedAssetUrl(relativePath: string): Promise<string> {
  const absolutePath = await resolveCachedAsset(relativePath);

  return convertFileSrc(absolutePath);
}

export async function exportPlainBackup(
  accountId: string,
  outputPath: string,
): Promise<BackupResult> {
  const result = await invoke<RawBackupResult>("export_plain_backup", {
    accountId,
    outputPath,
  });

  return mapBackupResult(result);
}

export async function exportEncryptedBackup(
  accountId: string,
  outputPath: string,
  password: string,
): Promise<BackupResult> {
  const result = await invoke<RawBackupResult>("export_encrypted_backup", {
    accountId,
    outputPath,
    password,
  });

  return mapBackupResult(result);
}

export async function importPlainBackup(
  accountId: string,
  inputPath: string,
): Promise<ImportResult> {
  const result = await invoke<RawImportResult>("import_plain_backup", {
    accountId,
    inputPath,
  });

  return mapImportResult(result);
}

export async function importEncryptedBackup(
  accountId: string,
  inputPath: string,
  password: string,
): Promise<ImportResult> {
  const result = await invoke<RawImportResult>("import_encrypted_backup", {
    accountId,
    inputPath,
    password,
  });

  return mapImportResult(result);
}

export async function registerAccount(
  username: string,
  password: string,
): Promise<AccountSession> {
  const session = await invoke<RawAccountSession>("register_account", {
    username,
    password,
  });

  return mapAccountSession(session);
}

export async function loginAccount(
  username: string,
  password: string,
): Promise<AccountSession> {
  const session = await invoke<RawAccountSession>("login_account", {
    username,
    password,
  });

  return mapAccountSession(session);
}

export async function getAccountDataSummary(
  accountId: string,
): Promise<AccountDataSummary> {
  const summary = await invoke<RawAccountDataSummary>("account_data_summary", {
    accountId,
  });

  return {
    accountId: summary.account_id,
    videoCount: summary.video_count,
    actressCount: summary.actress_count,
  };
}

export async function initializeAccountDefaults(
  accountId: string,
): Promise<AccountDefaults> {
  const defaults = await invoke<RawAccountDefaults>("initialize_account_defaults", {
    accountId,
  });

  return {
    accountId: defaults.account_id,
    recommendationReferenceLimit: defaults.recommendation_reference_limit,
    videoTagCount: defaults.video_tag_count,
    actressTagCount: defaults.actress_tag_count,
  };
}

export async function listVideos(accountId: string): Promise<VideoRecord[]> {
  const videos = await invoke<RawVideoRecord[]>("list_videos", { accountId });

  return videos.map(mapVideoRecord);
}

export async function createVideo(
  accountId: string,
  input: VideoInput,
): Promise<VideoRecord> {
  const video = await invoke<RawVideoRecord>("create_video", {
    accountId,
    input: toRawVideoInput(input),
  });

  return mapVideoRecord(video);
}

export async function updateVideo(
  accountId: string,
  videoId: string,
  input: VideoInput,
): Promise<VideoRecord> {
  const video = await invoke<RawVideoRecord>("update_video", {
    accountId,
    videoId,
    input: toRawVideoInput(input),
  });

  return mapVideoRecord(video);
}

export async function listActresses(
  accountId: string,
): Promise<ActressRecord[]> {
  const actresses = await invoke<RawActressRecord[]>("list_actresses", {
    accountId,
  });

  return actresses.map(mapActressRecord);
}

export async function createActress(
  accountId: string,
  input: ActressInput,
): Promise<ActressRecord> {
  const actress = await invoke<RawActressRecord>("create_actress", {
    accountId,
    input: toRawActressInput(input),
  });

  return mapActressRecord(actress);
}

export async function updateActress(
  accountId: string,
  actressId: string,
  input: ActressInput,
): Promise<ActressRecord> {
  const actress = await invoke<RawActressRecord>("update_actress", {
    accountId,
    actressId,
    input: toRawActressInput(input),
  });

  return mapActressRecord(actress);
}

export async function listVideoActresses(
  accountId: string,
  videoId: string,
): Promise<ActressRecord[]> {
  const actresses = await invoke<RawActressRecord[]>("list_video_actresses", {
    accountId,
    videoId,
  });

  return actresses.map(mapActressRecord);
}

export async function listActressVideos(
  accountId: string,
  actressId: string,
): Promise<VideoRecord[]> {
  const videos = await invoke<RawVideoRecord[]>("list_actress_videos", {
    accountId,
    actressId,
  });

  return videos.map(mapVideoRecord);
}

export async function setVideoActresses(
  accountId: string,
  videoId: string,
  actressIds: string[],
): Promise<ActressRecord[]> {
  const actresses = await invoke<RawActressRecord[]>("set_video_actresses", {
    accountId,
    videoId,
    actressIds,
  });

  return actresses.map(mapActressRecord);
}

export async function addVideoActress(
  accountId: string,
  videoId: string,
  actressId: string,
): Promise<ActressRecord[]> {
  const actresses = await invoke<RawActressRecord[]>("add_video_actress", {
    accountId,
    videoId,
    actressId,
  });

  return actresses.map(mapActressRecord);
}

export async function listAssociationSuggestions(
  accountId: string,
  actressId: string,
): Promise<AssociationSuggestion[]> {
  const suggestions = await invoke<RawAssociationSuggestion[]>(
    "list_association_suggestions",
    {
      accountId,
      actressId,
    },
  );

  return suggestions.map((suggestion) => ({
    video: mapVideoRecord(suggestion.video),
    actress: mapActressRecord(suggestion.actress),
  }));
}

export async function deleteVideo(
  accountId: string,
  videoId: string,
): Promise<void> {
  await invoke("delete_video", { accountId, videoId });
}

export async function deleteActress(
  accountId: string,
  actressId: string,
): Promise<void> {
  await invoke("delete_actress", { accountId, actressId });
}

export async function listTags(
  accountId: string,
  scope: TagScope,
): Promise<TagRecord[]> {
  const tags = await invoke<RawTagRecord[]>("list_tags", { accountId, scope });

  return tags.map(mapTagRecord);
}

export async function createTag(
  accountId: string,
  input: TagInput,
): Promise<TagRecord> {
  const tag = await invoke<RawTagRecord>("create_tag", {
    accountId,
    input: toRawTagInput(input),
  });

  return mapTagRecord(tag);
}

export async function updateTag(
  accountId: string,
  tagId: string,
  input: TagInput,
): Promise<TagRecord> {
  const tag = await invoke<RawTagRecord>("update_tag", {
    accountId,
    tagId,
    input: toRawTagInput(input),
  });

  return mapTagRecord(tag);
}

export async function deleteTag(
  accountId: string,
  tagId: string,
): Promise<void> {
  await invoke("delete_tag", { accountId, tagId });
}

export async function matchTags(
  accountId: string,
  scope: TagScope,
  query: string,
): Promise<TagMatch[]> {
  const matches = await invoke<RawTagMatch[]>("match_tags", {
    accountId,
    scope,
    query,
  });

  return matches.map((match) => ({
    tag: mapTagRecord(match.tag),
    matchedBy: match.matched_by,
  }));
}

export async function listVideoTags(
  accountId: string,
  videoId: string,
): Promise<TagRecord[]> {
  const tags = await invoke<RawTagRecord[]>("list_video_tags", {
    accountId,
    videoId,
  });

  return tags.map(mapTagRecord);
}

export async function setVideoTags(
  accountId: string,
  videoId: string,
  tagIds: string[],
): Promise<TagRecord[]> {
  const tags = await invoke<RawTagRecord[]>("set_video_tags", {
    accountId,
    videoId,
    tagIds,
  });

  return tags.map(mapTagRecord);
}

export async function listActressTags(
  accountId: string,
  actressId: string,
): Promise<TagRecord[]> {
  const tags = await invoke<RawTagRecord[]>("list_actress_tags", {
    accountId,
    actressId,
  });

  return tags.map(mapTagRecord);
}

export async function setActressTags(
  accountId: string,
  actressId: string,
  tagIds: string[],
): Promise<TagRecord[]> {
  const tags = await invoke<RawTagRecord[]>("set_actress_tags", {
    accountId,
    actressId,
    tagIds,
  });

  return tags.map(mapTagRecord);
}

export async function autoTagVideo(
  accountId: string,
  videoId: string,
): Promise<TagRecord[]> {
  const tags = await invoke<RawTagRecord[]>("auto_tag_video", {
    accountId,
    videoId,
  });

  return tags.map(mapTagRecord);
}

export async function openExternalUrl(url: string): Promise<void> {
  await invoke("plugin:opener|open_url", { url });
}

export async function matchVideoMetadata(
  accountId: string,
  videoId: string,
  query: string,
): Promise<MetadataCandidate[]> {
  const candidates = await invoke<RawMetadataCandidate[]>("match_video_metadata", {
    accountId,
    videoId,
    query,
  });

  return candidates.map(mapMetadataCandidate);
}

export async function matchActressMetadata(
  accountId: string,
  actressId: string,
  query: string,
): Promise<MetadataCandidate[]> {
  const candidates = await invoke<RawMetadataCandidate[]>(
    "match_actress_metadata",
    {
      accountId,
      actressId,
      query,
    },
  );

  return candidates.map(mapMetadataCandidate);
}

export async function applyMetadataCandidate(
  accountId: string,
  candidateId: string,
): Promise<MetadataCandidate> {
  const candidate = await invoke<RawMetadataCandidate>("apply_metadata_candidate", {
    accountId,
    candidateId,
  });

  return mapMetadataCandidate(candidate);
}

export async function getLlmSettings(accountId: string): Promise<LlmSettings> {
  const settings = await invoke<RawLlmSettings>("get_llm_settings", { accountId });
  const secretStatus = await getLlmApiKeyStatus(accountId);

  return {
    ...mapLlmSettings(settings),
    hasApiKey: secretStatus.hasApiKey,
  };
}

export async function saveLlmSettings(
  accountId: string,
  input: LlmSettingsInput,
): Promise<LlmSettings> {
  const settings = await invoke<RawLlmSettings>("save_llm_settings", {
    accountId,
    input: toRawLlmSettingsInput(input),
  });
  const secretStatus = await getLlmApiKeyStatus(accountId);

  return {
    ...mapLlmSettings(settings),
    hasApiKey: secretStatus.hasApiKey,
  };
}

export async function saveLlmApiKey(
  accountId: string,
  apiKey: string,
): Promise<SecretStatus> {
  return saveLlmApiKeySecret(accountId, apiKey);
}

export async function clearLlmApiKey(accountId: string): Promise<SecretStatus> {
  return clearLlmApiKeySecret(accountId);
}

export async function buildRecommendationPayload(
  accountId: string,
  userText: string,
): Promise<RecommendationPayload> {
  const payload = await invoke<RawRecommendationPayload>(
    "build_recommendation_payload",
    {
      accountId,
      userText,
    },
  );

  return mapRecommendationPayload(payload);
}

export async function requestVideoTranslation(
  accountId: string,
  videoId: string,
): Promise<TranslationState> {
  const state = await invoke<RawTranslationState>("request_video_translation", {
    accountId,
    videoId,
  });

  return mapTranslationState(state);
}

export async function cancelVideoTranslation(
  accountId: string,
  videoId: string,
): Promise<TranslationState> {
  const state = await invoke<RawTranslationState>("cancel_video_translation", {
    accountId,
    videoId,
  });

  return mapTranslationState(state);
}

function mapAccountSession(session: RawAccountSession): AccountSession {
  return {
    accountId: session.account_id,
    username: session.username,
  };
}

function mapBackupResult(result: RawBackupResult): BackupResult {
  return {
    path: result.path,
    format: result.format,
    entryCount: result.entry_count,
  };
}

function mapImportResult(result: RawImportResult): ImportResult {
  return {
    accountId: result.account_id,
    videoCount: result.video_count,
    actressCount: result.actress_count,
    tagCount: result.tag_count,
    assetCount: result.asset_count,
  };
}

function mapMetadataCandidate(candidate: RawMetadataCandidate): MetadataCandidate {
  return {
    id: candidate.id,
    accountId: candidate.account_id,
    targetType: candidate.target_type,
    query: candidate.query,
    source: candidate.source,
    payloadJson: candidate.payload_json,
    createdAt: candidate.created_at,
  };
}

function mapLlmSettings(settings: RawLlmSettings): LlmSettings {
  return {
    accountId: settings.account_id,
    apiType: settings.api_type ?? undefined,
    baseUrl: settings.base_url ?? undefined,
    providerName: settings.provider_name ?? undefined,
    model: settings.model ?? undefined,
    temperature: settings.temperature ?? undefined,
    maxTokens: settings.max_tokens ?? undefined,
    recommendationReferenceLimit: settings.recommendation_reference_limit,
    translationPrompt: settings.translation_prompt,
    recommendationPrompt: settings.recommendation_prompt,
    enableLlmTranslation: settings.enable_llm_translation,
    recommendationDefaultEnabled: settings.recommendation_default_enabled,
    metadataAllowBrowserCookies: settings.metadata_allow_browser_cookies,
    hasApiKey: settings.has_api_key,
  };
}

function mapSecretStatus(status: RawSecretStatus): SecretStatus {
  return {
    hasApiKey: status.has_api_key,
    storage: status.storage,
  };
}

function mapRecommendationPayload(payload: RawRecommendationPayload): RecommendationPayload {
  return {
    userText: payload.user_text,
    prompt: payload.prompt,
    referenceLimit: payload.reference_limit,
    videos: payload.videos.map((video) => ({
      id: video.id,
      code: video.code,
      title: video.title ?? undefined,
      summary: video.summary ?? undefined,
      actorNames: video.actor_names ?? undefined,
      workType: video.work_type,
      review: video.review ?? undefined,
    })),
    actresses: payload.actresses.map((actress) => ({
      id: actress.id,
      name: actress.name,
      simplifiedChineseName: actress.simplified_chinese_name ?? undefined,
      japaneseName: actress.japanese_name ?? undefined,
      romanizedName: actress.romanized_name ?? undefined,
      measurements: actress.measurements ?? undefined,
      cupSize: actress.cup_size ?? undefined,
      birthday: actress.birthday ?? undefined,
      note: actress.note ?? undefined,
    })),
  };
}

function mapTranslationState(state: RawTranslationState): TranslationState {
  return {
    videoId: state.video_id,
    translationStatus: state.translation_status ?? undefined,
  };
}

function mapVideoRecord(video: RawVideoRecord): VideoRecord {
  return {
    id: video.id,
    accountId: video.account_id,
    code: video.code,
    title: video.title ?? undefined,
    coverPath: video.cover_path ?? undefined,
    releaseDate: video.release_date ?? undefined,
    durationMinutes: video.duration_minutes ?? undefined,
    sourceUrl: video.source_url ?? undefined,
    summary: video.summary ?? undefined,
    actorNames: video.actor_names ?? undefined,
    workType: video.work_type,
    review: video.review ?? undefined,
    createdAt: video.created_at,
    updatedAt: video.updated_at,
  };
}

function mapActressRecord(actress: RawActressRecord): ActressRecord {
  return {
    id: actress.id,
    accountId: actress.account_id,
    name: actress.name,
    simplifiedChineseName: actress.simplified_chinese_name ?? undefined,
    formerChineseNames: actress.former_chinese_names ?? undefined,
    traditionalChineseName: actress.traditional_chinese_name ?? undefined,
    japaneseName: actress.japanese_name ?? undefined,
    romanizedName: actress.romanized_name ?? undefined,
    defaultDisplayNameType: actress.default_display_name_type ?? undefined,
    avatarPath: actress.avatar_path ?? undefined,
    measurements: actress.measurements ?? undefined,
    cupSize: actress.cup_size ?? undefined,
    birthday: actress.birthday ?? undefined,
    heightCm: actress.height_cm ?? undefined,
    debutDate: actress.debut_date ?? undefined,
    wikipediaZhUrl: actress.wikipedia_zh_url ?? undefined,
    note: actress.note ?? undefined,
    createdAt: actress.created_at,
    updatedAt: actress.updated_at,
  };
}

function mapTagRecord(tag: RawTagRecord): TagRecord {
  return {
    id: tag.id,
    accountId: tag.account_id,
    scope: tag.scope,
    canonicalName: tag.canonical_name,
    aliases: tag.aliases ?? undefined,
    relatedTags: tag.related_tags ?? undefined,
    isPreset: tag.is_preset,
    createdAt: tag.created_at,
    updatedAt: tag.updated_at,
  };
}

function toRawVideoInput(input: VideoInput) {
  return {
    code: input.code,
    title: input.title,
    cover_path: input.coverPath,
    release_date: input.releaseDate,
    duration_minutes: input.durationMinutes,
    source_url: input.sourceUrl,
    summary: input.summary,
    actor_names: input.actorNames,
    work_type: input.workType,
    review: input.review,
  };
}

function toRawActressInput(input: ActressInput) {
  return {
    name: input.name,
    simplified_chinese_name: input.simplifiedChineseName,
    former_chinese_names: input.formerChineseNames,
    traditional_chinese_name: input.traditionalChineseName,
    japanese_name: input.japaneseName,
    romanized_name: input.romanizedName,
    default_display_name_type: input.defaultDisplayNameType,
    avatar_path: input.avatarPath,
    measurements: input.measurements,
    cup_size: input.cupSize,
    birthday: input.birthday,
    height_cm: input.heightCm,
    debut_date: input.debutDate,
    wikipedia_zh_url: input.wikipediaZhUrl,
    note: input.note,
  };
}

function toRawTagInput(input: TagInput) {
  return {
    scope: input.scope,
    canonical_name: input.canonicalName,
    aliases: input.aliases,
    related_tags: input.relatedTags,
  };
}

function toRawLlmSettingsInput(input: LlmSettingsInput) {
  return {
    api_type: input.apiType,
    base_url: input.baseUrl,
    provider_name: input.providerName,
    model: input.model,
    temperature: input.temperature,
    max_tokens: input.maxTokens,
    recommendation_reference_limit: input.recommendationReferenceLimit,
    translation_prompt: input.translationPrompt,
    recommendation_prompt: input.recommendationPrompt,
    enable_llm_translation: input.enableLlmTranslation,
    recommendation_default_enabled: input.recommendationDefaultEnabled,
    metadata_allow_browser_cookies: input.metadataAllowBrowserCookies,
  };
}
