pub mod cat_file;
pub mod hash_object;
pub mod init;
pub mod ls_tree;
pub mod write_tree;

use self::{
    cat_file::CatFile, hash_object::HashObject, init::Init, ls_tree::LsTree, write_tree::WriteTree,
};
use crate::error::{Error, Result};

pub enum Command {
    Init(Init),
    CatFile(CatFile),
    HashObject(HashObject),
    LsTree(LsTree),
    WriteTree(WriteTree),
}

impl Command {
    pub fn parse(args: &[String]) -> Result<Self> {
        let command = &args
            .get(1)
            .ok_or_else(|| Error::ParseCommand(String::from("missing command name")))?;
        let args = &args[2..];

        let cmd = match command.as_str() {
            "init" => Self::Init(Init),
            "cat-file" => Self::CatFile(CatFile::parse(args)?),
            "hash-object" => Self::HashObject(HashObject::parse(args)?),
            "ls-tree" => Self::LsTree(LsTree::parse(args)?),
            "write-tree" => Self::WriteTree(WriteTree),
            _ => return Err(Error::ParseCommand(format!("unknown command: {}", command))),
        };

        Ok(cmd)
    }

    pub fn exec(self) -> Result<()> {
        match self {
            Self::Init(init) => init.exec(),
            Self::CatFile(cat_file) => cat_file.exec(),
            Self::HashObject(hash_object) => hash_object.exec(),
            Self::LsTree(ls_tree) => ls_tree.exec(),
            Self::WriteTree(write_tree) => write_tree.exec(),
        }
    }
}
