use std::io::{self, Write};

use crate::{
    error::{Error, Result},
    obj::store,
};

pub struct CatFile {
    blob_sha: String,
}

impl CatFile {
    pub fn new(blob_sha: impl ToString) -> Self {
        Self {
            blob_sha: blob_sha.to_string(),
        }
    }

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

    pub fn inner(&self) -> Result<Vec<u8>> {
        store::read(&self.blob_sha)?
            .as_blob()
            .map(|b| b.to_bytes())
            .ok_or_else(|| Error::Generic(String::from("git object must be blob")))
    }

    pub fn exec(self) -> Result<()> {
        let bytes = self.inner()?;

        io::stdout().write_all(&bytes)?;

        Ok(())
    }
}
