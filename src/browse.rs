use std::env;
use std::path::Path;
use std::fs::File;

use exitfailure::ExitFailure;
use failure::ResultExt;

#[derive(Serialize, Deserialize)]
struct Deck {
    name: String,
    decks: Vec<String>,
}

pub fn browse_decks (_name: Option<&str>) -> Result<(), ExitFailure> {
    let path = env::current_dir()?;

    // let json_file_path = Path::new(&path.join("recall.json"));
    // let json_file = File::open(json_file_path)?;
    // let decks: Deck = serde_json::from_reader(json_file)?;

    // println!("{:?} and {}", decks.name, decks.decks[0]);

    Ok(())
}
