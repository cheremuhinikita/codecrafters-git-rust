use std::fs;

use crate::{
    error::{Error, Result},
    obj::{blob::Blob, store, Object},
};

pub struct HashObject {
    file: String,
}

impl HashObject {
    pub fn new(file: impl ToString) -> Self {
        Self {
            file: file.to_string(),
        }
    }

    pub fn parse(args: &[String]) -> Result<Self> {
        match args.get(0).map(|a| a.as_str()) {
            Some("-w") => {}
            _ => return Err(Error::ParseCommand(String::from("option must be \"-w\""))),
        }

        match args.get(1) {
            Some(file) => Ok(Self {
                file: file.to_owned(),
            }),
            None => Err(Error::ParseCommand(String::from("missing file arg"))),
        }
    }

    pub fn inner(&self) -> Result<String> {
        let content = fs::read(&self.file)?;

        let blob = Blob::new(content.as_slice());
        let object = Object::from_blob(blob);

        store::write(&object)
    }

    pub fn exec(self) -> Result<()> {
        let sha = self.inner()?;

        println!("{}", sha);

        Ok(())
    }
}
