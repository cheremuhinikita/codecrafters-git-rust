pub mod blob;
pub mod decode;
pub mod encode;
pub mod parser;
pub mod raw;
pub mod store;

use std::str;

use self::{blob::Blob, raw::RawObject};
use crate::{error::Error, Result};

pub enum Object {
    Blob(Blob),
    Tree,
    Commit,
}

impl Object {
    pub fn from_raw(raw: RawObject) -> Result<Self> {
        let obj = match raw.kind.as_str() {
            "blob" => Self::from_blob(Blob::parse(&raw.content)),
            kind => return Err(Error::ParseObject(format!("unknown object kind: {}", kind))),
        };

        Ok(obj)
    }

    fn to_raw(&self) -> RawObject {
        let (kind, content) = match self {
            Object::Blob(blob) => ("blob", blob.to_bytes()),
            _ => todo!(),
        };

        let content = str::from_utf8(&content).unwrap();

        RawObject::new(kind, content)
    }

    pub fn from_blob(blob: Blob) -> Self {
        Self::Blob(blob)
    }

    pub fn as_blob(&self) -> Option<&Blob> {
        match self {
            Self::Blob(blob) => Some(blob),
            _ => None,
        }
    }

    pub fn is_blob(&self) -> bool {
        self.as_blob().is_some()
    }
}
