use std::fs;

use crate::error::Result;

pub struct Init;

impl Init {
    pub fn exec(self) -> Result<()> {
        fs::create_dir(".git")?;
        fs::create_dir(".git/objects")?;
        fs::create_dir(".git/refs")?;
        fs::write(".git/HEAD", "ref: refs/heads/master\n")?;

        println!("initialized git directory");

        Ok(())
    }
}
