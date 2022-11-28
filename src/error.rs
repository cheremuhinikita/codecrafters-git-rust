use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("IO error")]
    IoError(#[from] std::io::Error),
    #[error("utf-8 error")]
    Utf8Error(#[from] std::str::Utf8Error),
    #[error("from utf-8 error")]
    FromUtf8Error(#[from] std::string::FromUtf8Error),
    #[error("os string error")]
    OsString(std::ffi::OsString),
    #[error("failed parse command args")]
    ParseCommand(String),
    #[error("failed parse object")]
    ParseObject(String),
    #[error("{0}")]
    Generic(String),
}

pub type Result<T> = core::result::Result<T, Error>;
