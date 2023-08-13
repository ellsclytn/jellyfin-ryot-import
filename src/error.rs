use thiserror::Error as ThisError;

#[derive(Debug, ThisError)]
pub enum Error {
    #[error("IO error: `{0}`")]
    Io(#[from] std::io::Error),
    #[error("Env var error: `{0}`")]
    Env(#[from] std::env::VarError),
    #[error("Reqwest header error: `{0}`")]
    ReqwestHeader(#[from] reqwest::header::InvalidHeaderValue),
    #[error("Reqwest error: `{0}`")]
    Reqwest(#[from] reqwest::Error),
    #[error("Serde JSON error: `{0}`")]
    SerdeJson(#[from] serde_json::Error),
}

/// Type alias for the standard [`Result`] type.
pub type Result<T> = std::result::Result<T, Error>;
