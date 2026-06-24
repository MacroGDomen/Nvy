const JAVBUS_BASE_URL: &str = "https://www.javbus.com";
const JAVBUS_PUBLIC_COOKIE_HEADER: &str = "existmag=all; dv=1; age=1; over18=1; adult=1";

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct JavBusVideoMetadata {
    pub code: String,
    pub title: Option<String>,
    pub cover_url: Option<String>,
    pub source_url: String,
    pub summary: Option<String>,
    pub actor_names: Option<String>,
    pub release_date: Option<String>,
    pub duration_minutes: Option<i64>,
}

pub fn fetch_video_metadata(
    query: &str,
    browser_cookie_header: Option<&str>,
) -> Result<Option<JavBusVideoMetadata>, String> {
    let code = normalize_code(query)?;
    fetch_video_metadata_by_code(&code, browser_cookie_header)
}

#[cfg(not(test))]
fn fetch_video_metadata_by_code(
    code: &str,
    browser_cookie_header: Option<&str>,
) -> Result<Option<JavBusVideoMetadata>, String> {
    use reqwest::{
        blocking::Client,
        header::{COOKIE, USER_AGENT},
    };
    use std::time::Duration;

    let source_url = format!("{JAVBUS_BASE_URL}/{code}");
    let client = Client::builder()
        .timeout(Duration::from_secs(20))
        .redirect(reqwest::redirect::Policy::limited(5))
        .build()
        .map_err(|error| error.to_string())?;
    let response = client
        .get(&source_url)
        .header(USER_AGENT, "Mozilla/5.0")
        .header(COOKIE, request_cookie_header(browser_cookie_header))
        .send()
        .map_err(|error| error.to_string())?;
    if response.status().as_u16() == 404 {
        return Ok(None);
    }
    if !response.status().is_success() {
        return Err(format!(
            "JavBus request failed with status {}.",
            response.status()
        ));
    }
    let html = response.text().map_err(|error| error.to_string())?;
    parse_video_page(&html, &source_url, code)
}

#[cfg(test)]
fn fetch_video_metadata_by_code(
    _code: &str,
    _browser_cookie_header: Option<&str>,
) -> Result<Option<JavBusVideoMetadata>, String> {
    Ok(None)
}

fn parse_video_page(
    html: &str,
    source_url: &str,
    fallback_code: &str,
) -> Result<Option<JavBusVideoMetadata>, String> {
    if is_age_verification_page(html) || is_not_found_page(html) {
        return Ok(None);
    }

    let code = extract_labeled_text(html, "識別碼:")
        .or_else(|| extract_labeled_text(html, "识别码:"))
        .unwrap_or_else(|| fallback_code.to_string());
    let title = extract_page_title(html)
        .map(|title| remove_code_prefix(&title, &code))
        .filter(|title| !title.is_empty());
    let release_date =
        extract_labeled_text(html, "發行日期:").or_else(|| extract_labeled_text(html, "发行日期:"));
    let duration_minutes = extract_labeled_text(html, "長度:")
        .or_else(|| extract_labeled_text(html, "长度:"))
        .and_then(|duration| parse_duration_minutes(&duration));
    let actor_names = extract_actor_names(html);
    let cover_url = extract_big_image_url(html).map(|url| absolutize_url(&url));
    let summary = build_summary(&title, &actor_names);

    Ok(Some(JavBusVideoMetadata {
        code,
        title,
        cover_url,
        source_url: source_url.to_string(),
        summary,
        actor_names,
        release_date,
        duration_minutes,
    }))
}

pub(crate) fn request_cookie_header(browser_cookie_header: Option<&str>) -> String {
    match browser_cookie_header {
        Some(value) if !value.trim().is_empty() => {
            format!("{JAVBUS_PUBLIC_COOKIE_HEADER}; {}", value.trim())
        }
        _ => JAVBUS_PUBLIC_COOKIE_HEADER.to_string(),
    }
}

fn normalize_code(query: &str) -> Result<String, String> {
    let code = query.trim().to_ascii_uppercase();
    if code.is_empty() {
        return Err("Metadata query is required.".to_string());
    }
    Ok(code)
}

fn is_age_verification_page(html: &str) -> bool {
    html.contains("Age Verification JavBus") || html.contains("id=\"ageVerify\"")
}

fn is_not_found_page(html: &str) -> bool {
    html.contains("404 Page Not Found") || html.contains("影片不存在")
}

fn extract_page_title(html: &str) -> Option<String> {
    text_between(html, "<title>", "</title>")
        .map(|value| value.replace(" - JavBus", ""))
        .map(|value| decode_html_entities(&strip_tags(&value)))
        .map(|value| collapse_whitespace(&value))
        .filter(|value| !value.is_empty())
}

fn extract_labeled_text(html: &str, label: &str) -> Option<String> {
    let label_position = html.find(label)?;
    let after_label = &html[label_position + label.len()..];
    let end_position = after_label.find("</p>").unwrap_or(after_label.len());
    let raw = &after_label[..end_position];
    let value = collapse_whitespace(&decode_html_entities(&strip_tags(raw)));
    (!value.is_empty()).then_some(value)
}

fn extract_big_image_url(html: &str) -> Option<String> {
    let class_position = html.find("class=\"bigImage\"")?;
    let before = &html[..class_position];
    let anchor_start = before.rfind("<a")?;
    let after_anchor = &html[anchor_start..];
    extract_attribute(after_anchor, "href")
}

fn extract_actor_names(html: &str) -> Option<String> {
    let mut names = Vec::new();
    let mut remaining = html;
    while let Some(position) = remaining.find("class=\"avatar-box\"") {
        remaining = &remaining[position..];
        if let Some(name) = text_between(remaining, "<span>", "</span>") {
            let name = collapse_whitespace(&decode_html_entities(&strip_tags(&name)));
            if !name.is_empty() && !names.contains(&name) {
                names.push(name);
            }
        }
        remaining = &remaining["class=\"avatar-box\"".len()..];
    }
    (!names.is_empty()).then_some(names.join(", "))
}

fn extract_attribute(html: &str, attribute_name: &str) -> Option<String> {
    let pattern = format!("{attribute_name}=\"");
    let start = html.find(&pattern)? + pattern.len();
    let end = html[start..].find('"')? + start;
    Some(decode_html_entities(&html[start..end]))
}

fn parse_duration_minutes(value: &str) -> Option<i64> {
    let digits = value
        .chars()
        .filter(|value| value.is_ascii_digit())
        .collect::<String>();
    digits.parse().ok()
}

fn build_summary(title: &Option<String>, actor_names: &Option<String>) -> Option<String> {
    match (title, actor_names) {
        (Some(title), Some(actor_names)) => Some(format!("{title}\n{actor_names}")),
        (Some(title), None) => Some(title.clone()),
        _ => None,
    }
}

fn remove_code_prefix(title: &str, code: &str) -> String {
    title
        .strip_prefix(code)
        .unwrap_or(title)
        .trim_start_matches([' ', '-', '_'])
        .trim()
        .to_string()
}

fn absolutize_url(url: &str) -> String {
    if url.starts_with("http://") || url.starts_with("https://") {
        url.to_string()
    } else if url.starts_with('/') {
        format!("{JAVBUS_BASE_URL}{url}")
    } else {
        format!("{JAVBUS_BASE_URL}/{url}")
    }
}

fn text_between(value: &str, start: &str, end: &str) -> Option<String> {
    let start_position = value.find(start)? + start.len();
    let end_position = value[start_position..].find(end)? + start_position;
    Some(value[start_position..end_position].to_string())
}

fn strip_tags(value: &str) -> String {
    let mut output = String::new();
    let mut in_tag = false;
    for character in value.chars() {
        match character {
            '<' => in_tag = true,
            '>' => in_tag = false,
            _ if !in_tag => output.push(character),
            _ => {}
        }
    }
    output
}

fn decode_html_entities(value: &str) -> String {
    value
        .replace("&amp;", "&")
        .replace("&quot;", "\"")
        .replace("&#039;", "'")
        .replace("&apos;", "'")
        .replace("&lt;", "<")
        .replace("&gt;", ">")
        .replace("&nbsp;", " ")
}

fn collapse_whitespace(value: &str) -> String {
    value.split_whitespace().collect::<Vec<_>>().join(" ")
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE_HTML: &str = r#"
        <html>
          <head>
            <title>IPX-177 Sample Title 相沢みなみ - JavBus</title>
            <meta name="description" content="【發行日期】2018-07-14，【長度】170分鐘">
          </head>
          <body>
            <a class="bigImage" href="/pics/cover/6n54_b.jpg"><img title="Sample Title"></a>
            <div class="info">
              <p><span class="header">識別碼:</span> <span style="color:#CC0000;">IPX-177</span></p>
              <p><span class="header">發行日期:</span> 2018-07-14</p>
              <p><span class="header">長度:</span> 170分鐘</p>
            </div>
            <div id="avatar-waterfall">
              <a class="avatar-box" href="https://www.javbus.com/star/qfy">
                <div class="photo-frame"><img src="/pics/actress/qfy_a.jpg" title="相沢みなみ"></div>
                <span>相沢みなみ</span>
              </a>
            </div>
          </body>
        </html>
    "#;

    #[test]
    fn parses_video_page_fields() {
        let parsed = parse_video_page(SAMPLE_HTML, "https://www.javbus.com/IPX-177", "IPX-177")
            .unwrap()
            .unwrap();

        assert_eq!(parsed.code, "IPX-177");
        assert_eq!(parsed.title.as_deref(), Some("Sample Title 相沢みなみ"));
        assert_eq!(
            parsed.cover_url.as_deref(),
            Some("https://www.javbus.com/pics/cover/6n54_b.jpg")
        );
        assert_eq!(parsed.release_date.as_deref(), Some("2018-07-14"));
        assert_eq!(parsed.duration_minutes, Some(170));
        assert_eq!(parsed.actor_names.as_deref(), Some("相沢みなみ"));
    }

    #[test]
    fn combines_public_and_browser_cookies() {
        let header = request_cookie_header(Some("_jdb_session=abc"));

        assert!(header.contains("existmag=all"));
        assert!(header.contains("_jdb_session=abc"));
    }

    #[test]
    fn age_verification_page_returns_none() {
        let parsed = parse_video_page(
            "<title>Age Verification JavBus - JavBus</title><div id=\"ageVerify\"></div>",
            "https://www.javbus.com/IPX-177",
            "IPX-177",
        )
        .unwrap();

        assert!(parsed.is_none());
    }
}
