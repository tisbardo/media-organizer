use std::env;
use std::fs;
use std::ffi::OsString;
use regex::Regex;


struct TvShow {
    path: String,
    normalized_name: String,
    seasons: Vec<Season>
}

impl TvShow {
    pub fn new(path: OsString, seasons: Vec<Season>) -> TvShow {
        let path = path.to_str().unwrap();
        TvShow {
            path: String::from(path),
            normalized_name: normalize_show_name(path),
            seasons,
        }
    }
}

struct Season {
    path: String,
    season_number: u16,
}

impl Season {
    pub fn parse(path: OsString) -> Option<Season> {
        let path = path.to_str().unwrap();
        let regex = Regex::new(r"(\d{1,2})").unwrap();

        regex.captures(path).map(|captures| {
            Season {
                path: String::from(path),
                season_number: captures[1].parse().unwrap()
            }
        })
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
            Episode {
                path: String::from(path),
                normalized_show_name: normalize_show_name(&captures[1]),
                season_number: captures[2].parse().unwrap(),
                episode_number: captures[3].parse().unwrap()
            }
        })
    }
}

fn main() {
    let input_dir = env::var("INPUT_DIR").expect("You must define INPUT_DIR environment variable");
    let library_dir = env::var("LIBRARY_DIR").expect("You must define LIBRARY_DIR environment variable");

    println!("Start watching input directory {} and put into library {}", input_dir, library_dir);

    let files = get_input_dir_files(input_dir.as_str());
    let shows = get_series_in_library(library_dir.as_str());

    move_files(&files, &shows);
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
    fs::read_dir(library_dir)
        .expect("Cannot read library_dir")
        .filter_map(|tv_show_dir| {
            let tv_show_dir = tv_show_dir.expect("cannot read file");
            if tv_show_dir.path().is_dir() {
                let seasons = fs::read_dir(tv_show_dir.path()).unwrap()
                    .flat_map(|dir| {
                        dir.ok().map(|dir| Season::parse(dir.file_name())).flatten()
                    }).collect();

                Some(TvShow::new(tv_show_dir.file_name(), seasons))
            } else {
                None
            }
        })
        .collect()
}

fn normalize_show_name(name: &str) -> String {
    name.replace(&['.', ' '][..], "")
}

fn move_files(files: &Vec<Episode>, shows: &Vec<TvShow>) {
    for file in files {
        shows.iter()
            .find(|&show| show.normalized_name == file.normalized_show_name)
            .map(|show| move_file(file, &show));
    }
}

fn move_file(file: &Episode, show: &TvShow) {
    println!("Move {} to {}", file.path, show.path);
}
