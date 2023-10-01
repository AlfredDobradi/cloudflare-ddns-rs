use thiserror::Error;

#[derive(Error, Debug)]
pub enum ApplicationError {
    #[error("failed to read file")]
    IOError(#[from] std::io::Error),
    #[error("failed to decode config")]
    JSONError(#[from] serde_json::Error),
}