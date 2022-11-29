use std::{env, process};

fn main() {
    let args = env::args().collect::<Vec<String>>();

    if let Err(e) = git_starter_rust::run(args) {
        eprintln!("error: {}", e);
        process::exit(1);
    }
}
