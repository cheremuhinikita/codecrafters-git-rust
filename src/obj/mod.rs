pub mod blob;
pub mod parser;
pub mod store;

use self::{blob::Blob, parser::RawObject};
use crate::error::Error;

pub enum Object {
    Blob(Blob),
    Tree,
    Commit,
}

impl TryFrom<RawObject> for Object {
    type Error = Error;

    fn try_from(value: RawObject) -> Result<Self, Self::Error> {
        let obj = match value.kind.as_str() {
            "blob" => Self::Blob(Blob::parse(&value.content)),
            kind => return Err(Error::ParseObject(format!("unknown object kind: {}", kind))),
        };

        Ok(obj)
    }
}

impl Object {
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
