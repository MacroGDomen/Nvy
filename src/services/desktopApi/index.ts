import { invoke } from "@tauri-apps/api/core";
import type {
  AccountDataSummary,
  AccountDefaults,
  AccountSession,
  AppInfo,
  DatabaseStatus,
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

function mapAccountSession(session: RawAccountSession): AccountSession {
  return {
    accountId: session.account_id,
    username: session.username,
  };
}
