use std::env;
use std::fs;
use std::ffi::OsString;

struct TvShow {
    normalized_name: String,
    path: String,
}

impl TvShow {
    pub fn new(path: OsString) -> TvShow {
        let str_slice = path.to_str().unwrap();
        TvShow {
            path: String::from(str_slice),
            normalized_name: normalize_show_name(str_slice)
        }
    }
}

struct File {
    path: String,
    
}

fn main() {
    println!("Hello, world!");

    let input_dir = env::var("INPUT_DIR").expect("You must define INPUT_DIR environment variable");
    let library_dir = env::var("LIBRARY_DIR").expect("You must define LIBRARY_DIR environment variable");

    println!("Start watching input directory {} and put into library {}", input_dir, library_dir);

    let files = get_input_dir_files(input_dir.as_str());
    let shows = get_series_in_library(library_dir.as_str());

//    for file in files { println!("{}", file.to_str().unwrap()) }
//    for show in shows { println!("{}", file.to_str().unwrap()) }
}

fn get_input_dir_files(input_dir: &str) -> Vec<OsString> {
    return fs::read_dir(input_dir).expect("Cannot read input_dir").map(|file| {
        file.expect("cannot read file").file_name()
    }).collect()
}

fn get_series_in_library(library_dir: &str) -> Vec<TvShow> {
    return fs::read_dir(library_dir).expect("Cannot read library_dir").map(|file| {
        TvShow::new(file.expect("cannot read file").file_name())
    }).collect()
}

fn normalize_show_name(name: &str) -> String {
    name.replace(&['.', ' '][..], "")
}
