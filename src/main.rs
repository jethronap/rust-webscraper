mod config;

use config::load_config;
use anyhow::Result;
use scraper::{Html, Selector};

#[tokio::main]
async fn main() -> Result<()> {
    let config = load_config()?;
    println!("Target URL: {}", config.url);
    println!("Request Timeout: {} seconds", config.timeout);

    let response = reqwest::get(config.url).await?.text().await?;
    // println!("Fetched document from {}", config.url.to_string());

    let document = Html::parse_document(&response);

    let selector = Selector::parse("a").unwrap();  // Select all anchor tags

    for element in document.select(&selector) {
        if let Some(link) = element.value().attr("href") {
            println!("Found link: {}", link);
        }
    }

    Ok(())

}
