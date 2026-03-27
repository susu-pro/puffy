use std::path::{Path, PathBuf};

#[derive(Debug, Clone)]
pub struct AudioPrepToolchain {
    pub ffmpeg_path: PathBuf,
    pub ffprobe_path: PathBuf,
}

#[derive(Debug, Clone)]
pub struct AudioPrepRequest {
    pub source_path: PathBuf,
    pub output_dir: PathBuf,
    pub output_stem: Option<String>,
    pub chunk_duration_ms: i64,
    pub toolchain: AudioPrepToolchain,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AudioPrepProfile {
    Aac32k,
    Aac24k,
}

#[derive(Debug, Clone)]
pub struct AudioPrepChunk {
    pub index: usize,
    pub output_path: PathBuf,
    pub start_ms: i64,
    pub duration_ms: i64,
    pub estimated_bytes: u64,
    pub actual_bytes: Option<u64>,
}

#[derive(Debug, Clone)]
pub struct AudioPrepReport {
    pub source_path: PathBuf,
    pub prepared_audio_path: PathBuf,
    pub profile: AudioPrepProfile,
    pub total_duration_ms: i64,
    pub chunk_duration_ms: i64,
    pub target_channels: u8,
    pub target_sample_rate_hz: u32,
    pub prepared_audio_estimated_bytes: u64,
    pub prepared_audio_actual_bytes: Option<u64>,
    pub estimated_total_bytes: u64,
    pub chunks: Vec<AudioPrepChunk>,
}

const OUTPUT_EXTENSION: &str = "m4a";
const TARGET_CHANNELS: u8 = 1;
const TARGET_SAMPLE_RATE_HZ: u32 = 16_000;
const PRIMARY_BITRATE_KBPS: u32 = 32;
const FALLBACK_BITRATE_KBPS: u32 = 24;
const DEFAULT_CHUNK_DURATION_MS: i64 = 45 * 60 * 1000;

pub fn plan_audio_prep(request: &AudioPrepRequest) -> AudioPrepReport {
    let total_duration_ms = request.chunk_duration_ms.max(DEFAULT_CHUNK_DURATION_MS);
    let profile = AudioPrepProfile::Aac32k;
    let prepared_audio_path = build_prepared_audio_path(
        &request.output_dir,
        request.output_stem.as_deref().unwrap_or("prepared"),
        profile,
    );
    let chunks = vec![AudioPrepChunk {
        index: 0,
        output_path: request.output_dir.join("chunk-0000.m4a"),
        start_ms: 0,
        duration_ms: total_duration_ms,
        estimated_bytes: estimate_bytes(total_duration_ms, PRIMARY_BITRATE_KBPS),
        actual_bytes: None,
    }];

    AudioPrepReport {
        source_path: request.source_path.clone(),
        prepared_audio_path,
        profile,
        total_duration_ms,
        chunk_duration_ms: request.chunk_duration_ms.max(DEFAULT_CHUNK_DURATION_MS),
        target_channels: TARGET_CHANNELS,
        target_sample_rate_hz: TARGET_SAMPLE_RATE_HZ,
        prepared_audio_estimated_bytes: estimate_bytes(total_duration_ms, PRIMARY_BITRATE_KBPS),
        prepared_audio_actual_bytes: None,
        estimated_total_bytes: estimate_bytes(total_duration_ms, FALLBACK_BITRATE_KBPS),
        chunks,
    }
}

pub fn build_prepared_audio_path(output_dir: &Path, output_stem: &str, profile: AudioPrepProfile) -> PathBuf {
    let suffix = match profile {
        AudioPrepProfile::Aac32k => "aac-32k",
        AudioPrepProfile::Aac24k => "aac-24k",
    };
    output_dir.join(format!("{output_stem}.{suffix}.{OUTPUT_EXTENSION}"))
}

fn estimate_bytes(duration_ms: i64, bitrate_kbps: u32) -> u64 {
    let seconds = duration_ms.max(0) as u64 / 1000;
    seconds.saturating_mul(bitrate_kbps as u64).saturating_mul(125)
}

