use crate::card::Card;
use dirs;
use std::fs::{create_dir, File, OpenOptions};
use std::io::{BufRead, BufReader, BufWriter, Write};
use std::iter::Iterator;
use std::path::PathBuf;

/// All Recall files are place to a directory inside users home directory. This
/// is name of the directory.
const RECALL_DIR_NAME: &str = ".recall";
/// File name of the file storing all cards.
const CARDS_FILE_NAME: &str = "cards.txt";

impl Card {
    /// Serialize `Card` to a `String` of a single line; with line-feed at the
    /// end.
    pub fn to_line(&self) -> String {
        let id = Card::serialize_id(self.id());
        format!("{}\t{}\t{}\n", id, self.question(), self.answer())
    }

    /// Parse `Card` from a `&str` of a single line (ending with line-feed).
    fn from_line(line: &str) -> Result<Card, String> {
        let parts: Vec<&str> = line.trim().split('\t').collect();

        if parts.len() != 3 {
            let reason = format!("Expected three TAB separated tokens, got: {}", line);
            return Err(reason);
        }

        let id: u64 = Card::parse_id(parts[0])?;
        let question = parts[1].to_string();
        let answer = parts[2].to_string();
        Ok(Card::new(id, question, answer))
    }
}

/// Append a slice `Card`-s into cards file. This opens cards wile in append
/// mode and writes at the end of it.
pub fn store_cards(cards: &[Card]) -> Result<(), String> {
    let cards_file_path = get_cards_file_path()?;

    let mut open_options = OpenOptions::new();
    open_options.append(true);
    let mut writer = match open_options.open(&cards_file_path) {
        Ok(file) => BufWriter::new(file),
        Err(error) => {
            let reason = format!(
                "Couldn't open file \"{}\": {}",
                cards_file_path.to_string_lossy(),
                error
            );
            return Err(reason);
        }
    };

    for card in cards {
        if let Err(error) = writer.write_all(card.to_line().as_bytes()) {
            let reason = format!(
                "Couldn't append to file \"{}\": {}",
                cards_file_path.to_string_lossy(),
                error
            );
            return Err(reason);
        }
    }

    if let Err(why) = writer.flush() {
        return Err(format!(
            "Couldn't append to file \"{}\": {}",
            cards_file_path.to_string_lossy(),
            why
        ));
    }

    Ok(())
}

pub struct CardsReader {
    error: bool,
    reader: BufReader<File>,
    line_nr: usize,
}

impl Iterator for CardsReader {
    type Item = Result<Card, String>;

    fn next(&mut self) -> Option<Result<Card, String>> {
        if self.error {
            return None;
        }

        let mut line = String::new();

        if let Err(error) = self.reader.read_line(&mut line) {
            self.error = true;
            let result = Err(format!("Couldn't read card file: {}", error));
            return Some(result);
        }

        if line.is_empty() {
            return None;
        }

        self.line_nr += 1;

        let result = Card::from_line(&line).map_err(|error| {
            self.error = true;
            format!("Error on line {}: {}", self.line_nr, error)
        });

        Some(result)
    }
}

/// Load cards gradually in form of an iterator from cards file.
pub fn read_cards() -> Result<CardsReader, String> {
    let cards_file_path = get_cards_file_path()?;

    let file = File::open(&cards_file_path)
        .map_err(|error| format!("Couldn't open card file: {}", error))?;

    Ok(CardsReader {
        error: false,
        reader: BufReader::new(file),
        line_nr: 0,
    })
}

/// This returns path to use's card file and creates recall directory and card
/// file along the way if necessary.
///
/// # Errors
///
/// In case of an I/O or other error a `String` with reason is returned.
fn get_cards_file_path() -> Result<PathBuf, String> {
    let mut file_path = get_recall_dir()?;

    file_path.push(&CARDS_FILE_NAME);
    if !file_path.exists() {
        if let Err(error) = File::create(&file_path) {
            let reason = format!(
                "Couldn't create \"{}\" file: {}",
                file_path.to_string_lossy(),
                error
            );
            return Err(reason);
        }
    }

    Ok(file_path)
}

/// Get directory where Recall configuration and other files are stored (possible
/// deeper in the hierarchy). This function creates the directory if it doesn't
/// exist.
pub fn get_recall_dir() -> Result<PathBuf, String> {
    let mut file_path = match dirs::home_dir() {
        Some(path_buf) => path_buf,
        None => return Err("Couldn't locate home directory.".to_string()),
    };

    file_path.push(&RECALL_DIR_NAME);
    if !file_path.exists() {
        if let Err(error) = create_dir(&file_path) {
            let reason = format!(
                "Couldn't create \"{}\" directory: {}",
                file_path.to_string_lossy(),
                error
            );
            return Err(reason);
        }
    }

    Ok(file_path)
}
