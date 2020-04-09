use std::ffi::OsString;
use crate::media::Season;
use crate::media::normalize_show_name;

pub struct TvShow {
    pub path: String,
    pub normalized_name: String,
    pub seasons: Vec<Season>
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
