use rusqlite::{params, Connection};
use serde::Serialize;
use std::path::PathBuf;
use uuid::Uuid;

pub const DEFAULT_RECOMMENDATION_REFERENCE_LIMIT: i64 = 30;

pub const DEFAULT_RECOMMENDATION_PROMPT: &str = r#"你是 Nvy 的本地资料库推荐助手。你只能根据用户提供的本地资料库资料进行推荐，不要编造不存在的影片或女优。

任务：
1. 根据用户输入的偏好，从候选影片和候选女优中找出最匹配的结果。
2. 优先使用影片标题、简介、标签、作品类型、出演女优、用户影评和女优评价进行判断。
3. 如果信息不足，请说明“不确定”，并给出需要补充的筛选条件。
4. 输出内容使用简体中文。
5. 不要输出露骨描写，不要处理或推荐任何未成年人相关内容。

输出格式：
- 推荐影片：
- 推荐女优：
- 推荐理由：
- 仍需确认的信息："#;

pub const DEFAULT_TRANSLATION_PROMPT: &str = r#"你是 Nvy 的影片资料翻译助手。请把输入的影片标题或简介翻译为自然、简洁的简体中文。

要求：
1. 保留人名、番号、片商名、系列名等专有名词，不要擅自改写。
2. 不要添加原文没有的信息。
3. 如果原文含义不明确，请直译并保持克制。
4. 不要输出露骨扩写，不要加入评价。
5. 只输出翻译结果，不要解释翻译过程。"#;

const PRESET_VIDEO_TAGS: &[&str] = &[
    "#按摩", "#NTR", "#巨乳", "#贫乳", "#精油", "#人妻", "#乱伦", "#熟女",
];

const PRESET_ACTRESS_TAGS: &[&str] = &["#苗条", "#强气"];

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct AccountDefaults {
    pub account_id: String,
    pub recommendation_reference_limit: i64,
    pub video_tag_count: i64,
    pub actress_tag_count: i64,
}

pub fn initialize_account_defaults(
    db_path: &PathBuf,
    account_id: &str,
) -> Result<AccountDefaults, String> {
    if account_id.trim().is_empty() {
        return Err("Account id is required.".to_string());
    }

    crate::db::initialize_database(db_path).map_err(|error| error.to_string())?;
    let mut connection = Connection::open(db_path).map_err(|error| error.to_string())?;
    connection
        .pragma_update(None, "foreign_keys", "ON")
        .map_err(|error| error.to_string())?;

    let transaction = connection
        .transaction()
        .map_err(|error| error.to_string())?;
    insert_default_settings(&transaction, account_id)?;
    insert_preset_tags(&transaction, account_id, "video", PRESET_VIDEO_TAGS)?;
    insert_preset_tags(&transaction, account_id, "actress", PRESET_ACTRESS_TAGS)?;
    transaction.commit().map_err(|error| error.to_string())?;

    read_account_defaults(db_path, account_id)
}

pub fn read_account_defaults(
    db_path: &PathBuf,
    account_id: &str,
) -> Result<AccountDefaults, String> {
    crate::db::initialize_database(db_path).map_err(|error| error.to_string())?;
    let connection = Connection::open(db_path).map_err(|error| error.to_string())?;

    let recommendation_reference_limit = connection
        .query_row(
            "SELECT llm_recommendation_reference_limit
             FROM account_settings
             WHERE account_id = ?1",
            params![account_id],
            |row| row.get(0),
        )
        .map_err(|error| error.to_string())?;
    let video_tag_count = count_tags(&connection, account_id, "video")?;
    let actress_tag_count = count_tags(&connection, account_id, "actress")?;

    Ok(AccountDefaults {
        account_id: account_id.to_string(),
        recommendation_reference_limit,
        video_tag_count,
        actress_tag_count,
    })
}

fn insert_default_settings(connection: &Connection, account_id: &str) -> Result<(), String> {
    connection
        .execute(
            "INSERT OR IGNORE INTO account_settings (
                account_id,
                llm_recommendation_reference_limit,
                llm_translation_prompt,
                llm_recommendation_prompt,
                enable_llm_translation,
                llm_recommendation_default_enabled
            ) VALUES (?1, ?2, ?3, ?4, 0, 1)",
            params![
                account_id,
                DEFAULT_RECOMMENDATION_REFERENCE_LIMIT,
                DEFAULT_TRANSLATION_PROMPT,
                DEFAULT_RECOMMENDATION_PROMPT
            ],
        )
        .map(|_| ())
        .map_err(|error| error.to_string())
}

fn insert_preset_tags(
    connection: &Connection,
    account_id: &str,
    scope: &str,
    tags: &[&str],
) -> Result<(), String> {
    for tag in tags {
        connection
            .execute(
                "INSERT OR IGNORE INTO tag_library (
                    id,
                    account_id,
                    scope,
                    canonical_name,
                    is_preset
                ) VALUES (?1, ?2, ?3, ?4, 1)",
                params![Uuid::new_v4().to_string(), account_id, scope, tag],
            )
            .map_err(|error| error.to_string())?;
    }

    Ok(())
}

fn count_tags(connection: &Connection, account_id: &str, scope: &str) -> Result<i64, String> {
    connection
        .query_row(
            "SELECT COUNT(*)
             FROM tag_library
             WHERE account_id = ?1 AND scope = ?2",
            params![account_id, scope],
            |row| row.get(0),
        )
        .map_err(|error| error.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::auth::register_account;
    use rusqlite::params;
    use std::{fs, time::SystemTime};

    fn test_db_path(label: &str) -> PathBuf {
        let nanos = SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .expect("system time is valid")
            .as_nanos();
        std::env::temp_dir().join(format!("nvy-{label}-{}-{nanos}.sqlite", std::process::id()))
    }

    #[test]
    fn initializes_default_settings_and_prompts() {
        let db_path = test_db_path("defaults-settings");
        let session =
            register_account(&db_path, "PromptUser", "abc123").expect("account registers");

        let connection = Connection::open(&db_path).expect("database opens");
        let row: (i64, String, String, i64, i64) = connection
            .query_row(
                "SELECT
                    llm_recommendation_reference_limit,
                    llm_translation_prompt,
                    llm_recommendation_prompt,
                    enable_llm_translation,
                    llm_recommendation_default_enabled
                 FROM account_settings
                 WHERE account_id = ?1",
                params![session.account_id],
                |row| {
                    Ok((
                        row.get(0)?,
                        row.get(1)?,
                        row.get(2)?,
                        row.get(3)?,
                        row.get(4)?,
                    ))
                },
            )
            .expect("settings exist");

        assert_eq!(row.0, DEFAULT_RECOMMENDATION_REFERENCE_LIMIT);
        assert_eq!(row.1, DEFAULT_TRANSLATION_PROMPT);
        assert_eq!(row.2, DEFAULT_RECOMMENDATION_PROMPT);
        assert_eq!(row.3, 0);
        assert_eq!(row.4, 1);

        let _ = fs::remove_file(db_path);
    }

    #[test]
    fn initializes_preset_tags_idempotently() {
        let db_path = test_db_path("defaults-tags");
        let session = register_account(&db_path, "TagUser", "abc123").expect("account registers");

        let first =
            initialize_account_defaults(&db_path, &session.account_id).expect("first init works");
        let second =
            initialize_account_defaults(&db_path, &session.account_id).expect("second init works");

        assert_eq!(first.video_tag_count, PRESET_VIDEO_TAGS.len() as i64);
        assert_eq!(first.actress_tag_count, PRESET_ACTRESS_TAGS.len() as i64);
        assert_eq!(second.video_tag_count, PRESET_VIDEO_TAGS.len() as i64);
        assert_eq!(second.actress_tag_count, PRESET_ACTRESS_TAGS.len() as i64);

        let connection = Connection::open(&db_path).expect("database opens");
        let massage_count: i64 = connection
            .query_row(
                "SELECT COUNT(*)
                 FROM tag_library
                 WHERE account_id = ?1 AND scope = 'video' AND canonical_name = '#按摩'",
                params![session.account_id],
                |row| row.get(0),
            )
            .expect("preset tag can be counted");

        assert_eq!(massage_count, 1);

        let _ = fs::remove_file(db_path);
    }
}
