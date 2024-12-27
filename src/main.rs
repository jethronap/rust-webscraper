mod config;
mod cli_args;
mod data;

use config::load_config;
use cli_args::CliArgs;
use data::{ScrapedLink, save_to_json};
use clap::Parser;
use anyhow::Result;
use scraper::{Html, Selector};

#[tokio::main]
async fn main() -> Result<()> {
    let config = load_config()?;
    let cli_args = CliArgs::parse();


    let url = cli_args.url.or(config.url).expect("No URL provided");
    let timeout = cli_args.timeout.or(config.timeout).expect("No timeout provided");

    println!("Target URL: {}", url);
    println!("Request Timeout: {} seconds", timeout);

    let response = reqwest::get(&url).await?.text().await?;
    println!("Fetched document from {}", url);

    let document = Html::parse_document(&response);

    let selector = Selector::parse("a").unwrap();  // Select all anchor tags

    let mut scraped_links: Vec<ScrapedLink> = Vec::new();

    for element in document.select(&selector) {
        if let Some(link) = element.value().attr("href") {
            println!("Found link: {}", link);
            scraped_links.push(ScrapedLink { url: link.to_string() });
        }
    }

    // Save scraped data to JSON
    save_to_json(&scraped_links, "scraped_links.json")?;

    Ok(())

}
