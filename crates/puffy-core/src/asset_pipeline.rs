use crate::{
    asset_bundle,
    asset_job::{AssetJobRequest, AssetJobStage, NormalizedAssetJobRequest},
    downloader::{self, DownloadPlan},
    media_sources,
};

#[derive(Debug, Clone)]
pub struct ContentCandidate {
    pub title: Option<String>,
    pub video_url: Option<String>,
}

#[derive(Debug, Clone)]
pub enum AssetRouteDecision {
    MediaOnly,
    ContentOnly(ContentCandidate),
    MediaWithContentFallback(ContentCandidate),
}

#[derive(Debug, Clone)]
pub struct AssetJobPlan {
    pub route: AssetRouteDecision,
    pub stages: Vec<AssetJobStage>,
    pub download_plan: DownloadPlan,
    pub save_dir: String,
}

pub fn decide_asset_route(url: &str) -> AssetRouteDecision {
    if is_note_host(url) {
        return AssetRouteDecision::ContentOnly(ContentCandidate {
            title: None,
            video_url: None,
        });
    }

    AssetRouteDecision::MediaOnly
}

pub fn plan_asset_job(request: &AssetJobRequest) -> Result<AssetJobPlan, String> {
    let normalized = request.clone().normalize(|job_id| {
        asset_bundle::default_job_work_dir(job_id)
            .display()
            .to_string()
    }).map_err(|failure| failure.message)?;

    Ok(plan_from_normalized(normalized))
}

pub fn plan_from_normalized(request: NormalizedAssetJobRequest) -> AssetJobPlan {
    let route = decide_asset_route(&request.source_url);
    let download_plan = downloader::build_download_plan(
        &request.source_url,
        Some(&request.save_dir),
        request.download_profile,
    );

    AssetJobPlan {
        route,
        stages: vec![
            AssetJobStage::Queued,
            AssetJobStage::Resolving,
            AssetJobStage::Downloading,
            AssetJobStage::Extracting,
            AssetJobStage::Transcribing,
            AssetJobStage::Packaging,
            AssetJobStage::Ready,
        ],
        download_plan,
        save_dir: request.save_dir,
    }
}

pub fn is_note_host(url: &str) -> bool {
    let normalized = url.trim().to_ascii_lowercase();
    (normalized.contains("xiaohongshu.com/discovery/item/")
        || normalized.contains("xhslink.com/")
        || normalized.contains("douyin.com/note/"))
        && media_sources::extract_host(url).is_some()
}
