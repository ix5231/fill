use std::{env, process};
use std::path::Path;
use std::fs::File;
use std::io::{BufReader, BufRead};

fn usage() {
    println!("USAGE");
    println!("    fill INPUT");
}

fn show<P: AsRef<Path>>(path: P) {
    let input = File::open(path).unwrap();
    let buf = BufReader::new(&input);

    for l in buf.lines() {
        println!("{}", l.unwrap());
    }
}

fn main() {
    if let Some(file_path) = env::args().nth(1) {
        show(file_path);
    } else {
        usage();
        process::exit(1);
    }

    process::exit(0);
}
