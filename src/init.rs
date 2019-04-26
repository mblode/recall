use std::fs::{canonicalize, create_dir};
use std::path::Path;
use std::fs::File;
use std::io::Write;
use std::env;

use exitfailure::ExitFailure;
use failure::ResultExt;

const CONFIG: &str = r#"
{
    "name": "Recall",
    "decks": {
        "id": 1,
        "name": "My first deck",
        "cards": [
            {
                "question": "Example question?",
                "answer": "Example answer."
            },
            {
                "question": "Example question 2?",
                "answer": "Example answer 0."
            }
        ]
    }
}
"#;

pub fn create_file(path: &Path, content: &str) -> Result<(), ExitFailure> {
    let mut file = File::create(&path)
        .with_context(|_| format!("Failed to create file"))?;
    file.write_all(content.as_bytes())?;
    Ok(())
}

pub fn init_recall () -> Result<(), ExitFailure> {
    let config = CONFIG.trim_start();
    let path = env::current_dir()?;

    create_file(&path.join("recall.json"), &config)?;

    println!("Successfully initialised Recall at {:?}", canonicalize(path).unwrap());
    Ok(())
}
