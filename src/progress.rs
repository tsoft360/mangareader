use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

const PROGRESS_FILE: &str = ".komadata.json";

#[derive(Debug, Serialize, Deserialize)]
pub struct ReadingProgress {
    pub manga_path: PathBuf,
    pub chapter: usize,
    pub page: usize,
}

pub save(progress: ReadingProgress) -> Result<(), Box<dyn std::error::Error>> {
    let json = serde_json::to_string_pretty(progress)?;

    let data_dir = dirs::data_dir()?;

    fs::write(PROGRESS_FILE, json)?;

    Ok(())
}