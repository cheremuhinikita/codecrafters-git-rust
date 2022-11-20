use std::env;
use std::fs;

fn main() {
    let args = env::args().collect::<Vec<String>>();

    let command = &args[1];

    match command.as_str() {
        "init" => {
            fs::create_dir(".git").unwrap();
            fs::create_dir(".git/objects").unwrap();
            fs::create_dir(".git/refs").unwrap();
            fs::write(".git/HEAD", "ref: refs/heads/master\n").unwrap();
            println!("initialized git directory")
        }
        _ => eprintln!("unknown command: {}", args[1]),
    }
}
