// Import submodules
mod season;
mod episode;
mod tv_show;

// Re-export classes
pub use season::Season;
pub use episode::Episode;
pub use tv_show::TvShow;

use std::{fs, thread};
use std::time::Duration;

fn normalize_show_name(name: &str) -> String {
    name
        .to_lowercase()
        .replace(&['.', ' '][..], "")
}

pub struct MediaOrganizer {
    pub input_dir: String,
    pub library_dir: String,
}

impl MediaOrganizer {
    pub fn start_watching(&self) {
        loop {
            println!("Start checking");

            let files = MediaOrganizer::get_input_dir_files(self.input_dir.as_str());
            let shows = MediaOrganizer::get_series_in_library(self.library_dir.as_str());

            self.move_files(&files, &shows);

            thread::sleep(Duration::from_millis(30 * 1000));
        }
    }

    fn get_input_dir_files(directory: &str) -> Vec<Episode> {
        let mut episodes: Vec<Episode> = Vec::new();

        for file in fs::read_dir(directory).expect("Cannot read input_dir") {
            let file = file.expect("cannot read file");
            if file.path().is_dir() {
                // Explore recursively the folder
                file.path().to_str()
                    .map(|f| episodes.append(&mut MediaOrganizer::get_input_dir_files(f)) );
            } else {
                // Try to parse filename into an Episode and add to list
                Episode::parse(file.path().to_str().unwrap(), file.file_name())
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

    fn move_files(&self, episodes: &Vec<Episode>, shows: &Vec<TvShow>) {
        for episode in episodes {
            shows.iter()
                .find(|&show| show.normalized_name == episode.normalized_show_name)
                .map(|show| {
                    let season = show.seasons.iter()
                        .find(|&season| season.season_number == episode.season_number);

                    self.move_file(episode, &show, season)
                });
        }
    }

    fn move_file(&self, file: &Episode, show: &TvShow, season: Option<&Season>) {
        let mut to = format!("{}/{}", &self.library_dir, &show.path);
        if season.is_some() {
            to.push_str(&format!("/{}", &season.unwrap().path));
        }
        to.push_str(&format!("/{}", &file.filename));

        match fs::rename(&file.path, &to) {
            Ok(_) => println!("Moved {} to {}", &file.path, &to),
            Err(error) => println!("Error while moving {} to {} : {}", &file.path, &to, error)
        }
    }
}
