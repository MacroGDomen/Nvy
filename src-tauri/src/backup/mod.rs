use argon2::Argon2;
use rand_core::{OsRng, RngCore};
use rusqlite::{params, Connection};
use serde::{Deserialize, Serialize};
use std::{
    collections::{BTreeMap, HashMap},
    fs,
    path::{Component, Path, PathBuf},
    process::Command,
};

const PLAIN_MAGIC: &[u8; 8] = b"NVYZIP1\0";
const ENCRYPTED_MAGIC: &[u8; 8] = b"NVYENC1\0";
const EXPORT_VERSION: u32 = 1;
const SCHEMA_VERSION: i64 = 2;
const KEY_SIZE: usize = 32;

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct BackupResult {
    pub path: String,
    pub format: String,
    pub entry_count: usize,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct ImportResult {
    pub account_id: String,
    pub video_count: usize,
    pub actress_count: usize,
    pub tag_count: usize,
    pub asset_count: usize,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
struct ExportManifest {
    app_name: String,
    export_version: u32,
    schema_version: i64,
    exported_at: String,
    source_account_id: String,
    source_username: Option<String>,
    encrypted: bool,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
struct ExportData {
    account: ExportAccount,
    account_settings: Option<AccountSettingsExport>,
    videos: Vec<VideoExport>,
    actresses: Vec<ActressExport>,
    tags: Vec<TagExport>,
    video_actresses: Vec<VideoActressExport>,
    video_tags: Vec<VideoTagExport>,
    actress_tags: Vec<ActressTagExport>,
    metadata_candidates: Vec<MetadataCandidateExport>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
struct ExportAccount {
    id: String,
    username: String,
    created_at: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
struct AccountSettingsExport {
    llm_api_type: Option<String>,
    llm_base_url: Option<String>,
    llm_provider_name: Option<String>,
    llm_model: Option<String>,
    llm_temperature: Option<f64>,
    llm_max_tokens: Option<i64>,
    llm_recommendation_reference_limit: i64,
    llm_translation_prompt: Option<String>,
    llm_recommendation_prompt: Option<String>,
    enable_llm_translation: i64,
    llm_recommendation_default_enabled: i64,
    #[serde(default)]
    metadata_allow_browser_cookies: i64,
    created_at: String,
    updated_at: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
struct VideoExport {
    id: String,
    code: String,
    title: Option<String>,
    original_title: Option<String>,
    cover_path: Option<String>,
    actress_names: Option<String>,
    release_date: Option<String>,
    duration_minutes: Option<i64>,
    source_url: Option<String>,
    metadata_source: Option<String>,
    summary: Option<String>,
    original_summary: Option<String>,
    translation_status: Option<String>,
    translated_at: Option<String>,
    work_type: String,
    review: Option<String>,
    review_created_at: Option<String>,
    review_updated_at: Option<String>,
    created_at: String,
    updated_at: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
struct ActressExport {
    id: String,
    name: String,
    simplified_chinese_name: Option<String>,
    former_chinese_names: Option<String>,
    traditional_chinese_name: Option<String>,
    japanese_name: Option<String>,
    romanized_name: Option<String>,
    default_display_name_type: Option<String>,
    avatar_path: Option<String>,
    measurements: Option<String>,
    cup_size: Option<String>,
    birthday: Option<String>,
    height_cm: Option<i64>,
    debut_date: Option<String>,
    wikipedia_zh_url: Option<String>,
    note: Option<String>,
    metadata_source: Option<String>,
    created_at: String,
    updated_at: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
struct TagExport {
    id: String,
    scope: String,
    canonical_name: String,
    aliases: Option<String>,
    related_tags: Option<String>,
    is_preset: i64,
    created_at: String,
    updated_at: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
struct VideoActressExport {
    video_id: String,
    actress_id: String,
    created_at: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
struct VideoTagExport {
    video_id: String,
    tag_id: String,
    created_at: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
struct ActressTagExport {
    actress_id: String,
    tag_id: String,
    created_at: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
struct MetadataCandidateExport {
    id: String,
    target_type: String,
    query: String,
    source: String,
    payload_json: String,
    created_at: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
struct Checksums {
    algorithm: String,
    entries: BTreeMap<String, String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct ArchiveEntry {
    path: String,
    bytes: Vec<u8>,
}

pub fn export_plain_backup(
    db_path: &PathBuf,
    app_data_dir: &Path,
    account_id: &str,
    output_path: &Path,
) -> Result<BackupResult, String> {
    let archive = build_backup_archive(db_path, app_data_dir, account_id, false)?;
    write_parent_dir(output_path)?;
    fs::write(output_path, archive).map_err(|error| error.to_string())?;

    Ok(BackupResult {
        path: path_to_string(output_path),
        format: "nvyzip".to_string(),
        entry_count: parse_archive(&fs::read(output_path).map_err(|error| error.to_string())?)?
            .len(),
    })
}

pub fn export_encrypted_backup(
    db_path: &PathBuf,
    app_data_dir: &Path,
    account_id: &str,
    output_path: &Path,
    password: &str,
) -> Result<BackupResult, String> {
    validate_backup_password(password)?;
    let archive = build_backup_archive(db_path, app_data_dir, account_id, true)?;
    let encrypted = encrypt_archive(&archive, password)?;
    write_parent_dir(output_path)?;
    fs::write(output_path, encrypted).map_err(|error| error.to_string())?;

    Ok(BackupResult {
        path: path_to_string(output_path),
        format: "nvyenc".to_string(),
        entry_count: parse_archive(&archive)?.len(),
    })
}

pub fn import_plain_backup(
    db_path: &PathBuf,
    app_data_dir: &Path,
    target_account_id: &str,
    input_path: &Path,
) -> Result<ImportResult, String> {
    let bytes = fs::read(input_path).map_err(|error| error.to_string())?;
    import_archive_bytes(db_path, app_data_dir, target_account_id, &bytes)
}

pub fn import_encrypted_backup(
    db_path: &PathBuf,
    app_data_dir: &Path,
    target_account_id: &str,
    input_path: &Path,
    password: &str,
) -> Result<ImportResult, String> {
    validate_backup_password(password)?;
    let bytes = fs::read(input_path).map_err(|error| error.to_string())?;
    let archive = decrypt_archive(&bytes, password)
        .map_err(|_| "Encrypted backup password is incorrect or file is damaged.".to_string())?;
    import_archive_bytes(db_path, app_data_dir, target_account_id, &archive)
}

fn build_backup_archive(
    db_path: &PathBuf,
    app_data_dir: &Path,
    account_id: &str,
    encrypted: bool,
) -> Result<Vec<u8>, String> {
    crate::db::initialize_database(db_path).map_err(|error| error.to_string())?;
    let connection = open_connection(db_path)?;
    let data = export_data(&connection, account_id)?;
    let manifest = ExportManifest {
        app_name: "Nvy".to_string(),
        export_version: EXPORT_VERSION,
        schema_version: SCHEMA_VERSION,
        exported_at: current_timestamp(&connection)?,
        source_account_id: data.account.id.clone(),
        source_username: Some(data.account.username.clone()),
        encrypted,
    };

    let mut entries = vec![
        ArchiveEntry {
            path: "manifest.json".to_string(),
            bytes: serde_json::to_vec_pretty(&manifest).map_err(|error| error.to_string())?,
        },
        ArchiveEntry {
            path: "data.json".to_string(),
            bytes: serde_json::to_vec_pretty(&data).map_err(|error| error.to_string())?,
        },
    ];
    entries.extend(collect_asset_entries(app_data_dir, &data)?);

    let checksums = build_checksums(&entries);
    entries.push(ArchiveEntry {
        path: "checksums.json".to_string(),
        bytes: serde_json::to_vec_pretty(&checksums).map_err(|error| error.to_string())?,
    });

    archive_entries_to_zip(&entries)
}

fn import_archive_bytes(
    db_path: &PathBuf,
    app_data_dir: &Path,
    target_account_id: &str,
    bytes: &[u8],
) -> Result<ImportResult, String> {
    crate::db::initialize_database(db_path).map_err(|error| error.to_string())?;
    let entries = parse_archive(bytes)?;
    validate_archive_entries(&entries)?;
    let data: ExportData = read_json_entry(&entries, "data.json")?;
    let asset_count = entries
        .iter()
        .filter(|entry| entry.path.starts_with("assets/"))
        .count();

    let staging_dir = app_data_dir.join(format!("import-staging-{}", uuid::Uuid::new_v4()));
    stage_assets(&staging_dir, &entries)?;
    restore_database(db_path, target_account_id, &data)?;
    install_staged_assets(app_data_dir, &staging_dir)?;
    let _ = fs::remove_dir_all(&staging_dir);

    Ok(ImportResult {
        account_id: target_account_id.to_string(),
        video_count: data.videos.len(),
        actress_count: data.actresses.len(),
        tag_count: data.tags.len(),
        asset_count,
    })
}

fn export_data(connection: &Connection, account_id: &str) -> Result<ExportData, String> {
    let account = connection
        .query_row(
            "SELECT id, username, created_at FROM accounts WHERE id = ?1",
            params![account_id],
            |row| {
                Ok(ExportAccount {
                    id: row.get(0)?,
                    username: row.get(1)?,
                    created_at: row.get(2)?,
                })
            },
        )
        .map_err(|error| error.to_string())?;

    Ok(ExportData {
        account_settings: read_account_settings(connection, account_id)?,
        videos: read_videos(connection, account_id)?,
        actresses: read_actresses(connection, account_id)?,
        tags: read_tags(connection, account_id)?,
        video_actresses: read_video_actresses(connection, account_id)?,
        video_tags: read_video_tags(connection, account_id)?,
        actress_tags: read_actress_tags(connection, account_id)?,
        metadata_candidates: read_metadata_candidates(connection, account_id)?,
        account,
    })
}

fn read_account_settings(
    connection: &Connection,
    account_id: &str,
) -> Result<Option<AccountSettingsExport>, String> {
    let row = connection.query_row(
        "SELECT llm_api_type,
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
                metadata_allow_browser_cookies,
                created_at,
                updated_at
         FROM account_settings
         WHERE account_id = ?1",
        params![account_id],
        |row| {
            Ok(AccountSettingsExport {
                llm_api_type: row.get(0)?,
                llm_base_url: row.get(1)?,
                llm_provider_name: row.get(2)?,
                llm_model: row.get(3)?,
                llm_temperature: row.get(4)?,
                llm_max_tokens: row.get(5)?,
                llm_recommendation_reference_limit: row.get(6)?,
                llm_translation_prompt: row.get(7)?,
                llm_recommendation_prompt: row.get(8)?,
                enable_llm_translation: row.get(9)?,
                llm_recommendation_default_enabled: row.get(10)?,
                metadata_allow_browser_cookies: row.get(11)?,
                created_at: row.get(12)?,
                updated_at: row.get(13)?,
            })
        },
    );

    match row {
        Ok(settings) => Ok(Some(settings)),
        Err(rusqlite::Error::QueryReturnedNoRows) => Ok(None),
        Err(error) => Err(error.to_string()),
    }
}

fn read_videos(connection: &Connection, account_id: &str) -> Result<Vec<VideoExport>, String> {
    let mut statement = connection
        .prepare(
            "SELECT id,
                    code,
                    title,
                    original_title,
                    cover_path,
                    actress_names,
                    release_date,
                    duration_minutes,
                    source_url,
                    metadata_source,
                    summary,
                    original_summary,
                    translation_status,
                    translated_at,
                    work_type,
                    review,
                    review_created_at,
                    review_updated_at,
                    created_at,
                    updated_at
             FROM videos
             WHERE account_id = ?1
             ORDER BY created_at ASC",
        )
        .map_err(|error| error.to_string())?;
    let rows = statement
        .query_map(params![account_id], |row| {
            Ok(VideoExport {
                id: row.get(0)?,
                code: row.get(1)?,
                title: row.get(2)?,
                original_title: row.get(3)?,
                cover_path: row.get(4)?,
                actress_names: row.get(5)?,
                release_date: row.get(6)?,
                duration_minutes: row.get(7)?,
                source_url: row.get(8)?,
                metadata_source: row.get(9)?,
                summary: row.get(10)?,
                original_summary: row.get(11)?,
                translation_status: row.get(12)?,
                translated_at: row.get(13)?,
                work_type: row.get(14)?,
                review: row.get(15)?,
                review_created_at: row.get(16)?,
                review_updated_at: row.get(17)?,
                created_at: row.get(18)?,
                updated_at: row.get(19)?,
            })
        })
        .map_err(|error| error.to_string())?;

    collect_rows(rows)
}

fn read_actresses(connection: &Connection, account_id: &str) -> Result<Vec<ActressExport>, String> {
    let mut statement = connection
        .prepare(
            "SELECT id,
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
                    metadata_source,
                    created_at,
                    updated_at
             FROM actresses
             WHERE account_id = ?1
             ORDER BY created_at ASC",
        )
        .map_err(|error| error.to_string())?;
    let rows = statement
        .query_map(params![account_id], |row| {
            Ok(ActressExport {
                id: row.get(0)?,
                name: row.get(1)?,
                simplified_chinese_name: row.get(2)?,
                former_chinese_names: row.get(3)?,
                traditional_chinese_name: row.get(4)?,
                japanese_name: row.get(5)?,
                romanized_name: row.get(6)?,
                default_display_name_type: row.get(7)?,
                avatar_path: row.get(8)?,
                measurements: row.get(9)?,
                cup_size: row.get(10)?,
                birthday: row.get(11)?,
                height_cm: row.get(12)?,
                debut_date: row.get(13)?,
                wikipedia_zh_url: row.get(14)?,
                note: row.get(15)?,
                metadata_source: row.get(16)?,
                created_at: row.get(17)?,
                updated_at: row.get(18)?,
            })
        })
        .map_err(|error| error.to_string())?;

    collect_rows(rows)
}

fn read_tags(connection: &Connection, account_id: &str) -> Result<Vec<TagExport>, String> {
    let mut statement = connection
        .prepare(
            "SELECT id,
                    scope,
                    canonical_name,
                    aliases,
                    related_tags,
                    is_preset,
                    created_at,
                    updated_at
             FROM tag_library
             WHERE account_id = ?1
             ORDER BY created_at ASC",
        )
        .map_err(|error| error.to_string())?;
    let rows = statement
        .query_map(params![account_id], |row| {
            Ok(TagExport {
                id: row.get(0)?,
                scope: row.get(1)?,
                canonical_name: row.get(2)?,
                aliases: row.get(3)?,
                related_tags: row.get(4)?,
                is_preset: row.get(5)?,
                created_at: row.get(6)?,
                updated_at: row.get(7)?,
            })
        })
        .map_err(|error| error.to_string())?;

    collect_rows(rows)
}

fn read_video_actresses(
    connection: &Connection,
    account_id: &str,
) -> Result<Vec<VideoActressExport>, String> {
    let mut statement = connection
        .prepare(
            "SELECT va.video_id, va.actress_id, va.created_at
             FROM video_actresses va
             JOIN videos v ON v.id = va.video_id
             WHERE v.account_id = ?1",
        )
        .map_err(|error| error.to_string())?;
    let rows = statement
        .query_map(params![account_id], |row| {
            Ok(VideoActressExport {
                video_id: row.get(0)?,
                actress_id: row.get(1)?,
                created_at: row.get(2)?,
            })
        })
        .map_err(|error| error.to_string())?;

    collect_rows(rows)
}

fn read_video_tags(
    connection: &Connection,
    account_id: &str,
) -> Result<Vec<VideoTagExport>, String> {
    let mut statement = connection
        .prepare(
            "SELECT vt.video_id, vt.tag_id, vt.created_at
             FROM video_tags vt
             JOIN videos v ON v.id = vt.video_id
             WHERE v.account_id = ?1",
        )
        .map_err(|error| error.to_string())?;
    let rows = statement
        .query_map(params![account_id], |row| {
            Ok(VideoTagExport {
                video_id: row.get(0)?,
                tag_id: row.get(1)?,
                created_at: row.get(2)?,
            })
        })
        .map_err(|error| error.to_string())?;

    collect_rows(rows)
}

fn read_actress_tags(
    connection: &Connection,
    account_id: &str,
) -> Result<Vec<ActressTagExport>, String> {
    let mut statement = connection
        .prepare(
            "SELECT at.actress_id, at.tag_id, at.created_at
             FROM actress_tags at
             JOIN actresses a ON a.id = at.actress_id
             WHERE a.account_id = ?1",
        )
        .map_err(|error| error.to_string())?;
    let rows = statement
        .query_map(params![account_id], |row| {
            Ok(ActressTagExport {
                actress_id: row.get(0)?,
                tag_id: row.get(1)?,
                created_at: row.get(2)?,
            })
        })
        .map_err(|error| error.to_string())?;

    collect_rows(rows)
}

fn read_metadata_candidates(
    connection: &Connection,
    account_id: &str,
) -> Result<Vec<MetadataCandidateExport>, String> {
    let mut statement = connection
        .prepare(
            "SELECT id, target_type, query, source, payload_json, created_at
             FROM metadata_candidates
             WHERE account_id = ?1",
        )
        .map_err(|error| error.to_string())?;
    let rows = statement
        .query_map(params![account_id], |row| {
            Ok(MetadataCandidateExport {
                id: row.get(0)?,
                target_type: row.get(1)?,
                query: row.get(2)?,
                source: row.get(3)?,
                payload_json: row.get(4)?,
                created_at: row.get(5)?,
            })
        })
        .map_err(|error| error.to_string())?;

    collect_rows(rows)
}

fn collect_asset_entries(
    app_data_dir: &Path,
    data: &ExportData,
) -> Result<Vec<ArchiveEntry>, String> {
    let mut paths = Vec::new();
    for video in &data.videos {
        if let Some(path) = &video.cover_path {
            paths.push(path.clone());
        }
    }
    for actress in &data.actresses {
        if let Some(path) = &actress.avatar_path {
            paths.push(path.clone());
        }
    }
    paths.sort();
    paths.dedup();

    let mut entries = Vec::new();
    for relative_path in paths {
        let relative = validate_relative_path(&relative_path)?;
        let source_path = app_data_dir.join(&relative);
        if source_path.is_file() {
            entries.push(ArchiveEntry {
                path: format!("assets/{}", path_to_slash_string(&relative)),
                bytes: fs::read(&source_path).map_err(|error| error.to_string())?,
            });
        }
    }

    Ok(entries)
}

fn restore_database(
    db_path: &PathBuf,
    target_account_id: &str,
    data: &ExportData,
) -> Result<(), String> {
    let mut connection = open_connection(db_path)?;
    ensure_account_exists(&connection, target_account_id)?;
    let transaction = connection
        .transaction()
        .map_err(|error| error.to_string())?;
    clear_account_data(&transaction, target_account_id)?;
    let data = remap_conflicting_ids(&transaction, data)?;
    insert_account_settings(
        &transaction,
        target_account_id,
        data.account_settings.as_ref(),
    )?;
    insert_videos(&transaction, target_account_id, &data.videos)?;
    insert_actresses(&transaction, target_account_id, &data.actresses)?;
    insert_tags(&transaction, target_account_id, &data.tags)?;
    insert_video_actresses(&transaction, &data.video_actresses)?;
    insert_video_tags(&transaction, &data.video_tags)?;
    insert_actress_tags(&transaction, &data.actress_tags)?;
    insert_metadata_candidates(&transaction, target_account_id, &data.metadata_candidates)?;
    transaction.commit().map_err(|error| error.to_string())
}

fn remap_conflicting_ids(connection: &Connection, data: &ExportData) -> Result<ExportData, String> {
    let mut data = data.clone();
    let mut video_ids = HashMap::new();
    let mut actress_ids = HashMap::new();
    let mut tag_ids = HashMap::new();

    for video in &mut data.videos {
        if id_exists(connection, "videos", &video.id)? {
            let next_id = uuid::Uuid::new_v4().to_string();
            video_ids.insert(video.id.clone(), next_id.clone());
            video.id = next_id;
        }
    }
    for actress in &mut data.actresses {
        if id_exists(connection, "actresses", &actress.id)? {
            let next_id = uuid::Uuid::new_v4().to_string();
            actress_ids.insert(actress.id.clone(), next_id.clone());
            actress.id = next_id;
        }
    }
    for tag in &mut data.tags {
        if id_exists(connection, "tag_library", &tag.id)? {
            let next_id = uuid::Uuid::new_v4().to_string();
            tag_ids.insert(tag.id.clone(), next_id.clone());
            tag.id = next_id;
        }
    }
    for candidate in &mut data.metadata_candidates {
        if id_exists(connection, "metadata_candidates", &candidate.id)? {
            candidate.id = uuid::Uuid::new_v4().to_string();
        }
    }

    for link in &mut data.video_actresses {
        if let Some(next_id) = video_ids.get(&link.video_id) {
            link.video_id = next_id.clone();
        }
        if let Some(next_id) = actress_ids.get(&link.actress_id) {
            link.actress_id = next_id.clone();
        }
    }
    for link in &mut data.video_tags {
        if let Some(next_id) = video_ids.get(&link.video_id) {
            link.video_id = next_id.clone();
        }
        if let Some(next_id) = tag_ids.get(&link.tag_id) {
            link.tag_id = next_id.clone();
        }
    }
    for link in &mut data.actress_tags {
        if let Some(next_id) = actress_ids.get(&link.actress_id) {
            link.actress_id = next_id.clone();
        }
        if let Some(next_id) = tag_ids.get(&link.tag_id) {
            link.tag_id = next_id.clone();
        }
    }

    Ok(data)
}

fn id_exists(connection: &Connection, table_name: &str, id: &str) -> Result<bool, String> {
    let sql = format!("SELECT COUNT(*) FROM {table_name} WHERE id = ?1");
    let count: i64 = connection
        .query_row(&sql, params![id], |row| row.get(0))
        .map_err(|error| error.to_string())?;

    Ok(count > 0)
}

fn clear_account_data(connection: &Connection, account_id: &str) -> Result<(), String> {
    connection
        .execute(
            "DELETE FROM video_tags
             WHERE video_id IN (SELECT id FROM videos WHERE account_id = ?1)",
            params![account_id],
        )
        .map_err(|error| error.to_string())?;
    connection
        .execute(
            "DELETE FROM actress_tags
             WHERE actress_id IN (SELECT id FROM actresses WHERE account_id = ?1)",
            params![account_id],
        )
        .map_err(|error| error.to_string())?;
    connection
        .execute(
            "DELETE FROM video_actresses
             WHERE video_id IN (SELECT id FROM videos WHERE account_id = ?1)
                OR actress_id IN (SELECT id FROM actresses WHERE account_id = ?1)",
            params![account_id],
        )
        .map_err(|error| error.to_string())?;
    for table_name in [
        "metadata_candidates",
        "videos",
        "actresses",
        "tag_library",
        "account_settings",
    ] {
        let sql = format!("DELETE FROM {table_name} WHERE account_id = ?1");
        connection
            .execute(&sql, params![account_id])
            .map_err(|error| error.to_string())?;
    }

    Ok(())
}

fn insert_account_settings(
    connection: &Connection,
    account_id: &str,
    settings: Option<&AccountSettingsExport>,
) -> Result<(), String> {
    if let Some(settings) = settings {
        connection
            .execute(
                "INSERT INTO account_settings (
                    account_id,
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
                    metadata_allow_browser_cookies,
                    created_at,
                    updated_at
                ) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12, ?13, ?14, ?15)",
                params![
                    account_id,
                    settings.llm_api_type,
                    settings.llm_base_url,
                    settings.llm_provider_name,
                    settings.llm_model,
                    settings.llm_temperature,
                    settings.llm_max_tokens,
                    settings.llm_recommendation_reference_limit,
                    settings.llm_translation_prompt,
                    settings.llm_recommendation_prompt,
                    settings.enable_llm_translation,
                    settings.llm_recommendation_default_enabled,
                    settings.metadata_allow_browser_cookies,
                    settings.created_at,
                    settings.updated_at
                ],
            )
            .map_err(|error| error.to_string())?;
    } else {
        let timestamp = current_timestamp(connection)?;
        connection
            .execute(
                "INSERT INTO account_settings (
                    account_id,
                    llm_recommendation_reference_limit,
                    llm_translation_prompt,
                    llm_recommendation_prompt,
                    enable_llm_translation,
                    llm_recommendation_default_enabled,
                    metadata_allow_browser_cookies,
                    created_at,
                    updated_at
                ) VALUES (?1, ?2, ?3, ?4, 0, 1, 0, ?5, ?5)",
                params![
                    account_id,
                    crate::defaults::DEFAULT_RECOMMENDATION_REFERENCE_LIMIT,
                    crate::defaults::DEFAULT_TRANSLATION_PROMPT,
                    crate::defaults::DEFAULT_RECOMMENDATION_PROMPT,
                    timestamp
                ],
            )
            .map_err(|error| error.to_string())?;
    }

    Ok(())
}

fn insert_videos(
    connection: &Connection,
    account_id: &str,
    videos: &[VideoExport],
) -> Result<(), String> {
    for video in videos {
        connection
            .execute(
                "INSERT INTO videos (
                    id,
                    account_id,
                    code,
                    title,
                    original_title,
                    cover_path,
                    actress_names,
                    release_date,
                    duration_minutes,
                    source_url,
                    metadata_source,
                    summary,
                    original_summary,
                    translation_status,
                    translated_at,
                    work_type,
                    review,
                    review_created_at,
                    review_updated_at,
                    created_at,
                    updated_at
                ) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12, ?13, ?14,
                    ?15, ?16, ?17, ?18, ?19, ?20, ?21)",
                params![
                    video.id,
                    account_id,
                    video.code,
                    video.title,
                    video.original_title,
                    video.cover_path,
                    video.actress_names,
                    video.release_date,
                    video.duration_minutes,
                    video.source_url,
                    video.metadata_source,
                    video.summary,
                    video.original_summary,
                    video.translation_status,
                    video.translated_at,
                    video.work_type,
                    video.review,
                    video.review_created_at,
                    video.review_updated_at,
                    video.created_at,
                    video.updated_at
                ],
            )
            .map_err(|error| error.to_string())?;
    }

    Ok(())
}

fn insert_actresses(
    connection: &Connection,
    account_id: &str,
    actresses: &[ActressExport],
) -> Result<(), String> {
    for actress in actresses {
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
                    metadata_source,
                    created_at,
                    updated_at
                ) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12,
                    ?13, ?14, ?15, ?16, ?17, ?18, ?19, ?20)",
                params![
                    actress.id,
                    account_id,
                    actress.name,
                    actress.simplified_chinese_name,
                    actress.former_chinese_names,
                    actress.traditional_chinese_name,
                    actress.japanese_name,
                    actress.romanized_name,
                    actress.default_display_name_type,
                    actress.avatar_path,
                    actress.measurements,
                    actress.cup_size,
                    actress.birthday,
                    actress.height_cm,
                    actress.debut_date,
                    actress.wikipedia_zh_url,
                    actress.note,
                    actress.metadata_source,
                    actress.created_at,
                    actress.updated_at
                ],
            )
            .map_err(|error| error.to_string())?;
    }

    Ok(())
}

fn insert_tags(
    connection: &Connection,
    account_id: &str,
    tags: &[TagExport],
) -> Result<(), String> {
    for tag in tags {
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
                    created_at,
                    updated_at
                ) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9)",
                params![
                    tag.id,
                    account_id,
                    tag.scope,
                    tag.canonical_name,
                    tag.aliases,
                    tag.related_tags,
                    tag.is_preset,
                    tag.created_at,
                    tag.updated_at
                ],
            )
            .map_err(|error| error.to_string())?;
    }

    Ok(())
}

fn insert_video_actresses(
    connection: &Connection,
    links: &[VideoActressExport],
) -> Result<(), String> {
    for link in links {
        connection
            .execute(
                "INSERT INTO video_actresses (video_id, actress_id, created_at)
                 VALUES (?1, ?2, ?3)",
                params![link.video_id, link.actress_id, link.created_at],
            )
            .map_err(|error| error.to_string())?;
    }

    Ok(())
}

fn insert_video_tags(connection: &Connection, links: &[VideoTagExport]) -> Result<(), String> {
    for link in links {
        connection
            .execute(
                "INSERT INTO video_tags (video_id, tag_id, created_at)
                 VALUES (?1, ?2, ?3)",
                params![link.video_id, link.tag_id, link.created_at],
            )
            .map_err(|error| error.to_string())?;
    }

    Ok(())
}

fn insert_actress_tags(connection: &Connection, links: &[ActressTagExport]) -> Result<(), String> {
    for link in links {
        connection
            .execute(
                "INSERT INTO actress_tags (actress_id, tag_id, created_at)
                 VALUES (?1, ?2, ?3)",
                params![link.actress_id, link.tag_id, link.created_at],
            )
            .map_err(|error| error.to_string())?;
    }

    Ok(())
}

fn insert_metadata_candidates(
    connection: &Connection,
    account_id: &str,
    candidates: &[MetadataCandidateExport],
) -> Result<(), String> {
    for candidate in candidates {
        connection
            .execute(
                "INSERT INTO metadata_candidates (
                    id,
                    account_id,
                    target_type,
                    query,
                    source,
                    payload_json,
                    created_at
                ) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)",
                params![
                    candidate.id,
                    account_id,
                    candidate.target_type,
                    candidate.query,
                    candidate.source,
                    candidate.payload_json,
                    candidate.created_at
                ],
            )
            .map_err(|error| error.to_string())?;
    }

    Ok(())
}

fn parse_archive(bytes: &[u8]) -> Result<Vec<ArchiveEntry>, String> {
    if bytes.len() >= PLAIN_MAGIC.len() && &bytes[..PLAIN_MAGIC.len()] == PLAIN_MAGIC {
        return parse_internal_archive(bytes);
    }

    zip_bytes_to_entries(bytes)
}

fn parse_internal_archive(bytes: &[u8]) -> Result<Vec<ArchiveEntry>, String> {
    let mut cursor = 0;
    if bytes.len() < PLAIN_MAGIC.len() || &bytes[..PLAIN_MAGIC.len()] != PLAIN_MAGIC {
        return Err("Backup file format is invalid.".to_string());
    }
    cursor += PLAIN_MAGIC.len();
    let count = read_u32(bytes, &mut cursor)? as usize;
    let mut entries = Vec::with_capacity(count);
    for _ in 0..count {
        let path_len = read_u16(bytes, &mut cursor)? as usize;
        let path_bytes = read_exact(bytes, &mut cursor, path_len)?;
        let path = String::from_utf8(path_bytes.to_vec()).map_err(|error| error.to_string())?;
        validate_archive_entry_path(&path)?;
        let byte_len = read_u64(bytes, &mut cursor)? as usize;
        let entry_bytes = read_exact(bytes, &mut cursor, byte_len)?.to_vec();
        entries.push(ArchiveEntry {
            path,
            bytes: entry_bytes,
        });
    }
    if cursor != bytes.len() {
        return Err("Backup file has trailing invalid data.".to_string());
    }

    Ok(entries)
}

fn archive_entries_to_zip(entries: &[ArchiveEntry]) -> Result<Vec<u8>, String> {
    let root = temp_dir("zip-build")?;
    let content_dir = root.join("content");
    let output_path = root.join("backup.zip");
    fs::create_dir_all(&content_dir).map_err(|error| error.to_string())?;

    for entry in entries {
        validate_archive_entry_path(&entry.path)?;
        let target = content_dir.join(validate_relative_path(&entry.path)?);
        if let Some(parent) = target.parent() {
            fs::create_dir_all(parent).map_err(|error| error.to_string())?;
        }
        fs::write(target, &entry.bytes).map_err(|error| error.to_string())?;
    }

    run_powershell(&format!(
        "Compress-Archive -Path '{}' -DestinationPath '{}' -Force",
        escape_powershell_path(&content_dir.join("*")),
        escape_powershell_path(&output_path)
    ))?;
    let bytes = fs::read(&output_path).map_err(|error| error.to_string())?;
    let _ = fs::remove_dir_all(root);

    Ok(bytes)
}

fn zip_bytes_to_entries(bytes: &[u8]) -> Result<Vec<ArchiveEntry>, String> {
    let root = temp_dir("zip-read")?;
    let input_path = root.join("backup.zip");
    let output_dir = root.join("expanded");
    fs::create_dir_all(&output_dir).map_err(|error| error.to_string())?;
    fs::write(&input_path, bytes).map_err(|error| error.to_string())?;
    run_powershell(&format!(
        "Expand-Archive -LiteralPath '{}' -DestinationPath '{}' -Force",
        escape_powershell_path(&input_path),
        escape_powershell_path(&output_dir)
    ))?;

    let mut entries = Vec::new();
    collect_zip_entries(&output_dir, &output_dir, &mut entries)?;
    entries.sort_by(|left, right| left.path.cmp(&right.path));
    let _ = fs::remove_dir_all(root);

    Ok(entries)
}

fn collect_zip_entries(
    root: &Path,
    current: &Path,
    entries: &mut Vec<ArchiveEntry>,
) -> Result<(), String> {
    for entry in fs::read_dir(current).map_err(|error| error.to_string())? {
        let entry = entry.map_err(|error| error.to_string())?;
        let path = entry.path();
        if path.is_dir() {
            collect_zip_entries(root, &path, entries)?;
        } else {
            let relative = path.strip_prefix(root).map_err(|error| error.to_string())?;
            let archive_path = path_to_slash_string(relative);
            validate_archive_entry_path(&archive_path)?;
            entries.push(ArchiveEntry {
                path: archive_path,
                bytes: fs::read(path).map_err(|error| error.to_string())?,
            });
        }
    }

    Ok(())
}

fn validate_archive_entries(entries: &[ArchiveEntry]) -> Result<(), String> {
    let by_path = entries
        .iter()
        .map(|entry| (entry.path.as_str(), entry))
        .collect::<HashMap<_, _>>();
    let manifest: ExportManifest = read_json_entry(entries, "manifest.json")?;
    if manifest.app_name != "Nvy" || manifest.export_version != EXPORT_VERSION {
        return Err("Backup manifest is not compatible.".to_string());
    }
    if !by_path.contains_key("data.json") || !by_path.contains_key("checksums.json") {
        return Err("Backup file is missing data or checksums.".to_string());
    }
    let checksums: Checksums = read_json_entry(entries, "checksums.json")?;
    if checksums.algorithm != "fnv1a64" {
        return Err("Backup checksum algorithm is unsupported.".to_string());
    }

    for (path, expected) in &checksums.entries {
        let entry = by_path
            .get(path.as_str())
            .ok_or_else(|| "Backup file is missing a checksummed entry.".to_string())?;
        if checksum_hex(&entry.bytes) != *expected {
            return Err("Backup checksum validation failed.".to_string());
        }
    }

    Ok(())
}

fn build_checksums(entries: &[ArchiveEntry]) -> Checksums {
    Checksums {
        algorithm: "fnv1a64".to_string(),
        entries: entries
            .iter()
            .map(|entry| (entry.path.clone(), checksum_hex(&entry.bytes)))
            .collect(),
    }
}

fn encrypt_archive(archive: &[u8], password: &str) -> Result<Vec<u8>, String> {
    let mut salt = [0_u8; 16];
    OsRng.fill_bytes(&mut salt);
    let ciphertext = xor_with_key_stream(archive, password, &salt)?;
    let mut bytes = Vec::with_capacity(ENCRYPTED_MAGIC.len() + salt.len() + ciphertext.len());
    bytes.extend_from_slice(ENCRYPTED_MAGIC);
    bytes.extend_from_slice(&salt);
    bytes.extend_from_slice(&ciphertext);
    Ok(bytes)
}

fn decrypt_archive(bytes: &[u8], password: &str) -> Result<Vec<u8>, String> {
    if bytes.len() <= ENCRYPTED_MAGIC.len() + 16
        || &bytes[..ENCRYPTED_MAGIC.len()] != ENCRYPTED_MAGIC
    {
        return Err("Encrypted backup file format is invalid.".to_string());
    }
    let salt_start = ENCRYPTED_MAGIC.len();
    let salt_end = salt_start + 16;
    let salt = &bytes[salt_start..salt_end];
    xor_with_key_stream(&bytes[salt_end..], password, salt)
}

fn xor_with_key_stream(bytes: &[u8], password: &str, salt: &[u8]) -> Result<Vec<u8>, String> {
    let mut output = Vec::with_capacity(bytes.len());
    let mut key = [0_u8; KEY_SIZE];
    Argon2::default()
        .hash_password_into(password.as_bytes(), salt, &mut key)
        .map_err(|error| error.to_string())?;
    let mut state = u64::from_le_bytes(key[0..8].try_into().expect("key chunk exists"))
        ^ u64::from_le_bytes(key[8..16].try_into().expect("key chunk exists"))
        ^ u64::from_le_bytes(key[16..24].try_into().expect("key chunk exists"))
        ^ u64::from_le_bytes(key[24..32].try_into().expect("key chunk exists"));
    for byte in bytes {
        state = state.wrapping_add(0x9e3779b97f4a7c15);
        let mut stream = state;
        stream = (stream ^ (stream >> 30)).wrapping_mul(0xbf58476d1ce4e5b9);
        stream = (stream ^ (stream >> 27)).wrapping_mul(0x94d049bb133111eb);
        stream ^= stream >> 31;
        output.push(byte ^ (stream as u8));
    }

    Ok(output)
}

fn stage_assets(staging_dir: &Path, entries: &[ArchiveEntry]) -> Result<(), String> {
    for entry in entries
        .iter()
        .filter(|entry| entry.path.starts_with("assets/"))
    {
        let relative_path = entry
            .path
            .strip_prefix("assets/")
            .ok_or_else(|| "Asset path is invalid.".to_string())?;
        let relative = validate_relative_path(relative_path)?;
        let target = staging_dir.join(relative);
        if let Some(parent) = target.parent() {
            fs::create_dir_all(parent).map_err(|error| error.to_string())?;
        }
        fs::write(target, &entry.bytes).map_err(|error| error.to_string())?;
    }

    Ok(())
}

fn install_staged_assets(app_data_dir: &Path, staging_dir: &Path) -> Result<(), String> {
    if !staging_dir.exists() {
        return Ok(());
    }
    copy_dir_contents(staging_dir, app_data_dir)
}

fn copy_dir_contents(source_dir: &Path, target_dir: &Path) -> Result<(), String> {
    for entry in fs::read_dir(source_dir).map_err(|error| error.to_string())? {
        let entry = entry.map_err(|error| error.to_string())?;
        let source_path = entry.path();
        let target_path = target_dir.join(entry.file_name());
        if source_path.is_dir() {
            fs::create_dir_all(&target_path).map_err(|error| error.to_string())?;
            copy_dir_contents(&source_path, &target_path)?;
        } else {
            if let Some(parent) = target_path.parent() {
                fs::create_dir_all(parent).map_err(|error| error.to_string())?;
            }
            fs::copy(&source_path, &target_path).map_err(|error| error.to_string())?;
        }
    }

    Ok(())
}

fn read_json_entry<T: for<'de> Deserialize<'de>>(
    entries: &[ArchiveEntry],
    path: &str,
) -> Result<T, String> {
    let entry = entries
        .iter()
        .find(|entry| entry.path == path)
        .ok_or_else(|| format!("Backup file is missing {path}."))?;
    serde_json::from_slice(&entry.bytes).map_err(|error| error.to_string())
}

fn validate_archive_entry_path(path: &str) -> Result<(), String> {
    if path.is_empty() || path.starts_with('/') || path.starts_with('\\') {
        return Err("Archive entry path must be relative.".to_string());
    }
    validate_relative_path(path).map(|_| ())
}

fn validate_relative_path(relative_path: &str) -> Result<PathBuf, String> {
    let path = PathBuf::from(relative_path);
    if path.is_absolute()
        || path.components().any(|component| {
            matches!(
                component,
                Component::ParentDir | Component::Prefix(_) | Component::RootDir
            )
        })
    {
        return Err("Backup asset path must be relative.".to_string());
    }

    Ok(path)
}

fn validate_backup_password(password: &str) -> Result<(), String> {
    if password.chars().count() < 6 {
        return Err("Backup password must be at least 6 characters.".to_string());
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

fn ensure_account_exists(connection: &Connection, account_id: &str) -> Result<(), String> {
    let count: i64 = connection
        .query_row(
            "SELECT COUNT(*) FROM accounts WHERE id = ?1",
            params![account_id],
            |row| row.get(0),
        )
        .map_err(|error| error.to_string())?;
    if count == 0 {
        return Err("Target account was not found.".to_string());
    }

    Ok(())
}

fn current_timestamp(connection: &Connection) -> Result<String, String> {
    connection
        .query_row("SELECT CURRENT_TIMESTAMP", [], |row| row.get(0))
        .map_err(|error| error.to_string())
}

fn collect_rows<T>(
    rows: rusqlite::MappedRows<'_, impl FnMut(&rusqlite::Row<'_>) -> rusqlite::Result<T>>,
) -> Result<Vec<T>, String> {
    rows.collect::<Result<Vec<_>, _>>()
        .map_err(|error| error.to_string())
}

fn read_u16(bytes: &[u8], cursor: &mut usize) -> Result<u16, String> {
    let slice = read_exact(bytes, cursor, 2)?;
    Ok(u16::from_le_bytes([slice[0], slice[1]]))
}

fn read_u32(bytes: &[u8], cursor: &mut usize) -> Result<u32, String> {
    let slice = read_exact(bytes, cursor, 4)?;
    Ok(u32::from_le_bytes([slice[0], slice[1], slice[2], slice[3]]))
}

fn read_u64(bytes: &[u8], cursor: &mut usize) -> Result<u64, String> {
    let slice = read_exact(bytes, cursor, 8)?;
    Ok(u64::from_le_bytes([
        slice[0], slice[1], slice[2], slice[3], slice[4], slice[5], slice[6], slice[7],
    ]))
}

fn read_exact<'a>(bytes: &'a [u8], cursor: &mut usize, count: usize) -> Result<&'a [u8], String> {
    if bytes.len().saturating_sub(*cursor) < count {
        return Err("Backup file ended unexpectedly.".to_string());
    }
    let start = *cursor;
    *cursor += count;
    Ok(&bytes[start..start + count])
}

fn checksum_hex(bytes: &[u8]) -> String {
    let mut hash = 0xcbf29ce484222325_u64;
    for byte in bytes {
        hash ^= u64::from(*byte);
        hash = hash.wrapping_mul(0x100000001b3);
    }

    format!("{hash:016x}")
}

fn write_parent_dir(path: &Path) -> Result<(), String> {
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent).map_err(|error| error.to_string())?;
    }

    Ok(())
}

fn temp_dir(label: &str) -> Result<PathBuf, String> {
    let path = std::env::temp_dir().join(format!("nvy-{label}-{}", uuid::Uuid::new_v4()));
    fs::create_dir_all(&path).map_err(|error| error.to_string())?;

    Ok(path)
}

fn run_powershell(command: &str) -> Result<(), String> {
    let output = Command::new("powershell.exe")
        .args(["-NoProfile", "-Command", command])
        .output()
        .map_err(|error| error.to_string())?;
    if !output.status.success() {
        let message = String::from_utf8_lossy(&output.stderr).trim().to_string();
        return Err(if message.is_empty() {
            "PowerShell archive command failed.".to_string()
        } else {
            message
        });
    }

    Ok(())
}

fn escape_powershell_path(path: &Path) -> String {
    path.to_string_lossy().replace('\'', "''")
}

fn path_to_string(path: &Path) -> String {
    path.to_string_lossy().to_string()
}

fn path_to_slash_string(path: &Path) -> String {
    path.components()
        .map(|component| component.as_os_str().to_string_lossy())
        .collect::<Vec<_>>()
        .join("/")
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        auth::register_account,
        library::{self, ActressInput, VideoInput},
        tags::{self, TagInput},
    };
    use std::time::SystemTime;

    fn test_path(label: &str) -> PathBuf {
        let nanos = SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .expect("system time is valid")
            .as_nanos();
        std::env::temp_dir().join(format!("nvy-backup-{label}-{}-{nanos}", std::process::id()))
    }

    fn video_input(cover_path: Option<String>) -> VideoInput {
        VideoInput {
            code: "ABC-001".to_string(),
            title: Some("Title".to_string()),
            cover_path,
            release_date: Some("2024-01-02".to_string()),
            duration_minutes: Some(120),
            source_url: None,
            summary: Some("Summary".to_string()),
            actor_names: None,
            work_type: "single".to_string(),
            review: Some("Review".to_string()),
        }
    }

    fn actress_input(avatar_path: Option<String>) -> ActressInput {
        ActressInput {
            name: "Name".to_string(),
            simplified_chinese_name: Some("中文名".to_string()),
            former_chinese_names: None,
            traditional_chinese_name: None,
            japanese_name: None,
            romanized_name: None,
            default_display_name_type: Some("simplifiedChinese".to_string()),
            avatar_path,
            measurements: None,
            cup_size: Some("F".to_string()),
            birthday: Some("2000-01-01".to_string()),
            height_cm: Some(160),
            debut_date: None,
            wikipedia_zh_url: None,
            note: Some("Note".to_string()),
        }
    }

    #[test]
    fn exports_and_imports_plain_backup_with_assets() {
        let root = test_path("plain");
        let db_path = root.join("nvy.sqlite");
        let app_data_dir = root.join("app-data");
        let source = register_account(&db_path, "Source1", "abc123").expect("source registers");
        let target = register_account(&db_path, "Target1", "abc123").expect("target registers");
        let cover_path = "cache/covers/video.png".to_string();
        let avatar_path = "cache/actresses/actress.png".to_string();
        fs::create_dir_all(app_data_dir.join("cache/covers")).expect("cover dir creates");
        fs::create_dir_all(app_data_dir.join("cache/actresses")).expect("avatar dir creates");
        fs::write(app_data_dir.join(&cover_path), b"cover").expect("cover writes");
        fs::write(app_data_dir.join(&avatar_path), b"avatar").expect("avatar writes");

        let video =
            library::create_video(&db_path, &source.account_id, video_input(Some(cover_path)))
                .expect("video creates");
        let actress = library::create_actress(
            &db_path,
            &source.account_id,
            actress_input(Some(avatar_path)),
        )
        .expect("actress creates");
        library::set_video_actresses(
            &db_path,
            &source.account_id,
            &video.id,
            vec![actress.id.clone()],
        )
        .expect("link creates");
        let tag = tags::create_tag(
            &db_path,
            &source.account_id,
            TagInput {
                scope: "video".to_string(),
                canonical_name: "#测试".to_string(),
                aliases: None,
                related_tags: None,
            },
        )
        .expect("tag creates");
        tags::set_video_tags(&db_path, &source.account_id, &video.id, vec![tag.id])
            .expect("video tag links");

        let output_path = root.join("backup.nvyzip");
        export_plain_backup(&db_path, &app_data_dir, &source.account_id, &output_path)
            .expect("plain backup exports");
        import_plain_backup(&db_path, &app_data_dir, &target.account_id, &output_path)
            .expect("plain backup imports");

        let target_videos = library::list_videos(&db_path, &target.account_id).unwrap();
        let target_actresses = library::list_actresses(&db_path, &target.account_id).unwrap();
        let target_tags = tags::list_video_tags(&db_path, &target.account_id, &target_videos[0].id)
            .expect("target tags load");

        assert_eq!(target_videos.len(), 1);
        assert_eq!(target_actresses.len(), 1);
        assert_eq!(target_tags.len(), 1);
        assert!(app_data_dir.join("cache/covers/video.png").is_file());
        assert!(app_data_dir.join("cache/actresses/actress.png").is_file());

        let _ = fs::remove_dir_all(root);
    }

    #[test]
    fn encrypted_backup_rejects_wrong_password_without_changing_data() {
        let root = test_path("encrypted");
        let db_path = root.join("nvy.sqlite");
        let app_data_dir = root.join("app-data");
        let source = register_account(&db_path, "Source2", "abc123").expect("source registers");
        let target = register_account(&db_path, "Target2", "abc123").expect("target registers");
        library::create_video(&db_path, &source.account_id, video_input(None))
            .expect("source video creates");
        library::create_video(&db_path, &target.account_id, video_input(None))
            .expect("target video creates");

        let output_path = root.join("backup.nvyenc");
        export_encrypted_backup(
            &db_path,
            &app_data_dir,
            &source.account_id,
            &output_path,
            "secret1",
        )
        .expect("encrypted backup exports");

        assert!(import_encrypted_backup(
            &db_path,
            &app_data_dir,
            &target.account_id,
            &output_path,
            "wrong-password",
        )
        .is_err());
        assert_eq!(
            library::list_videos(&db_path, &target.account_id)
                .expect("target videos load")
                .len(),
            1
        );

        import_encrypted_backup(
            &db_path,
            &app_data_dir,
            &target.account_id,
            &output_path,
            "secret1",
        )
        .expect("encrypted backup imports");
        assert_eq!(
            library::list_videos(&db_path, &target.account_id)
                .expect("target videos load")
                .len(),
            1
        );

        let _ = fs::remove_dir_all(root);
    }

    #[test]
    fn checksum_failure_rejects_import() {
        let root = test_path("checksum");
        let db_path = root.join("nvy.sqlite");
        let app_data_dir = root.join("app-data");
        let source = register_account(&db_path, "Source3", "abc123").expect("source registers");
        let target = register_account(&db_path, "Target3", "abc123").expect("target registers");
        library::create_video(&db_path, &source.account_id, video_input(None))
            .expect("source video creates");
        let output_path = root.join("backup.nvyzip");
        export_plain_backup(&db_path, &app_data_dir, &source.account_id, &output_path)
            .expect("plain backup exports");

        let mut bytes = fs::read(&output_path).expect("backup reads");
        bytes.truncate(bytes.len() / 2);
        let bad_path = root.join("bad.nvyzip");
        fs::write(&bad_path, bytes).expect("bad backup writes");

        assert!(
            import_plain_backup(&db_path, &app_data_dir, &target.account_id, &bad_path).is_err()
        );
        assert!(library::list_videos(&db_path, &target.account_id)
            .expect("target videos load")
            .is_empty());

        let _ = fs::remove_dir_all(root);
    }
}
