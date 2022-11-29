use super::parser::parse_tree_entries;
use crate::{hex, Error, Result};

#[derive(Debug, PartialEq, Eq)]
pub enum TreeEntryMode {
    Blob,
    BlobExecutable,
    Tree,
}

impl TreeEntryMode {
    pub fn as_bytes(&self) -> &'static [u8] {
        match self {
            TreeEntryMode::Blob => b"100644",
            TreeEntryMode::BlobExecutable => b"100755",
            TreeEntryMode::Tree => b"40000",
        }
    }
}

impl TryFrom<String> for TreeEntryMode {
    type Error = Error;

    fn try_from(value: String) -> Result<Self> {
        let mode = match value.as_str() {
            "100644" => Self::Blob,
            "100755" => Self::BlobExecutable,
            "40000" => Self::Tree,
            mode => return Err(Error::Generic(format!("unknown tree entry mode {}", mode))),
        };

        Ok(mode)
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct TreeEntry {
    mode: TreeEntryMode,
    name: String,
    sha: String,
}

impl TreeEntry {
    pub fn new(mode: TreeEntryMode, name: impl ToString, sha: impl ToString) -> Self {
        Self {
            mode,
            name: name.to_string(),
            sha: sha.to_string(),
        }
    }

    pub fn build(mode: String, name: String, sha: &[u8]) -> Result<Self> {
        Ok(Self {
            mode: mode.try_into()?,
            name,
            sha: hex::encode(sha),
        })
    }
}

pub struct Tree(Vec<TreeEntry>);

impl Tree {
    pub fn new(tree_entries: Vec<TreeEntry>) -> Self {
        Self(tree_entries)
    }

    pub fn sort_entries(&mut self) {
        self.0.sort_by(|a, b| a.name.cmp(&b.name));
    }

    pub fn parse(input: &[u8]) -> Result<Self> {
        parse_tree_entries(input)
            .map_err(|e| Error::ParseObject(String::from_utf8_lossy(e).into_owned()))
            .map(|(_, tree_entries)| Self::new(tree_entries))
    }

    pub fn entry_names(&self) -> Vec<&str> {
        self.0.iter().map(|e| e.name.as_str()).collect()
    }

    pub fn to_bytes(&self) -> Vec<u8> {
        let mut result = Vec::<u8>::new();

        for tree_entry in self.0.iter() {
            result.extend_from_slice(tree_entry.mode.as_bytes());
            result.push(b' ');
            result.extend_from_slice(tree_entry.name.as_bytes());
            result.push(b'\0');
            result.extend_from_slice(hex::decode(&tree_entry.sha).unwrap().as_slice());
        }

        result
    }
}
