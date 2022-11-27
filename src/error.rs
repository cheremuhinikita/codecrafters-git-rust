use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("IO error")]
    IoError(#[from] std::io::Error),
    #[error("utf-8 error")]
    Utf8Error(#[from] std::str::Utf8Error),
    #[error("failed parse command args")]
    ParseCommand(String),
    #[error("failed parse object")]
    ParseObject(String),
    #[error("{0}")]
    Generic(String),
}

pub type Result<T> = core::result::Result<T, Error>;
