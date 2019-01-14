use std::{env, process};

fn usage() {
    println!("USAGE");
    println!("    fill INPUT");
}

fn main() {
    if let Some(file_path) = env::args().nth(1) {
        println!("{}", file_path);
    } else {
        usage();
        process::exit(1);
    }

    process::exit(0);
}
