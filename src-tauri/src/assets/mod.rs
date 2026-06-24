use serde::Serialize;
use std::{
    fs,
    path::{Component, Path, PathBuf},
    time::Duration,
};

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct AppDataPaths {
    pub app_data_dir: String,
    pub database_path: String,
    pub cache_dir: String,
    pub covers_dir: String,
    pub actresses_dir: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct CachedImage {
    pub relative_path: String,
    pub absolute_path: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ImageCacheKind {
    Cover,
    Actress,
}

impl ImageCacheKind {
    pub fn from_str(value: &str) -> Result<Self, String> {
        match value {
            "cover" => Ok(Self::Cover),
            "actress" => Ok(Self::Actress),
            _ => Err("Image cache kind must be cover or actress.".to_string()),
        }
    }

    fn directory_name(self) -> &'static str {
        match self {
            Self::Cover => "covers",
            Self::Actress => "actresses",
        }
    }
}

pub fn app_data_paths(app_data_dir: &Path, database_path: &Path) -> Result<AppDataPaths, String> {
    let cache_dir = app_data_dir.join("cache");
    let covers_dir = cache_dir.join("covers");
    let actresses_dir = cache_dir.join("actresses");

    fs::create_dir_all(&covers_dir).map_err(|error| error.to_string())?;
    fs::create_dir_all(&actresses_dir).map_err(|error| error.to_string())?;

    Ok(AppDataPaths {
        app_data_dir: path_to_string(app_data_dir),
        database_path: path_to_string(database_path),
        cache_dir: path_to_string(&cache_dir),
        covers_dir: path_to_string(&covers_dir),
        actresses_dir: path_to_string(&actresses_dir),
    })
}

pub fn cache_local_image(
    app_data_dir: &Path,
    source_path: &Path,
    kind: ImageCacheKind,
    owner_id: &str,
) -> Result<CachedImage, String> {
    let owner_id = normalize_owner_id(owner_id)?;
    if !source_path.is_file() {
        return Err("Source image does not exist.".to_string());
    }

    let extension = image_extension(source_path)?;
    let relative_path = PathBuf::from("cache")
        .join(kind.directory_name())
        .join(format!("{owner_id}.{extension}"));
    let absolute_path = app_data_dir.join(&relative_path);

    if let Some(parent) = absolute_path.parent() {
        fs::create_dir_all(parent).map_err(|error| error.to_string())?;
    }
    fs::copy(source_path, &absolute_path).map_err(|error| error.to_string())?;

    Ok(CachedImage {
        relative_path: path_to_slash_string(&relative_path),
        absolute_path: path_to_string(&absolute_path),
    })
}

pub fn cache_remote_image_with_headers(
    app_data_dir: &Path,
    source_url: &str,
    kind: ImageCacheKind,
    owner_id: &str,
    cookie_header: Option<&str>,
    referer: Option<&str>,
) -> Result<CachedImage, String> {
    let owner_id = normalize_owner_id(owner_id)?;
    let extension = image_extension_from_url(source_url)?;
    let client = reqwest::blocking::Client::builder()
        .timeout(Duration::from_secs(20))
        .build()
        .map_err(|error| error.to_string())?;
    let mut request = client
        .get(source_url)
        .header(reqwest::header::USER_AGENT, "Mozilla/5.0");
    if let Some(cookie_header) = cookie_header.filter(|value| !value.trim().is_empty()) {
        request = request.header(reqwest::header::COOKIE, cookie_header.trim());
    }
    if let Some(referer) = referer.filter(|value| !value.trim().is_empty()) {
        request = request.header(reqwest::header::REFERER, referer.trim());
    }
    let response = request.send().map_err(|error| error.to_string())?;
    if !response.status().is_success() {
        return Err(format!(
            "Remote image request failed with status {}.",
            response.status()
        ));
    }
    let bytes = response.bytes().map_err(|error| error.to_string())?;
    let relative_path = PathBuf::from("cache")
        .join(kind.directory_name())
        .join(format!("{owner_id}.{extension}"));
    let absolute_path = app_data_dir.join(&relative_path);

    if let Some(parent) = absolute_path.parent() {
        fs::create_dir_all(parent).map_err(|error| error.to_string())?;
    }
    fs::write(&absolute_path, bytes).map_err(|error| error.to_string())?;

    Ok(CachedImage {
        relative_path: path_to_slash_string(&relative_path),
        absolute_path: path_to_string(&absolute_path),
    })
}

pub fn resolve_cached_asset(app_data_dir: &Path, relative_path: &str) -> Result<String, String> {
    let relative_path = validate_relative_path(relative_path)?;
    let absolute_path = app_data_dir.join(relative_path);
    if !absolute_path.exists() {
        return Err("Cached asset does not exist.".to_string());
    }

    Ok(path_to_string(&absolute_path))
}

fn normalize_owner_id(owner_id: &str) -> Result<&str, String> {
    let owner_id = owner_id.trim();
    if owner_id.is_empty() {
        return Err("Owner id is required.".to_string());
    }
    if !owner_id
        .bytes()
        .all(|value| value.is_ascii_alphanumeric() || matches!(value, b'-' | b'_'))
    {
        return Err("Owner id contains invalid path characters.".to_string());
    }

    Ok(owner_id)
}

fn validate_relative_path(relative_path: &str) -> Result<PathBuf, String> {
    let path = PathBuf::from(relative_path);
    if path.is_absolute()
        || path
            .components()
            .any(|component| matches!(component, Component::ParentDir))
    {
        return Err("Cached asset path must be relative.".to_string());
    }

    Ok(path)
}

fn image_extension(source_path: &Path) -> Result<String, String> {
    let extension = source_path
        .extension()
        .and_then(|value| value.to_str())
        .map(|value| value.to_ascii_lowercase())
        .ok_or_else(|| "Source image extension is required.".to_string())?;

    match extension.as_str() {
        "jpg" | "jpeg" | "png" | "webp" | "gif" => Ok(extension),
        _ => Err("Only jpg, jpeg, png, webp, and gif images are supported.".to_string()),
    }
}

fn image_extension_from_url(source_url: &str) -> Result<String, String> {
    let clean_url = source_url
        .split(['?', '#'])
        .next()
        .unwrap_or(source_url)
        .trim();
    let extension = clean_url
        .rsplit('/')
        .next()
        .and_then(|file_name| file_name.rsplit('.').next())
        .map(|value| value.to_ascii_lowercase())
        .ok_or_else(|| "Remote image extension is required.".to_string())?;

    match extension.as_str() {
        "jpg" | "jpeg" | "png" | "webp" | "gif" => Ok(extension),
        _ => Err("Only jpg, jpeg, png, webp, and gif images are supported.".to_string()),
    }
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
    use std::time::SystemTime;

    fn test_dir(label: &str) -> PathBuf {
        let nanos = SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .expect("system time is valid")
            .as_nanos();
        std::env::temp_dir().join(format!("nvy-assets-{label}-{}-{nanos}", std::process::id()))
    }

    #[test]
    fn caches_images_with_relative_paths() {
        let base_dir = test_dir("cache");
        let source_dir = test_dir("source");
        fs::create_dir_all(&source_dir).expect("source dir creates");
        let source_path = source_dir.join("cover.PNG");
        fs::write(&source_path, b"image").expect("source image writes");

        let cached = cache_local_image(&base_dir, &source_path, ImageCacheKind::Cover, "video-1")
            .expect("image caches");

        assert_eq!(cached.relative_path, "cache/covers/video-1.png");
        assert!(PathBuf::from(&cached.absolute_path).is_file());
        assert_eq!(
            PathBuf::from(resolve_cached_asset(&base_dir, &cached.relative_path).unwrap()),
            PathBuf::from(&cached.absolute_path)
        );

        let _ = fs::remove_dir_all(base_dir);
        let _ = fs::remove_dir_all(source_dir);
    }

    #[test]
    fn rejects_invalid_cache_paths_and_extensions() {
        let base_dir = test_dir("invalid");
        let source_dir = test_dir("bad-source");
        fs::create_dir_all(&source_dir).expect("source dir creates");
        let source_path = source_dir.join("file.txt");
        fs::write(&source_path, b"text").expect("source file writes");

        assert!(cache_local_image(&base_dir, &source_path, ImageCacheKind::Cover, "x").is_err());
        let image_path = source_dir.join("avatar.png");
        fs::write(&image_path, b"image").expect("valid image writes");
        assert!(
            cache_local_image(&base_dir, &image_path, ImageCacheKind::Actress, "../x").is_err()
        );
        assert!(resolve_cached_asset(&base_dir, "../secret.png").is_err());

        let _ = fs::remove_dir_all(base_dir);
        let _ = fs::remove_dir_all(source_dir);
    }

    #[test]
    fn extracts_image_extension_from_remote_url() {
        assert_eq!(
            image_extension_from_url("https://www.javbus.com/pics/cover/6n54_b.jpg?x=1").unwrap(),
            "jpg"
        );
        assert!(image_extension_from_url("https://example.com/file.txt").is_err());
    }
}
