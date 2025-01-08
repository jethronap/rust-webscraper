use serde::Serialize;
use std::{fs, path::Path};
use anyhow::Result;


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

#[cfg(test)]
mod tests {
    use crate::models::ExtractedElement;

    use super::*;
    use std::{collections::HashMap, fs};

    #[test]
    fn test_save_to_json() {
        // Create test data
        let mut attributes = HashMap::new();
        attributes.insert("id".to_string(), "paragraph1".to_string());
        attributes.insert("class".to_string(), "example".to_string());
        
        let elements = vec![ExtractedElement {
            tag: "p".to_string(),
            content: "This is a paragraph.".to_string(),
            attributes: Some(attributes),
        },
        ExtractedElement {
            tag: "h1".to_string(),
            content: "Title".to_string(),
            attributes: None,
        }];

        // Save data to JSON
        save_to_json(&elements, "test_output.json").unwrap();

        // Verify file exists and contains the expected data
        let content = fs::read_to_string("backup/test_output.json").unwrap();
        assert!(content.contains("\"tag\": \"p\""));
        assert!(content.contains("\"content\": \"This is a paragraph.\""));
        assert!(content.contains("\"id\": \"paragraph1\""));
        assert!(content.contains("\"class\": \"example\""));

        // Clean up
        fs::remove_file("backup/test_output.json").unwrap();
    }
}