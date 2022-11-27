use std::{
    fs,
    io::{Read, Write},
    path::{Path, PathBuf},
    str,
};

use flate2::{read::ZlibDecoder, write::ZlibEncoder, Compression};
use sha1::{Digest, Sha1};

use super::{decode::decode, encode::encode, Object};
use crate::Result;

fn get_path_from_sha(sha: &str) -> PathBuf {
    Path::new(".git/objects").join(&sha[..2]).join(&sha[2..])
}

pub fn read(sha: &str) -> Result<Object> {
    let path = get_path_from_sha(sha);
    let bytes = fs::read(path)?;

    let mut decoder = ZlibDecoder::new(bytes.as_slice());
    let mut buf = Vec::<u8>::new();
    decoder.read_to_end(&mut buf)?;

    decode(buf.as_slice())
}

pub fn write(object: &Object) -> Result<String> {
    let encoded = encode(object);

    let mut zlib_encoder = ZlibEncoder::new(Vec::new(), Compression::default());
    zlib_encoder.write_all(&encoded)?;
    let compressed = zlib_encoder.finish()?;

    let mut hasher = Sha1::new();
    hasher.update(&compressed);
    let sha = format!("{:x}", hasher.finalize());

    let path = get_path_from_sha(&sha);

    fs::write(path, compressed)?;

    Ok(sha)
}
