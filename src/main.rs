mod media;

use std::env;
use media::MediaOrganizer;

fn main() {
    let input_dir = env::var("INPUT_DIR").expect("You must define INPUT_DIR environment variable");
    let library_dir = env::var("LIBRARY_DIR").expect("You must define LIBRARY_DIR environment variable");

    println!("Start watching new Episodes in directory {} to move into library {}", input_dir, library_dir);

    let organizer = MediaOrganizer { input_dir, library_dir };
    organizer.start_watching()
}
