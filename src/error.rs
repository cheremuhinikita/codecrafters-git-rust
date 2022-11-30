use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("first letter must be lowercase but was {0}")]
    WrongCase(String),
    #[error("IO error")]
    IoError(#[from] std::io::Error),
    #[error("utf-8 error")]
    Utf8Error(#[from] std::str::Utf8Error),
    #[error("from utf-8 error")]
    FromUtf8Error(#[from] std::string::FromUtf8Error),
    #[error("parse int error")]
    ParseIntError(#[from] std::num::ParseIntError),
    #[error("os string error")]
    OsString(std::ffi::OsString),
    #[error("failed parse command args - {0}")]
    ParseCommand(String),
    #[error("failed parse object - {0}")]
    ParseObject(String),
    #[error("{0}")]
    Generic(String),
}

pub type Result<T> = core::result::Result<T, Error>;
