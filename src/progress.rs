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

pub fn save(progress: ReadingProgress) -> Result<(), Box<dyn std::error::Error>> {
    let json = serde_json::to_string_pretty(&progress)?;

    let mut data_dir = dirs::data_dir()
        .ok_or("Could not find data directory")?;

    data_dir.push("koma");
    fs::create_dir_all(&data_dir)?;
    data_dir.push(PROGRESS_FILE);
    fs::write(data_dir, json)?;

    Ok(())
}

pub fn load() -> Result<ReadingProgress, Box<dyn std::error::Error>> {
    let mut data_dir = dirs::data_dir()
        .ok_or("Could not find data directory")?;

    data_dir.push("koma");
    data_dir.push(PROGRESS_FILE);
    
    let file = fs::read_to_string(data_dir)?;

    let progress: ReadingProgress = serde_json::from_str(&file)?;

    Ok(progress)
}