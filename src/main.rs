use std::env;
use std::fs;
use std::ffi::OsString;
use regex::Regex;


struct TvShow {
    normalized_name: String,
    path: String,
}

impl TvShow {
    pub fn new(path: OsString) -> TvShow {
        let path = path.to_str().unwrap();
        TvShow {
            path: String::from(path),
            normalized_name: normalize_show_name(path)
        }
    }
}

struct Episode {
    path: String,
    normalized_show_name: String,
    season_number: u16,
    episode_number: u16,
}

impl Episode {
    pub fn parse(path: OsString) -> Option<Episode> {
        let path = path.to_str().unwrap();

        let se_regex = Regex::new(r"^(.+)S(\d{1,2})E(\d{1,2})").unwrap();
        let x_regex = Regex::new(r"^(.+[^\d])(\d{1,2})x(\d{1,2})[^\d]").unwrap();

        se_regex.captures(path).or(x_regex.captures(path)).map(|captures| {

            println!("Show name: {} Season: {} Episode: {}", &captures[1], &captures[2], &captures[3]);

            Episode {
                path: String::from(path),
                normalized_show_name: normalize_show_name(&captures[1]),
                season_number: 0,
                episode_number: 0
            }
        })
    }
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

fn get_input_dir_files(input_dir: &str) -> Vec<Episode> {
    return fs::read_dir(input_dir)
        .expect("Cannot read input_dir")
        .map(|file| {
            Episode::parse(file.expect("cannot read file").file_name())
        })
        .filter_map(|x| x) // Remove None
        .collect()
}

fn get_series_in_library(library_dir: &str) -> Vec<TvShow> {
    return fs::read_dir(library_dir).expect("Cannot read library_dir").map(|file| {
        TvShow::new(file.expect("cannot read file").file_name())
    }).collect()
}

fn normalize_show_name(name: &str) -> String {
    name.replace(&['.', ' '][..], "")
}
