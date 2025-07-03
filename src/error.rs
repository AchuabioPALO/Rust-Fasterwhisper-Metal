use thiserror::Error;

#[derive(Error, Debug)]
pub enum TranscriptionError {
    #[error("Python error: {0}")]
    PythonError(#[from] pyo3::PyErr),

    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),

    #[error("JSON serialization error: {0}")]
    JsonError(#[from] serde_json::Error),

    #[error("Invalid file path: {0}")]
    InvalidPath(String),

    #[error("Unsupported audio format: {0}")]
    UnsupportedFormat(String),

    #[error("Model initialization failed: {0}")]
    ModelInitError(String),

    #[error("Transcription failed: {0}")]
    TranscriptionFailed(String),
}

pub type Result<T> = std::result::Result<T, TranscriptionError>;
