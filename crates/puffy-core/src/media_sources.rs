const SUPPORTED_SITE_SLUGS: &[&str] = &[
    "youtube",
    "tiktok",
    "douyin",
    "bilibili",
    "x",
    "xiaohongshu",
    "kuaishou",
    "instagram",
];

pub fn supported_site_slugs() -> &'static [&'static str] {
    SUPPORTED_SITE_SLUGS
}

pub fn extract_host(url: &str) -> Option<String> {
    let trimmed = url.trim();
    let without_scheme = trimmed
        .split_once("://")
        .map(|(_, rest)| rest)
        .unwrap_or(trimmed);
    let host = without_scheme.split('/').next()?.trim();
    let host = host.trim_start_matches("www.");
    if host.is_empty() {
        None
    } else {
        Some(host.to_ascii_lowercase())
    }
}

pub fn is_supported_source_url(url: &str) -> bool {
    let Some(host) = extract_host(url) else {
        return false;
    };

    host.ends_with("youtube.com")
        || host == "youtu.be"
        || host.ends_with("tiktok.com")
        || host.ends_with("douyin.com")
        || host.ends_with("bilibili.com")
        || host == "b23.tv"
        || host.ends_with("twitter.com")
        || host.ends_with("x.com")
        || host.ends_with("xiaohongshu.com")
        || host == "xhslink.com"
        || host.ends_with("kuaishou.com")
        || host.ends_with("gifshow.com")
        || host.ends_with("instagram.com")
}

pub fn normalize_supported_media_url(url: &str) -> Result<String, String> {
    let trimmed = url.trim();
    if trimmed.is_empty() {
        return Err("Missing media URL.".to_string());
    }

    let url = if trimmed.contains("://") {
        trimmed.to_string()
    } else {
        format!("https://{trimmed}")
    };

    let normalized = normalize_host_aliases(&url);
    if !is_supported_source_url(&normalized) {
        return Err(format!("Unsupported media URL: {normalized}"));
    }

    Ok(normalized)
}

fn normalize_host_aliases(url: &str) -> String {
    let mut output = url.to_string();
    output = output.replace("https://x.com/", "https://twitter.com/");
    output = output.replace("http://x.com/", "http://twitter.com/");
    output = output.replace("https://www.x.com/", "https://www.twitter.com/");
    output = output.replace("http://www.x.com/", "http://www.twitter.com/");
    output
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn normalizes_x_to_twitter() {
        let url = normalize_supported_media_url("https://x.com/openai/status/1").unwrap();
        assert!(url.contains("twitter.com"));
    }

    #[test]
    fn rejects_blank_url() {
        assert!(normalize_supported_media_url(" ").is_err());
    }
}

