use serde_json::Value;
use std::{
    fs,
    path::{Path, PathBuf},
    time::{Duration, SystemTime},
};

const GFRIENDS_BASE_URL: &str = "https://raw.githubusercontent.com/gfriends/gfriends/master";
#[cfg(not(test))]
const GFRIENDS_FILETREE_URL: &str =
    "https://raw.githubusercontent.com/gfriends/gfriends/master/Filetree.json";
const GFRIENDS_CACHE_MAX_AGE: Duration = Duration::from_secs(7 * 24 * 60 * 60);

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GfriendsActressMetadata {
    pub query_name: String,
    pub matched_name: String,
    pub avatar_url: String,
}

pub fn fetch_actress_metadata(
    app_data_dir: &Path,
    query: &str,
) -> Result<Option<GfriendsActressMetadata>, String> {
    let query = normalize_name(query)?;
    let filetree = read_filetree(app_data_dir)?;
    let Some((matched_name, avatar_url)) = find_avatar_url(&filetree, &query) else {
        return Ok(None);
    };

    Ok(Some(GfriendsActressMetadata {
        query_name: query,
        matched_name,
        avatar_url,
    }))
}

fn read_filetree(app_data_dir: &Path) -> Result<Value, String> {
    let cache_path = filetree_cache_path(app_data_dir);
    if is_fresh_cache(&cache_path)? {
        let text = fs::read_to_string(&cache_path).map_err(|error| error.to_string())?;
        return serde_json::from_str(&text).map_err(|error| error.to_string());
    }

    let text = download_filetree()?;
    if let Some(parent) = cache_path.parent() {
        fs::create_dir_all(parent).map_err(|error| error.to_string())?;
    }
    fs::write(&cache_path, text.as_bytes()).map_err(|error| error.to_string())?;
    serde_json::from_str(&text).map_err(|error| error.to_string())
}

#[cfg(not(test))]
fn download_filetree() -> Result<String, String> {
    use reqwest::{blocking::Client, header::USER_AGENT};

    let client = Client::builder()
        .timeout(Duration::from_secs(60))
        .build()
        .map_err(|error| error.to_string())?;
    let response = client
        .get(GFRIENDS_FILETREE_URL)
        .header(USER_AGENT, "Nvy/0.1 gfriends metadata")
        .send()
        .map_err(|error| error.to_string())?;
    if !response.status().is_success() {
        return Err(format!(
            "Gfriends filetree request failed with status {}.",
            response.status()
        ));
    }

    response.text().map_err(|error| error.to_string())
}

#[cfg(test)]
fn download_filetree() -> Result<String, String> {
    Err("Network is disabled in tests.".to_string())
}

fn find_avatar_url(filetree: &Value, query: &str) -> Option<(String, String)> {
    let normalized_query = normalize_match_key(query);
    let content = filetree.get("Content")?.as_object()?;

    for (company_name, entries) in content {
        let Some(entries) = entries.as_object() else {
            continue;
        };
        for (file_name, stored_file_name) in entries {
            let alias_name = strip_jpg_suffix(file_name);
            if normalize_match_key(&alias_name) != normalized_query {
                continue;
            }

            let stored_file_name = stored_file_name.as_str()?;
            let matched_name = strip_jpg_suffix(stored_file_name);
            let avatar_url = build_avatar_url(company_name, stored_file_name);
            return Some((matched_name, avatar_url));
        }
    }

    let loose_query = remove_parenthesized(query);
    if loose_query == query {
        return None;
    }

    find_avatar_url(filetree, &loose_query)
}

fn build_avatar_url(company_name: &str, stored_file_name: &str) -> String {
    let (file_path, query) = stored_file_name
        .split_once('?')
        .map_or((stored_file_name, ""), |(path, query)| (path, query));
    let mut url = format!(
        "{GFRIENDS_BASE_URL}/Content/{}/{}",
        percent_encode_path_segment(company_name),
        percent_encode_path_segment(file_path)
    );
    if !query.is_empty() {
        url.push('?');
        url.push_str(query);
    }
    url
}

fn filetree_cache_path(app_data_dir: &Path) -> PathBuf {
    app_data_dir.join("cache").join("gfriends-filetree.json")
}

fn is_fresh_cache(cache_path: &Path) -> Result<bool, String> {
    let metadata = match fs::metadata(cache_path) {
        Ok(metadata) => metadata,
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => return Ok(false),
        Err(error) => return Err(error.to_string()),
    };
    let modified = metadata.modified().map_err(|error| error.to_string())?;
    let age = SystemTime::now()
        .duration_since(modified)
        .unwrap_or(GFRIENDS_CACHE_MAX_AGE);

    Ok(age < GFRIENDS_CACHE_MAX_AGE)
}

fn normalize_name(value: &str) -> Result<String, String> {
    let name = value.trim();
    if name.is_empty() {
        return Err("Actress metadata query is required.".to_string());
    }
    Ok(name.to_string())
}

fn strip_jpg_suffix(value: &str) -> String {
    value
        .split_once('?')
        .map_or(value, |(path, _)| path)
        .strip_suffix(".jpg")
        .unwrap_or(value)
        .to_string()
}

fn remove_parenthesized(value: &str) -> String {
    let mut output = String::new();
    let mut depth = 0usize;
    for character in value.chars() {
        match character {
            '(' | '（' => depth += 1,
            ')' | '）' if depth > 0 => depth -= 1,
            _ if depth == 0 => output.push(character),
            _ => {}
        }
    }
    output.trim().to_string()
}

fn normalize_match_key(value: &str) -> String {
    value
        .trim()
        .to_lowercase()
        .split_whitespace()
        .collect::<Vec<_>>()
        .join(" ")
}

fn percent_encode_path_segment(value: &str) -> String {
    let mut output = String::new();
    for byte in value.as_bytes() {
        match *byte {
            b'A'..=b'Z' | b'a'..=b'z' | b'0'..=b'9' | b'-' | b'_' | b'.' | b'~' => {
                output.push(*byte as char);
            }
            _ => output.push_str(&format!("%{byte:02X}")),
        }
    }
    output
}

#[cfg(test)]
mod tests {
    use super::*;
    use uuid::Uuid;

    fn sample_filetree() -> Value {
        serde_json::json!({
            "Content": {
                "Example Maker": {
                    "妃月由衣.jpg": "妃月るい.jpg?t=1700000000",
                    "Rui Hiduki.jpg": "妃月るい.jpg?t=1700000000"
                }
            }
        })
    }

    #[test]
    fn finds_avatar_by_alias_and_builds_raw_url() {
        let (matched_name, url) = find_avatar_url(&sample_filetree(), "妃月由衣").unwrap();

        assert_eq!(matched_name, "妃月るい");
        assert_eq!(
            url,
            "https://raw.githubusercontent.com/gfriends/gfriends/master/Content/Example%20Maker/%E5%A6%83%E6%9C%88%E3%82%8B%E3%81%84.jpg?t=1700000000"
        );
    }

    #[test]
    fn removes_parenthesized_suffix_when_matching() {
        let (matched_name, _) = find_avatar_url(&sample_filetree(), "Rui Hiduki（测试）").unwrap();

        assert_eq!(matched_name, "妃月るい");
    }

    #[test]
    fn reads_cached_filetree_without_network() {
        let app_data_dir =
            std::env::temp_dir().join(format!("nvy-gfriends-cache-{}", Uuid::new_v4()));
        let cache_path = filetree_cache_path(&app_data_dir);
        fs::create_dir_all(cache_path.parent().unwrap()).unwrap();
        fs::write(&cache_path, sample_filetree().to_string()).unwrap();

        let metadata = fetch_actress_metadata(&app_data_dir, "妃月由衣")
            .unwrap()
            .unwrap();

        assert_eq!(metadata.query_name, "妃月由衣");
        assert_eq!(metadata.matched_name, "妃月るい");

        let _ = fs::remove_dir_all(app_data_dir);
    }
}
