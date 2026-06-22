CREATE TABLE accounts (
  id TEXT PRIMARY KEY,
  username TEXT NOT NULL UNIQUE,
  password_hash TEXT NOT NULL,
  created_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE account_settings (
  account_id TEXT PRIMARY KEY NOT NULL,
  llm_api_type TEXT,
  llm_base_url TEXT,
  llm_provider_name TEXT,
  llm_model TEXT,
  llm_temperature REAL,
  llm_max_tokens INTEGER,
  llm_recommendation_reference_limit INTEGER NOT NULL DEFAULT 30,
  llm_translation_prompt TEXT,
  llm_recommendation_prompt TEXT,
  enable_llm_translation INTEGER NOT NULL DEFAULT 0,
  llm_recommendation_default_enabled INTEGER NOT NULL DEFAULT 1,
  created_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP,
  updated_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP,
  FOREIGN KEY (account_id) REFERENCES accounts(id) ON DELETE CASCADE
);

CREATE TABLE videos (
  id TEXT PRIMARY KEY,
  account_id TEXT NOT NULL,
  code TEXT NOT NULL,
  title TEXT,
  original_title TEXT,
  cover_path TEXT,
  actress_names TEXT,
  release_date TEXT,
  duration_minutes INTEGER,
  source_url TEXT,
  metadata_source TEXT,
  summary TEXT,
  original_summary TEXT,
  translation_status TEXT,
  translated_at TEXT,
  work_type TEXT NOT NULL,
  review TEXT,
  review_created_at TEXT,
  review_updated_at TEXT,
  created_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP,
  updated_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP,
  FOREIGN KEY (account_id) REFERENCES accounts(id) ON DELETE CASCADE
);

CREATE TABLE actresses (
  id TEXT PRIMARY KEY,
  account_id TEXT NOT NULL,
  name TEXT NOT NULL,
  simplified_chinese_name TEXT,
  former_chinese_names TEXT,
  traditional_chinese_name TEXT,
  japanese_name TEXT,
  romanized_name TEXT,
  default_display_name_type TEXT,
  avatar_path TEXT,
  measurements TEXT,
  cup_size TEXT,
  birthday TEXT,
  height_cm INTEGER,
  debut_date TEXT,
  wikipedia_zh_url TEXT,
  note TEXT,
  metadata_source TEXT,
  created_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP,
  updated_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP,
  FOREIGN KEY (account_id) REFERENCES accounts(id) ON DELETE CASCADE
);

CREATE TABLE video_actresses (
  video_id TEXT NOT NULL,
  actress_id TEXT NOT NULL,
  created_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP,
  PRIMARY KEY (video_id, actress_id),
  FOREIGN KEY (video_id) REFERENCES videos(id) ON DELETE CASCADE,
  FOREIGN KEY (actress_id) REFERENCES actresses(id) ON DELETE CASCADE
);

CREATE TABLE tag_library (
  id TEXT PRIMARY KEY,
  account_id TEXT NOT NULL,
  scope TEXT NOT NULL,
  canonical_name TEXT NOT NULL,
  aliases TEXT,
  related_tags TEXT,
  is_preset INTEGER NOT NULL DEFAULT 0,
  created_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP,
  updated_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP,
  UNIQUE (account_id, scope, canonical_name),
  FOREIGN KEY (account_id) REFERENCES accounts(id) ON DELETE CASCADE
);

CREATE TABLE video_tags (
  video_id TEXT NOT NULL,
  tag_id TEXT NOT NULL,
  created_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP,
  PRIMARY KEY (video_id, tag_id),
  FOREIGN KEY (video_id) REFERENCES videos(id) ON DELETE CASCADE,
  FOREIGN KEY (tag_id) REFERENCES tag_library(id) ON DELETE CASCADE
);

CREATE TABLE actress_tags (
  actress_id TEXT NOT NULL,
  tag_id TEXT NOT NULL,
  created_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP,
  PRIMARY KEY (actress_id, tag_id),
  FOREIGN KEY (actress_id) REFERENCES actresses(id) ON DELETE CASCADE,
  FOREIGN KEY (tag_id) REFERENCES tag_library(id) ON DELETE CASCADE
);

CREATE TABLE metadata_candidates (
  id TEXT PRIMARY KEY,
  account_id TEXT NOT NULL,
  target_type TEXT NOT NULL,
  query TEXT NOT NULL,
  source TEXT NOT NULL,
  payload_json TEXT NOT NULL,
  created_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP,
  FOREIGN KEY (account_id) REFERENCES accounts(id) ON DELETE CASCADE
);

CREATE INDEX idx_videos_account_code ON videos(account_id, code);
CREATE INDEX idx_videos_account_work_type ON videos(account_id, work_type);
CREATE INDEX idx_actresses_account_name ON actresses(account_id, name);
CREATE INDEX idx_tag_library_account_scope ON tag_library(account_id, scope);

