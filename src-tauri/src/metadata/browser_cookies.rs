use aes_gcm::{
    aead::{Aead, KeyInit},
    Aes256Gcm, Nonce,
};
use base64::{engine::general_purpose, Engine as _};
use rusqlite::Connection;
use serde_json::Value;
use std::{
    collections::BTreeMap,
    env, fs,
    path::{Path, PathBuf},
    time::{SystemTime, UNIX_EPOCH},
};
use uuid::Uuid;

const CHROME_EPOCH_OFFSET_SECONDS: u64 = 11_644_473_600;
const TARGET_DOMAINS: &[&str] = &["javdb.com", "javlibrary.com", "javbus.com"];

#[derive(Debug)]
struct BrowserProfile {
    user_data_dir: PathBuf,
    cookies_path: PathBuf,
}

#[derive(Debug)]
struct BrowserCookie {
    host: String,
    name: String,
    value: String,
}

pub fn load_metadata_cookie_header(allow_browser_cookies: bool) -> Result<Option<String>, String> {
    if !allow_browser_cookies {
        return Ok(None);
    }

    let mut cookies = Vec::new();
    for profile in discover_browser_profiles() {
        if let Ok(mut profile_cookies) = read_profile_cookies(&profile) {
            cookies.append(&mut profile_cookies);
        }
    }

    Ok(build_cookie_header(&cookies))
}

fn discover_browser_profiles() -> Vec<BrowserProfile> {
    browser_user_data_dirs()
        .into_iter()
        .flat_map(|user_data_dir| discover_profiles_for_user_data_dir(&user_data_dir))
        .collect()
}

fn browser_user_data_dirs() -> Vec<PathBuf> {
    let Some(local_app_data) = env::var_os("LOCALAPPDATA") else {
        return Vec::new();
    };
    let local_app_data = PathBuf::from(local_app_data);

    vec![
        local_app_data
            .join("Google")
            .join("Chrome")
            .join("User Data"),
        local_app_data
            .join("Microsoft")
            .join("Edge")
            .join("User Data"),
    ]
}

fn discover_profiles_for_user_data_dir(user_data_dir: &Path) -> Vec<BrowserProfile> {
    let Ok(entries) = fs::read_dir(user_data_dir) else {
        return Vec::new();
    };

    entries
        .flatten()
        .filter_map(|entry| {
            let profile_dir = entry.path();
            let cookies_path = profile_dir.join("Network").join("Cookies");
            if cookies_path.is_file() {
                Some(BrowserProfile {
                    user_data_dir: user_data_dir.to_path_buf(),
                    cookies_path,
                })
            } else {
                None
            }
        })
        .collect()
}

fn read_profile_cookies(profile: &BrowserProfile) -> Result<Vec<BrowserCookie>, String> {
    let decryption_key = read_chromium_decryption_key(&profile.user_data_dir).ok();
    let copied_db_path = copy_cookie_database(&profile.cookies_path)?;
    let result = read_cookies_from_database(&copied_db_path, decryption_key.as_deref());
    let _ = fs::remove_file(copied_db_path);
    result
}

fn copy_cookie_database(cookies_path: &Path) -> Result<PathBuf, String> {
    let copied_db_path =
        env::temp_dir().join(format!("nvy-browser-cookies-{}.sqlite", Uuid::new_v4()));
    fs::copy(cookies_path, &copied_db_path)
        .map_err(|_| "Browser cookie database could not be copied.".to_string())?;
    Ok(copied_db_path)
}

fn read_cookies_from_database(
    copied_db_path: &Path,
    decryption_key: Option<&[u8]>,
) -> Result<Vec<BrowserCookie>, String> {
    let connection = Connection::open(copied_db_path)
        .map_err(|_| "Browser cookie database could not be opened.".to_string())?;
    let mut statement = connection
        .prepare("SELECT host_key, name, value, encrypted_value, expires_utc FROM cookies")
        .map_err(|_| "Browser cookie database schema is not supported.".to_string())?;
    let now = current_chrome_timestamp();
    let rows = statement
        .query_map([], |row| {
            Ok((
                row.get::<_, String>(0)?,
                row.get::<_, String>(1)?,
                row.get::<_, String>(2)?,
                row.get::<_, Vec<u8>>(3)?,
                row.get::<_, i64>(4)?,
            ))
        })
        .map_err(|error| error.to_string())?;

    let mut cookies = Vec::new();
    for row in rows {
        let (host, name, value, encrypted_value, expires_utc) =
            row.map_err(|error| error.to_string())?;
        if !is_target_host(&host) || is_expired_cookie(expires_utc, now) {
            continue;
        }
        let value = if value.is_empty() {
            decrypt_chromium_cookie_value(&encrypted_value, decryption_key).unwrap_or_default()
        } else {
            value
        };
        if name.is_empty() || value.is_empty() {
            continue;
        }
        cookies.push(BrowserCookie { host, name, value });
    }

    Ok(cookies)
}

fn read_chromium_decryption_key(user_data_dir: &Path) -> Result<Vec<u8>, String> {
    let local_state = fs::read_to_string(user_data_dir.join("Local State"))
        .map_err(|_| "Chromium Local State could not be read.".to_string())?;
    let local_state: Value = serde_json::from_str(&local_state)
        .map_err(|_| "Chromium Local State is invalid.".to_string())?;
    let encrypted_key = local_state
        .get("os_crypt")
        .and_then(|value| value.get("encrypted_key"))
        .and_then(Value::as_str)
        .ok_or_else(|| "Chromium cookie key is missing.".to_string())?;
    let mut encrypted_key = general_purpose::STANDARD
        .decode(encrypted_key)
        .map_err(|_| "Chromium cookie key is invalid.".to_string())?;
    if encrypted_key.starts_with(b"DPAPI") {
        encrypted_key.drain(..5);
    }

    crypt_unprotect_data(&encrypted_key)
}

fn decrypt_chromium_cookie_value(
    encrypted_value: &[u8],
    decryption_key: Option<&[u8]>,
) -> Result<String, String> {
    if encrypted_value.is_empty() {
        return Ok(String::new());
    }

    if encrypted_value.starts_with(b"v10") || encrypted_value.starts_with(b"v11") {
        let decryption_key =
            decryption_key.ok_or_else(|| "Chromium cookie key is unavailable.".to_string())?;
        if encrypted_value.len() <= 15 {
            return Err("Chromium cookie value is invalid.".to_string());
        }
        let cipher = Aes256Gcm::new_from_slice(decryption_key)
            .map_err(|_| "Chromium cookie key length is invalid.".to_string())?;
        let nonce = Nonce::from_slice(&encrypted_value[3..15]);
        let plaintext = cipher
            .decrypt(nonce, &encrypted_value[15..])
            .map_err(|_| "Chromium cookie value could not be decrypted.".to_string())?;
        return String::from_utf8(plaintext)
            .map_err(|_| "Chromium cookie value is not UTF-8.".to_string());
    }

    let plaintext = crypt_unprotect_data(encrypted_value)?;
    String::from_utf8(plaintext).map_err(|_| "Chromium cookie value is not UTF-8.".to_string())
}

fn build_cookie_header(cookies: &[BrowserCookie]) -> Option<String> {
    let mut values_by_name = BTreeMap::new();
    for cookie in cookies {
        if is_target_host(&cookie.host) && !cookie.name.is_empty() && !cookie.value.is_empty() {
            values_by_name.insert(cookie.name.clone(), cookie.value.clone());
        }
    }
    if values_by_name.is_empty() {
        return None;
    }

    Some(
        values_by_name
            .into_iter()
            .map(|(name, value)| format!("{name}={value}"))
            .collect::<Vec<_>>()
            .join("; "),
    )
}

fn is_target_host(host: &str) -> bool {
    TARGET_DOMAINS
        .iter()
        .any(|domain| host_matches_domain(host, domain))
}

fn host_matches_domain(host: &str, domain: &str) -> bool {
    let host = host.trim_start_matches('.').to_ascii_lowercase();
    let domain = domain.to_ascii_lowercase();
    host == domain || host.ends_with(&format!(".{domain}"))
}

fn is_expired_cookie(expires_utc: i64, now: i64) -> bool {
    expires_utc != 0 && expires_utc <= now
}

fn current_chrome_timestamp() -> i64 {
    let unix_seconds = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|duration| duration.as_secs())
        .unwrap_or(0);
    ((unix_seconds + CHROME_EPOCH_OFFSET_SECONDS) * 1_000_000) as i64
}

#[cfg(windows)]
fn crypt_unprotect_data(encrypted_value: &[u8]) -> Result<Vec<u8>, String> {
    use std::{ffi::c_void, ptr};
    use windows_sys::Win32::{
        Foundation::LocalFree,
        Security::Cryptography::{CryptUnprotectData, CRYPT_INTEGER_BLOB},
    };

    let mut input = CRYPT_INTEGER_BLOB {
        cbData: encrypted_value.len() as u32,
        pbData: encrypted_value.as_ptr() as *mut u8,
    };
    let mut output = CRYPT_INTEGER_BLOB {
        cbData: 0,
        pbData: ptr::null_mut(),
    };

    let success = unsafe {
        CryptUnprotectData(
            &mut input,
            ptr::null_mut(),
            ptr::null_mut(),
            ptr::null_mut::<c_void>(),
            ptr::null_mut(),
            0,
            &mut output,
        )
    };
    if success == 0 {
        return Err("Windows DPAPI could not decrypt browser cookie data.".to_string());
    }

    let decrypted =
        unsafe { std::slice::from_raw_parts(output.pbData, output.cbData as usize).to_vec() };
    unsafe {
        LocalFree(output.pbData as *mut c_void);
    }

    Ok(decrypted)
}

#[cfg(not(windows))]
fn crypt_unprotect_data(_encrypted_value: &[u8]) -> Result<Vec<u8>, String> {
    Err("Browser cookie decryption is only supported on Windows.".to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn matches_target_domains_without_matching_prefix_spoofs() {
        assert!(host_matches_domain("javdb.com", "javdb.com"));
        assert!(host_matches_domain(".javdb.com", "javdb.com"));
        assert!(host_matches_domain("www.javdb.com", "javdb.com"));
        assert!(!host_matches_domain("eviljavdb.com", "javdb.com"));
        assert!(!host_matches_domain("javdb.com.example.com", "javdb.com"));
    }

    #[test]
    fn builds_cookie_header_from_target_hosts_only() {
        let cookies = vec![
            BrowserCookie {
                host: ".javdb.com".to_string(),
                name: "_jdb_session".to_string(),
                value: "abc".to_string(),
            },
            BrowserCookie {
                host: "example.com".to_string(),
                name: "ignored".to_string(),
                value: "value".to_string(),
            },
        ];

        assert_eq!(
            build_cookie_header(&cookies).as_deref(),
            Some("_jdb_session=abc")
        );
    }

    #[test]
    fn disabled_cookie_access_returns_none() {
        assert_eq!(load_metadata_cookie_header(false).unwrap(), None);
    }
}
