use std::ffi::OsString;
use regex::Regex;
use crate::media::normalize_show_name;

pub struct Episode {
    pub path: String,
    pub filename: String,
    pub normalized_show_name: String,
    pub season_number: u16,
    pub episode_number: u16,
}

impl Episode {
    pub fn parse(path: &str, filename: OsString) -> Option<Episode> {
        let filename = filename.to_str().unwrap();

        let se_regex = Regex::new(r"^(.+)S(\d{1,2})E(\d{1,2})").unwrap();
        let x_regex = Regex::new(r"^(.+[^\d])(\d{1,2})x(\d{1,2})[^\d]").unwrap();

        se_regex.captures(filename).or(x_regex.captures(filename)).map(|captures| {
            Episode {
                path: String::from(path),
                filename: String::from(filename),
                normalized_show_name: normalize_show_name(&captures[1]),
                season_number: captures[2].parse().unwrap(),
                episode_number: captures[3].parse().unwrap()
            }
        })
    }
}
