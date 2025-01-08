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

Run the webscraper with a target URL a designated timeout and css selector:
```sh
cargo run -- --url https://example.com --timeout 15 --selector a
```

Run the websraper without cli arguments:
```sh
cargo run
```

The scraper will use the default values provided in the [configuration file](/config.toml).

Currently, the scraper saves extracted data to a `.json` file inside a `backup` folder at the root of the project. 

Below is a excerpt of the output when run without any args. 

```json
[
...

  {
    "tag": "a",
    "content": "\n        Read Contribution Guide\n      ",
    "attributes": {
      "class": "button button-secondary",
      "href": "https://rustc-dev-guide.rust-lang.org/getting-started.html"
    }
  },
  {
    "tag": "a",
    "content": "See individual contributors",
    "attributes": {
      "class": "button button-secondary",
      "href": "https://thanks.rust-lang.org/"
    }
  },
  {
    "tag": "a",
    "content": "See Foundation members",
    "attributes": {
      "class": "button button-secondary",
      "href": "https://foundation.rust-lang.org/members"
    }
  },
  {
    "tag": "a",
    "content": "Documentation",
    "attributes": {
      "href": "/learn"
    }
  },
  {
    "tag": "a",
    "content": "Rust Forge (Contributor Documentation)",
    "attributes": {
      "href": "http://forge.rust-lang.org"
    }
  },
  {
    "tag": "a",
    "content": "Ask a Question on the Users Forum",
    "attributes": {
      "href": "https://users.rust-lang.org"
    }
  },
  {
    "tag": "a",
    "content": "Code of Conduct",
    "attributes": {
      "href": "/policies/code-of-conduct"
    }
  },
  {
    "tag": "a",
    "content": "Licenses",
    "attributes": {
      "href": "/policies/licenses"
    }
  },
  {
    "tag": "a",
    "content": "Logo Policy and Media Guide",
    "attributes": {
      "href": "https://foundation.rust-lang.org/policies/logo-policy-and-media-guide/"
    }
  },
  ...
]
```

## Testing
Use the following command to run the unit tests
```sh
cargo test
```


## License

This project is licensed under the MIT License.