// Import submodules
mod season;
mod episode;
mod tv_show;

// Re-export classes
pub use season::Season;
pub use episode::Episode;
pub use tv_show::TvShow;

fn normalize_show_name(name: &str) -> String {
    name
        .to_lowercase()
        .replace(&['.', ' '][..], "")
}
