use std::path::{Path, PathBuf};

#[derive(Debug, Clone)]
pub struct WhisperModelSpec {
    pub name: String,
    pub source_url: String,
}

#[derive(Debug, Clone)]
pub struct TranscriptSegment {
    pub start_ms: i64,
    pub end_ms: i64,
    pub text: String,
}

#[derive(Debug, Clone)]
pub struct TranscriptOutputPaths {
    pub txt_path: String,
    pub md_path: String,
    pub srt_path: String,
    pub vtt_path: String,
}

#[derive(Debug, Clone)]
pub struct WhisperTranscriptPlan {
    pub model: WhisperModelSpec,
    pub source_path: String,
    pub output_dir: String,
    pub outputs: TranscriptOutputPaths,
}

const DEFAULT_MODEL: &str = "ggml-tiny.bin";
const HUGGINGFACE_BASE: &str = "https://huggingface.co/ggerganov/whisper.cpp/resolve/main";

pub fn default_model_name() -> &'static str {
    DEFAULT_MODEL
}

pub fn default_model_spec() -> WhisperModelSpec {
    WhisperModelSpec {
        name: default_model_name().to_string(),
        source_url: format!("{HUGGINGFACE_BASE}/{DEFAULT_MODEL}"),
    }
}

pub fn models_dir() -> PathBuf {
    let home = std::env::var("HOME")
        .ok()
        .or_else(|| std::env::var("USERPROFILE").ok())
        .unwrap_or_else(|| "/tmp".to_string());
    PathBuf::from(home).join(".puffy").join("models")
}

pub fn planned_transcription(
    source_path: &Path,
    output_dir: &Path,
    model_name: Option<&str>,
) -> WhisperTranscriptPlan {
    let model_name = model_name.unwrap_or(default_model_name());
    WhisperTranscriptPlan {
        model: WhisperModelSpec {
            name: model_name.to_string(),
            source_url: format!("{HUGGINGFACE_BASE}/{model_name}"),
        },
        source_path: source_path.display().to_string(),
        output_dir: output_dir.display().to_string(),
        outputs: TranscriptOutputPaths {
            txt_path: output_dir.join("transcript.txt").display().to_string(),
            md_path: output_dir.join("transcript.md").display().to_string(),
            srt_path: output_dir.join("subtitle.srt").display().to_string(),
            vtt_path: output_dir.join("subtitle.vtt").display().to_string(),
        },
    }
}

