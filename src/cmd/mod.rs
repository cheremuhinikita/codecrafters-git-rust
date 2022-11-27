pub mod cat_file;
pub mod init;

use crate::error::{Error, Result};

use self::{cat_file::CatFile, init::Init};

pub enum Command {
    Init(Init),
    CatFile(CatFile),
}

impl Command {
    pub fn parse(args: &[String]) -> Result<Self> {
        let command = &args
            .get(1)
            .ok_or(Error::ParseCommand(String::from("missing command name")))?;
        let args = &args[2..];

        let cmd = match command.as_str() {
            "init" => Self::Init(Init),
            "cat-file" => Self::CatFile(CatFile::parse(args)?),
            _ => return Err(Error::ParseCommand(format!("unknown command: {}", command))),
        };

        Ok(cmd)
    }

    pub fn exec(self) -> Result<()> {
        match self {
            Self::Init(init) => init.exec(),
            Self::CatFile(cat_file) => cat_file.exec(),
        }
    }
}
