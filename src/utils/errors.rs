use thiserror::Error;

#[allow(dead_code)]
#[derive(Debug, Error)]
pub enum ProxError {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    #[error("Serialization error: {0}")]
    Serde(#[from] serde_json::Error),
    #[error("Storage error: {0}")]
    Storage(String),
}
