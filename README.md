# Rust Webscraper

This is a simple webscraper written in Rust. It allows you to fetch and parse HTML content from web pages.

## Features

- Fetch HTML content from a given URL
- Parse and extract data from HTML

## Requirements

- Rust (latest stable version)

## Installation

1. Clone the repository:
    ```sh
    git clone https://github.com/yourusername/rust-webscraper.git
    ```
2. Navigate to the project directory:
    ```sh
    cd rust-webscraper
    ```
3. Build the project:
    ```sh
    cargo build
    ```

## Usage

Run the webscraper with a target URL and designated timeout:
```sh
cargo run -- --url https://example.com --timeout 15
```

Run the websraper without cli arguments:
```sh
cargo run
```

The scraper will use the default values provided in the configuration file.

## License

This project is licensed under the MIT License.