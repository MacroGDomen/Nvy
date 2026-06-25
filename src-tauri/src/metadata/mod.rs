mod browser_cookies;
mod gfriends;
mod javbus;

use rusqlite::{params, Connection, Error as SqlError};
use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};
use uuid::Uuid;

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct MetadataCandidate {
    pub id: String,
    pub account_id: String,
    pub target_type: String,
    pub query: String,
    pub source: String,
    pub payload_json: String,
    pub created_at: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
struct VideoCandidatePayload {
    video_id: String,
    code: Option<String>,
    title: Option<String>,
    cover_source_path: Option<String>,
    source_url: Option<String>,
    summary: Option<String>,
    actor_names: Option<String>,
    release_date: Option<String>,
    duration_minutes: Option<i64>,
    work_type: Option<String>,
    metadata_source: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
struct ActressCandidatePayload {
    actress_id: String,
    name: Option<String>,
    simplified_chinese_name: Option<String>,
    traditional_chinese_name: Option<String>,
    japanese_name: Option<String>,
    romanized_name: Option<String>,
    avatar_source_path: Option<String>,
    measurements: Option<String>,
    cup_size: Option<String>,
    birthday: Option<String>,
    height_cm: Option<i64>,
    debut_date: Option<String>,
    wikipedia_zh_url: Option<String>,
    metadata_source: String,
}

pub fn match_video_metadata(
    db_path: &PathBuf,
    account_id: &str,
    video_id: &str,
    query: &str,
) -> Result<Vec<MetadataCandidate>, String> {
    validate_account_id(account_id)?;
    validate_record_id(video_id, "Video id")?;
    let query = normalize_query(query)?;
    crate::db::initialize_database(db_path).map_err(|error| error.to_string())?;

    let connection = open_connection(db_path)?;
    ensure_video_exists(&connection, account_id, video_id)?;
    delete_existing_candidates(&connection, account_id, "video", video_id)?;

    if is_forced_empty_query(&query) {
        return Ok(Vec::new());
    }

    let allow_browser_cookies = read_metadata_cookie_access(&connection, account_id)?;
    let metadata_cookie_header =
        browser_cookies::load_metadata_cookie_header(allow_browser_cookies).unwrap_or(None);
    let (source, payload) =
        match javbus::fetch_video_metadata(&query, metadata_cookie_header.as_deref()) {
            Ok(Some(metadata)) => (
                "javbus",
                VideoCandidatePayload {
                    video_id: video_id.to_string(),
                    code: Some(metadata.code),
                    title: metadata.title,
                    cover_source_path: metadata.cover_url,
                    source_url: Some(metadata.source_url),
                    summary: metadata.summary,
                    actor_names: metadata.actor_names,
                    release_date: metadata.release_date,
                    duration_minutes: metadata.duration_minutes,
                    work_type: Some("single".to_string()),
                    metadata_source: "javbus".to_string(),
                },
            ),
            _ => (
                "local-placeholder-video-provider",
                VideoCandidatePayload {
                    video_id: video_id.to_string(),
                    code: Some(query.clone()),
                    title: Some(query.clone()),
                    cover_source_path: None,
                    source_url: None,
                    summary: None,
                    actor_names: None,
                    release_date: None,
                    duration_minutes: None,
                    work_type: None,
                    metadata_source: "local-placeholder-video-provider".to_string(),
                },
            ),
        };
    let candidate = insert_candidate(
        &connection,
        account_id,
        "video",
        &candidate_query(video_id, &query),
        source,
        &serde_json::to_string(&payload).map_err(|error| error.to_string())?,
    )?;

    Ok(vec![candidate])
}

pub fn match_actress_metadata(
    db_path: &PathBuf,
    app_data_dir: &Path,
    account_id: &str,
    actress_id: &str,
    query: &str,
) -> Result<Vec<MetadataCandidate>, String> {
    validate_account_id(account_id)?;
    validate_record_id(actress_id, "Actress id")?;
    let query = normalize_query(query)?;
    crate::db::initialize_database(db_path).map_err(|error| error.to_string())?;

    let connection = open_connection(db_path)?;
    ensure_actress_exists(&connection, account_id, actress_id)?;
    delete_existing_candidates(&connection, account_id, "actress", actress_id)?;

    if is_forced_empty_query(&query) {
        return Ok(Vec::new());
    }

    let (source, payload) = match gfriends::fetch_actress_metadata(app_data_dir, &query) {
        Ok(Some(metadata)) => (
            "gfriends",
            ActressCandidatePayload {
                actress_id: actress_id.to_string(),
                name: Some(query.clone()),
                simplified_chinese_name: Some(query.clone()),
                traditional_chinese_name: None,
                japanese_name: (metadata.matched_name != query).then_some(metadata.matched_name),
                romanized_name: None,
                avatar_source_path: Some(metadata.avatar_url),
                measurements: None,
                cup_size: None,
                birthday: None,
                height_cm: None,
                debut_date: None,
                wikipedia_zh_url: None,
                metadata_source: "gfriends".to_string(),
            },
        ),
        _ => (
            "local-placeholder-actress-provider",
            ActressCandidatePayload {
                actress_id: actress_id.to_string(),
                name: Some(query.clone()),
                simplified_chinese_name: Some(query.clone()),
                traditional_chinese_name: None,
                japanese_name: None,
                romanized_name: None,
                avatar_source_path: None,
                measurements: None,
                cup_size: None,
                birthday: None,
                height_cm: None,
                debut_date: None,
                wikipedia_zh_url: None,
                metadata_source: "local-placeholder-actress-provider".to_string(),
            },
        ),
    };
    let candidate = insert_candidate(
        &connection,
        account_id,
        "actress",
        &candidate_query(actress_id, &query),
        source,
        &serde_json::to_string(&payload).map_err(|error| error.to_string())?,
    )?;

    Ok(vec![candidate])
}

pub fn apply_metadata_candidate(
    db_path: &PathBuf,
    app_data_dir: &Path,
    account_id: &str,
    candidate_id: &str,
) -> Result<MetadataCandidate, String> {
    validate_account_id(account_id)?;
    validate_record_id(candidate_id, "Candidate id")?;
    crate::db::initialize_database(db_path).map_err(|error| error.to_string())?;

    let connection = open_connection(db_path)?;
    let candidate = get_candidate(&connection, account_id, candidate_id)?;
    match candidate.target_type.as_str() {
        "video" => apply_video_candidate(&connection, app_data_dir, account_id, &candidate)?,
        "actress" => apply_actress_candidate(&connection, app_data_dir, account_id, &candidate)?,
        _ => return Err("Metadata candidate target type is invalid.".to_string()),
    }

    Ok(candidate)
}

fn apply_video_candidate(
    connection: &Connection,
    app_data_dir: &Path,
    account_id: &str,
    candidate: &MetadataCandidate,
) -> Result<(), String> {
    let payload: VideoCandidatePayload =
        serde_json::from_str(&candidate.payload_json).map_err(|error| error.to_string())?;
    ensure_video_exists(connection, account_id, &payload.video_id)?;
    let browser_cookie_header = browser_cookies::load_metadata_cookie_header(
        read_metadata_cookie_access(connection, account_id)?,
    )
    .unwrap_or(None);
    let image_cookie_header = javbus::request_cookie_header(browser_cookie_header.as_deref());
    let cover_path = if let Some(source_path) = payload.cover_source_path.as_deref() {
        cache_candidate_image(
            app_data_dir,
            source_path,
            crate::assets::ImageCacheKind::Cover,
            &payload.video_id,
            Some(&image_cookie_header),
            payload.source_url.as_deref(),
        )
        .ok()
    } else {
        None
    };

    connection
        .execute(
            "UPDATE videos
             SET code = COALESCE(?1, code),
                 title = COALESCE(?2, title),
                 original_title = COALESCE(?2, original_title),
                 cover_path = COALESCE(?3, cover_path),
                 source_url = COALESCE(?4, source_url),
                 summary = COALESCE(?5, summary),
                 original_summary = COALESCE(?5, original_summary),
                 actress_names = COALESCE(?6, actress_names),
                 release_date = COALESCE(?7, release_date),
                 duration_minutes = COALESCE(?8, duration_minutes),
                 work_type = COALESCE(?9, work_type),
                 metadata_source = ?10,
                 updated_at = CURRENT_TIMESTAMP
             WHERE id = ?11 AND account_id = ?12",
            params![
                payload.code,
                payload.title,
                cover_path,
                payload.source_url,
                payload.summary,
                payload.actor_names,
                payload.release_date,
                payload.duration_minutes,
                payload.work_type,
                payload.metadata_source,
                payload.video_id,
                account_id
            ],
        )
        .map_err(|error| error.to_string())?;

    crate::library::sync_matching_actresses_for_video(
        connection,
        account_id,
        &payload.video_id,
        payload.actor_names.as_deref(),
    )?;

    Ok(())
}

fn apply_actress_candidate(
    connection: &Connection,
    app_data_dir: &Path,
    account_id: &str,
    candidate: &MetadataCandidate,
) -> Result<(), String> {
    let payload: ActressCandidatePayload =
        serde_json::from_str(&candidate.payload_json).map_err(|error| error.to_string())?;
    ensure_actress_exists(connection, account_id, &payload.actress_id)?;
    let avatar_path = if let Some(source_path) = payload.avatar_source_path.as_deref() {
        cache_candidate_image(
            app_data_dir,
            source_path,
            crate::assets::ImageCacheKind::Actress,
            &payload.actress_id,
            None,
            payload.wikipedia_zh_url.as_deref(),
        )
        .ok()
    } else {
        None
    };

    connection
        .execute(
            "UPDATE actresses
             SET name = COALESCE(?1, name),
                 simplified_chinese_name = COALESCE(?2, simplified_chinese_name),
                 traditional_chinese_name = COALESCE(?3, traditional_chinese_name),
                 japanese_name = COALESCE(?4, japanese_name),
                 romanized_name = COALESCE(?5, romanized_name),
                 avatar_path = COALESCE(?6, avatar_path),
                 measurements = COALESCE(?7, measurements),
                 cup_size = COALESCE(?8, cup_size),
                 birthday = COALESCE(?9, birthday),
                 height_cm = COALESCE(?10, height_cm),
                 debut_date = COALESCE(?11, debut_date),
                 wikipedia_zh_url = COALESCE(?12, wikipedia_zh_url),
                 metadata_source = ?13,
                 updated_at = CURRENT_TIMESTAMP
             WHERE id = ?14 AND account_id = ?15",
            params![
                payload.name,
                payload.simplified_chinese_name,
                payload.traditional_chinese_name,
                payload.japanese_name,
                payload.romanized_name,
                avatar_path,
                payload.measurements,
                payload.cup_size,
                payload.birthday,
                payload.height_cm,
                payload.debut_date,
                payload.wikipedia_zh_url,
                payload.metadata_source,
                payload.actress_id,
                account_id
            ],
        )
        .map_err(|error| error.to_string())?;

    crate::library::sync_matching_videos_for_actress(connection, account_id, &payload.actress_id)?;

    Ok(())
}

fn cache_candidate_image(
    app_data_dir: &Path,
    source: &str,
    kind: crate::assets::ImageCacheKind,
    owner_id: &str,
    cookie_header: Option<&str>,
    referer: Option<&str>,
) -> Result<String, String> {
    if source.starts_with("http://") || source.starts_with("https://") {
        crate::assets::cache_remote_image_with_headers(
            app_data_dir,
            source,
            kind,
            owner_id,
            cookie_header,
            referer,
        )
        .map(|image| image.relative_path)
    } else {
        crate::assets::cache_local_image(app_data_dir, Path::new(source), kind, owner_id)
            .map(|image| image.relative_path)
    }
}

fn insert_candidate(
    connection: &Connection,
    account_id: &str,
    target_type: &str,
    query: &str,
    source: &str,
    payload_json: &str,
) -> Result<MetadataCandidate, String> {
    let candidate_id = Uuid::new_v4().to_string();
    connection
        .execute(
            "INSERT INTO metadata_candidates (
                id,
                account_id,
                target_type,
                query,
                source,
                payload_json
            ) VALUES (?1, ?2, ?3, ?4, ?5, ?6)",
            params![
                candidate_id,
                account_id,
                target_type,
                query,
                source,
                payload_json
            ],
        )
        .map_err(|error| error.to_string())?;

    get_candidate(connection, account_id, &candidate_id)
}

fn get_candidate(
    connection: &Connection,
    account_id: &str,
    candidate_id: &str,
) -> Result<MetadataCandidate, String> {
    connection
        .query_row(
            "SELECT id,
                    account_id,
                    target_type,
                    query,
                    source,
                    payload_json,
                    created_at
             FROM metadata_candidates
             WHERE id = ?1 AND account_id = ?2",
            params![candidate_id, account_id],
            map_candidate_row,
        )
        .map_err(|error| match error {
            SqlError::QueryReturnedNoRows => "Metadata candidate was not found.".to_string(),
            other => other.to_string(),
        })
}

fn delete_existing_candidates(
    connection: &Connection,
    account_id: &str,
    target_type: &str,
    target_id: &str,
) -> Result<(), String> {
    let prefix = format!("{target_id}:");
    connection
        .execute(
            "DELETE FROM metadata_candidates
             WHERE account_id = ?1 AND target_type = ?2 AND query LIKE ?3",
            params![account_id, target_type, format!("{prefix}%")],
        )
        .map(|_| ())
        .map_err(|error| error.to_string())
}

fn ensure_video_exists(
    connection: &Connection,
    account_id: &str,
    video_id: &str,
) -> Result<(), String> {
    ensure_record_exists(connection, "videos", account_id, video_id, "Video")
}

fn ensure_actress_exists(
    connection: &Connection,
    account_id: &str,
    actress_id: &str,
) -> Result<(), String> {
    ensure_record_exists(connection, "actresses", account_id, actress_id, "Actress")
}

fn ensure_record_exists(
    connection: &Connection,
    table_name: &str,
    account_id: &str,
    record_id: &str,
    label: &str,
) -> Result<(), String> {
    let sql = format!("SELECT COUNT(*) FROM {table_name} WHERE id = ?1 AND account_id = ?2");
    let count: i64 = connection
        .query_row(&sql, params![record_id, account_id], |row| row.get(0))
        .map_err(|error| error.to_string())?;
    if count == 0 {
        return Err(format!("{label} was not found."));
    }

    Ok(())
}

fn read_metadata_cookie_access(connection: &Connection, account_id: &str) -> Result<bool, String> {
    connection
        .query_row(
            "SELECT metadata_allow_browser_cookies
             FROM account_settings
             WHERE account_id = ?1",
            params![account_id],
            |row| Ok(row.get::<_, i64>(0)? == 1),
        )
        .map_err(|error| match error {
            SqlError::QueryReturnedNoRows => "Account settings were not found.".to_string(),
            other => other.to_string(),
        })
}

fn map_candidate_row(row: &rusqlite::Row<'_>) -> rusqlite::Result<MetadataCandidate> {
    Ok(MetadataCandidate {
        id: row.get(0)?,
        account_id: row.get(1)?,
        target_type: row.get(2)?,
        query: row.get(3)?,
        source: row.get(4)?,
        payload_json: row.get(5)?,
        created_at: row.get(6)?,
    })
}

fn open_connection(db_path: &PathBuf) -> Result<Connection, String> {
    let connection = Connection::open(db_path).map_err(|error| error.to_string())?;
    connection
        .pragma_update(None, "foreign_keys", "ON")
        .map_err(|error| error.to_string())?;
    Ok(connection)
}

fn normalize_query(query: &str) -> Result<String, String> {
    let query = query.trim();
    if query.is_empty() {
        return Err("Metadata query is required.".to_string());
    }

    Ok(query.to_string())
}

fn candidate_query(target_id: &str, query: &str) -> String {
    format!("{target_id}:{query}")
}

fn is_forced_empty_query(query: &str) -> bool {
    query.eq_ignore_ascii_case("__nvy_no_results__")
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
            "nvy-metadata-{label}-{}-{nanos}.sqlite",
            std::process::id()
        ))
    }

    fn video_input(code: &str) -> VideoInput {
        VideoInput {
            code: code.to_string(),
            title: None,
            cover_path: None,
            release_date: None,
            duration_minutes: None,
            source_url: None,
            summary: None,
            actor_names: None,
            work_type: "single".to_string(),
            review: None,
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
            avatar_path: None,
            measurements: None,
            cup_size: None,
            birthday: None,
            height_cm: None,
            debut_date: None,
            wikipedia_zh_url: None,
            note: None,
        }
    }

    #[test]
    fn creates_and_applies_video_candidate() {
        let db_path = test_db_path("video");
        let app_data_dir =
            std::env::temp_dir().join(format!("nvy-metadata-assets-{}", Uuid::new_v4()));
        let session = register_account(&db_path, "MetaUser1", "abc123").unwrap();
        let video =
            library::create_video(&db_path, &session.account_id, video_input("OLD-001")).unwrap();

        let candidates =
            match_video_metadata(&db_path, &session.account_id, &video.id, "NEW-001").unwrap();
        assert_eq!(candidates.len(), 1);

        apply_metadata_candidate(
            &db_path,
            &app_data_dir,
            &session.account_id,
            &candidates[0].id,
        )
        .unwrap();
        let updated = library::list_videos(&db_path, &session.account_id).unwrap();
        assert_eq!(updated[0].code, "NEW-001");
        assert_eq!(updated[0].title.as_deref(), Some("NEW-001"));

        let _ = fs::remove_file(db_path);
        let _ = fs::remove_dir_all(app_data_dir);
    }

    #[test]
    fn empty_metadata_result_does_not_create_candidate() {
        let db_path = test_db_path("empty");
        let session = register_account(&db_path, "MetaUser2", "abc123").unwrap();
        let actress =
            library::create_actress(&db_path, &session.account_id, actress_input("Old")).unwrap();

        let candidates = match_actress_metadata(
            &db_path,
            std::env::temp_dir().as_path(),
            &session.account_id,
            &actress.id,
            "__nvy_no_results__",
        )
        .unwrap();
        assert!(candidates.is_empty());

        let _ = fs::remove_file(db_path);
    }

    #[test]
    fn applies_actress_candidate() {
        let db_path = test_db_path("actress");
        let session = register_account(&db_path, "MetaUser3", "abc123").unwrap();
        let actress =
            library::create_actress(&db_path, &session.account_id, actress_input("Old")).unwrap();

        let app_data_dir =
            std::env::temp_dir().join(format!("nvy-metadata-assets-{}", Uuid::new_v4()));
        let candidates = match_actress_metadata(
            &db_path,
            &app_data_dir,
            &session.account_id,
            &actress.id,
            "New Name",
        )
        .unwrap();
        apply_metadata_candidate(
            &db_path,
            &app_data_dir,
            &session.account_id,
            &candidates[0].id,
        )
        .unwrap();
        let updated = library::list_actresses(&db_path, &session.account_id).unwrap();
        assert_eq!(updated[0].name, "New Name");
        assert_eq!(
            updated[0].simplified_chinese_name.as_deref(),
            Some("New Name")
        );

        let _ = fs::remove_file(db_path);
        let _ = fs::remove_dir_all(app_data_dir);
    }
}
