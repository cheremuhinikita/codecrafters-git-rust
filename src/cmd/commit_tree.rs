use super::args::get_named_arg;
use crate::{
    obj::{commit::Commit, store, user::User, Object},
    Error, Result,
};

pub struct CommitTree {
    tree_sha: String,
    commit_sha: Option<String>,
    msg: String,
}

impl CommitTree {
    pub fn new(tree_sha: impl ToString, commit_sha: Option<impl ToString>, msg: String) -> Self {
        Self {
            tree_sha: tree_sha.to_string(),
            commit_sha: commit_sha.map(|c| c.to_string()),
            msg,
        }
    }

    pub fn parse(args: &[String]) -> Result<Self> {
        let tree_sha = args
            .get(0)
            .ok_or_else(|| Error::ParseCommand(String::from("arg tree sha is required")))?;

        let (args, commit_sha) = get_named_arg(args, "-p");

        let (_, msg) = get_named_arg(&args, "-m");

        let msg =
            msg.ok_or_else(|| Error::ParseCommand(String::from("arg message is required")))?;

        Ok(Self::new(tree_sha, commit_sha, msg))
    }

    pub fn inner(&self) -> Result<String> {
        let commit = Commit::new(
            &self.tree_sha,
            self.commit_sha.as_deref(),
            &self.msg,
            User::default(),
            User::default(),
        );
        let object = Object::from_commit(commit);

        store::write(&object)
    }

    pub fn exec(self) -> Result<()> {
        let sha = self.inner()?;

        println!("{}", sha);

        Ok(())
    }
}
