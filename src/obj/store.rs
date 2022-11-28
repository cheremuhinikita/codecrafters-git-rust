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

fn get_paths_from_sha(sha: &str) -> (PathBuf, PathBuf) {
    let dir_path = Path::new(".git/objects").join(&sha[..2]);
    let file_path = dir_path.join(&sha[2..]);

    (dir_path, file_path)
}

pub fn read(sha: &str) -> Result<Object> {
    let (_, file_path) = get_paths_from_sha(sha);
    let bytes = fs::read(file_path)?;

    let mut zlib_decoder = ZlibDecoder::new(bytes.as_slice());
    let mut buf = Vec::<u8>::new();
    zlib_decoder.read_to_end(&mut buf)?;

    decode(&buf)
}

pub fn write(object: &Object) -> Result<String> {
    let encoded = encode(object);

    let mut hasher = Sha1::new();
    hasher.update(&encoded);
    let sha = format!("{:x}", hasher.finalize());

    let mut zlib_encoder = ZlibEncoder::new(Vec::new(), Compression::default());
    zlib_encoder.write_all(&encoded)?;
    let compressed = zlib_encoder.finish()?;

    let (dir_path, file_path) = get_paths_from_sha(&sha);

    fs::create_dir_all(dir_path)?;
    fs::write(file_path, compressed)?;

    Ok(sha)
}
