use std::env;
use std::fs;

mod media;
use media::TvShow;
use media::Episode;
use media::Season;

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
