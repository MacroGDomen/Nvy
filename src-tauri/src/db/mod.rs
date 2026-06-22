use rusqlite::{params, Connection};
use std::{fs, path::PathBuf};
use tauri::{AppHandle, Manager};

const LATEST_SCHEMA_VERSION: i64 = 1;
const MIGRATIONS: &[(i64, &str)] = &[(1, include_str!("../../migrations/001_initial_schema.sql"))];

pub fn database_path(app: &AppHandle) -> Result<PathBuf, Box<dyn std::error::Error>> {
    let data_dir = app.path().app_data_dir()?;
    fs::create_dir_all(&data_dir)?;
    Ok(data_dir.join("nvy.sqlite"))
}

pub fn initialize_database(path: &PathBuf) -> Result<i64, Box<dyn std::error::Error>> {
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)?;
    }

    let mut connection = Connection::open(path)?;
    connection.pragma_update(None, "foreign_keys", "ON")?;
    connection.execute(
        "CREATE TABLE IF NOT EXISTS schema_migrations (
            version INTEGER PRIMARY KEY,
            applied_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP
        )",
        [],
    )?;

    for (version, sql) in MIGRATIONS {
        if !is_migration_applied(&connection, *version)? {
            let transaction = connection.transaction()?;
            transaction.execute_batch(sql)?;
            transaction.execute(
                "INSERT INTO schema_migrations (version) VALUES (?1)",
                params![version],
            )?;
            transaction.commit()?;
        }
    }

    Ok(LATEST_SCHEMA_VERSION)
}

fn is_migration_applied(
    connection: &Connection,
    version: i64,
) -> Result<bool, Box<dyn std::error::Error>> {
    let count: i64 = connection.query_row(
        "SELECT COUNT(*) FROM schema_migrations WHERE version = ?1",
        params![version],
        |row| row.get(0),
    )?;

    Ok(count > 0)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn initializes_database_only_once() {
        let db_path =
            std::env::temp_dir().join(format!("nvy-schema-test-{}.sqlite", std::process::id()));

        let first_version = initialize_database(&db_path).expect("first initialization succeeds");
        let second_version = initialize_database(&db_path).expect("second initialization succeeds");

        assert_eq!(first_version, LATEST_SCHEMA_VERSION);
        assert_eq!(second_version, LATEST_SCHEMA_VERSION);

        let connection = Connection::open(&db_path).expect("database can be opened");
        let table_count: i64 = connection
            .query_row(
                "SELECT COUNT(*) FROM sqlite_master WHERE type = 'table' AND name = 'accounts'",
                [],
                |row| row.get(0),
            )
            .expect("accounts table can be queried");

        assert_eq!(table_count, 1);

        let _ = fs::remove_file(db_path);
    }
}
