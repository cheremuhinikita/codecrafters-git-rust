pub mod cmd;
pub mod error;
pub mod obj;

use cmd::Command;

pub use crate::error::{Error, Result};

pub fn run(args: Vec<String>) -> Result<()> {
    Command::parse(&args)?.exec()
}
