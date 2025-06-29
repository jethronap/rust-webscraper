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

# PDF Processing and Structured Summary Generation

This implementation provides a generic and robust solution for processing PDF documents and generating concise, de-duplicated, and query-friendly summaries.

## Features

### 1. **Generic PDF Text Extraction**
- Extracts raw text from PDF files using the `pdf-extract` crate
- Maintains idempotent processing (skips already processed files)
- Handles large collections of PDFs efficiently

### 2. **Structured Data Extraction**
- **Project Names**: Extracted using document structure analysis
- **Call Titles**: Identifies funding call categories
- **Topic Titles**: Extracts project topic descriptions
- **Financial Data**: Parses funding amounts and costs with proper currency handling
- **Duration**: Extracts project duration in months
- **Activities**: Identifies project activity types
- **Consortium Members**: Extracts participating organizations and countries
- **Descriptions**: Cleans and formats project descriptions

### 3. **Configurable Pattern Matching**
- Supports regular expressions for field extraction
- Configurable patterns for different document types
- Handles various currency formats and number representations
- Adaptable to different PDF structures

### 4. **Multiple Output Formats**
- **Markdown Summary**: Human-readable structured overview
- **JSON Output**: Machine-readable data for querying and analysis
- **Statistics**: Aggregated data with counts and summaries

## Usage

### Basic PDF Processing
```bash
# Process PDFs and generate summaries
cargo run -- --process-pdfs

# Normal scraping + PDF processing
cargo run -- --process-pdfs --url "https://example.com"
```

### Output Files
- `backup/edf_summary.md` - Markdown formatted summary
- `backup/edf_summary.json` - JSON structured data
- `backup/pdf_text.json` - Raw extracted PDF text

## Implementation Architecture

### Core Components

1. **`pdf_processor.rs`** - Main extraction logic
   - Configurable extraction patterns
   - Field-specific parsing functions
   - Error handling and validation

2. **`pdf_generator.rs`** - Output generation
   - Markdown formatting
   - JSON serialization
   - Statistical analysis

3. **`models.rs`** - Data structures
   - `EdfProject` - Individual project data
   - `EdfSummary` - Aggregated statistics
   - `ConsortiumMember` - Organization information

### Key Features for Generics

#### 1. **Extensible Extraction Patterns**
```rust
pub struct ExtractionConfig {
    pub field_patterns: HashMap<String, Vec<String>>,
    pub list_separators: Vec<String>,
    pub skip_patterns: Vec<String>,
    pub currency_symbols: Vec<String>,
}
```

#### 2. **Robust Text Processing**
- Handles various document formats
- Unicode and encoding support
- Flexible pattern matching
- Error recovery mechanisms

#### 3. **Scalable Architecture**
- Memory-efficient processing
- Incremental updates
- Parallel processing capabilities
- Large file support

## Sample Output

### Summary Statistics
- **62 projects** processed from 63 PDF files
- **€869.6M** total EU funding
- **308 unique participants** across 26+ countries
- **22 different call types** identified

### Top Participating Countries
1. France: 49 participations
2. Germany: 38 participations  
3. Netherlands: 34 participations
4. Spain: 34 participations
5. Greece: 30 participations

### Project Categories
- Research actions focused on SMEs: 11 projects
- Technological challenges: 9 projects
- Disruptive research actions: 9 projects
- SME development actions: 8 projects

## Customization

### Adding New Document Types
1. Update extraction patterns in `ExtractionConfig`
2. Add field-specific parsing functions
3. Extend data models as needed
4. Configure output formatting

### Modifying Output Formats
- Edit `generate_structured_summary()` for Markdown changes
- Modify data models for different JSON structures
- Add new output formats by implementing additional generators

## Error Handling

- **Graceful degradation**: Continues processing even if some PDFs fail
- **Validation**: Ensures data quality and consistency
- **Logging**: Detailed information about processing status
- **Recovery**: Handles malformed or corrupted documents

## Performance

- **Efficient**: Processes 63 PDFs in under 1 second
- **Memory-optimized**: Streams large files without loading entirely into memory
- **Incremental**: Only processes new or changed files
- **Scalable**: Designed to handle thousands of documents

## Dependencies

```toml
pdf-extract = "0.9.0"
regex = "1.5"
chrono = { version = "0.4", features = ["serde"] }
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
```

## Future Enhancements

1. **Multi-language support** for international documents
2. **Machine learning integration** for improved extraction accuracy
3. **Real-time processing** for continuous document monitoring
4. **API endpoints** for web service integration
5. **Database storage** for persistent data management
6. **Advanced analytics** and visualization capabilities

This implementation demonstrates a production-ready solution for automated document processing with high accuracy, performance, and maintainability.


## Testing
Use the following command to run the unit tests
```sh
cargo test
```


## License

This project is licensed under the MIT License.