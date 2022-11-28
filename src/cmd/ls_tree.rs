use crate::{
    error::{Error, Result},
    obj::store,
};

pub struct LsTree {
    tree_sha: String,
}

impl LsTree {
    pub fn parse(args: &[String]) -> Result<Self> {
        match args.get(0).map(|a| a.as_str()) {
            Some("--name-only") => {}
            _ => {
                return Err(Error::ParseCommand(String::from(
                    "option must be \"--name-only\"",
                )))
            }
        }

        match args.get(1) {
            Some(tree_sha) => Ok(Self {
                tree_sha: tree_sha.to_owned(),
            }),
            None => Err(Error::ParseCommand(String::from("missing tree sha arg"))),
        }
    }

    pub fn exec(self) -> Result<()> {
        let object = store::read(&self.tree_sha)?;

        let tree = object
            .as_tree()
            .ok_or_else(|| Error::Generic(String::from("git object must be tree")))?;

        for tree_entry in tree.0.iter() {
            println!("{}", tree_entry.name);
        }

        Ok(())
    }
}
