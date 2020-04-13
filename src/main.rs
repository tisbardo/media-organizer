use std::env;
use std::fs;
use std::thread;
use std::time::Duration;

mod media;
use media::TvShow;
use media::Episode;
use media::Season;

fn main() {
    let input_dir = env::var("INPUT_DIR").expect("You must define INPUT_DIR environment variable");
    let library_dir = env::var("LIBRARY_DIR").expect("You must define LIBRARY_DIR environment variable");

    println!("Start watching new Episodes in directory {} to move into library {}", input_dir, library_dir);

    loop {
        println!("Start checking");

        let files = get_input_dir_files(input_dir.as_str());
        let shows = get_series_in_library(library_dir.as_str());

        move_files(&files, &shows);

        thread::sleep(Duration::from_millis(30 * 1000));
    }
}

fn get_input_dir_files(input_dir: &str) -> Vec<Episode> {
    let mut episodes: Vec<Episode> = Vec::new();

    for file in fs::read_dir(input_dir).expect("Cannot read input_dir") {
        let file = file.expect("cannot read file");
        if file.path().is_dir() {
            // Explore recursively the folder
            file.path().to_str()
                .map(|f| episodes.append(&mut get_input_dir_files(f)) );
        } else {
            // Try to parse filename into an Episode and add to list
            Episode::parse(file.file_name())
                .map(|e| episodes.push(e));
        }
    };

    episodes
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

fn move_files(episodes: &Vec<Episode>, shows: &Vec<TvShow>) {
    for episode in episodes {
        shows.iter()
            .find(|&show| show.normalized_name == episode.normalized_show_name)
            .map(|show| {
                let season = show.seasons.iter()
                    .find(|&season| season.season_number == episode.season_number);

                move_file(episode, &show, season)
            });
    }
}

fn move_file(file: &Episode, show: &TvShow, season: Option<&Season>) {
    let mut path = String::new();
    path.push_str(&show.path);

    if season.is_some() {
        path.push_str(&format!("{}{}", "/", &season.unwrap().path));
    }

    println!("Move {} to {}", file.path, path);
}
