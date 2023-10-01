use std::fmt::Display;

use thiserror::Error;

#[derive(Error, Debug)]
pub enum ApplicationError {
    IOError{
        #[from]
        source: std::io::Error,
    },
    JSONError{
        #[from]
        source: serde_json::Error,
    },
    HTTPError{
        #[from]
        source: reqwest::Error,
    }
}

impl Display for ApplicationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ApplicationError::IOError { source } => write!(f, "{source}"),
            ApplicationError::JSONError { source } => write!(f, "{source}"),
            ApplicationError::HTTPError { source } => write!(f, "{source}"),
        }
    }
}