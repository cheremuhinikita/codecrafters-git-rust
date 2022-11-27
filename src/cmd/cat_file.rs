use std::io::{self, Write};

use crate::{
    error::{Error, Result},
    obj::store,
};

pub struct CatFile {
    blob_sha: String,
}

impl CatFile {
    pub fn parse(args: &[String]) -> Result<Self> {
        match args.get(0).map(|a| a.as_str()) {
            Some("-p") => {}
            _ => return Err(Error::ParseCommand(String::from("option must be \"-p\""))),
        }

        match args.get(1) {
            Some(blob_sha) => Ok(Self {
                blob_sha: blob_sha.to_owned(),
            }),
            None => Err(Error::ParseCommand(String::from("missing blob sha arg"))),
        }
    }

    pub fn exec(self) -> Result<()> {
        let object = store::read(&self.blob_sha)?;
        let blob = object
            .as_blob()
            .ok_or_else(|| Error::Generic(String::from("git object must be blob")))?;

        io::stdout().write_all(blob.0.as_slice())?;

        Ok(())
    }
}
