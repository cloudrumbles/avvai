//! Error types for the morphological analyzer.

use std::path::PathBuf;
use thiserror::Error;

/// Errors that can occur during morphological analysis.
#[derive(Debug, Error)]
pub enum MorphError {
    /// FST-related errors
    #[error(transparent)]
    Fst(#[from] FstError),

    /// Invalid input
    #[error("invalid input: {0}")]
    InvalidInput(String),

    /// Analysis failed
    #[error("analysis failed: {0}")]
    AnalysisFailed(String),
}

/// Errors specific to the FST wrapper.
#[derive(Debug, Error)]
pub enum FstError {
    /// The specified FST file does not exist.
    #[error("FST file not found: {0}")]
    FstFileNotFound(PathBuf),

    /// Error during FFI operation.
    #[error("FFI error: {0}")]
    FfiError(String),

    /// UTF-8 decoding error.
    #[error("UTF-8 decoding error: {0}")]
    Utf8Error(String),

    /// Invalid FST file format.
    #[error("invalid FST file format: {0}")]
    InvalidFstFormat(String),

    /// I/O error.
    #[error("I/O error: {0}")]
    IoError(#[from] std::io::Error),
}

/// Result type alias for FST operations.
pub type FstResult<T> = Result<T, FstError>;

/// Result type alias for morphological analysis operations.
pub type MorphResult<T> = Result<T, MorphError>;
