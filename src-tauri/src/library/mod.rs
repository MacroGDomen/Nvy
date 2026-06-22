use rusqlite::{params, Connection, Error as SqlError};
use serde::Serialize;
use std::path::PathBuf;
use uuid::Uuid;

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct VideoRecord {
    pub id: String,
    pub account_id: String,
    pub code: String,
    pub title: Option<String>,
    pub cover_path: Option<String>,
    pub release_date: Option<String>,
    pub duration_minutes: Option<i64>,
    pub source_url: Option<String>,
    pub summary: Option<String>,
    pub actor_names: Option<String>,
    pub work_type: String,
    pub review: Option<String>,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct VideoInput {
    pub code: String,
    pub title: Option<String>,
    pub cover_path: Option<String>,
    pub release_date: Option<String>,
    pub duration_minutes: Option<i64>,
    pub source_url: Option<String>,
    pub summary: Option<String>,
    pub actor_names: Option<String>,
    pub work_type: String,
    pub review: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct AssociationSuggestion {
    pub video: VideoRecord,
    pub actress: ActressRecord,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct ActressRecord {
    pub id: String,
    pub account_id: String,
    pub name: String,
    pub simplified_chinese_name: Option<String>,
    pub former_chinese_names: Option<String>,
    pub traditional_chinese_name: Option<String>,
    pub japanese_name: Option<String>,
    pub romanized_name: Option<String>,
    pub default_display_name_type: Option<String>,
    pub avatar_path: Option<String>,
    pub measurements: Option<String>,
    pub cup_size: Option<String>,
    pub birthday: Option<String>,
    pub height_cm: Option<i64>,
    pub debut_date: Option<String>,
    pub wikipedia_zh_url: Option<String>,
    pub note: Option<String>,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ActressInput {
    pub name: String,
    pub simplified_chinese_name: Option<String>,
    pub former_chinese_names: Option<String>,
    pub traditional_chinese_name: Option<String>,
    pub japanese_name: Option<String>,
    pub romanized_name: Option<String>,
    pub default_display_name_type: Option<String>,
    pub avatar_path: Option<String>,
    pub measurements: Option<String>,
    pub cup_size: Option<String>,
    pub birthday: Option<String>,
    pub height_cm: Option<i64>,
    pub debut_date: Option<String>,
    pub wikipedia_zh_url: Option<String>,
    pub note: Option<String>,
}

pub fn create_video(
    db_path: &PathBuf,
    account_id: &str,
    input: VideoInput,
) -> Result<VideoRecord, String> {
    validate_account_id(account_id)?;
    let input = normalize_video_input(input)?;
    crate::db::initialize_database(db_path).map_err(|error| error.to_string())?;

    let connection = open_connection(db_path)?;
    let video_id = Uuid::new_v4().to_string();
    connection
        .execute(
            "INSERT INTO videos (
                id,
                account_id,
                code,
                title,
                cover_path,
                release_date,
                duration_minutes,
                source_url,
                summary,
                actress_names,
                work_type,
                review,
                review_created_at,
                review_updated_at,
                updated_at
            ) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12,
                CASE WHEN ?12 IS NULL THEN NULL ELSE CURRENT_TIMESTAMP END,
                CASE WHEN ?12 IS NULL THEN NULL ELSE CURRENT_TIMESTAMP END,
                CURRENT_TIMESTAMP)",
            params![
                video_id,
                account_id,
                input.code,
                input.title,
                input.cover_path,
                input.release_date,
                input.duration_minutes,
                input.source_url,
                input.summary,
                input.actor_names,
                input.work_type,
                input.review
            ],
        )
        .map_err(|error| error.to_string())?;

    if input.work_type == "multiple" {
        sync_matching_actresses_for_video(
            &connection,
            account_id,
            &video_id,
            input.actor_names.as_deref(),
        )?;
    }

    get_video(db_path, account_id, &video_id)
}

pub fn update_video(
    db_path: &PathBuf,
    account_id: &str,
    video_id: &str,
    input: VideoInput,
) -> Result<VideoRecord, String> {
    validate_account_id(account_id)?;
    validate_record_id(video_id, "Video id")?;
    let input = normalize_video_input(input)?;
    crate::db::initialize_database(db_path).map_err(|error| error.to_string())?;

    let connection = open_connection(db_path)?;
    let changed = connection
        .execute(
            "UPDATE videos
             SET code = ?1,
                 title = ?2,
                 cover_path = ?3,
                 release_date = ?4,
                 duration_minutes = ?5,
                 source_url = ?6,
                 summary = ?7,
                 actress_names = ?8,
                 work_type = ?9,
                 review = ?10,
                 review_created_at = CASE
                    WHEN ?10 IS NULL THEN NULL
                    WHEN review_created_at IS NULL THEN CURRENT_TIMESTAMP
                    ELSE review_created_at
                 END,
                 review_updated_at = CASE
                    WHEN ?10 IS NULL THEN NULL
                    ELSE CURRENT_TIMESTAMP
                 END,
                 updated_at = CURRENT_TIMESTAMP
             WHERE id = ?11 AND account_id = ?12",
            params![
                input.code,
                input.title,
                input.cover_path,
                input.release_date,
                input.duration_minutes,
                input.source_url,
                input.summary,
                input.actor_names,
                input.work_type,
                input.review,
                video_id,
                account_id
            ],
        )
        .map_err(|error| error.to_string())?;

    if changed == 0 {
        return Err("Video record was not found.".to_string());
    }

    if input.work_type == "multiple" {
        sync_matching_actresses_for_video(
            &connection,
            account_id,
            video_id,
            input.actor_names.as_deref(),
        )?;
    }

    get_video(db_path, account_id, video_id)
}

pub fn list_videos(db_path: &PathBuf, account_id: &str) -> Result<Vec<VideoRecord>, String> {
    validate_account_id(account_id)?;
    crate::db::initialize_database(db_path).map_err(|error| error.to_string())?;

    let connection = open_connection(db_path)?;
    let mut statement = connection
        .prepare(
            "SELECT id,
                    account_id,
                    code,
                    title,
                    cover_path,
                    release_date,
                    duration_minutes,
                    source_url,
                    summary,
                    actress_names,
                    work_type,
                    review,
                    created_at,
                    updated_at
             FROM videos
             WHERE account_id = ?1
             ORDER BY updated_at DESC, created_at DESC",
        )
        .map_err(|error| error.to_string())?;
    let rows = statement
        .query_map(params![account_id], map_video_row)
        .map_err(|error| error.to_string())?;

    collect_rows(rows)
}

pub fn get_video(
    db_path: &PathBuf,
    account_id: &str,
    video_id: &str,
) -> Result<VideoRecord, String> {
    validate_account_id(account_id)?;
    validate_record_id(video_id, "Video id")?;
    crate::db::initialize_database(db_path).map_err(|error| error.to_string())?;

    let connection = open_connection(db_path)?;
    let row = connection.query_row(
        "SELECT id,
                account_id,
                code,
                title,
                cover_path,
                release_date,
                duration_minutes,
                source_url,
                summary,
                actress_names,
                work_type,
                review,
                created_at,
                updated_at
         FROM videos
         WHERE id = ?1 AND account_id = ?2",
        params![video_id, account_id],
        map_video_row,
    );

    match row {
        Ok(record) => Ok(record),
        Err(SqlError::QueryReturnedNoRows) => Err("Video record was not found.".to_string()),
        Err(error) => Err(error.to_string()),
    }
}

pub fn create_actress(
    db_path: &PathBuf,
    account_id: &str,
    input: ActressInput,
) -> Result<ActressRecord, String> {
    validate_account_id(account_id)?;
    let input = normalize_actress_input(input)?;
    crate::db::initialize_database(db_path).map_err(|error| error.to_string())?;

    let connection = open_connection(db_path)?;
    let actress_id = Uuid::new_v4().to_string();
    connection
        .execute(
            "INSERT INTO actresses (
                id,
                account_id,
                name,
                simplified_chinese_name,
                former_chinese_names,
                traditional_chinese_name,
                japanese_name,
                romanized_name,
                default_display_name_type,
                avatar_path,
                measurements,
                cup_size,
                birthday,
                height_cm,
                debut_date,
                wikipedia_zh_url,
                note,
                updated_at
            ) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12, ?13, ?14, ?15, ?16, ?17, CURRENT_TIMESTAMP)",
            params![
                actress_id,
                account_id,
                input.name,
                input.simplified_chinese_name,
                input.former_chinese_names,
                input.traditional_chinese_name,
                input.japanese_name,
                input.romanized_name,
                input.default_display_name_type,
                input.avatar_path,
                input.measurements,
                input.cup_size,
                input.birthday,
                input.height_cm,
                input.debut_date,
                input.wikipedia_zh_url,
                input.note
            ],
        )
        .map_err(|error| error.to_string())?;

    get_actress(db_path, account_id, &actress_id)
}

pub fn update_actress(
    db_path: &PathBuf,
    account_id: &str,
    actress_id: &str,
    input: ActressInput,
) -> Result<ActressRecord, String> {
    validate_account_id(account_id)?;
    validate_record_id(actress_id, "Actress id")?;
    let input = normalize_actress_input(input)?;
    crate::db::initialize_database(db_path).map_err(|error| error.to_string())?;

    let connection = open_connection(db_path)?;
    let changed = connection
        .execute(
            "UPDATE actresses
             SET name = ?1,
                 simplified_chinese_name = ?2,
                 former_chinese_names = ?3,
                 traditional_chinese_name = ?4,
                 japanese_name = ?5,
                 romanized_name = ?6,
                 default_display_name_type = ?7,
                 avatar_path = ?8,
                 measurements = ?9,
                 cup_size = ?10,
                 birthday = ?11,
                 height_cm = ?12,
                 debut_date = ?13,
                 wikipedia_zh_url = ?14,
                 note = ?15,
                 updated_at = CURRENT_TIMESTAMP
             WHERE id = ?16 AND account_id = ?17",
            params![
                input.name,
                input.simplified_chinese_name,
                input.former_chinese_names,
                input.traditional_chinese_name,
                input.japanese_name,
                input.romanized_name,
                input.default_display_name_type,
                input.avatar_path,
                input.measurements,
                input.cup_size,
                input.birthday,
                input.height_cm,
                input.debut_date,
                input.wikipedia_zh_url,
                input.note,
                actress_id,
                account_id
            ],
        )
        .map_err(|error| error.to_string())?;

    if changed == 0 {
        return Err("Actress record was not found.".to_string());
    }

    get_actress(db_path, account_id, actress_id)
}

pub fn list_actresses(db_path: &PathBuf, account_id: &str) -> Result<Vec<ActressRecord>, String> {
    validate_account_id(account_id)?;
    crate::db::initialize_database(db_path).map_err(|error| error.to_string())?;

    let connection = open_connection(db_path)?;
    let mut statement = connection
        .prepare(
            "SELECT id,
                    account_id,
                    name,
                    simplified_chinese_name,
                    former_chinese_names,
                    traditional_chinese_name,
                    japanese_name,
                    romanized_name,
                    default_display_name_type,
                    avatar_path,
                    measurements,
                    cup_size,
                    birthday,
                    height_cm,
                    debut_date,
                    wikipedia_zh_url,
                    note,
                    created_at,
                    updated_at
             FROM actresses
             WHERE account_id = ?1
             ORDER BY updated_at DESC, created_at DESC",
        )
        .map_err(|error| error.to_string())?;
    let rows = statement
        .query_map(params![account_id], map_actress_row)
        .map_err(|error| error.to_string())?;

    collect_rows(rows)
}

pub fn get_actress(
    db_path: &PathBuf,
    account_id: &str,
    actress_id: &str,
) -> Result<ActressRecord, String> {
    validate_account_id(account_id)?;
    validate_record_id(actress_id, "Actress id")?;
    crate::db::initialize_database(db_path).map_err(|error| error.to_string())?;

    let connection = open_connection(db_path)?;
    let row = connection.query_row(
        "SELECT id,
                account_id,
                name,
                simplified_chinese_name,
                former_chinese_names,
                traditional_chinese_name,
                japanese_name,
                romanized_name,
                default_display_name_type,
                avatar_path,
                measurements,
                cup_size,
                birthday,
                height_cm,
                debut_date,
                wikipedia_zh_url,
                note,
                created_at,
                updated_at
         FROM actresses
         WHERE id = ?1 AND account_id = ?2",
        params![actress_id, account_id],
        map_actress_row,
    );

    match row {
        Ok(record) => Ok(record),
        Err(SqlError::QueryReturnedNoRows) => Err("Actress record was not found.".to_string()),
        Err(error) => Err(error.to_string()),
    }
}

pub fn list_video_actresses(
    db_path: &PathBuf,
    account_id: &str,
    video_id: &str,
) -> Result<Vec<ActressRecord>, String> {
    validate_account_id(account_id)?;
    validate_record_id(video_id, "Video id")?;
    crate::db::initialize_database(db_path).map_err(|error| error.to_string())?;

    let connection = open_connection(db_path)?;
    ensure_video_exists(&connection, account_id, video_id)?;
    let mut statement = connection
        .prepare(
            "SELECT a.id,
                    a.account_id,
                    a.name,
                    a.simplified_chinese_name,
                    a.former_chinese_names,
                    a.traditional_chinese_name,
                    a.japanese_name,
                    a.romanized_name,
                    a.default_display_name_type,
                    a.avatar_path,
                    a.measurements,
                    a.cup_size,
                    a.birthday,
                    a.height_cm,
                    a.debut_date,
                    a.wikipedia_zh_url,
                    a.note,
                    a.created_at,
                    a.updated_at
             FROM video_actresses va
             JOIN actresses a ON a.id = va.actress_id
             WHERE va.video_id = ?1 AND a.account_id = ?2
             ORDER BY a.name ASC",
        )
        .map_err(|error| error.to_string())?;
    let rows = statement
        .query_map(params![video_id, account_id], map_actress_row)
        .map_err(|error| error.to_string())?;

    collect_rows(rows)
}

pub fn list_actress_videos(
    db_path: &PathBuf,
    account_id: &str,
    actress_id: &str,
) -> Result<Vec<VideoRecord>, String> {
    validate_account_id(account_id)?;
    validate_record_id(actress_id, "Actress id")?;
    crate::db::initialize_database(db_path).map_err(|error| error.to_string())?;

    let connection = open_connection(db_path)?;
    ensure_actress_exists(&connection, account_id, actress_id)?;
    let mut statement = connection
        .prepare(
            "SELECT v.id,
                    v.account_id,
                    v.code,
                    v.title,
                    v.cover_path,
                    v.release_date,
                    v.duration_minutes,
                    v.source_url,
                    v.summary,
                    v.actress_names,
                    v.work_type,
                    v.review,
                    v.created_at,
                    v.updated_at
             FROM video_actresses va
             JOIN videos v ON v.id = va.video_id
             WHERE va.actress_id = ?1 AND v.account_id = ?2
             ORDER BY v.updated_at DESC, v.created_at DESC",
        )
        .map_err(|error| error.to_string())?;
    let rows = statement
        .query_map(params![actress_id, account_id], map_video_row)
        .map_err(|error| error.to_string())?;

    collect_rows(rows)
}

pub fn set_video_actresses(
    db_path: &PathBuf,
    account_id: &str,
    video_id: &str,
    actress_ids: Vec<String>,
) -> Result<Vec<ActressRecord>, String> {
    validate_account_id(account_id)?;
    validate_record_id(video_id, "Video id")?;
    crate::db::initialize_database(db_path).map_err(|error| error.to_string())?;

    let connection = open_connection(db_path)?;
    let video = ensure_video_exists(&connection, account_id, video_id)?;
    if video.work_type == "single" && actress_ids.is_empty() {
        return Err("Single works must be linked to at least one actress.".to_string());
    }

    let mut unique_ids = Vec::new();
    for actress_id in actress_ids {
        validate_record_id(&actress_id, "Actress id")?;
        if !unique_ids.contains(&actress_id) {
            ensure_actress_exists(&connection, account_id, &actress_id)?;
            unique_ids.push(actress_id);
        }
    }

    connection
        .execute(
            "DELETE FROM video_actresses WHERE video_id = ?1",
            params![video_id],
        )
        .map_err(|error| error.to_string())?;
    for actress_id in unique_ids {
        connection
            .execute(
                "INSERT OR IGNORE INTO video_actresses (video_id, actress_id) VALUES (?1, ?2)",
                params![video_id, actress_id],
            )
            .map_err(|error| error.to_string())?;
    }

    list_video_actresses(db_path, account_id, video_id)
}

pub fn add_video_actress(
    db_path: &PathBuf,
    account_id: &str,
    video_id: &str,
    actress_id: &str,
) -> Result<Vec<ActressRecord>, String> {
    validate_account_id(account_id)?;
    validate_record_id(video_id, "Video id")?;
    validate_record_id(actress_id, "Actress id")?;
    crate::db::initialize_database(db_path).map_err(|error| error.to_string())?;

    let connection = open_connection(db_path)?;
    ensure_video_exists(&connection, account_id, video_id)?;
    ensure_actress_exists(&connection, account_id, actress_id)?;
    connection
        .execute(
            "INSERT OR IGNORE INTO video_actresses (video_id, actress_id) VALUES (?1, ?2)",
            params![video_id, actress_id],
        )
        .map_err(|error| error.to_string())?;

    list_video_actresses(db_path, account_id, video_id)
}

pub fn list_association_suggestions(
    db_path: &PathBuf,
    account_id: &str,
    actress_id: &str,
) -> Result<Vec<AssociationSuggestion>, String> {
    validate_account_id(account_id)?;
    validate_record_id(actress_id, "Actress id")?;
    crate::db::initialize_database(db_path).map_err(|error| error.to_string())?;

    let connection = open_connection(db_path)?;
    let actress = ensure_actress_exists(&connection, account_id, actress_id)?;
    let match_names = actress_match_names(&actress);
    if match_names.is_empty() {
        return Ok(Vec::new());
    }

    let videos = list_videos(db_path, account_id)?;
    let suggestions = videos
        .into_iter()
        .filter(|video| video.work_type == "multiple")
        .filter(|video| {
            video
                .actor_names
                .as_deref()
                .is_some_and(|names| actor_names_match(names, &match_names))
        })
        .filter(|video| {
            !is_video_actress_linked(&connection, &video.id, actress_id).unwrap_or(false)
        })
        .map(|video| AssociationSuggestion {
            video,
            actress: actress.clone(),
        })
        .collect();

    Ok(suggestions)
}

pub fn delete_video(db_path: &PathBuf, account_id: &str, video_id: &str) -> Result<(), String> {
    validate_account_id(account_id)?;
    validate_record_id(video_id, "Video id")?;
    crate::db::initialize_database(db_path).map_err(|error| error.to_string())?;

    let connection = open_connection(db_path)?;
    let changed = connection
        .execute(
            "DELETE FROM videos WHERE id = ?1 AND account_id = ?2",
            params![video_id, account_id],
        )
        .map_err(|error| error.to_string())?;
    if changed == 0 {
        return Err("Video record was not found.".to_string());
    }

    Ok(())
}

pub fn delete_actress(db_path: &PathBuf, account_id: &str, actress_id: &str) -> Result<(), String> {
    validate_account_id(account_id)?;
    validate_record_id(actress_id, "Actress id")?;
    crate::db::initialize_database(db_path).map_err(|error| error.to_string())?;

    let connection = open_connection(db_path)?;
    let changed = connection
        .execute(
            "DELETE FROM actresses WHERE id = ?1 AND account_id = ?2",
            params![actress_id, account_id],
        )
        .map_err(|error| error.to_string())?;
    if changed == 0 {
        return Err("Actress record was not found.".to_string());
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

fn normalize_video_input(input: VideoInput) -> Result<VideoInput, String> {
    let code = normalize_required_text(&input.code, "Video code")?;
    let title = normalize_optional_text(input.title);
    let cover_path = normalize_optional_text(input.cover_path);
    let release_date = normalize_optional_text(input.release_date);
    let duration_minutes = normalize_optional_positive_i64(input.duration_minutes, "Duration")?;
    let source_url = normalize_optional_text(input.source_url);
    let summary = normalize_optional_text(input.summary);
    let actor_names = normalize_optional_text(input.actor_names);
    let work_type = normalize_work_type(&input.work_type)?;
    let review = normalize_optional_text(input.review);

    Ok(VideoInput {
        code,
        title,
        cover_path,
        release_date,
        duration_minutes,
        source_url,
        summary,
        actor_names,
        work_type,
        review,
    })
}

fn normalize_actress_input(input: ActressInput) -> Result<ActressInput, String> {
    let name = normalize_required_text(&input.name, "Actress name")?;
    let simplified_chinese_name = normalize_optional_text(input.simplified_chinese_name);
    let former_chinese_names = normalize_optional_text(input.former_chinese_names);
    let traditional_chinese_name = normalize_optional_text(input.traditional_chinese_name);
    let japanese_name = normalize_optional_text(input.japanese_name);
    let romanized_name = normalize_optional_text(input.romanized_name);
    let default_display_name_type =
        normalize_default_display_name_type(input.default_display_name_type)?;
    let avatar_path = normalize_optional_text(input.avatar_path);
    let measurements = normalize_optional_text(input.measurements);
    let cup_size = normalize_optional_text(input.cup_size);
    let birthday = normalize_optional_text(input.birthday);
    let height_cm = normalize_optional_positive_i64(input.height_cm, "Height")?;
    let debut_date = normalize_optional_text(input.debut_date);
    let wikipedia_zh_url = normalize_optional_text(input.wikipedia_zh_url);
    let note = normalize_optional_text(input.note);

    Ok(ActressInput {
        name,
        simplified_chinese_name,
        former_chinese_names,
        traditional_chinese_name,
        japanese_name,
        romanized_name,
        default_display_name_type,
        avatar_path,
        measurements,
        cup_size,
        birthday,
        height_cm,
        debut_date,
        wikipedia_zh_url,
        note,
    })
}

fn normalize_required_text(value: &str, label: &str) -> Result<String, String> {
    let trimmed = value.trim();
    if trimmed.is_empty() {
        return Err(format!("{label} is required."));
    }

    Ok(trimmed.to_string())
}

fn normalize_optional_text(value: Option<String>) -> Option<String> {
    value.and_then(|text| {
        let trimmed = text.trim();
        if trimmed.is_empty() {
            None
        } else {
            Some(trimmed.to_string())
        }
    })
}

fn normalize_default_display_name_type(value: Option<String>) -> Result<Option<String>, String> {
    let Some(value) = normalize_optional_text(value) else {
        return Ok(None);
    };

    match value.as_str() {
        "simplifiedChinese" | "traditionalChinese" | "japanese" | "romanized" | "name" => {
            Ok(Some(value))
        }
        _ => Err("Default display name type is invalid.".to_string()),
    }
}

fn normalize_optional_positive_i64(value: Option<i64>, label: &str) -> Result<Option<i64>, String> {
    match value {
        Some(number) if number < 0 => Err(format!("{label} cannot be negative.")),
        Some(0) | None => Ok(None),
        Some(number) => Ok(Some(number)),
    }
}

fn normalize_work_type(value: &str) -> Result<String, String> {
    let normalized = value.trim();
    match normalized {
        "single" | "multiple" | "amateur" => Ok(normalized.to_string()),
        _ => Err("Work type must be single, multiple, or amateur.".to_string()),
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

fn ensure_video_exists(
    connection: &Connection,
    account_id: &str,
    video_id: &str,
) -> Result<VideoRecord, String> {
    let row = connection.query_row(
        "SELECT id,
                account_id,
                code,
                title,
                cover_path,
                release_date,
                duration_minutes,
                source_url,
                summary,
                actress_names,
                work_type,
                review,
                created_at,
                updated_at
         FROM videos
         WHERE id = ?1 AND account_id = ?2",
        params![video_id, account_id],
        map_video_row,
    );

    match row {
        Ok(record) => Ok(record),
        Err(SqlError::QueryReturnedNoRows) => Err("Video record was not found.".to_string()),
        Err(error) => Err(error.to_string()),
    }
}

fn ensure_actress_exists(
    connection: &Connection,
    account_id: &str,
    actress_id: &str,
) -> Result<ActressRecord, String> {
    let row = connection.query_row(
        "SELECT id,
                account_id,
                name,
                simplified_chinese_name,
                former_chinese_names,
                traditional_chinese_name,
                japanese_name,
                romanized_name,
                default_display_name_type,
                avatar_path,
                measurements,
                cup_size,
                birthday,
                height_cm,
                debut_date,
                wikipedia_zh_url,
                note,
                created_at,
                updated_at
         FROM actresses
         WHERE id = ?1 AND account_id = ?2",
        params![actress_id, account_id],
        map_actress_row,
    );

    match row {
        Ok(record) => Ok(record),
        Err(SqlError::QueryReturnedNoRows) => Err("Actress record was not found.".to_string()),
        Err(error) => Err(error.to_string()),
    }
}

fn sync_matching_actresses_for_video(
    connection: &Connection,
    account_id: &str,
    video_id: &str,
    actor_names: Option<&str>,
) -> Result<(), String> {
    let Some(actor_names) = actor_names else {
        return Ok(());
    };

    let mut statement = connection
        .prepare(
            "SELECT id,
                    account_id,
                    name,
                    simplified_chinese_name,
                    former_chinese_names,
                    traditional_chinese_name,
                    japanese_name,
                    romanized_name,
                    default_display_name_type,
                    avatar_path,
                    measurements,
                    cup_size,
                    birthday,
                    height_cm,
                    debut_date,
                    wikipedia_zh_url,
                    note,
                    created_at,
                    updated_at
             FROM actresses
             WHERE account_id = ?1",
        )
        .map_err(|error| error.to_string())?;
    let rows = statement
        .query_map(params![account_id], map_actress_row)
        .map_err(|error| error.to_string())?;
    let actresses = collect_rows(rows)?;

    for actress in actresses {
        if actor_names_match(actor_names, &actress_match_names(&actress)) {
            connection
                .execute(
                    "INSERT OR IGNORE INTO video_actresses (video_id, actress_id) VALUES (?1, ?2)",
                    params![video_id, actress.id],
                )
                .map_err(|error| error.to_string())?;
        }
    }

    Ok(())
}

fn is_video_actress_linked(
    connection: &Connection,
    video_id: &str,
    actress_id: &str,
) -> Result<bool, String> {
    let count: i64 = connection
        .query_row(
            "SELECT COUNT(*) FROM video_actresses WHERE video_id = ?1 AND actress_id = ?2",
            params![video_id, actress_id],
            |row| row.get(0),
        )
        .map_err(|error| error.to_string())?;

    Ok(count > 0)
}

fn actress_match_names(actress: &ActressRecord) -> Vec<String> {
    [
        Some(actress.name.as_str()),
        actress.simplified_chinese_name.as_deref(),
        actress.traditional_chinese_name.as_deref(),
        actress.japanese_name.as_deref(),
        actress.romanized_name.as_deref(),
    ]
    .into_iter()
    .flatten()
    .chain(
        actress
            .former_chinese_names
            .as_deref()
            .into_iter()
            .flat_map(split_actor_names),
    )
    .map(normalize_match_text)
    .filter(|name| !name.is_empty())
    .fold(Vec::new(), |mut names, name| {
        if !names.contains(&name) {
            names.push(name);
        }
        names
    })
}

fn actor_names_match(actor_names: &str, match_names: &[String]) -> bool {
    let normalized_actor_names = split_actor_names(actor_names)
        .into_iter()
        .map(normalize_match_text)
        .collect::<Vec<_>>();

    match_names
        .iter()
        .any(|name| normalized_actor_names.iter().any(|actor| actor == name))
}

fn split_actor_names(value: &str) -> Vec<&str> {
    value
        .split(|character: char| {
            matches!(
                character,
                ',' | '，' | '、' | '/' | '／' | ';' | '；' | '\n' | '\r' | '\t'
            )
        })
        .collect()
}

fn normalize_match_text(value: &str) -> String {
    value
        .chars()
        .filter(|character| !character.is_whitespace())
        .flat_map(|character| character.to_lowercase())
        .collect()
}

fn map_video_row(row: &rusqlite::Row<'_>) -> rusqlite::Result<VideoRecord> {
    Ok(VideoRecord {
        id: row.get(0)?,
        account_id: row.get(1)?,
        code: row.get(2)?,
        title: row.get(3)?,
        cover_path: row.get(4)?,
        release_date: row.get(5)?,
        duration_minutes: row.get(6)?,
        source_url: row.get(7)?,
        summary: row.get(8)?,
        actor_names: row.get(9)?,
        work_type: row.get(10)?,
        review: row.get(11)?,
        created_at: row.get(12)?,
        updated_at: row.get(13)?,
    })
}

fn map_actress_row(row: &rusqlite::Row<'_>) -> rusqlite::Result<ActressRecord> {
    Ok(ActressRecord {
        id: row.get(0)?,
        account_id: row.get(1)?,
        name: row.get(2)?,
        simplified_chinese_name: row.get(3)?,
        former_chinese_names: row.get(4)?,
        traditional_chinese_name: row.get(5)?,
        japanese_name: row.get(6)?,
        romanized_name: row.get(7)?,
        default_display_name_type: row.get(8)?,
        avatar_path: row.get(9)?,
        measurements: row.get(10)?,
        cup_size: row.get(11)?,
        birthday: row.get(12)?,
        height_cm: row.get(13)?,
        debut_date: row.get(14)?,
        wikipedia_zh_url: row.get(15)?,
        note: row.get(16)?,
        created_at: row.get(17)?,
        updated_at: row.get(18)?,
    })
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
    use crate::auth::register_account;
    use std::{fs, time::SystemTime};

    fn test_db_path(label: &str) -> PathBuf {
        let nanos = SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .expect("system time is valid")
            .as_nanos();
        std::env::temp_dir().join(format!("nvy-{label}-{}-{nanos}.sqlite", std::process::id()))
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
    fn creates_reads_and_updates_video_for_one_account() {
        let db_path = test_db_path("video-crud");
        let first =
            register_account(&db_path, "VideoUser", "abc123").expect("first account registers");
        let second =
            register_account(&db_path, "OtherUser", "abc123").expect("second account registers");

        let created = create_video(
            &db_path,
            &first.account_id,
            VideoInput {
                code: " aaa-001 ".to_string(),
                title: Some(" First title ".to_string()),
                cover_path: Some(" ".to_string()),
                release_date: Some("2026-06-01".to_string()),
                duration_minutes: Some(125),
                source_url: Some("https://example.test/video".to_string()),
                summary: Some(" Summary ".to_string()),
                actor_names: None,
                work_type: "single".to_string(),
                review: Some(" Review ".to_string()),
            },
        )
        .expect("video creates");

        assert_eq!(created.code, "aaa-001");
        assert_eq!(created.title.as_deref(), Some("First title"));
        assert_eq!(created.cover_path, None);
        assert_eq!(created.release_date.as_deref(), Some("2026-06-01"));
        assert_eq!(created.duration_minutes, Some(125));
        assert_eq!(created.summary.as_deref(), Some("Summary"));
        assert_eq!(created.review.as_deref(), Some("Review"));
        assert_eq!(list_videos(&db_path, &first.account_id).unwrap().len(), 1);
        assert!(list_videos(&db_path, &second.account_id)
            .unwrap()
            .is_empty());
        assert!(get_video(&db_path, &second.account_id, &created.id).is_err());

        let updated = update_video(
            &db_path,
            &first.account_id,
            &created.id,
            VideoInput {
                code: "AAA-002".to_string(),
                title: Some("Updated".to_string()),
                cover_path: Some("covers/video.jpg".to_string()),
                release_date: Some("2026-06-02".to_string()),
                duration_minutes: Some(98),
                source_url: Some("https://example.test/updated".to_string()),
                summary: Some("Updated summary".to_string()),
                actor_names: Some("Alice Updated, Unknown".to_string()),
                work_type: "multiple".to_string(),
                review: Some("Updated review".to_string()),
            },
        )
        .expect("video updates");

        assert_eq!(updated.code, "AAA-002");
        assert_eq!(updated.title.as_deref(), Some("Updated"));
        assert_eq!(updated.cover_path.as_deref(), Some("covers/video.jpg"));
        assert_eq!(updated.release_date.as_deref(), Some("2026-06-02"));
        assert_eq!(updated.duration_minutes, Some(98));
        assert_eq!(
            updated.source_url.as_deref(),
            Some("https://example.test/updated")
        );
        assert_eq!(updated.summary.as_deref(), Some("Updated summary"));
        assert_eq!(updated.work_type, "multiple");
        assert_eq!(updated.review.as_deref(), Some("Updated review"));

        let _ = fs::remove_file(db_path);
    }

    #[test]
    fn rejects_invalid_video_input() {
        let db_path = test_db_path("video-invalid");
        let session = register_account(&db_path, "BadVideo", "abc123").expect("account registers");

        assert!(create_video(&db_path, &session.account_id, video_input(" "),).is_err());
        assert!(create_video(
            &db_path,
            &session.account_id,
            VideoInput {
                duration_minutes: Some(-1),
                ..video_input("AAA-001")
            },
        )
        .is_err());
        assert!(create_video(
            &db_path,
            &session.account_id,
            VideoInput {
                work_type: "unknown".to_string(),
                ..video_input("AAA-001")
            },
        )
        .is_err());

        let _ = fs::remove_file(db_path);
    }

    #[test]
    fn creates_reads_and_updates_actress_for_one_account() {
        let db_path = test_db_path("actress-crud");
        let first =
            register_account(&db_path, "ActUser", "abc123").expect("first account registers");
        let second =
            register_account(&db_path, "AltUser", "abc123").expect("second account registers");

        let created = create_actress(
            &db_path,
            &first.account_id,
            ActressInput {
                name: " Alice ".to_string(),
                simplified_chinese_name: Some(" 爱丽丝 ".to_string()),
                former_chinese_names: Some("旧名".to_string()),
                traditional_chinese_name: Some("愛麗絲".to_string()),
                japanese_name: Some("アリス".to_string()),
                romanized_name: Some("Alice".to_string()),
                default_display_name_type: Some("traditionalChinese".to_string()),
                avatar_path: Some(" ".to_string()),
                measurements: Some("90-60-90".to_string()),
                cup_size: Some("C".to_string()),
                birthday: Some("2000-01-01".to_string()),
                height_cm: Some(165),
                debut_date: Some("2020-01-01".to_string()),
                wikipedia_zh_url: Some("https://example.test/wiki".to_string()),
                note: Some(" Note ".to_string()),
            },
        )
        .expect("actress creates");

        assert_eq!(created.name, "Alice");
        assert_eq!(created.simplified_chinese_name.as_deref(), Some("爱丽丝"));
        assert_eq!(
            created.default_display_name_type.as_deref(),
            Some("traditionalChinese")
        );
        assert_eq!(created.avatar_path, None);
        assert_eq!(created.height_cm, Some(165));
        assert_eq!(created.note.as_deref(), Some("Note"));
        assert_eq!(
            list_actresses(&db_path, &first.account_id).unwrap().len(),
            1
        );
        assert!(list_actresses(&db_path, &second.account_id)
            .unwrap()
            .is_empty());
        assert!(get_actress(&db_path, &second.account_id, &created.id).is_err());

        let updated = update_actress(
            &db_path,
            &first.account_id,
            &created.id,
            ActressInput {
                name: "Alice Updated".to_string(),
                simplified_chinese_name: Some("更新名".to_string()),
                former_chinese_names: None,
                traditional_chinese_name: None,
                japanese_name: None,
                romanized_name: Some("Alice Updated".to_string()),
                default_display_name_type: Some("romanized".to_string()),
                avatar_path: Some("avatars/alice.jpg".to_string()),
                measurements: None,
                cup_size: Some("D".to_string()),
                birthday: None,
                height_cm: Some(166),
                debut_date: None,
                wikipedia_zh_url: Some("https://example.test/wiki-updated".to_string()),
                note: Some("Updated note".to_string()),
            },
        )
        .expect("actress updates");

        assert_eq!(updated.name, "Alice Updated");
        assert_eq!(updated.avatar_path.as_deref(), Some("avatars/alice.jpg"));
        assert_eq!(updated.cup_size.as_deref(), Some("D"));
        assert_eq!(
            updated.default_display_name_type.as_deref(),
            Some("romanized")
        );
        assert_eq!(updated.height_cm, Some(166));
        assert_eq!(updated.note.as_deref(), Some("Updated note"));

        let _ = fs::remove_file(db_path);
    }

    #[test]
    fn rejects_invalid_actress_input() {
        let db_path = test_db_path("actress-invalid");
        let session = register_account(&db_path, "BadAct", "abc123").expect("account registers");

        assert!(create_actress(&db_path, &session.account_id, actress_input(" "),).is_err());

        let _ = fs::remove_file(db_path);
    }

    #[test]
    fn manages_video_actress_links_and_delete_rules() {
        let db_path = test_db_path("associations");
        let session = register_account(&db_path, "AssocUser", "abc123").expect("account registers");
        let actress =
            create_actress(&db_path, &session.account_id, actress_input("Alice")).unwrap();
        let video = create_video(&db_path, &session.account_id, video_input("AAA-001")).unwrap();

        assert!(set_video_actresses(&db_path, &session.account_id, &video.id, vec![]).is_err());

        let linked = set_video_actresses(
            &db_path,
            &session.account_id,
            &video.id,
            vec![actress.id.clone(), actress.id.clone()],
        )
        .expect("single work links actress");
        assert_eq!(linked.len(), 1);
        assert_eq!(
            list_actress_videos(&db_path, &session.account_id, &actress.id)
                .unwrap()
                .len(),
            1
        );

        delete_actress(&db_path, &session.account_id, &actress.id).expect("actress deletes");
        assert_eq!(list_videos(&db_path, &session.account_id).unwrap().len(), 1);
        assert!(list_actress_videos(&db_path, &session.account_id, &actress.id).is_err());

        delete_video(&db_path, &session.account_id, &video.id).expect("video deletes");
        assert!(list_videos(&db_path, &session.account_id)
            .unwrap()
            .is_empty());

        let _ = fs::remove_file(db_path);
    }

    #[test]
    fn matches_existing_actresses_from_multiple_work_actor_names() {
        let db_path = test_db_path("actor-names");
        let session = register_account(&db_path, "ActorUser", "abc123").expect("account registers");
        let alice = create_actress(&db_path, &session.account_id, actress_input("Alice")).unwrap();

        let video = create_video(
            &db_path,
            &session.account_id,
            VideoInput {
                actor_names: Some("Alice, Not Collected".to_string()),
                work_type: "multiple".to_string(),
                ..video_input("BBB-001")
            },
        )
        .expect("multiple work creates");

        let linked = list_video_actresses(&db_path, &session.account_id, &video.id).unwrap();
        assert_eq!(linked.len(), 1);
        assert_eq!(linked[0].id, alice.id);

        let later = create_actress(
            &db_path,
            &session.account_id,
            actress_input("Not Collected"),
        )
        .unwrap();
        let suggestions =
            list_association_suggestions(&db_path, &session.account_id, &later.id).unwrap();
        assert_eq!(suggestions.len(), 1);
        assert_eq!(suggestions[0].video.id, video.id);

        add_video_actress(&db_path, &session.account_id, &video.id, &later.id)
            .expect("suggestion can be confirmed");
        assert!(
            list_association_suggestions(&db_path, &session.account_id, &later.id)
                .unwrap()
                .is_empty()
        );
        assert_eq!(
            list_video_actresses(&db_path, &session.account_id, &video.id)
                .unwrap()
                .len(),
            2
        );

        let _ = fs::remove_file(db_path);
    }
}
