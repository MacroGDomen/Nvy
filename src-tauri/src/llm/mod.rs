use rusqlite::{params, Connection, Error as SqlError};
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct LlmSettings {
    pub account_id: String,
    pub api_type: Option<String>,
    pub base_url: Option<String>,
    pub provider_name: Option<String>,
    pub model: Option<String>,
    pub temperature: Option<f64>,
    pub max_tokens: Option<i64>,
    pub recommendation_reference_limit: i64,
    pub translation_prompt: String,
    pub recommendation_prompt: String,
    pub enable_llm_translation: bool,
    pub recommendation_default_enabled: bool,
    pub metadata_allow_browser_cookies: bool,
    pub has_api_key: bool,
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct LlmSettingsInput {
    pub api_type: Option<String>,
    pub base_url: Option<String>,
    pub provider_name: Option<String>,
    pub model: Option<String>,
    pub temperature: Option<f64>,
    pub max_tokens: Option<i64>,
    pub recommendation_reference_limit: i64,
    pub translation_prompt: Option<String>,
    pub recommendation_prompt: Option<String>,
    pub enable_llm_translation: bool,
    pub recommendation_default_enabled: bool,
    pub metadata_allow_browser_cookies: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct SecretStatus {
    pub has_api_key: bool,
    pub storage: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct RecommendationPayload {
    pub user_text: String,
    pub prompt: String,
    pub reference_limit: i64,
    pub videos: Vec<RecommendationVideoCandidate>,
    pub actresses: Vec<RecommendationActressCandidate>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct RecommendationVideoCandidate {
    pub id: String,
    pub code: String,
    pub title: Option<String>,
    pub summary: Option<String>,
    pub actor_names: Option<String>,
    pub work_type: String,
    pub review: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct RecommendationActressCandidate {
    pub id: String,
    pub name: String,
    pub simplified_chinese_name: Option<String>,
    pub japanese_name: Option<String>,
    pub romanized_name: Option<String>,
    pub measurements: Option<String>,
    pub cup_size: Option<String>,
    pub birthday: Option<String>,
    pub note: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct TranslationState {
    pub video_id: String,
    pub translation_status: Option<String>,
}

pub fn get_llm_settings(db_path: &PathBuf, account_id: &str) -> Result<LlmSettings, String> {
    validate_account_id(account_id)?;
    crate::db::initialize_database(db_path).map_err(|error| error.to_string())?;
    let connection = open_connection(db_path)?;
    ensure_settings_row(&connection, account_id)?;
    read_llm_settings(&connection, account_id)
}

pub fn save_llm_settings(
    db_path: &PathBuf,
    account_id: &str,
    input: LlmSettingsInput,
) -> Result<LlmSettings, String> {
    validate_account_id(account_id)?;
    validate_settings_input(&input)?;
    crate::db::initialize_database(db_path).map_err(|error| error.to_string())?;
    let connection = open_connection(db_path)?;
    ensure_settings_row(&connection, account_id)?;

    connection
        .execute(
            "UPDATE account_settings
             SET llm_api_type = ?1,
                 llm_base_url = ?2,
                 llm_provider_name = ?3,
                 llm_model = ?4,
                 llm_temperature = ?5,
                 llm_max_tokens = ?6,
                 llm_recommendation_reference_limit = ?7,
                 llm_translation_prompt = ?8,
                 llm_recommendation_prompt = ?9,
                 enable_llm_translation = ?10,
                 llm_recommendation_default_enabled = ?11,
                 metadata_allow_browser_cookies = ?12,
                 updated_at = CURRENT_TIMESTAMP
             WHERE account_id = ?13",
            params![
                normalize_optional(input.api_type),
                normalize_optional(input.base_url),
                normalize_optional(input.provider_name),
                normalize_optional(input.model),
                input.temperature,
                input.max_tokens,
                input.recommendation_reference_limit,
                normalize_prompt(
                    input.translation_prompt,
                    crate::defaults::DEFAULT_TRANSLATION_PROMPT,
                ),
                normalize_prompt(
                    input.recommendation_prompt,
                    crate::defaults::DEFAULT_RECOMMENDATION_PROMPT,
                ),
                bool_to_i64(input.enable_llm_translation),
                bool_to_i64(input.recommendation_default_enabled),
                bool_to_i64(input.metadata_allow_browser_cookies),
                account_id
            ],
        )
        .map_err(|error| error.to_string())?;

    read_llm_settings(&connection, account_id)
}

pub fn save_llm_api_key(_account_id: &str, api_key: &str) -> Result<SecretStatus, String> {
    if api_key.trim().is_empty() {
        return Err("API Key is required.".to_string());
    }

    Err("Tauri Stronghold storage is not connected yet.".to_string())
}

pub fn clear_llm_api_key(_account_id: &str) -> SecretStatus {
    SecretStatus {
        has_api_key: false,
        storage: "stronghold-pending".to_string(),
    }
}

pub fn build_recommendation_payload(
    db_path: &PathBuf,
    account_id: &str,
    user_text: &str,
) -> Result<RecommendationPayload, String> {
    validate_account_id(account_id)?;
    let user_text = user_text.trim();
    if user_text.is_empty() {
        return Err("Recommendation text is required.".to_string());
    }

    crate::db::initialize_database(db_path).map_err(|error| error.to_string())?;
    let connection = open_connection(db_path)?;
    ensure_settings_row(&connection, account_id)?;
    let settings = read_llm_settings(&connection, account_id)?;
    let videos = read_recommendation_videos(&connection, account_id)?;
    let actresses = read_recommendation_actresses(&connection, account_id)?;
    let (videos, actresses) =
        apply_reference_limit(videos, actresses, settings.recommendation_reference_limit);

    Ok(RecommendationPayload {
        user_text: user_text.to_string(),
        prompt: settings.recommendation_prompt,
        reference_limit: settings.recommendation_reference_limit,
        videos,
        actresses,
    })
}

pub fn cancel_video_translation(
    db_path: &PathBuf,
    account_id: &str,
    video_id: &str,
) -> Result<TranslationState, String> {
    validate_account_id(account_id)?;
    validate_record_id(video_id, "Video id")?;
    crate::db::initialize_database(db_path).map_err(|error| error.to_string())?;
    let connection = open_connection(db_path)?;

    let changed = connection
        .execute(
            "UPDATE videos
             SET title = COALESCE(original_title, title),
                 summary = COALESCE(original_summary, summary),
                 translation_status = 'cancelled',
                 translated_at = NULL,
                 updated_at = CURRENT_TIMESTAMP
             WHERE id = ?1 AND account_id = ?2",
            params![video_id, account_id],
        )
        .map_err(|error| error.to_string())?;
    if changed == 0 {
        return Err("Video record was not found.".to_string());
    }

    Ok(TranslationState {
        video_id: video_id.to_string(),
        translation_status: Some("cancelled".to_string()),
    })
}

pub fn request_video_translation(
    db_path: &PathBuf,
    account_id: &str,
    video_id: &str,
) -> Result<TranslationState, String> {
    validate_account_id(account_id)?;
    validate_record_id(video_id, "Video id")?;
    crate::db::initialize_database(db_path).map_err(|error| error.to_string())?;
    let connection = open_connection(db_path)?;
    ensure_settings_row(&connection, account_id)?;
    let settings = read_llm_settings(&connection, account_id)?;
    if !settings.enable_llm_translation {
        return Err("LLM translation is disabled.".to_string());
    }
    if !settings.has_api_key {
        return Err("API Key is missing.".to_string());
    }

    Err("LLM HTTP client is not connected yet.".to_string())
}

fn read_llm_settings(connection: &Connection, account_id: &str) -> Result<LlmSettings, String> {
    connection
        .query_row(
            "SELECT account_id,
                    llm_api_type,
                    llm_base_url,
                    llm_provider_name,
                    llm_model,
                    llm_temperature,
                    llm_max_tokens,
                    llm_recommendation_reference_limit,
                    llm_translation_prompt,
                    llm_recommendation_prompt,
                    enable_llm_translation,
                    llm_recommendation_default_enabled,
                    metadata_allow_browser_cookies
             FROM account_settings
             WHERE account_id = ?1",
            params![account_id],
            |row| {
                Ok(LlmSettings {
                    account_id: row.get(0)?,
                    api_type: row.get(1)?,
                    base_url: row.get(2)?,
                    provider_name: row.get(3)?,
                    model: row.get(4)?,
                    temperature: row.get(5)?,
                    max_tokens: row.get(6)?,
                    recommendation_reference_limit: row.get(7)?,
                    translation_prompt: row
                        .get::<_, Option<String>>(8)?
                        .unwrap_or_else(|| crate::defaults::DEFAULT_TRANSLATION_PROMPT.to_string()),
                    recommendation_prompt: row.get::<_, Option<String>>(9)?.unwrap_or_else(|| {
                        crate::defaults::DEFAULT_RECOMMENDATION_PROMPT.to_string()
                    }),
                    enable_llm_translation: row.get::<_, i64>(10)? == 1,
                    recommendation_default_enabled: row.get::<_, i64>(11)? == 1,
                    metadata_allow_browser_cookies: row.get::<_, i64>(12)? == 1,
                    has_api_key: false,
                })
            },
        )
        .map_err(|error| match error {
            SqlError::QueryReturnedNoRows => "LLM settings were not found.".to_string(),
            other => other.to_string(),
        })
}

fn ensure_settings_row(connection: &Connection, account_id: &str) -> Result<(), String> {
    connection
        .execute(
            "INSERT OR IGNORE INTO account_settings (
                account_id,
                llm_recommendation_reference_limit,
                llm_translation_prompt,
                llm_recommendation_prompt,
                enable_llm_translation,
                llm_recommendation_default_enabled,
                metadata_allow_browser_cookies
            ) VALUES (?1, ?2, ?3, ?4, 0, 1, 0)",
            params![
                account_id,
                crate::defaults::DEFAULT_RECOMMENDATION_REFERENCE_LIMIT,
                crate::defaults::DEFAULT_TRANSLATION_PROMPT,
                crate::defaults::DEFAULT_RECOMMENDATION_PROMPT
            ],
        )
        .map(|_| ())
        .map_err(|error| error.to_string())
}

fn read_recommendation_videos(
    connection: &Connection,
    account_id: &str,
) -> Result<Vec<RecommendationVideoCandidate>, String> {
    let mut statement = connection
        .prepare(
            "SELECT id,
                    code,
                    title,
                    summary,
                    actress_names,
                    work_type,
                    review
             FROM videos
             WHERE account_id = ?1
             ORDER BY updated_at DESC, created_at DESC",
        )
        .map_err(|error| error.to_string())?;
    let rows = statement
        .query_map(params![account_id], |row| {
            Ok(RecommendationVideoCandidate {
                id: row.get(0)?,
                code: row.get(1)?,
                title: row.get(2)?,
                summary: row.get(3)?,
                actor_names: row.get(4)?,
                work_type: row.get(5)?,
                review: row.get(6)?,
            })
        })
        .map_err(|error| error.to_string())?;

    collect_rows(rows)
}

fn read_recommendation_actresses(
    connection: &Connection,
    account_id: &str,
) -> Result<Vec<RecommendationActressCandidate>, String> {
    let mut statement = connection
        .prepare(
            "SELECT id,
                    name,
                    simplified_chinese_name,
                    japanese_name,
                    romanized_name,
                    measurements,
                    cup_size,
                    birthday,
                    note
             FROM actresses
             WHERE account_id = ?1
             ORDER BY updated_at DESC, created_at DESC",
        )
        .map_err(|error| error.to_string())?;
    let rows = statement
        .query_map(params![account_id], |row| {
            Ok(RecommendationActressCandidate {
                id: row.get(0)?,
                name: row.get(1)?,
                simplified_chinese_name: row.get(2)?,
                japanese_name: row.get(3)?,
                romanized_name: row.get(4)?,
                measurements: row.get(5)?,
                cup_size: row.get(6)?,
                birthday: row.get(7)?,
                note: row.get(8)?,
            })
        })
        .map_err(|error| error.to_string())?;

    collect_rows(rows)
}

fn apply_reference_limit(
    videos: Vec<RecommendationVideoCandidate>,
    actresses: Vec<RecommendationActressCandidate>,
    limit: i64,
) -> (
    Vec<RecommendationVideoCandidate>,
    Vec<RecommendationActressCandidate>,
) {
    if limit == 0 {
        return (videos, actresses);
    }

    let mut remaining = limit.max(0) as usize;
    let mut limited_videos = Vec::new();
    let mut limited_actresses = Vec::new();

    for video in videos {
        if remaining == 0 {
            break;
        }
        limited_videos.push(video);
        remaining -= 1;
    }

    for actress in actresses {
        if remaining == 0 {
            break;
        }
        limited_actresses.push(actress);
        remaining -= 1;
    }

    (limited_videos, limited_actresses)
}

fn validate_settings_input(input: &LlmSettingsInput) -> Result<(), String> {
    validate_reference_limit(input.recommendation_reference_limit)?;
    if let Some(api_type) = input.api_type.as_deref().map(str::trim) {
        if !api_type.is_empty() && !matches!(api_type, "responses" | "chat_completions" | "custom")
        {
            return Err("LLM API type is invalid.".to_string());
        }
    }
    if let Some(temperature) = input.temperature {
        if !(0.0..=2.0).contains(&temperature) {
            return Err("LLM temperature must be between 0 and 2.".to_string());
        }
    }
    if let Some(max_tokens) = input.max_tokens {
        if max_tokens <= 0 {
            return Err("LLM max tokens must be positive.".to_string());
        }
    }

    Ok(())
}

fn validate_reference_limit(limit: i64) -> Result<(), String> {
    if !(0..=999).contains(&limit) {
        return Err("Recommendation reference limit must be between 0 and 999.".to_string());
    }

    Ok(())
}

fn normalize_optional(value: Option<String>) -> Option<String> {
    value
        .map(|item| item.trim().to_string())
        .filter(|item| !item.is_empty())
}

fn normalize_prompt(value: Option<String>, default_value: &str) -> String {
    value
        .map(|item| item.trim().to_string())
        .filter(|item| !item.is_empty())
        .unwrap_or_else(|| default_value.to_string())
}

fn bool_to_i64(value: bool) -> i64 {
    if value {
        1
    } else {
        0
    }
}

fn validate_account_id(account_id: &str) -> Result<(), String> {
    if account_id.trim().is_empty() {
        return Err("Account id is required.".to_string());
    }

    Ok(())
}

fn validate_record_id(record_id: &str, label: &str) -> Result<(), String> {
    if record_id.trim().is_empty() {
        return Err(format!("{label} is required."));
    }

    Ok(())
}

fn open_connection(db_path: &PathBuf) -> Result<Connection, String> {
    let connection = Connection::open(db_path).map_err(|error| error.to_string())?;
    connection
        .pragma_update(None, "foreign_keys", "ON")
        .map_err(|error| error.to_string())?;
    Ok(connection)
}

fn collect_rows<T>(
    rows: rusqlite::MappedRows<'_, impl FnMut(&rusqlite::Row<'_>) -> rusqlite::Result<T>>,
) -> Result<Vec<T>, String> {
    rows.collect::<Result<Vec<_>, _>>()
        .map_err(|error| error.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        auth::register_account,
        library::{self, ActressInput, VideoInput},
    };
    use std::{fs, time::SystemTime};

    fn test_db_path(label: &str) -> PathBuf {
        let nanos = SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .expect("system time is valid")
            .as_nanos();
        std::env::temp_dir().join(format!(
            "nvy-llm-{label}-{}-{nanos}.sqlite",
            std::process::id()
        ))
    }

    fn video_input(code: &str) -> VideoInput {
        VideoInput {
            code: code.to_string(),
            title: Some(format!("{code} title")),
            cover_path: Some("C:\\secret\\cover.jpg".to_string()),
            release_date: None,
            duration_minutes: None,
            source_url: Some("C:\\secret\\source.txt".to_string()),
            summary: Some("summary".to_string()),
            actor_names: Some("Alice".to_string()),
            work_type: "multiple".to_string(),
            review: Some("review".to_string()),
        }
    }

    fn actress_input(name: &str) -> ActressInput {
        ActressInput {
            name: name.to_string(),
            simplified_chinese_name: None,
            former_chinese_names: None,
            traditional_chinese_name: None,
            japanese_name: None,
            romanized_name: None,
            default_display_name_type: None,
            avatar_path: Some("C:\\secret\\avatar.jpg".to_string()),
            measurements: None,
            cup_size: Some("F".to_string()),
            birthday: None,
            height_cm: None,
            debut_date: None,
            wikipedia_zh_url: None,
            note: Some("note".to_string()),
        }
    }

    #[test]
    fn saves_settings_and_validates_reference_limit() {
        let db_path = test_db_path("settings");
        let session = register_account(&db_path, "LlmUser1", "abc123").unwrap();

        assert!(save_llm_settings(
            &db_path,
            &session.account_id,
            LlmSettingsInput {
                api_type: Some("responses".to_string()),
                base_url: Some("https://api.example.test".to_string()),
                provider_name: Some("Example".to_string()),
                model: Some("model-a".to_string()),
                temperature: Some(0.7),
                max_tokens: Some(1000),
                recommendation_reference_limit: 999,
                translation_prompt: Some("translate".to_string()),
                recommendation_prompt: Some("recommend".to_string()),
                enable_llm_translation: true,
                recommendation_default_enabled: true,
                metadata_allow_browser_cookies: true,
            },
        )
        .is_ok());

        assert!(save_llm_settings(
            &db_path,
            &session.account_id,
            LlmSettingsInput {
                api_type: None,
                base_url: None,
                provider_name: None,
                model: None,
                temperature: None,
                max_tokens: None,
                recommendation_reference_limit: 1000,
                translation_prompt: None,
                recommendation_prompt: None,
                enable_llm_translation: false,
                recommendation_default_enabled: true,
                metadata_allow_browser_cookies: false,
            },
        )
        .is_err());

        let _ = fs::remove_file(db_path);
    }

    #[test]
    fn recommendation_payload_excludes_sensitive_paths_and_respects_limit() {
        let db_path = test_db_path("payload");
        let session = register_account(&db_path, "LlmUser2", "abc123").unwrap();
        library::create_video(&db_path, &session.account_id, video_input("AAA-001")).unwrap();
        library::create_video(&db_path, &session.account_id, video_input("AAA-002")).unwrap();
        library::create_actress(&db_path, &session.account_id, actress_input("Alice")).unwrap();

        save_llm_settings(
            &db_path,
            &session.account_id,
            LlmSettingsInput {
                api_type: Some("responses".to_string()),
                base_url: None,
                provider_name: None,
                model: None,
                temperature: None,
                max_tokens: None,
                recommendation_reference_limit: 2,
                translation_prompt: None,
                recommendation_prompt: Some("prompt".to_string()),
                enable_llm_translation: false,
                recommendation_default_enabled: true,
                metadata_allow_browser_cookies: false,
            },
        )
        .unwrap();

        let payload =
            build_recommendation_payload(&db_path, &session.account_id, "find something").unwrap();
        let serialized = serde_json::to_string(&payload).unwrap();

        assert_eq!(payload.videos.len() + payload.actresses.len(), 2);
        assert!(!serialized.contains("C:\\"));
        assert!(!serialized.to_ascii_lowercase().contains("api_key"));

        let _ = fs::remove_file(db_path);
    }
}
