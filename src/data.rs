use serde::Serialize;
use std::{fs, path::Path};
use anyhow::Result;

#[derive(Serialize)]
pub struct ScrapedLink {
    pub url: String,
}

/// Serialize and save data to a JSON file inside the `backup` folder
pub fn save_to_json<T: Serialize>(data: &T, file_path: &str) -> Result<()> {
    // Ensure backup folder exists
    let backup_dir = "backup";
    if !Path::new(backup_dir).exists() {
        fs::create_dir(backup_dir)
        .expect("Failed to create backup directory");
    }

    // Construct file path
    let file_path = format!("{}/{}", backup_dir, file_path);

    // Serialise data to JSON
    let json_data = serde_json::to_string_pretty(data)
        .expect("Failed to serialize scraped links");
    // Write data to file
    fs::write(&file_path, json_data)
    .expect("Failed to write to file");

    println!("Data saved to {}", file_path);
    Ok(())
}