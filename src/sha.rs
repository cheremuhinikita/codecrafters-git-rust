use sha1::{Digest, Sha1};

pub fn get_sha(bytes: &[u8]) -> String {
    let mut hasher = Sha1::new();
    hasher.update(bytes);

    format!("{:x}", hasher.finalize())
}
