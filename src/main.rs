mod config;
mod cli_args;
mod data;
mod models;

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
            .collect::<Vec<_>>();

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
    use scraper::{Html, Selector};

    #[test]
    fn test_extract_elements_with_selector() {
        // Mock HTML content
        let html = r#"
            <html>
                <body>
                    <h1>Title</h1>
                    <p>Paragraph 1</p>
                    <p>Paragraph 2</p>
                </body>
            </html>
        "#;

        let document = Html::parse_document(html);
        let selector = Selector::parse("p").unwrap(); // Extract all <p> elements

        let mut elements = Vec::new();
        for element in document.select(&selector) {
            elements.push(element.inner_html());
        }

        assert_eq!(elements, vec!["Paragraph 1", "Paragraph 2"]);
    }
}