use std::path::{Path, PathBuf};

#[derive(Debug, Clone)]
pub struct LocalTextImportRequest {
    pub title: Option<String>,
    pub source_url: Option<String>,
    pub transcript_path: Option<String>,
    pub subtitle_path: Option<String>,
}

#[derive(Debug, Clone)]
pub struct LocalTextImportPlan {
    pub title: Option<String>,
    pub source_url: Option<String>,
    pub asset_dir: PathBuf,
    pub transcript_txt_path: Option<PathBuf>,
    pub subtitle_path: Option<PathBuf>,
}

pub fn plan_text_import(
    request: LocalTextImportRequest,
    library_root: &Path,
) -> LocalTextImportPlan {
    let asset_dir_name = request
        .title
        .as_deref()
        .map(sanitize)
        .filter(|value| !value.is_empty())
        .unwrap_or_else(|| "text-import".to_string());

    LocalTextImportPlan {
        title: request.title,
        source_url: request.source_url,
        asset_dir: library_root.join(asset_dir_name),
        transcript_txt_path: request.transcript_path.map(PathBuf::from),
        subtitle_path: request.subtitle_path.map(PathBuf::from),
    }
}

fn sanitize(value: &str) -> String {
    let mut output = String::new();
    for ch in value.chars() {
        if ch.is_alphanumeric() || matches!(ch, ' ' | '-' | '_' | '.') {
            output.push(ch);
        } else {
            output.push('-');
        }
    }
    output.trim().trim_matches('-').chars().take(48).collect()
}

