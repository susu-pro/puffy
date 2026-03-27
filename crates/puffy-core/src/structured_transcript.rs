use std::{
    fs,
    path::{Path, PathBuf},
};

pub const SEGMENTS_FILE_NAME: &str = "segments.json";
pub const CHUNKS_FILE_NAME: &str = "chunks.jsonl";
pub const CHAPTERS_FILE_NAME: &str = "chapters.json";

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StructuredSegment {
    pub segment_id: String,
    pub start_ms: i64,
    pub end_ms: i64,
    pub text: String,
    pub anchor: String,
    pub timestamp: String,
    pub chapter_id: Option<String>,
    pub word_count: usize,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StructuredChapter {
    pub chapter_id: String,
    pub title: String,
    pub start_ms: i64,
    pub end_ms: i64,
    pub start_segment_id: String,
    pub end_segment_id: String,
    pub anchor: String,
}

#[derive(Debug, Clone, Default)]
pub struct StructuredTranscriptArtifacts {
    pub segments_json: Option<String>,
    pub chunks_jsonl: Option<String>,
    pub chapters_json: Option<String>,
}

pub fn segments_path_for(asset_dir: &Path) -> PathBuf {
    asset_dir.join(SEGMENTS_FILE_NAME)
}

pub fn chunks_path_for(asset_dir: &Path) -> PathBuf {
    asset_dir.join(CHUNKS_FILE_NAME)
}

pub fn chapters_path_for(asset_dir: &Path) -> PathBuf {
    asset_dir.join(CHAPTERS_FILE_NAME)
}

pub fn build_sidecar_manifest(
    asset_dir: &Path,
    transcript_md: Option<&str>,
    subtitle_srt: Option<&str>,
    subtitle_vtt: Option<&str>,
) -> Result<StructuredTranscriptArtifacts, String> {
    let has_any = [transcript_md, subtitle_srt, subtitle_vtt]
        .iter()
        .flatten()
        .any(|value| Path::new(value).is_file());

    if !has_any {
        return Ok(StructuredTranscriptArtifacts::default());
    }

    let _ = fs::create_dir_all(asset_dir);
    Ok(StructuredTranscriptArtifacts {
        segments_json: Some(SEGMENTS_FILE_NAME.to_string()),
        chunks_jsonl: Some(CHUNKS_FILE_NAME.to_string()),
        chapters_json: Some(CHAPTERS_FILE_NAME.to_string()),
    })
}

pub fn infer_duration_ms_from_segments_file(path: &Path) -> Option<i64> {
    let raw = fs::read_to_string(path).ok()?;
    raw.lines()
        .filter_map(|line| line.split("\"end_ms\":").nth(1))
        .filter_map(|value| value.split([',', '}']).next())
        .filter_map(|value| value.trim().parse::<i64>().ok())
        .max()
}

