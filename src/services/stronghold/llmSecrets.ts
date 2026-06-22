import { invoke } from "@tauri-apps/api/core";
import type { SecretStatus } from "../desktopApi/types";

type RawAppDataPaths = {
  app_data_dir: string;
};

const CLIENT_NAME = "nvy-llm";
const SNAPSHOT_FILE_NAME = "llm-secrets.stronghold";
const STRONGHOLD_PASSWORD = "nvy-llm-secrets-v1";
const STORAGE_NAME = "tauri-stronghold";

const textEncoder = new TextEncoder();
const textDecoder = new TextDecoder();

export async function getLlmApiKeyStatus(accountId: string): Promise<SecretStatus> {
  const value = await readLlmApiKey(accountId);

  return {
    hasApiKey: Boolean(value),
    storage: STORAGE_NAME,
  };
}

export async function saveLlmApiKeySecret(
  accountId: string,
  apiKey: string,
): Promise<SecretStatus> {
  const normalizedKey = apiKey.trim();
  if (!normalizedKey) {
    throw new Error("API Key is required.");
  }

  const snapshotPath = await initializeStronghold(accountId);
  await invoke("plugin:stronghold|save_store_record", {
    snapshotPath,
    client: CLIENT_NAME,
    key: recordKey(accountId),
    value: Array.from(textEncoder.encode(normalizedKey)),
    lifetime: null,
  });
  await invoke("plugin:stronghold|save", { snapshotPath });

  return {
    hasApiKey: true,
    storage: STORAGE_NAME,
  };
}

export async function clearLlmApiKeySecret(accountId: string): Promise<SecretStatus> {
  const snapshotPath = await initializeStronghold(accountId);
  await invoke("plugin:stronghold|remove_store_record", {
    snapshotPath,
    client: CLIENT_NAME,
    key: recordKey(accountId),
  });
  await invoke("plugin:stronghold|save", { snapshotPath });

  return {
    hasApiKey: false,
    storage: STORAGE_NAME,
  };
}

export async function readLlmApiKey(accountId: string): Promise<string | null> {
  const snapshotPath = await initializeStronghold(accountId);
  const value = await invoke<number[] | null>("plugin:stronghold|get_store_record", {
    snapshotPath,
    client: CLIENT_NAME,
    key: recordKey(accountId),
  });

  if (!value?.length) {
    return null;
  }

  return textDecoder.decode(new Uint8Array(value));
}

async function initializeStronghold(accountId: string): Promise<string> {
  const snapshotPath = await strongholdSnapshotPath();
  await invoke("plugin:stronghold|initialize", {
    snapshotPath,
    password: strongholdPassword(accountId),
  });

  try {
    await invoke("plugin:stronghold|load_client", {
      snapshotPath,
      client: CLIENT_NAME,
    });
  } catch {
    await invoke("plugin:stronghold|create_client", {
      snapshotPath,
      client: CLIENT_NAME,
    });
  }

  return snapshotPath;
}

async function strongholdSnapshotPath(): Promise<string> {
  const paths = await invoke<RawAppDataPaths>("app_data_paths");
  const separator = paths.app_data_dir.includes("\\") ? "\\" : "/";

  return `${paths.app_data_dir}${separator}${SNAPSHOT_FILE_NAME}`;
}

function strongholdPassword(accountId: string): string {
  const normalizedAccountId = accountId.trim();
  if (!normalizedAccountId) {
    throw new Error("Account id is required.");
  }

  return STRONGHOLD_PASSWORD;
}

function recordKey(accountId: string): string {
  const normalizedAccountId = accountId.trim();
  if (!normalizedAccountId) {
    throw new Error("Account id is required.");
  }

  return `llm-api-key:${normalizedAccountId}`;
}
