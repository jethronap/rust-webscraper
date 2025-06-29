mod downloader;
mod config;
mod cli_args;
mod data;
mod models;
mod pdf_parser;
mod pdf_processor;
mod pdf_generator;

use std::collections::HashMap;

use config::load_config;
use cli_args::CliArgs;
use data::save_to_json;
use clap::Parser;
use anyhow::Result;
use std::{path::Path};
use models::ExtractedElement;
use scraper::{Html, Selector};
use downloader::{collect_pdf_links, download_pdfs};
use pdf_parser::parse_and_save;
use pdf_processor::process_pdf_texts;
use pdf_generator::{generate_structured_summary, generate_json_summary};
use models::PdfText;


#[tokio::main]
async fn main() -> Result<()> {
    let cfg = load_config("config.toml")?;
    let cli_args = CliArgs::parse();

    let json_path = "backup/extracted_elements.json";

    let url = cli_args
    .url
    .as_deref()
    .or(cfg.url.as_deref())
    .expect("No URL provided")
    .to_owned();           // owned String

    let timeout = cli_args
    .timeout
    .or(cfg.timeout)
    .expect("No timeout provided");

    let selector_str = cli_args
    .selector
    .as_deref()                       
    .or(cfg.selector.as_deref())
    .expect("No selector provided")
    .to_owned();                      

    println!("Target URL: {}", url);
    println!("Request Timeout: {} seconds", timeout);
    println!("Using selector: {}", selector_str);

    if !Path::new(json_path).exists() {
        println!("⏳ scraping (no cache yet) …");

        let response = reqwest::get(&url).await?.text().await?;
        let document = Html::parse_document(&response);
        println!("Fetched document from {}", url);
        let selector = Selector::parse(&selector_str).expect("Invalid CSS selector");

        let mut extracted_elements: Vec<ExtractedElement> = Vec::new();

        for element in document.select(&selector) {
            let tag = element.value().name().to_string();
            let content = element.inner_html();

            // Collect attributes
            let attributes = element
                .value()
                .attrs()
                .map(|(key, value)| (key.to_string(), value.to_string()))
                .collect::<HashMap<_, _>>();

            extracted_elements.push(ExtractedElement {
                tag,
                content,
                attributes: if attributes.is_empty() {
                    None
                } else {
                    Some(attributes)
                },
            });
        }

        // Save scraped data (helper puts it inside backup/)
        save_to_json(&extracted_elements, "extracted_elements.json")?;
        println!("Saved {}", json_path);
    } else {
        println!("Cached scrape found at {}", json_path);
    }

    let pdf_urls = collect_pdf_links(json_path, &cfg)?;
    download_pdfs(&pdf_urls, "backup").await?; // PDFs go to backup/

    parse_and_save("backup", std::path::Path::new("backup/pdf_text.json"))?;

    // Check if we should process PDFs into structured format
    if cli_args.process_pdfs {
        process_pdfs_to_summary()?;
    }

    Ok(())

}

/// Process existing PDF text data into structured summaries
fn process_pdfs_to_summary() -> Result<()> {
    let pdf_text_path = "backup/pdf_text.json";
    
    if !Path::new(pdf_text_path).exists() {
        println!("No PDF text file found at {}", pdf_text_path);
        return Ok(());
    }
    
    println!("Processing PDF texts into structured summary...");
    
    // Load the existing PDF text data
    let file = std::fs::File::open(pdf_text_path)?;
    let pdf_texts: Vec<PdfText> = serde_json::from_reader(file)?;
    
    // Process into structured format
    let summary = process_pdf_texts(&pdf_texts)?;
    
    println!("Extracted {} projects from {} PDF files", 
             summary.total_projects, pdf_texts.len());
    
    // Generate structured text summary
    generate_structured_summary("backup/edf_summary.md", &summary)?;
    println!("Generated structured summary: backup/edf_summary.md");
    
    // Generate JSON for querying
    generate_json_summary("backup/edf_summary.json", &summary)?;
    println!("Generated JSON summary: backup/edf_summary.json");
    
    Ok(())
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use scraper::{Html, Selector};
    use crate::models::ExtractedElement;

    #[test]
    fn test_scraper() {
        // Mock HTML content
        let html = r#"
            <html>
            <body>
                <h1>Title</h1>
                <p class="example" id="paragraph1">This is a paragraph.</p>
                <p>Another paragraph.</p>
            </body>
        </html>
        "#;

        let document = Html::parse_document(html);
        let selector = Selector::parse("p").unwrap(); // Extract all <p> elements

        let mut extracted_elements = Vec::new();

        for element in document.select(&selector) {
            let tag = element.value().name().to_string();
            let content = element.inner_html();
            let attributes = element
                .value()
                .attrs()
                .map(|(key, value)| (key.to_string(), value.to_string()))
                .collect::<HashMap<_, _>>();

            extracted_elements.push(ExtractedElement {
                tag,
                content,
                attributes: if attributes.is_empty() { None } else { Some(attributes) },
            });
        }

        assert_eq!(extracted_elements.len(), 2);
        assert_eq!(extracted_elements[0].tag, "p");
        assert_eq!(extracted_elements[0].content, "This is a paragraph.");
        let first_attributes = extracted_elements[0].attributes.as_ref().unwrap();
        assert_eq!(first_attributes.get("id").unwrap(), "paragraph1");
        assert_eq!(first_attributes.get("class").unwrap(), "example");

        assert_eq!(extracted_elements[1].tag, "p");
        assert_eq!(extracted_elements[1].content, "Another paragraph.");
        assert!(extracted_elements[1].attributes.is_none());
    }

    #[test]
    fn test_extracted_element_serialization() {
        use::std::collections::HashMap;

        let mut attributes = HashMap::new();
        attributes.insert("id".to_string(), "paragraph1".to_string());
        attributes.insert("class".to_string(), "example".to_string());

        let element = ExtractedElement {
            tag: "p".to_string(),
            content: "Paragraph 1".to_string(),
            attributes: Some(attributes),
        };

        let json = serde_json::to_string_pretty(&element).unwrap();
        assert!(json.contains("\"tag\": \"p\""));
        assert!(json.contains("\"content\": \"Paragraph 1\""));
        assert!(json.contains("\"class\": \"example\""));
    }
}