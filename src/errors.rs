use std::{io, path::PathBuf};

/// Shared error type used by all Rust command crates during migration.
#[derive(thiserror::Error, Debug)]
pub enum CoreError {
    /// File system I/O failure.
    #[error("I/O error while accessing {0}")]
    Io(PathBuf, #[source] io::Error),

    /// A path is invalid for the current operation.
    #[error("invalid path: {0}")]
    InvalidPath(String),

    /// A required input is missing.
    #[error("missing required value: {0}")]
    MissingValue(String),

    /// An operation was rejected due to configuration/argument issues.
    #[error("invalid command input: {0}")]
    InvalidInput(String),

    /// The operation could not be completed because a pre-condition failed.
    #[error("pre-condition failed: {0}")]
    PreconditionFailed(String),

    /// A conflict prevented the operation from proceeding.
    #[error("resource conflict: {0}")]
    Conflict(String),

    /// Platform-specific behavior not available in this environment.
    #[error("unsupported platform behavior: {0}")]
    UnsupportedPlatform(String),
}

impl CoreError {
    pub fn invalid_input(message: impl Into<String>) -> Self {
        Self::InvalidInput(message.into())
    }

    pub fn missing(message: impl Into<String>) -> Self {
        Self::MissingValue(message.into())
    }

    pub fn conflict(message: impl Into<String>) -> Self {
        Self::Conflict(message.into())
    }

    pub fn io(path: impl Into<PathBuf>, error: io::Error) -> Self {
        Self::Io(path.into(), error)
    }
}

/// Shared result alias for the core crate.
pub type Result<T> = std::result::Result<T, CoreError>;
