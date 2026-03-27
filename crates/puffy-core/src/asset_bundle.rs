use std::{
    fs,
    path::{Path, PathBuf},
};

use crate::{
    asset_job::{AssetJobRunSuccess, ResultSourceKind},
    media_sources, structured_transcript,
};

const MANIFEST_FILE_NAME: &str = ".puffy-manifest.json";

#[derive(Debug, Clone)]
pub struct AssetBundle {
    pub version: u32,
    pub asset_id: String,
    pub source_url: String,
    pub platform: Option<String>,
    pub title: Option<String>,
    pub duration_ms: Option<i64>,
    pub created_at: String,
    pub status: String,
    pub cache_hit: Option<bool>,
    pub result_source: Option<ResultSourceKind>,
    pub transcript_source: Option<ResultSourceKind>,
    pub subtitle_source: Option<ResultSourceKind>,
    pub files: AssetBundleFiles,
    pub meta: AssetBundleMeta,
    pub asset_dir: PathBuf,
}

#[derive(Debug, Clone, Default)]
pub struct AssetBundleFiles {
    pub raw_video: Option<String>,
    pub transcript_txt: Option<String>,
    pub transcript_md: Option<String>,
    pub subtitle_srt: Option<String>,
    pub subtitle_vtt: Option<String>,
    pub segments_json: Option<String>,
    pub chunks_jsonl: Option<String>,
    pub chapters_json: Option<String>,
}

#[derive(Debug, Clone, Default)]
pub struct AssetBundleMeta {
    pub job_id: Option<String>,
    pub transcription_task_id: Option<String>,
    pub file_size_bytes: Option<u64>,
}

pub fn default_library_root() -> PathBuf {
    if let Ok(custom_root) = std::env::var("PUFFY_LIBRARY_ROOT") {
        let trimmed = custom_root.trim();
        if !trimmed.is_empty() {
            return PathBuf::from(trimmed);
        }
    }

    let home = std::env::var("HOME")
        .ok()
        .or_else(|| std::env::var("USERPROFILE").ok())
        .unwrap_or_else(|| "/tmp".to_string());
    PathBuf::from(home).join("Documents").join("Puffy")
}

pub fn default_job_work_dir(job_id: &str) -> PathBuf {
    default_library_root().join(sanitize_label(job_id))
}

pub fn manifest_path_for(asset_dir: &Path) -> PathBuf {
    asset_dir.join(MANIFEST_FILE_NAME)
}

pub fn preferred_asset_dir(
    root: &Path,
    title: Option<&str>,
    source_url: &str,
    asset_id: &str,
) -> PathBuf {
    root.join(build_asset_dir_label(title, source_url, asset_id))
}

pub fn infer_platform_from_source_url(source_url: &str) -> Option<String> {
    let host = media_sources::extract_host(source_url)?;
    let platform = match host.as_str() {
        value if value.ends_with("youtube.com") || value == "youtu.be" => "youtube",
        value if value.ends_with("tiktok.com") => "tiktok",
        value if value.ends_with("douyin.com") => "douyin",
        value if value.ends_with("bilibili.com") || value == "b23.tv" => "bilibili",
        value if value.ends_with("twitter.com") || value.ends_with("x.com") => "x",
        value if value.ends_with("xiaohongshu.com") || value == "xhslink.com" => "xiaohongshu",
        value if value.ends_with("kuaishou.com") || value.ends_with("gifshow.com") => "kuaishou",
        value if value.ends_with("instagram.com") => "instagram",
        _ => return Some(host),
    };
    Some(platform.to_string())
}

pub fn create_asset_bundle(
    success: &AssetJobRunSuccess,
    library_root: Option<&Path>,
) -> Result<AssetBundle, String> {
    let root = library_root
        .map(PathBuf::from)
        .unwrap_or_else(default_library_root);
    fs::create_dir_all(&root).map_err(|error| format!("create library root: {error}"))?;
    let asset_id = generate_asset_id();
    let asset_dir = preferred_asset_dir(&root, success.title.as_deref(), &success.source_url, &asset_id);
    fs::create_dir_all(&asset_dir).map_err(|error| format!("create asset dir: {error}"))?;

    let files = AssetBundleFiles {
        raw_video: Some("video.mp4".to_string()),
        transcript_txt: success.transcript_txt_path.clone(),
        transcript_md: success.transcript_md_path.clone(),
        subtitle_srt: success.subtitle_srt_path.clone(),
        subtitle_vtt: success.subtitle_vtt_path.clone(),
        segments_json: Some(structured_transcript::SEGMENTS_FILE_NAME.to_string()),
        chunks_jsonl: Some(structured_transcript::CHUNKS_FILE_NAME.to_string()),
        chapters_json: Some(structured_transcript::CHAPTERS_FILE_NAME.to_string()),
    };

    let bundle = AssetBundle {
        version: 1,
        asset_id: asset_id.clone(),
        source_url: success.source_url.clone(),
        platform: infer_platform_from_source_url(&success.source_url),
        title: success.title.clone(),
        duration_ms: None,
        created_at: rehearsal_timestamp(),
        status: "ready".to_string(),
        cache_hit: Some(success.cache_hit),
        result_source: success.result_source,
        transcript_source: success.transcript_source,
        subtitle_source: success.subtitle_source,
        files,
        meta: AssetBundleMeta {
            job_id: Some(success.job_id.clone()),
            transcription_task_id: success.transcription_task_id.clone(),
            file_size_bytes: None,
        },
        asset_dir,
    };

    write_manifest(&bundle)?;
    Ok(bundle)
}

fn write_manifest(bundle: &AssetBundle) -> Result<(), String> {
    let manifest_path = manifest_path_for(&bundle.asset_dir);
    let json = format!(
        "{{\n  \"version\": {},\n  \"assetId\": {},\n  \"sourceUrl\": {},\n  \"platform\": {},\n  \"title\": {},\n  \"status\": {},\n  \"files\": {{\n    \"rawVideo\": {},\n    \"transcriptTxt\": {},\n    \"transcriptMd\": {},\n    \"subtitleSrt\": {},\n    \"subtitleVtt\": {},\n    \"segmentsJson\": {},\n    \"chunksJsonl\": {},\n    \"chaptersJson\": {}\n  }},\n  \"meta\": {{\n    \"jobId\": {},\n    \"transcriptionTaskId\": {},\n    \"fileSizeBytes\": {}\n  }}\n}}\n",
        bundle.version,
        q(&bundle.asset_id),
        q(&bundle.source_url),
        opt_str(bundle.platform.as_deref()),
        opt_str(bundle.title.as_deref()),
        q(&bundle.status),
        opt_str(bundle.files.raw_video.as_deref()),
        opt_str(bundle.files.transcript_txt.as_deref()),
        opt_str(bundle.files.transcript_md.as_deref()),
        opt_str(bundle.files.subtitle_srt.as_deref()),
        opt_str(bundle.files.subtitle_vtt.as_deref()),
        opt_str(bundle.files.segments_json.as_deref()),
        opt_str(bundle.files.chunks_jsonl.as_deref()),
        opt_str(bundle.files.chapters_json.as_deref()),
        opt_str(bundle.meta.job_id.as_deref()),
        opt_str(bundle.meta.transcription_task_id.as_deref()),
        opt_num(bundle.meta.file_size_bytes),
    );
    fs::write(&manifest_path, json).map_err(|error| format!("write manifest: {error}"))?;
    Ok(())
}

fn generate_asset_id() -> String {
    format!("asset-{}", rehearsal_timestamp())
}

fn rehearsal_timestamp() -> String {
    let millis = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|duration| duration.as_millis())
        .unwrap_or_default();
    format!("{millis}")
}

fn sanitize_label(value: &str) -> String {
    let mut output = String::new();
    for ch in value.trim().chars() {
        if ch.is_alphanumeric() || matches!(ch, ' ' | '-' | '_' | '.' | '(' | ')') {
            output.push(ch);
        } else {
            output.push('-');
        }
    }
    let output = output.trim().trim_matches('-').to_string();
    if output.is_empty() {
        "puffy-asset".to_string()
    } else {
        output.chars().take(64).collect()
    }
}

fn build_asset_dir_label(title: Option<&str>, source_url: &str, asset_id: &str) -> String {
    if let Some(title) = title {
        let title = sanitize_label(title);
        if !title.is_empty() {
            return title;
        }
    }
    if let Some(host) = media_sources::extract_host(source_url) {
        return sanitize_label(&format!("{host}-{asset_id}"));
    }
    sanitize_label(asset_id)
}

fn q(value: &str) -> String {
    format!("\"{}\"", escape_json(value))
}

fn opt_str(value: Option<&str>) -> String {
    value.map(q).unwrap_or_else(|| "null".to_string())
}

fn opt_num(value: Option<u64>) -> String {
    value.map(|n| n.to_string()).unwrap_or_else(|| "null".to_string())
}

fn escape_json(value: &str) -> String {
    let mut output = String::new();
    for ch in value.chars() {
        match ch {
            '\\' => output.push_str("\\\\"),
            '"' => output.push_str("\\\""),
            '\n' => output.push_str("\\n"),
            '\r' => output.push_str("\\r"),
            '\t' => output.push_str("\\t"),
            other => output.push(other),
        }
    }
    output
}

