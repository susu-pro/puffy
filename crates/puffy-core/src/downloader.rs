use std::path::PathBuf;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DownloadProfile {
    Auto,
    Video1080p,
    Video720p,
    AudioOnly,
}

impl DownloadProfile {
    pub fn parse(raw: Option<&str>) -> Result<Self, String> {
        let normalized = raw.map(str::trim).unwrap_or_default();
        if normalized.is_empty() || normalized.eq_ignore_ascii_case("auto") {
            return Ok(Self::Auto);
        }
        if normalized.eq_ignore_ascii_case("1080p") {
            return Ok(Self::Video1080p);
        }
        if normalized.eq_ignore_ascii_case("720p") {
            return Ok(Self::Video720p);
        }
        if matches!(normalized.to_ascii_lowercase().as_str(), "audio-only" | "audio") {
            return Ok(Self::AudioOnly);
        }
        Err(format!("Unsupported download profile: {normalized}"))
    }

    pub fn label(self) -> &'static str {
        match self {
            Self::Auto => "auto",
            Self::Video1080p => "1080p",
            Self::Video720p => "720p",
            Self::AudioOnly => "audio-only",
        }
    }
}

#[derive(Debug, Clone)]
pub struct DownloadPlan {
    pub source_url: String,
    pub save_dir: String,
    pub profile: DownloadProfile,
    pub output_template: String,
}

pub fn default_save_dir() -> String {
    let home = std::env::var("HOME")
        .ok()
        .or_else(|| std::env::var("USERPROFILE").ok())
        .unwrap_or_else(|| "/tmp".to_string());
    PathBuf::from(home)
        .join("Documents")
        .join("Puffy")
        .display()
        .to_string()
}

pub fn build_download_plan(
    source_url: &str,
    save_dir: Option<&str>,
    profile: DownloadProfile,
) -> DownloadPlan {
    DownloadPlan {
        source_url: source_url.trim().to_string(),
        save_dir: save_dir
            .map(str::trim)
            .filter(|value| !value.is_empty())
            .map(str::to_string)
            .unwrap_or_else(default_save_dir),
        profile,
        output_template: "%(title).200B [%(id)s].%(ext)s".to_string(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_audio_profile() {
        assert_eq!(DownloadProfile::parse(Some("audio")).unwrap(), DownloadProfile::AudioOnly);
    }
}

