export type AppInfo = {
  name: string;
  version: string;
};

export type DatabaseStatus = {
  path: string;
  schemaVersion: number;
};

export type AccountSession = {
  accountId: string;
  username: string;
};

export type AccountDataSummary = {
  accountId: string;
  videoCount: number;
  actressCount: number;
};

export type AccountDefaults = {
  accountId: string;
  recommendationReferenceLimit: number;
  videoTagCount: number;
  actressTagCount: number;
};

export type AppDataPaths = {
  appDataDir: string;
  databasePath: string;
  cacheDir: string;
  coversDir: string;
  actressesDir: string;
};

export type ImageCacheKind = "cover" | "actress";

export type CachedImage = {
  relativePath: string;
  absolutePath: string;
};

export type BackupResult = {
  path: string;
  format: string;
  entryCount: number;
};

export type ImportResult = {
  accountId: string;
  videoCount: number;
  actressCount: number;
  tagCount: number;
  assetCount: number;
};

export type MetadataCandidate = {
  id: string;
  accountId: string;
  targetType: "video" | "actress";
  query: string;
  source: string;
  payloadJson: string;
  createdAt: string;
};

export type LlmApiType = "responses" | "chat_completions" | "custom";

export type LlmSettings = {
  accountId: string;
  apiType?: LlmApiType;
  baseUrl?: string;
  providerName?: string;
  model?: string;
  temperature?: number;
  maxTokens?: number;
  recommendationReferenceLimit: number;
  translationPrompt: string;
  recommendationPrompt: string;
  enableLlmTranslation: boolean;
  recommendationDefaultEnabled: boolean;
  metadataAllowBrowserCookies: boolean;
  hasApiKey: boolean;
};

export type LlmSettingsInput = {
  apiType?: LlmApiType;
  baseUrl?: string;
  providerName?: string;
  model?: string;
  temperature?: number;
  maxTokens?: number;
  recommendationReferenceLimit: number;
  translationPrompt?: string;
  recommendationPrompt?: string;
  enableLlmTranslation: boolean;
  recommendationDefaultEnabled: boolean;
  metadataAllowBrowserCookies: boolean;
};

export type SecretStatus = {
  hasApiKey: boolean;
  storage: string;
};

export type RecommendationVideoCandidate = {
  id: string;
  code: string;
  title?: string;
  summary?: string;
  actorNames?: string;
  workType: WorkType;
  review?: string;
};

export type RecommendationActressCandidate = {
  id: string;
  name: string;
  simplifiedChineseName?: string;
  japaneseName?: string;
  romanizedName?: string;
  measurements?: string;
  cupSize?: string;
  birthday?: string;
  note?: string;
};

export type RecommendationPayload = {
  userText: string;
  prompt: string;
  referenceLimit: number;
  videos: RecommendationVideoCandidate[];
  actresses: RecommendationActressCandidate[];
};

export type RecommendationResult = {
  payload: RecommendationPayload;
  text: string;
};

export type TranslationState = {
  videoId: string;
  translationStatus?: string;
};

export type WorkType = "single" | "multiple" | "amateur";
export type TagScope = "video" | "actress";

export type VideoRecord = {
  id: string;
  accountId: string;
  code: string;
  title?: string;
  coverPath?: string;
  releaseDate?: string;
  durationMinutes?: number;
  sourceUrl?: string;
  summary?: string;
  actorNames?: string;
  workType: WorkType;
  review?: string;
  createdAt: string;
  updatedAt: string;
};

export type VideoInput = {
  code: string;
  title?: string;
  coverPath?: string;
  releaseDate?: string;
  durationMinutes?: number;
  sourceUrl?: string;
  summary?: string;
  actorNames?: string;
  workType: WorkType;
  review?: string;
};

export type ActressRecord = {
  id: string;
  accountId: string;
  name: string;
  simplifiedChineseName?: string;
  formerChineseNames?: string;
  traditionalChineseName?: string;
  japaneseName?: string;
  romanizedName?: string;
  defaultDisplayNameType?: string;
  avatarPath?: string;
  measurements?: string;
  cupSize?: string;
  birthday?: string;
  heightCm?: number;
  debutDate?: string;
  wikipediaZhUrl?: string;
  note?: string;
  createdAt: string;
  updatedAt: string;
};

export type ActressInput = {
  name: string;
  simplifiedChineseName?: string;
  formerChineseNames?: string;
  traditionalChineseName?: string;
  japaneseName?: string;
  romanizedName?: string;
  defaultDisplayNameType?: string;
  avatarPath?: string;
  measurements?: string;
  cupSize?: string;
  birthday?: string;
  heightCm?: number;
  debutDate?: string;
  wikipediaZhUrl?: string;
  note?: string;
};

export type AssociationSuggestion = {
  video: VideoRecord;
  actress: ActressRecord;
};

export type TagRecord = {
  id: string;
  accountId: string;
  scope: TagScope;
  canonicalName: string;
  aliases?: string;
  relatedTags?: string;
  isPreset: boolean;
  createdAt: string;
  updatedAt: string;
};

export type TagInput = {
  scope: TagScope;
  canonicalName: string;
  aliases?: string;
  relatedTags?: string;
};

export type TagMatch = {
  tag: TagRecord;
  matchedBy: string;
};
