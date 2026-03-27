use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};

use crate::asset_job::{AssetJobFailure, AssetJobSnapshot, AssetJobStatus, AssetJobRunSuccess};

#[derive(Debug, Clone)]
pub struct LocalApiJobRecord {
    pub job_id: String,
    pub status: AssetJobStatus,
    pub updated_at: String,
    pub snapshot: AssetJobSnapshot,
    pub result_summary: Option<String>,
    pub error: Option<String>,
}

#[derive(Debug, Clone, Default)]
pub struct LocalApiJobStore {
    jobs: Arc<Mutex<HashMap<String, LocalApiJobRecord>>>,
}

impl LocalApiJobStore {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn queue(&self, snapshot: AssetJobSnapshot) {
        let record = LocalApiJobRecord {
            job_id: snapshot.job_id.clone(),
            status: snapshot.status,
            updated_at: rehearsal_timestamp(),
            snapshot,
            result_summary: None,
            error: None,
        };
        self.jobs.lock().unwrap().insert(record.job_id.clone(), record);
    }

    pub fn store_success(&self, success: &AssetJobRunSuccess) {
        let mut jobs = self.jobs.lock().unwrap();
        if let Some(record) = jobs.get_mut(&success.job_id) {
            record.status = AssetJobStatus::Success;
            record.updated_at = rehearsal_timestamp();
            record.result_summary = Some(format!(
                "asset_dir={}, transcript_txt={}",
                success.save_dir,
                success.transcript_txt_path.clone().unwrap_or_default()
            ));
            record.error = None;
        }
    }

    pub fn store_failure(&self, failure: &AssetJobFailure) {
        let mut jobs = self.jobs.lock().unwrap();
        if let Some(record) = jobs.get_mut(&failure.job_id) {
            record.status = AssetJobStatus::Error;
            record.updated_at = rehearsal_timestamp();
            record.error = Some(failure.message.clone());
            record.result_summary = None;
        }
    }

    pub fn get(&self, job_id: &str) -> Option<LocalApiJobRecord> {
        self.jobs.lock().unwrap().get(job_id).cloned()
    }
}

fn rehearsal_timestamp() -> String {
    let millis = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|duration| duration.as_millis())
        .unwrap_or_default();
    millis.to_string()
}

