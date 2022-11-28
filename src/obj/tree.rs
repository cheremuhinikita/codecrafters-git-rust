use super::parser::parse_tree_entries;
use crate::{Error, Result};

#[derive(Debug, PartialEq, Eq)]
pub enum TreeEntryMode {
    Blob,
    Tree,
}

impl TryFrom<String> for TreeEntryMode {
    type Error = Error;

    fn try_from(value: String) -> Result<Self> {
        let mode = match value.as_str() {
            "40000" => Self::Tree,
            "100644" => Self::Blob,
            _ => return Err(Error::Generic(String::from("unknown tree entry mode"))),
        };

        Ok(mode)
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct TreeEntry {
    pub mode: TreeEntryMode,
    pub name: String,
    pub sha: String,
}

pub struct Tree(pub Vec<TreeEntry>);

impl Tree {
    pub fn new(tree_entries: Vec<TreeEntry>) -> Self {
        Self(tree_entries)
    }

    pub fn parse(input: &[u8]) -> Result<Self> {
        parse_tree_entries(input)
            .map_err(|e| Error::ParseObject(String::from_utf8_lossy(e).into_owned()))
            .map(|(_, tree_entries)| Self::new(tree_entries))
    }
}
