pub mod cmd;
pub mod error;
pub mod hex;
pub mod obj;
pub mod sha;

pub use crate::error::{Error, Result};
use cmd::Command;

pub fn run(args: Vec<String>) -> Result<()> {
    Command::parse(&args)?.exec()
}
