use argon2::{
    password_hash::{PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
    Argon2,
};
use rand_core::OsRng;
use rusqlite::{params, Connection, Error as SqlError, ErrorCode};
use serde::Serialize;
use std::path::PathBuf;
use uuid::Uuid;

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct AccountSession {
    pub account_id: String,
    pub username: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct AccountDataSummary {
    pub account_id: String,
    pub video_count: i64,
    pub actress_count: i64,
}

pub fn register_account(
    db_path: &PathBuf,
    username: &str,
    password: &str,
) -> Result<AccountSession, String> {
    let normalized_username = username.trim();
    validate_username(normalized_username)?;
    validate_password(password)?;
    crate::db::initialize_database(db_path).map_err(|error| error.to_string())?;

    let connection = Connection::open(db_path).map_err(|error| error.to_string())?;
    connection
        .pragma_update(None, "foreign_keys", "ON")
        .map_err(|error| error.to_string())?;

    let account_id = Uuid::new_v4().to_string();
    let password_hash = hash_password(password)?;
    let insert_result = connection.execute(
        "INSERT INTO accounts (id, username, password_hash) VALUES (?1, ?2, ?3)",
        params![account_id, normalized_username, password_hash],
    );

    match insert_result {
        Ok(_) => {
            crate::defaults::initialize_account_defaults(db_path, &account_id)?;
            Ok(AccountSession {
                account_id,
                username: normalized_username.to_string(),
            })
        }
        Err(SqlError::SqliteFailure(error, _)) if error.code == ErrorCode::ConstraintViolation => {
            Err("Username already exists.".to_string())
        }
        Err(error) => Err(error.to_string()),
    }
}

pub fn login_account(
    db_path: &PathBuf,
    username: &str,
    password: &str,
) -> Result<AccountSession, String> {
    let normalized_username = username.trim();
    validate_username(normalized_username)?;
    validate_password(password)?;
    crate::db::initialize_database(db_path).map_err(|error| error.to_string())?;

    let connection = Connection::open(db_path).map_err(|error| error.to_string())?;
    connection
        .pragma_update(None, "foreign_keys", "ON")
        .map_err(|error| error.to_string())?;

    let row = connection.query_row(
        "SELECT id, username, password_hash FROM accounts WHERE username = ?1",
        params![normalized_username],
        |row| {
            Ok((
                row.get::<_, String>(0)?,
                row.get::<_, String>(1)?,
                row.get::<_, String>(2)?,
            ))
        },
    );

    let (account_id, stored_username, password_hash) = match row {
        Ok(account) => account,
        Err(SqlError::QueryReturnedNoRows) => {
            return Err("Account or password is incorrect.".to_string())
        }
        Err(error) => return Err(error.to_string()),
    };

    if !verify_password(&password_hash, password)? {
        return Err("Account or password is incorrect.".to_string());
    }

    Ok(AccountSession {
        account_id,
        username: stored_username,
    })
}

pub fn account_data_summary(
    db_path: &PathBuf,
    account_id: &str,
) -> Result<AccountDataSummary, String> {
    if account_id.trim().is_empty() {
        return Err("Account id is required.".to_string());
    }

    crate::db::initialize_database(db_path).map_err(|error| error.to_string())?;
    let connection = Connection::open(db_path).map_err(|error| error.to_string())?;
    connection
        .pragma_update(None, "foreign_keys", "ON")
        .map_err(|error| error.to_string())?;

    let video_count = count_account_rows(&connection, "videos", account_id)?;
    let actress_count = count_account_rows(&connection, "actresses", account_id)?;

    Ok(AccountDataSummary {
        account_id: account_id.to_string(),
        video_count,
        actress_count,
    })
}

fn count_account_rows(
    connection: &Connection,
    table_name: &str,
    account_id: &str,
) -> Result<i64, String> {
    let sql = format!("SELECT COUNT(*) FROM {table_name} WHERE account_id = ?1");
    connection
        .query_row(&sql, params![account_id], |row| row.get(0))
        .map_err(|error| error.to_string())
}

fn validate_username(username: &str) -> Result<(), String> {
    let length = username.chars().count();
    if !(3..=10).contains(&length) {
        return Err("Username must be 3-10 characters.".to_string());
    }

    if !username.chars().all(is_supported_username_char) {
        return Err(
            "Username can only contain Chinese characters, letters, and numbers.".to_string(),
        );
    }

    Ok(())
}

fn validate_password(password: &str) -> Result<(), String> {
    let length = password.chars().count();
    if !(3..=20).contains(&length) {
        return Err("Password must be 3-20 characters.".to_string());
    }

    if !password
        .chars()
        .all(|character| character.is_ascii_alphanumeric())
    {
        return Err("Password can only contain letters and numbers.".to_string());
    }

    Ok(())
}

fn is_supported_username_char(character: char) -> bool {
    character.is_ascii_alphanumeric() || ('\u{4e00}'..='\u{9fff}').contains(&character)
}

fn hash_password(password: &str) -> Result<String, String> {
    let salt = SaltString::generate(&mut OsRng);
    Argon2::default()
        .hash_password(password.as_bytes(), &salt)
        .map(|hash| hash.to_string())
        .map_err(|error| error.to_string())
}

fn verify_password(password_hash: &str, password: &str) -> Result<bool, String> {
    let parsed_hash = PasswordHash::new(password_hash).map_err(|error| error.to_string())?;
    Ok(Argon2::default()
        .verify_password(password.as_bytes(), &parsed_hash)
        .is_ok())
}

#[cfg(test)]
mod tests {
    use super::*;
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
    fn registers_account_with_password_hash() {
        let db_path = test_db_path("register");
        let session =
            register_account(&db_path, "User123", "pass123").expect("account can be registered");

        assert_eq!(session.username, "User123");
        assert!(!session.account_id.is_empty());

        let connection = Connection::open(&db_path).expect("database opens");
        let stored_hash: String = connection
            .query_row(
                "SELECT password_hash FROM accounts WHERE id = ?1",
                params![session.account_id],
                |row| row.get(0),
            )
            .expect("hash exists");

        assert_ne!(stored_hash, "pass123");
        assert!(stored_hash.starts_with("$argon2"));

        let _ = fs::remove_file(db_path);
    }

    #[test]
    fn rejects_invalid_account_input() {
        let db_path = test_db_path("invalid");

        assert!(register_account(&db_path, "ab", "pass123").is_err());
        assert!(register_account(&db_path, "User!", "pass123").is_err());
        assert!(register_account(&db_path, "User123", "pa").is_err());
        assert!(register_account(&db_path, "User123", "pass-123").is_err());

        let _ = fs::remove_file(db_path);
    }

    #[test]
    fn logs_in_with_correct_password_only() {
        let db_path = test_db_path("login");
        let registered =
            register_account(&db_path, "LocalUser", "abc123").expect("account registers");

        let logged_in = login_account(&db_path, "LocalUser", "abc123").expect("login succeeds");
        assert_eq!(logged_in, registered);
        assert!(login_account(&db_path, "LocalUser", "wrong123").is_err());

        let _ = fs::remove_file(db_path);
    }

    #[test]
    fn keeps_account_data_summary_isolated() {
        let db_path = test_db_path("isolation");
        let first =
            register_account(&db_path, "FirstUser", "abc123").expect("first account registers");
        let second =
            register_account(&db_path, "SecondUser", "abc123").expect("second account registers");

        let connection = Connection::open(&db_path).expect("database opens");
        connection
            .execute(
                "INSERT INTO videos (id, account_id, code, work_type) VALUES (?1, ?2, ?3, ?4)",
                params![
                    Uuid::new_v4().to_string(),
                    first.account_id,
                    "AAA-001",
                    "single"
                ],
            )
            .expect("first account video inserts");
        connection
            .execute(
                "INSERT INTO videos (id, account_id, code, work_type) VALUES (?1, ?2, ?3, ?4)",
                params![
                    Uuid::new_v4().to_string(),
                    second.account_id,
                    "BBB-001",
                    "single"
                ],
            )
            .expect("second account video inserts");
        connection
            .execute(
                "INSERT INTO actresses (id, account_id, name) VALUES (?1, ?2, ?3)",
                params![Uuid::new_v4().to_string(), first.account_id, "Alice"],
            )
            .expect("first account actress inserts");

        let first_summary =
            account_data_summary(&db_path, &first.account_id).expect("first summary exists");
        let second_summary =
            account_data_summary(&db_path, &second.account_id).expect("second summary exists");

        assert_eq!(first_summary.video_count, 1);
        assert_eq!(first_summary.actress_count, 1);
        assert_eq!(second_summary.video_count, 1);
        assert_eq!(second_summary.actress_count, 0);

        let _ = fs::remove_file(db_path);
    }
}
