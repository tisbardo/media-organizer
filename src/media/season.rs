use std::ffi::OsString;
use regex::Regex;

pub struct Season {
    pub path: String,
    pub season_number: u16,
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
