use std::env;
use std::fs;

pub fn cat(path: &str) -> String {
    fs::read_to_string(path).expect("Something went wrong while reading the file")
}

fn main() {
    let args : Vec<String> = env::args().collect();

    println!("{}", if args.len() == 2 { cat(&args[1]) } else { String::from("Needs 1 argument: <path>") });
}
