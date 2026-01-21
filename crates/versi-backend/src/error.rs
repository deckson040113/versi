use thiserror::Error;

#[derive(Error, Debug, Clone)]
pub enum BackendError {
    #[error("Backend not found")]
    NotFound,

    #[error("Command failed: {stderr}")]
    CommandFailed { stderr: String },

    #[error("Failed to parse version: {0}")]
    ParseError(String),

    #[error("Installation failed: {0}")]
    InstallFailed(String),

    #[error("Network error: {0}")]
    NetworkError(String),

    #[error("Version not found: {0}")]
    VersionNotFound(String),

    #[error("IO error: {0}")]
    IoError(String),

    #[error("Operation not supported by this backend: {0}")]
    Unsupported(String),

    #[error("Backend-specific error: {0}")]
    BackendSpecific(String),

    #[error("Timeout waiting for command")]
    Timeout,
}

impl From<std::io::Error> for BackendError {
    fn from(err: std::io::Error) -> Self {
        BackendError::IoError(err.to_string())
    }
}
