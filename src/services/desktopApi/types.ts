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
