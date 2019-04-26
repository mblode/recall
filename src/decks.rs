use std::fs::{canonicalize, create_dir};
use std::path::Path;
use std::fs::File;
use std::io::Write;

use exitfailure::ExitFailure;
use failure::ResultExt;

pub fn create_file(path: &Path, content: &str) -> Result<(), ExitFailure> {
    let mut file = File::create(&path)
        .with_context(|_| format!("Failed to create file"))?;
    file.write_all(content.as_bytes())?;
    Ok(())
}

pub fn new_site (name: &str) -> Result<(), ExitFailure> {
    let path = Path::new(name);

    create_file(&path.join("config.toml"), &config)?;

    Ok(())
}
