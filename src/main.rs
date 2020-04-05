use std::env;
use std::fs;
use std::ffi::OsString;

fn main() {
    println!("Hello, world!");

    let input_dir = env::var("INPUT_DIR").expect("You must define INPUT_DIR environment variable");
    let library_dir = env::var("LIBRARY_DIR").expect("You must define LIBRARY_DIR environment variable");

    println!("Start watching input directory {} and put into library {}", input_dir, library_dir);

    let files = get_input_dir_files(input_dir);

    for file in files {
        println!("{}", file.to_str().unwrap())
    }
}

fn get_input_dir_files(input_dir: String) -> Vec<OsString> {
    return fs::read_dir(input_dir).expect("Cannot read input_dir").map(|file| {
        file.expect("cannot read file").file_name()
    }).collect()
}
