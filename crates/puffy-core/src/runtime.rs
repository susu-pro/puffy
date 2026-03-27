use std::path::PathBuf;

#[derive(Debug, Clone)]
pub struct RuntimeToolLock {
    pub tool: String,
    pub version: String,
    pub source_url: String,
    pub sha256: String,
    pub origin_type: String,
    pub macos_min_version: String,
    pub codesign_identity: String,
    pub notarized: bool,
    pub smoke_status: String,
    pub last_verified_at: Option<String>,
}

#[derive(Debug, Clone)]
pub struct RuntimeLock {
    pub schema_version: u32,
    pub status: String,
    pub tools: Vec<RuntimeToolLock>,
    pub notes: Vec<String>,
}

pub fn rehearsal_runtime_lock() -> RuntimeLock {
    RuntimeLock {
        schema_version: 1,
        status: "rehearsal".to_string(),
        tools: vec![
            blocked_tool("yt-dlp"),
            blocked_tool("ffmpeg"),
            blocked_tool("ffprobe"),
        ],
        notes: vec![
            "No bundled runtime binaries are shipped in this rehearsal tree.".to_string(),
            "Re-enter public distribution only after fixed version, sha256, signing, and smoke verification all pass.".to_string(),
        ],
    }
}

pub fn runtime_root_hint() -> PathBuf {
    PathBuf::from("runtime")
}

pub fn runtime_rules_summary() -> String {
    "runtime rules: external by default, fixed version + sha256 + signed smoke required for public distribution".to_string()
}

fn blocked_tool(tool: &str) -> RuntimeToolLock {
    RuntimeToolLock {
        tool: tool.to_string(),
        version: "unknown".to_string(),
        source_url: String::new(),
        sha256: String::new(),
        origin_type: "external".to_string(),
        macos_min_version: "pending".to_string(),
        codesign_identity: String::new(),
        notarized: false,
        smoke_status: "blocked".to_string(),
        last_verified_at: None,
    }
}

