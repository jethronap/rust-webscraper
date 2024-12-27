use anyhow::Result;
use scraper::{Html, Selector};

#[tokio::main]
async fn main() -> Result<()> {
    let url = "https://www.rust-lang.org";

    let response = reqwest::get(url).await?.text().await?;
    println!("Fetched document from {}", url);

    let document = Html::parse_document(&response);

    let selector = Selector::parse("a").unwrap();  // Select all anchor tags

    for element in document.select(&selector) {
        if let Some(link) = element.value().attr("href") {
            println!("Found link: {}", link);
        }
    }

    Ok(())

}
