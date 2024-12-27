use::clap::Parser;

// Cli args for the web scraper
#[derive(Parser, Debug)]
#[command(author,version, about, long_about = None)]
pub struct CliArgs {
    // The URL to scrape
    #[arg(short, long)]
    pub url: Option<String>,

    // The request timeout
    #[arg(short, long)]
    pub timeout: Option<u64>,
}