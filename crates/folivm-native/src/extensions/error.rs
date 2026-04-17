use thiserror::Error;

#[derive(Debug, Error)]
pub enum ExtensionError {
    #[error("Manifest error: {0}")]
    Manifest(String),

    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("JSON error: {0}")]
    Json(#[from] serde_json::Error),

    #[error("Permission denied: {0}")]
    PermissionDenied(String),

    #[error("Invalid extension package: {0}")]
    InvalidPackage(String),

    #[error("Extension not found: {0}")]
    NotFound(String),

    #[error("Skill registration failed: {0}")]
    SkillRegistration(String),

    #[error("Hook execution failed: {0}")]
    HookFailed(String),

    #[error("Storage error: {0}")]
    Storage(String),

    #[error("Deno runtime error: {0}")]
    Deno(#[from] deno_core::anyhow::Error),
}

pub type Result<T> = std::result::Result<T, ExtensionError>;
