use anyhow::{Context, Result};
use reqwest::Client;
use std::{fs, path::Path};
use url::Url;

use crate::models::ExtractedElement;

/// Return all absolute PDF URLs found in the JSON file
pub fn collect_pdf_links(json_path: &str, base_url: &str)-> Result<Vec<Url>> {
    let data = fs::read_to_string(json_path)
        .with_context(|| format!("Cannot reaf {}", json_path))?;
    
    let elements: Vec<ExtractedElement> = serde_json::from_str(&data)
        .with_context(|| "JSON deserialisation failed")?;

    let base = Url::parse(base_url)
        .with_context(|| format!("Invalid base URL {}", base_url))?;

    let mut pdf_urls = Vec::new();

    for el in elements {
        if el.tag != "a" {
            continue;
        }
        let attrs = match &el.attributes {
            Some(map) => map,
            None => continue,
        };
        if attrs.get("data-wt-preview").map(String::as_str) != Some("pdf") {
            continue;
        }
        let href = match attrs.get("href") {
            Some(h) => h,
            None => continue,
        };

        // Join relative link with base URL
        let url = base.join(href)
            .with_context(|| format!("Cannot join {} with {}", base, href))?;
        pdf_urls.push(url);
    }
    Ok(pdf_urls)
    }


/// Download every URL into `output_dir`; file names are taken from the URL’s
/// last path segment (falls back to GUID if absent).
pub async fn download_pdfs(urls: &[Url], output_dir: &str) -> Result<()> {
    fs::create_dir_all(output_dir)
        .with_context(|| format!("Cannot create outpur dir {}", output_dir))?;

    let client = Client::builder().build()?;

    for url in urls {
        let resp = client.get(url.clone()).send().await?
            .error_for_status()?;
        let bytes = resp.bytes().await?;

        let mut filename = url
            .path_segments()
            .and_then(|s| s.last())
            .filter(|s| !s.is_empty())
            .unwrap_or("download.pdf")
            .to_string();
        
        if !filename.to_ascii_lowercase().ends_with(".pdf") {
           filename.push_str(".pdf");
        }

        let path = Path::new(output_dir).join(filename);
        fs::write(&path, &bytes)
            .with_context(|| format!("Cannot write {:?}", path))?;
        println!("Saved {}", path.display());
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn extract_pdf_links_from_json() {
        let sample = r#"
        [
            {
                "tag":"a",
                "content":"PDF 1",
                "attributes":{
                    "href":"/file/doc1.pdf",
                    "data-wt-preview":"pdf"
                }
            },
            {
                "tag":"a",
                "content":"HTML link",
                "attributes":{
                    "href":"https://example.com",
                    "data-wt-preview":"html"
                }
            }
        ]
        "#;
        fs::write("test.json", sample).unwrap();
        let urls = collect_pdf_links("test.json", "https://base.example").unwrap();
        fs::remove_file("test.json").unwrap();
        assert_eq!(urls.len(), 1);
        assert_eq!(urls[0].as_str(), "https://base.example/file/doc1.pdf");
    }
}