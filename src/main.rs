mod config;
mod cli_args;
mod data;
mod models;

use std::collections::HashMap;

use config::load_config;
use cli_args::CliArgs;
use data::save_to_json;
use clap::Parser;
use anyhow::Result;
use models::ExtractedElement;
use scraper::{Html, Selector};

#[tokio::main]
async fn main() -> Result<()> {
    let config = load_config("config.toml")?;
    let cli_args = CliArgs::parse();


    let url = cli_args.url.or(config.url).expect("No URL provided");
    let timeout = cli_args.timeout.or(config.timeout).expect("No timeout provided");

    println!("Target URL: {}", url);
    println!("Request Timeout: {} seconds", timeout);

    let response = reqwest::get(&url).await?.text().await?;
    println!("Fetched document from {}", url);

    let document = Html::parse_document(&response);

    let selector_str = cli_args.selector.or(config.selector).expect("No selector provided");

    println!("Using selector: {}", selector_str);

    // Parse the CSS selector
    let selector = Selector::parse(&selector_str).expect("Invalid CSS selector");

    let mut extracted_elements: Vec<ExtractedElement> = Vec::new();

    for element in document.select(&selector) {

        // Get the tag name (e.g., "p", "h1")
        let tag = element.value().name().to_string();

        // Get the inner HTML content
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
            attributes: if attributes.is_empty() { None } else { Some(attributes) },
        });
    }

    // Save scraped data to JSON
    save_to_json(&extracted_elements, "extracted_elements.json")?;

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