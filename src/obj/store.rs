use std::{fs, io::Read, path::Path, str};

use flate2::read::ZlibDecoder;

use crate::{error::Error, Result};

use super::{parser, Object};

pub struct ObjectStore;

impl ObjectStore {
    pub fn read(sha: &str) -> Result<Object> {
        let path = Path::new(".git/objects").join(&sha[..2]).join(&sha[2..]);
        let bytes = fs::read(path)?;

        let mut decoder = ZlibDecoder::new(bytes.as_slice());
        let mut buf = Vec::<u8>::new();
        decoder.read_to_end(&mut buf)?;

        let input = str::from_utf8(buf.as_slice())?;

        let (_, raw_object) =
            parser::raw_object(input).map_err(|e| Error::ParseObject(e.to_string()))?;

        Object::try_from(raw_object)
    }
}
