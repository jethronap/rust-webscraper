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

    // The CSS selector to extract elements
    #[arg(short, long)]
    pub selector: Option<String>,

    #[arg(long)]
    pub force_scrape: bool,
    
    /// Process extracted PDF texts into structured summary
    #[arg(long)]
    pub process_pdfs: bool,
}