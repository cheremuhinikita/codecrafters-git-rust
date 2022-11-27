use std::{env, process};

use git_starter_rust::run;

fn main() {
    let args = env::args().collect::<Vec<String>>();

    if let Err(e) = run(args) {
        eprintln!("error: {:?}", e);
        process::exit(1)
    }
}
