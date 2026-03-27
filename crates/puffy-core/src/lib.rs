pub mod asset_bundle;
pub mod asset_events;
pub mod asset_job;
pub mod asset_pipeline;
pub mod audio_prep;
pub mod downloader;
pub mod local_api_jobs;
pub mod local_text_import;
pub mod local_whisper;
pub mod media_sources;
pub mod runtime;
pub mod structured_transcript;

pub const ENGINE_NAME: &str = "Puffy Public Engine";
pub const ENGINE_VERSION: &str = "0.1.0-rehearsal";

pub fn engine_overview() -> String {
    format!(
        "{ENGINE_NAME} {ENGINE_VERSION} | local-first media asset engine | supported: {}",
        media_sources::supported_site_slugs().join(", ")
    )
}

pub fn default_api_base() -> &'static str {
    "http://127.0.0.1:41480"
}
