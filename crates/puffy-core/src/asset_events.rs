use std::sync::Arc;

use crate::asset_job::{AssetJobSnapshot, AssetJobStage, AssetJobStatus};

pub const ASSET_JOB_PROGRESS_EVENT: &str = "asset-job-progress";
pub type AssetJobProgressSink = Arc<dyn Fn(AssetJobSnapshot) + Send + Sync>;

pub struct AssetJobEmitter {
    snapshot: AssetJobSnapshot,
    progress_sink: Option<AssetJobProgressSink>,
}

impl AssetJobEmitter {
    pub fn new(request: &crate::asset_job::NormalizedAssetJobRequest) -> Self {
        Self {
            snapshot: AssetJobSnapshot::new(request),
            progress_sink: None,
        }
    }

    pub fn new_with_sink(
        request: &crate::asset_job::NormalizedAssetJobRequest,
        progress_sink: Option<AssetJobProgressSink>,
    ) -> Self {
        let emitter = Self {
            snapshot: AssetJobSnapshot::new(request),
            progress_sink,
        };
        emitter.emit();
        emitter
    }

    pub fn snapshot(&self) -> &AssetJobSnapshot {
        &self.snapshot
    }

    pub fn stage(
        &mut self,
        stage: AssetJobStage,
        message: impl Into<String>,
        percent: Option<f64>,
    ) {
        self.snapshot.mark_running(stage, message, percent);
        self.emit();
    }

    pub fn set_downloaded_file(&mut self, file_path: impl Into<String>) {
        self.snapshot.artifacts.downloaded_file_path = Some(file_path.into());
        if matches!(self.snapshot.status, AssetJobStatus::Running)
            && matches!(self.snapshot.stage, AssetJobStage::Downloading)
        {
            self.snapshot.message = "Downloaded file is ready.".to_string();
        }
        self.emit();
    }

    pub fn set_title(&mut self, title: impl Into<String>) {
        self.snapshot.title = Some(title.into());
        self.emit();
    }

    pub fn set_transcript_paths(
        &mut self,
        txt_path: impl Into<String>,
        md_path: impl Into<String>,
    ) {
        self.snapshot.artifacts.transcript_txt_path = Some(txt_path.into());
        self.snapshot.artifacts.transcript_md_path = Some(md_path.into());
        self.emit();
    }

    pub fn set_subtitle_paths(&mut self, srt_path: impl Into<String>, vtt_path: impl Into<String>) {
        self.snapshot.artifacts.subtitle_srt_path = Some(srt_path.into());
        self.snapshot.artifacts.subtitle_vtt_path = Some(vtt_path.into());
        self.emit();
    }

    pub fn mark_cache_hit(&mut self) {
        self.snapshot.cache_hit = Some(true);
        self.emit();
    }

    pub fn ready(&mut self, message: impl Into<String>) {
        self.snapshot.mark_ready(message);
        self.emit();
    }

    pub fn fail(&mut self, failed_stage: AssetJobStage, message: impl Into<String>) {
        self.snapshot.mark_error(failed_stage, message);
        self.emit();
    }

    fn emit(&self) {
        if let Some(sink) = &self.progress_sink {
            sink(self.snapshot.clone());
        }
    }
}

