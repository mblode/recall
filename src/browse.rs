use serde::{Serialize, Deserialize};
use std::fs::File;
use std::path::Path;
use serde_json;

use exitfailure::ExitFailure;
use failure::ResultExt;

#[derive(Deserialize, Debug)]
struct Deck {
    x: String,
}

pub fn browse_decks (name: &str) -> Result<(), ExitFailure> {
    let json_file_path = Path::new("path/to/file.json");
    let json_file = File::open(json_file_path).expect("file not found");
    let deserialized_camera: Deck =
        serde_json::from_reader(json_file).expect("error while reading json");

    Ok(())
}
