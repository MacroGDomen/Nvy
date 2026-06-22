use rusqlite::{params, Connection, Error as SqlError};
use serde::Serialize;
use std::path::PathBuf;
use uuid::Uuid;

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct TagRecord {
    pub id: String,
    pub account_id: String,
    pub scope: String,
    pub canonical_name: String,
    pub aliases: Option<String>,
    pub related_tags: Option<String>,
    pub is_preset: bool,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TagInput {
    pub scope: String,
    pub canonical_name: String,
    pub aliases: Option<String>,
    pub related_tags: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct TagMatch {
    pub tag: TagRecord,
    pub matched_by: String,
}

pub fn list_tags(
    db_path: &PathBuf,
    account_id: &str,
    scope: &str,
) -> Result<Vec<TagRecord>, String> {
    validate_account_id(account_id)?;
    let scope = normalize_scope(scope)?;
    crate::db::initialize_database(db_path).map_err(|error| error.to_string())?;

    let connection = open_connection(db_path)?;
    let mut statement = connection
        .prepare(
            "SELECT id,
                    account_id,
                    scope,
                    canonical_name,
                    aliases,
                    related_tags,
                    is_preset,
                    created_at,
                    updated_at
             FROM tag_library
             WHERE account_id = ?1 AND scope = ?2
             ORDER BY is_preset DESC, canonical_name ASC",
        )
        .map_err(|error| error.to_string())?;
    let rows = statement
        .query_map(params![account_id, scope], map_tag_row)
        .map_err(|error| error.to_string())?;

    collect_rows(rows)
}

pub fn create_tag(
    db_path: &PathBuf,
    account_id: &str,
    input: TagInput,
) -> Result<TagRecord, String> {
    validate_account_id(account_id)?;
    let input = normalize_tag_input(input)?;
    crate::db::initialize_database(db_path).map_err(|error| error.to_string())?;

    let connection = open_connection(db_path)?;
    let tag_id = Uuid::new_v4().to_string();
    connection
        .execute(
            "INSERT INTO tag_library (
                id,
                account_id,
                scope,
                canonical_name,
                aliases,
                related_tags,
                is_preset,
                updated_at
            ) VALUES (?1, ?2, ?3, ?4, ?5, ?6, 0, CURRENT_TIMESTAMP)",
            params![
                tag_id,
                account_id,
                input.scope,
                input.canonical_name,
                input.aliases,
                input.related_tags
            ],
        )
        .map_err(|error| error.to_string())?;

    get_tag(&connection, account_id, &tag_id)
}

pub fn update_tag(
    db_path: &PathBuf,
    account_id: &str,
    tag_id: &str,
    input: TagInput,
) -> Result<TagRecord, String> {
    validate_account_id(account_id)?;
    validate_record_id(tag_id, "Tag id")?;
    let input = normalize_tag_input(input)?;
    crate::db::initialize_database(db_path).map_err(|error| error.to_string())?;

    let connection = open_connection(db_path)?;
    let changed = connection
        .execute(
            "UPDATE tag_library
             SET scope = ?1,
                 canonical_name = ?2,
                 aliases = ?3,
                 related_tags = ?4,
                 updated_at = CURRENT_TIMESTAMP
             WHERE id = ?5 AND account_id = ?6",
            params![
                input.scope,
                input.canonical_name,
                input.aliases,
                input.related_tags,
                tag_id,
                account_id
            ],
        )
        .map_err(|error| error.to_string())?;

    if changed == 0 {
        return Err("Tag was not found.".to_string());
    }

    get_tag(&connection, account_id, tag_id)
}

pub fn delete_tag(db_path: &PathBuf, account_id: &str, tag_id: &str) -> Result<(), String> {
    validate_account_id(account_id)?;
    validate_record_id(tag_id, "Tag id")?;
    crate::db::initialize_database(db_path).map_err(|error| error.to_string())?;

    let connection = open_connection(db_path)?;
    let changed = connection
        .execute(
            "DELETE FROM tag_library WHERE id = ?1 AND account_id = ?2",
            params![tag_id, account_id],
        )
        .map_err(|error| error.to_string())?;
    if changed == 0 {
        return Err("Tag was not found.".to_string());
    }

    Ok(())
}

pub fn match_tags(
    db_path: &PathBuf,
    account_id: &str,
    scope: &str,
    query: &str,
) -> Result<Vec<TagMatch>, String> {
    let tags = list_tags(db_path, account_id, scope)?;
    let normalized_query = normalize_tag_lookup(query);
    if normalized_query.is_empty() {
        return Ok(Vec::new());
    }

    Ok(tags
        .into_iter()
        .filter_map(|tag| {
            let names = tag_lookup_names(&tag);
            names.into_iter().find_map(|name| {
                if normalize_tag_lookup(&name) == normalized_query {
                    Some(TagMatch {
                        tag: tag.clone(),
                        matched_by: name,
                    })
                } else {
                    None
                }
            })
        })
        .collect())
}

pub fn list_video_tags(
    db_path: &PathBuf,
    account_id: &str,
    video_id: &str,
) -> Result<Vec<TagRecord>, String> {
    list_record_tags(db_path, account_id, "video", video_id)
}

pub fn set_video_tags(
    db_path: &PathBuf,
    account_id: &str,
    video_id: &str,
    tag_ids: Vec<String>,
) -> Result<Vec<TagRecord>, String> {
    set_record_tags(db_path, account_id, "video", video_id, tag_ids)
}

pub fn list_actress_tags(
    db_path: &PathBuf,
    account_id: &str,
    actress_id: &str,
) -> Result<Vec<TagRecord>, String> {
    list_record_tags(db_path, account_id, "actress", actress_id)
}

pub fn set_actress_tags(
    db_path: &PathBuf,
    account_id: &str,
    actress_id: &str,
    tag_ids: Vec<String>,
) -> Result<Vec<TagRecord>, String> {
    set_record_tags(db_path, account_id, "actress", actress_id, tag_ids)
}

pub fn auto_tag_video(
    db_path: &PathBuf,
    account_id: &str,
    video_id: &str,
) -> Result<Vec<TagRecord>, String> {
    validate_account_id(account_id)?;
    validate_record_id(video_id, "Video id")?;
    crate::db::initialize_database(db_path).map_err(|error| error.to_string())?;

    let connection = open_connection(db_path)?;
    ensure_video_exists(&connection, account_id, video_id)?;
    let mut tag_ids = list_video_tags(db_path, account_id, video_id)?
        .into_iter()
        .map(|tag| tag.id)
        .collect::<Vec<_>>();

    for tag_name in auto_tag_names(&connection, account_id, video_id)? {
        let tag = ensure_tag(&connection, account_id, "video", &tag_name)?;
        if !tag_ids.contains(&tag.id) {
            tag_ids.push(tag.id);
        }
    }

    set_video_tags(db_path, account_id, video_id, tag_ids)
}

fn list_record_tags(
    db_path: &PathBuf,
    account_id: &str,
    scope: &str,
    record_id: &str,
) -> Result<Vec<TagRecord>, String> {
    validate_account_id(account_id)?;
    validate_record_id(record_id, "Record id")?;
    let scope = normalize_scope(scope)?;
    crate::db::initialize_database(db_path).map_err(|error| error.to_string())?;

    let connection = open_connection(db_path)?;
    if scope == "video" {
        ensure_video_exists(&connection, account_id, record_id)?;
    } else {
        ensure_actress_exists(&connection, account_id, record_id)?;
    }

    let sql = if scope == "video" {
        "SELECT t.id,
                t.account_id,
                t.scope,
                t.canonical_name,
                t.aliases,
                t.related_tags,
                t.is_preset,
                t.created_at,
                t.updated_at
         FROM video_tags vt
         JOIN tag_library t ON t.id = vt.tag_id
         WHERE vt.video_id = ?1 AND t.account_id = ?2 AND t.scope = 'video'
         ORDER BY t.canonical_name ASC"
    } else {
        "SELECT t.id,
                t.account_id,
                t.scope,
                t.canonical_name,
                t.aliases,
                t.related_tags,
                t.is_preset,
                t.created_at,
                t.updated_at
         FROM actress_tags at
         JOIN tag_library t ON t.id = at.tag_id
         WHERE at.actress_id = ?1 AND t.account_id = ?2 AND t.scope = 'actress'
         ORDER BY t.canonical_name ASC"
    };
    let mut statement = connection.prepare(sql).map_err(|error| error.to_string())?;
    let rows = statement
        .query_map(params![record_id, account_id], map_tag_row)
        .map_err(|error| error.to_string())?;

    collect_rows(rows)
}

fn set_record_tags(
    db_path: &PathBuf,
    account_id: &str,
    scope: &str,
    record_id: &str,
    tag_ids: Vec<String>,
) -> Result<Vec<TagRecord>, String> {
    validate_account_id(account_id)?;
    validate_record_id(record_id, "Record id")?;
    let scope = normalize_scope(scope)?;
    crate::db::initialize_database(db_path).map_err(|error| error.to_string())?;

    let connection = open_connection(db_path)?;
    if scope == "video" {
        ensure_video_exists(&connection, account_id, record_id)?;
    } else {
        ensure_actress_exists(&connection, account_id, record_id)?;
    }

    let mut unique_ids = Vec::new();
    for tag_id in tag_ids {
        validate_record_id(&tag_id, "Tag id")?;
        let tag = get_tag(&connection, account_id, &tag_id)?;
        if tag.scope != scope {
            return Err("Tag scope does not match record scope.".to_string());
        }
        if !unique_ids.contains(&tag_id) {
            unique_ids.push(tag_id);
        }
    }

    if scope == "video" {
        connection
            .execute(
                "DELETE FROM video_tags WHERE video_id = ?1",
                params![record_id],
            )
            .map_err(|error| error.to_string())?;
        for tag_id in unique_ids {
            connection
                .execute(
                    "INSERT OR IGNORE INTO video_tags (video_id, tag_id) VALUES (?1, ?2)",
                    params![record_id, tag_id],
                )
                .map_err(|error| error.to_string())?;
        }
    } else {
        connection
            .execute(
                "DELETE FROM actress_tags WHERE actress_id = ?1",
                params![record_id],
            )
            .map_err(|error| error.to_string())?;
        for tag_id in unique_ids {
            connection
                .execute(
                    "INSERT OR IGNORE INTO actress_tags (actress_id, tag_id) VALUES (?1, ?2)",
                    params![record_id, tag_id],
                )
                .map_err(|error| error.to_string())?;
        }
    }

    list_record_tags(db_path, account_id, &scope, record_id)
}

fn auto_tag_names(
    connection: &Connection,
    account_id: &str,
    video_id: &str,
) -> Result<Vec<String>, String> {
    let video: (String, Option<String>) = connection
        .query_row(
            "SELECT work_type, release_date
             FROM videos
             WHERE id = ?1 AND account_id = ?2",
            params![video_id, account_id],
            |row| Ok((row.get(0)?, row.get(1)?)),
        )
        .map_err(|error| error.to_string())?;

    if video.0 != "single" {
        return Ok(Vec::new());
    }

    let actress_row = connection.query_row(
        "SELECT a.birthday, a.cup_size
         FROM video_actresses va
         JOIN actresses a ON a.id = va.actress_id
         WHERE va.video_id = ?1 AND a.account_id = ?2
         ORDER BY va.created_at ASC
         LIMIT 1",
        params![video_id, account_id],
        |row| {
            Ok((
                row.get::<_, Option<String>>(0)?,
                row.get::<_, Option<String>>(1)?,
            ))
        },
    );
    let (birthday, cup_size) = match actress_row {
        Ok(row) => row,
        Err(SqlError::QueryReturnedNoRows) => return Ok(Vec::new()),
        Err(error) => return Err(error.to_string()),
    };

    let mut tags = Vec::new();
    if let (Some(release_date), Some(birthday)) = (video.1.as_deref(), birthday.as_deref()) {
        if let Some(age) = age_at_release(birthday, release_date) {
            tags.push(format!("#{age}岁"));
        }
    }
    if let Some(cup_size) = cup_size.and_then(|value| normalize_cup_size(&value)) {
        tags.push(format!("#{cup_size}Cup"));
    }

    Ok(tags)
}

fn ensure_tag(
    connection: &Connection,
    account_id: &str,
    scope: &str,
    canonical_name: &str,
) -> Result<TagRecord, String> {
    match connection.query_row(
        "SELECT id,
                account_id,
                scope,
                canonical_name,
                aliases,
                related_tags,
                is_preset,
                created_at,
                updated_at
         FROM tag_library
         WHERE account_id = ?1 AND scope = ?2 AND canonical_name = ?3",
        params![account_id, scope, canonical_name],
        map_tag_row,
    ) {
        Ok(tag) => Ok(tag),
        Err(SqlError::QueryReturnedNoRows) => {
            let tag_id = Uuid::new_v4().to_string();
            connection
                .execute(
                    "INSERT INTO tag_library (
                        id,
                        account_id,
                        scope,
                        canonical_name,
                        is_preset,
                        updated_at
                    ) VALUES (?1, ?2, ?3, ?4, 0, CURRENT_TIMESTAMP)",
                    params![tag_id, account_id, scope, canonical_name],
                )
                .map_err(|error| error.to_string())?;
            get_tag(connection, account_id, &tag_id)
        }
        Err(error) => Err(error.to_string()),
    }
}

fn get_tag(connection: &Connection, account_id: &str, tag_id: &str) -> Result<TagRecord, String> {
    let row = connection.query_row(
        "SELECT id,
                account_id,
                scope,
                canonical_name,
                aliases,
                related_tags,
                is_preset,
                created_at,
                updated_at
         FROM tag_library
         WHERE id = ?1 AND account_id = ?2",
        params![tag_id, account_id],
        map_tag_row,
    );

    match row {
        Ok(record) => Ok(record),
        Err(SqlError::QueryReturnedNoRows) => Err("Tag was not found.".to_string()),
        Err(error) => Err(error.to_string()),
    }
}

fn ensure_video_exists(
    connection: &Connection,
    account_id: &str,
    video_id: &str,
) -> Result<(), String> {
    let count: i64 = connection
        .query_row(
            "SELECT COUNT(*) FROM videos WHERE id = ?1 AND account_id = ?2",
            params![video_id, account_id],
            |row| row.get(0),
        )
        .map_err(|error| error.to_string())?;
    if count == 0 {
        return Err("Video record was not found.".to_string());
    }

    Ok(())
}

fn ensure_actress_exists(
    connection: &Connection,
    account_id: &str,
    actress_id: &str,
) -> Result<(), String> {
    let count: i64 = connection
        .query_row(
            "SELECT COUNT(*) FROM actresses WHERE id = ?1 AND account_id = ?2",
            params![actress_id, account_id],
            |row| row.get(0),
        )
        .map_err(|error| error.to_string())?;
    if count == 0 {
        return Err("Actress record was not found.".to_string());
    }

    Ok(())
}

fn normalize_tag_input(input: TagInput) -> Result<TagInput, String> {
    let scope = normalize_scope(&input.scope)?;
    let canonical_name = normalize_tag_name(&input.canonical_name)?;
    validate_tag_content(&canonical_name)?;
    let aliases = normalize_tag_list(input.aliases)?;
    let related_tags = normalize_tag_list(input.related_tags)?;

    Ok(TagInput {
        scope,
        canonical_name,
        aliases,
        related_tags,
    })
}

fn normalize_scope(value: &str) -> Result<String, String> {
    match value.trim() {
        "video" => Ok("video".to_string()),
        "actress" => Ok("actress".to_string()),
        _ => Err("Tag scope must be video or actress.".to_string()),
    }
}

fn normalize_tag_name(value: &str) -> Result<String, String> {
    let trimmed = value.trim();
    if trimmed.is_empty() {
        return Err("Tag name is required.".to_string());
    }

    let without_hash = trimmed.trim_start_matches('#').trim();
    if without_hash.is_empty() {
        return Err("Tag name is required.".to_string());
    }

    Ok(format!("#{without_hash}"))
}

fn normalize_tag_list(value: Option<String>) -> Result<Option<String>, String> {
    let Some(value) = value else {
        return Ok(None);
    };

    let tags = value
        .split(|character: char| matches!(character, ',' | '，' | '、' | ';' | '；' | '\n' | '\r'))
        .map(str::trim)
        .filter(|tag| !tag.is_empty())
        .map(normalize_tag_name)
        .collect::<Result<Vec<_>, _>>()?;

    for tag in &tags {
        validate_tag_content(tag)?;
    }

    if tags.is_empty() {
        Ok(None)
    } else {
        Ok(Some(tags.join(",")))
    }
}

fn validate_tag_content(tag: &str) -> Result<(), String> {
    if tag.eq_ignore_ascii_case("#NTR") {
        return Ok(());
    }

    let name = tag.trim_start_matches('#');
    if contains_minor_related_word(name) {
        return Err("Minor-related tags are not allowed.".to_string());
    }
    if !name.chars().any(is_cjk) {
        return Err("Tags should use Chinese, except #NTR.".to_string());
    }

    Ok(())
}

fn contains_minor_related_word(value: &str) -> bool {
    ["未成年", "未成年人", "幼女", "萝莉", "儿童", "学生", "少女"]
        .iter()
        .any(|word| value.contains(word))
}

fn is_cjk(character: char) -> bool {
    ('\u{4e00}'..='\u{9fff}').contains(&character)
}

fn tag_lookup_names(tag: &TagRecord) -> Vec<String> {
    std::iter::once(tag.canonical_name.clone())
        .chain(split_tag_csv(tag.aliases.as_deref()))
        .chain(split_tag_csv(tag.related_tags.as_deref()))
        .collect()
}

fn split_tag_csv(value: Option<&str>) -> Vec<String> {
    value
        .unwrap_or_default()
        .split(',')
        .map(str::trim)
        .filter(|tag| !tag.is_empty())
        .map(str::to_string)
        .collect()
}

fn normalize_tag_lookup(value: &str) -> String {
    value
        .trim()
        .trim_start_matches('#')
        .chars()
        .filter(|character| !character.is_whitespace())
        .flat_map(|character| character.to_lowercase())
        .collect()
}

fn normalize_cup_size(value: &str) -> Option<String> {
    let normalized = value.trim().trim_start_matches('#').trim_end_matches("Cup");
    let upper = normalized.to_ascii_uppercase();
    match upper.as_str() {
        "A" | "B" | "C" | "D" | "E" | "F" | "G" | "H" | "K" => Some(upper),
        _ => None,
    }
}

fn age_at_release(birthday: &str, release_date: &str) -> Option<i64> {
    let birth = parse_ymd(birthday)?;
    let release = parse_ymd(release_date)?;
    if release.0 < birth.0 {
        return None;
    }

    let mut age = release.0 - birth.0;
    if (release.1, release.2) < (birth.1, birth.2) {
        age -= 1;
    }

    (age >= 0).then_some(age)
}

fn parse_ymd(value: &str) -> Option<(i64, i64, i64)> {
    let mut parts = value.split('-');
    let year = parts.next()?.parse().ok()?;
    let month = parts.next()?.parse().ok()?;
    let day = parts.next()?.parse().ok()?;
    if parts.next().is_some() || !(1..=12).contains(&month) || !(1..=31).contains(&day) {
        return None;
    }

    Some((year, month, day))
}

fn open_connection(db_path: &PathBuf) -> Result<Connection, String> {
    let connection = Connection::open(db_path).map_err(|error| error.to_string())?;
    connection
        .pragma_update(None, "foreign_keys", "ON")
        .map_err(|error| error.to_string())?;
    Ok(connection)
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

fn map_tag_row(row: &rusqlite::Row<'_>) -> rusqlite::Result<TagRecord> {
    let is_preset: i64 = row.get(6)?;
    Ok(TagRecord {
        id: row.get(0)?,
        account_id: row.get(1)?,
        scope: row.get(2)?,
        canonical_name: row.get(3)?,
        aliases: row.get(4)?,
        related_tags: row.get(5)?,
        is_preset: is_preset == 1,
        created_at: row.get(7)?,
        updated_at: row.get(8)?,
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
    use crate::{auth::register_account, library};
    use std::{fs, time::SystemTime};

    fn test_db_path(label: &str) -> PathBuf {
        let nanos = SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .expect("system time is valid")
            .as_nanos();
        std::env::temp_dir().join(format!("nvy-{label}-{}-{nanos}.sqlite", std::process::id()))
    }

    fn actress_input(name: &str) -> library::ActressInput {
        library::ActressInput {
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

    fn video_input(code: &str) -> library::VideoInput {
        library::VideoInput {
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

    #[test]
    fn manages_tags_and_match_aliases() {
        let db_path = test_db_path("tag-crud");
        let session = register_account(&db_path, "TagCrud", "abc123").unwrap();

        let tag = create_tag(
            &db_path,
            &session.account_id,
            TagInput {
                scope: "video".to_string(),
                canonical_name: "媚药".to_string(),
                aliases: Some("媚藥".to_string()),
                related_tags: Some("春药".to_string()),
            },
        )
        .expect("tag creates");

        assert_eq!(tag.canonical_name, "#媚药");
        assert_eq!(
            match_tags(&db_path, &session.account_id, "video", "#媚藥")
                .unwrap()
                .len(),
            1
        );
        assert_eq!(
            match_tags(&db_path, &session.account_id, "video", "#春药")
                .unwrap()
                .len(),
            1
        );

        let updated = update_tag(
            &db_path,
            &session.account_id,
            &tag.id,
            TagInput {
                scope: "video".to_string(),
                canonical_name: "#新标签".to_string(),
                aliases: None,
                related_tags: Some("#精油".to_string()),
            },
        )
        .expect("tag updates");
        assert_eq!(updated.related_tags.as_deref(), Some("#精油"));

        delete_tag(&db_path, &session.account_id, &tag.id).expect("tag deletes");
        assert!(!list_tags(&db_path, &session.account_id, "video")
            .unwrap()
            .iter()
            .any(|tag| tag.canonical_name == "#新标签"));

        let _ = fs::remove_file(db_path);
    }

    #[test]
    fn rejects_invalid_tags() {
        let db_path = test_db_path("tag-invalid");
        let session = register_account(&db_path, "BadTag", "abc123").unwrap();

        assert!(list_tags(&db_path, &session.account_id, "video")
            .unwrap()
            .iter()
            .any(|tag| tag.canonical_name == "#NTR"));
        assert!(create_tag(
            &db_path,
            &session.account_id,
            TagInput {
                scope: "video".to_string(),
                canonical_name: "#ABC".to_string(),
                aliases: None,
                related_tags: None,
            },
        )
        .is_err());
        assert!(create_tag(
            &db_path,
            &session.account_id,
            TagInput {
                scope: "video".to_string(),
                canonical_name: "#未成年".to_string(),
                aliases: None,
                related_tags: None,
            },
        )
        .is_err());

        let _ = fs::remove_file(db_path);
    }

    #[test]
    fn assigns_record_tags_and_auto_tags_single_work() {
        let db_path = test_db_path("record-tags");
        let session = register_account(&db_path, "RecordTag", "abc123").unwrap();
        let actress = library::create_actress(
            &db_path,
            &session.account_id,
            library::ActressInput {
                birthday: Some("2000-01-10".to_string()),
                cup_size: Some("F".to_string()),
                ..actress_input("Alice")
            },
        )
        .unwrap();
        let video = library::create_video(
            &db_path,
            &session.account_id,
            library::VideoInput {
                release_date: Some("2025-01-09".to_string()),
                ..video_input("AAA-001")
            },
        )
        .unwrap();
        library::set_video_actresses(&db_path, &session.account_id, &video.id, vec![actress.id])
            .unwrap();

        let manual = list_tags(&db_path, &session.account_id, "video")
            .unwrap()
            .into_iter()
            .find(|tag| tag.canonical_name == "#按摩")
            .expect("preset tag exists");
        assert_eq!(
            set_video_tags(&db_path, &session.account_id, &video.id, vec![manual.id])
                .unwrap()
                .len(),
            1
        );

        let tags = auto_tag_video(&db_path, &session.account_id, &video.id).unwrap();
        let names = tags
            .iter()
            .map(|tag| tag.canonical_name.as_str())
            .collect::<Vec<_>>();
        assert!(names.contains(&"#24岁"));
        assert!(names.contains(&"#FCup"));

        let _ = fs::remove_file(db_path);
    }
}
