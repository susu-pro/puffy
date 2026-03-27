use std::time::{SystemTime, UNIX_EPOCH};

use crate::{downloader::DownloadProfile, media_sources};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AssetJobStage {
    Queued,
    Resolving,
    Downloading,
    Extracting,
    Transcribing,
    Packaging,
    Ready,
    Error,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AssetJobStatus {
    Queued,
    Running,
    Success,
    Error,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ResultSourceKind {
    OfficialSubtitles,
    LocalWhisper,
    DownloadOnly,
    Unknown,
}

#[derive(Debug, Clone, Default)]
pub struct AssetJobArtifacts {
    pub downloaded_file_path: Option<String>,
    pub transcript_txt_path: Option<String>,
    pub transcript_md_path: Option<String>,
    pub subtitle_srt_path: Option<String>,
    pub subtitle_vtt_path: Option<String>,
}

#[derive(Debug, Clone)]
pub struct AssetJobRequest {
    pub job_id: Option<String>,
    pub source_url: String,
    pub save_dir: Option<String>,
    pub download_profile: Option<String>,
    pub force_rerun: bool,
}

#[derive(Debug, Clone)]
pub struct NormalizedAssetJobRequest {
    pub job_id: String,
    pub source_url: String,
    pub save_dir: String,
    pub download_profile: DownloadProfile,
    pub force_rerun: bool,
}

#[derive(Debug, Clone)]
pub struct AssetJobSnapshot {
    pub job_id: String,
    pub status: AssetJobStatus,
    pub stage: AssetJobStage,
    pub failed_stage: Option<AssetJobStage>,
    pub download_profile: DownloadProfile,
    pub source_url: String,
    pub save_dir: String,
    pub message: String,
    pub percent: Option<f64>,
    pub title: Option<String>,
    pub downloaded: Option<String>,
    pub total: Option<String>,
    pub speed: Option<String>,
    pub eta: Option<String>,
    pub artifacts: AssetJobArtifacts,
    pub cache_hit: Option<bool>,
    pub result_source: Option<ResultSourceKind>,
    pub transcription_task_id: Option<String>,
    pub transcript_source: Option<ResultSourceKind>,
    pub subtitle_source: Option<ResultSourceKind>,
    pub error: Option<String>,
}

#[derive(Debug, Clone)]
pub struct AssetJobRunSuccess {
    pub job_id: String,
    pub source_url: String,
    pub save_dir: String,
    pub download_profile: DownloadProfile,
    pub title: Option<String>,
    pub downloaded_file_path: String,
    pub transcript_txt_path: Option<String>,
    pub transcript_md_path: Option<String>,
    pub subtitle_srt_path: Option<String>,
    pub subtitle_vtt_path: Option<String>,
    pub transcription_task_id: Option<String>,
    pub cache_hit: bool,
    pub existing_asset_id: Option<String>,
    pub result_source: Option<ResultSourceKind>,
    pub transcript_source: Option<ResultSourceKind>,
    pub subtitle_source: Option<ResultSourceKind>,
    pub download_stdout: Option<String>,
    pub download_stderr: Option<String>,
    pub transcription_stdout: Option<String>,
    pub transcription_stderr: Option<String>,
}

#[derive(Debug, Clone)]
pub struct AssetJobFailure {
    pub job_id: String,
    pub source_url: String,
    pub save_dir: String,
    pub download_profile: DownloadProfile,
    pub stage: AssetJobStage,
    pub message: String,
    pub artifacts: AssetJobArtifacts,
    pub download_stdout: Option<String>,
    pub download_stderr: Option<String>,
    pub transcription_stdout: Option<String>,
    pub transcription_stderr: Option<String>,
}

impl AssetJobRequest {
    pub fn normalize(
        self,
        default_save_dir: impl Fn(&str) -> String,
    ) -> Result<NormalizedAssetJobRequest, AssetJobFailure> {
        let job_id = trim_to_option(self.job_id.unwrap_or_else(generate_job_id))
            .unwrap_or_else(generate_job_id);
        let save_dir = trim_to_option(self.save_dir.unwrap_or_default())
            .unwrap_or_else(|| default_save_dir(&job_id));
        let source_url = trim_to_option(self.source_url).ok_or_else(|| {
            AssetJobFailure::new(
                job_id.clone(),
                String::new(),
                save_dir.clone(),
                DownloadProfile::Auto,
                AssetJobStage::Queued,
                "Missing source URL.",
            )
        })?;
        let source_url = media_sources::normalize_supported_media_url(&source_url).map_err(|message| {
            AssetJobFailure::new(
                job_id.clone(),
                source_url.clone(),
                save_dir.clone(),
                DownloadProfile::Auto,
                AssetJobStage::Queued,
                message,
            )
        })?;
        let download_profile = DownloadProfile::parse(self.download_profile.as_deref()).map_err(|message| {
            AssetJobFailure::new(
                job_id.clone(),
                source_url.clone(),
                save_dir.clone(),
                DownloadProfile::Auto,
                AssetJobStage::Queued,
                message,
            )
        })?;

        Ok(NormalizedAssetJobRequest {
            job_id,
            source_url,
            save_dir,
            download_profile,
            force_rerun: self.force_rerun,
        })
    }
}

impl AssetJobSnapshot {
    pub fn new(request: &NormalizedAssetJobRequest) -> Self {
        Self {
            job_id: request.job_id.clone(),
            status: AssetJobStatus::Queued,
            stage: AssetJobStage::Queued,
            failed_stage: None,
            download_profile: request.download_profile,
            source_url: request.source_url.clone(),
            save_dir: request.save_dir.clone(),
            message: "Asset job queued.".to_string(),
            percent: Some(0.0),
            title: None,
            downloaded: None,
            total: None,
            speed: None,
            eta: None,
            artifacts: AssetJobArtifacts::default(),
            cache_hit: None,
            result_source: None,
            transcription_task_id: None,
            transcript_source: None,
            subtitle_source: None,
            error: None,
        }
    }

    pub fn mark_running(
        &mut self,
        stage: AssetJobStage,
        message: impl Into<String>,
        percent: Option<f64>,
    ) {
        self.status = AssetJobStatus::Running;
        self.stage = stage;
        self.failed_stage = None;
        self.message = message.into();
        self.error = None;
        if let Some(next_percent) = percent {
            self.percent = Some(next_percent.clamp(0.0, 100.0));
        }
    }

    pub fn mark_ready(&mut self, message: impl Into<String>) {
        self.status = AssetJobStatus::Success;
        self.stage = AssetJobStage::Ready;
        self.failed_stage = None;
        self.message = message.into();
        self.error = None;
        self.percent = Some(100.0);
    }

    pub fn mark_error(&mut self, failed_stage: AssetJobStage, message: impl Into<String>) {
        let message = message.into();
        self.status = AssetJobStatus::Error;
        self.stage = AssetJobStage::Error;
        self.failed_stage = Some(failed_stage);
        self.message = message.clone();
        self.error = Some(message);
    }
}

impl AssetJobFailure {
    pub fn new(
        job_id: String,
        source_url: String,
        save_dir: String,
        download_profile: DownloadProfile,
        stage: AssetJobStage,
        message: impl Into<String>,
    ) -> Self {
        Self {
            job_id,
            source_url,
            save_dir,
            download_profile,
            stage,
            message: message.into(),
            artifacts: AssetJobArtifacts::default(),
            download_stdout: None,
            download_stderr: None,
            transcription_stdout: None,
            transcription_stderr: None,
        }
    }
}

pub fn generate_job_id() -> String {
    let epoch_ms = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|duration| duration.as_millis())
        .unwrap_or_default();
    format!("asset-job-{epoch_ms}")
}

pub fn trim_to_option(value: impl Into<String>) -> Option<String> {
    let trimmed = value.into().trim().to_string();
    if trimmed.is_empty() {
        None
    } else {
        Some(trimmed)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn normalize_rejects_blank_url() {
        let request = AssetJobRequest {
            job_id: None,
            source_url: " ".to_string(),
            save_dir: None,
            download_profile: None,
            force_rerun: false,
        };
        assert!(request.normalize(|_| "/tmp/puffy".to_string()).is_err());
    }
}

