use std::time::{SystemTime, UNIX_EPOCH};

use super::user::User;

pub struct Commit {
    tree_sha: String,
    parent_sha: Option<String>,
    msg: String,
    timestamp: u128,
    author: User,
    committer: User,
}

impl Commit {
    pub fn new(
        tree_sha: impl ToString,
        parent_sha: Option<impl ToString>,
        msg: impl ToString,
        author: User,
        committer: User,
    ) -> Self {
        Self {
            tree_sha: tree_sha.to_string(),
            parent_sha: parent_sha.map(|p| p.to_string()),
            msg: msg.to_string(),
            timestamp: SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_millis(),
            author,
            committer,
        }
    }

    pub fn to_bytes(&self) -> Vec<u8> {
        let mut result = Vec::<u8>::new();

        result.extend_from_slice(b"tree ");
        result.extend_from_slice(self.tree_sha.as_bytes());
        result.push(b'\n');

        if let Some(parent_sha) = &self.parent_sha {
            result.extend_from_slice(b"parent ");
            result.extend_from_slice(parent_sha.as_bytes());
            result.push(b'\n');
        }

        result.extend_from_slice(b"author ");
        result.extend_from_slice(self.author.to_string().as_bytes());
        result.push(b' ');
        result.extend_from_slice(self.timestamp.to_string().as_bytes());
        result.extend_from_slice(b" -0700\n");

        result.extend_from_slice(b"committer ");
        result.extend_from_slice(self.committer.to_string().as_bytes());
        result.push(b' ');
        result.extend_from_slice(self.timestamp.to_string().as_bytes());
        result.extend_from_slice(b" -0700\n");

        result.push(b'\n');
        result.extend_from_slice(self.msg.as_bytes());

        result
    }
}
