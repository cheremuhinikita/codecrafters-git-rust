use std::fs;

use crate::{
    error::{Error, Result},
    obj::{blob::Blob, store, Object},
};

pub struct HashObject {
    path: String,
}

impl HashObject {
    pub fn parse(args: &[String]) -> Result<Self> {
        match args.get(0).map(|a| a.as_str()) {
            Some("-w") => {}
            _ => return Err(Error::ParseCommand(String::from("option must be \"-w\""))),
        }

        match args.get(1) {
            Some(path) => Ok(Self {
                path: path.to_owned(),
            }),
            None => Err(Error::ParseCommand(String::from("missing path arg"))),
        }
    }

    pub fn exec(self) -> Result<()> {
        let content = fs::read(&self.path)?;

        let blob = Blob::new(content.as_slice());
        let object = Object::from_blob(blob);

        let sha = store::write(&object)?;

        println!("{}", sha);

        Ok(())
    }
}
