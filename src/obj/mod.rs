pub mod blob;
pub mod decode;
pub mod encode;
pub mod parser;
pub mod raw;
pub mod store;
pub mod tree;

use self::{blob::Blob, raw::RawObject, tree::Tree};
use crate::{error::Error, Result};

pub enum Object {
    Blob(Blob),
    Tree(Tree),
    Commit,
}

impl Object {
    pub fn from_raw(raw: RawObject) -> Result<Self> {
        let obj = match raw.kind.as_str() {
            "blob" => Self::from_blob(Blob::parse(&raw.content)),
            "tree" => Self::from_tree(Tree::parse(&raw.content)?),
            kind => return Err(Error::ParseObject(format!("unknown object kind: {}", kind))),
        };

        Ok(obj)
    }

    fn to_raw(&self) -> RawObject {
        let (kind, content) = match self {
            Object::Blob(blob) => ("blob", blob.to_bytes()),
            _ => todo!(),
        };

        RawObject::new(kind, &content)
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

    pub fn from_tree(tree: Tree) -> Self {
        Self::Tree(tree)
    }

    pub fn as_tree(&self) -> Option<&Tree> {
        match self {
            Self::Tree(tree) => Some(tree),
            _ => None,
        }
    }

    pub fn is_tree(&self) -> bool {
        self.as_tree().is_some()
    }
}
