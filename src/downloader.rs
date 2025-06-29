use anyhow::{anyhow, Context, Result};
use reqwest::Client;
use std::{collections::HashMap, fs, path::Path};
use url::Url;

use crate::{config::Config, models::ExtractedElement};

/// Read `json_path`, find <a> elements whose attribute `cfg.pdf_selector`
/// (default `"data-wt-preview"`) indicates a PDF, and return absolute URLs.
pub fn collect_pdf_links(json_path: &str, cfg: &Config) -> Result<Vec<Url>> {
    // ------------------------------------------------------------------ load
    let raw = fs::read_to_string(json_path)
        .with_context(|| format!("Cannot read {json_path}"))?;

    let elements: Vec<ExtractedElement> =
        serde_json::from_str(&raw).context("JSON deserialisation failed")?;

    // ------------------------------------------------------------------ base
    let base = Url::parse(
        cfg.url
            .as_deref()
            .ok_or_else(|| anyhow!("`url` missing in config"))?,
    )
    .context("Invalid base URL")?;

    let pdf_attr = cfg
        .pdf_selector
        .as_deref()
        .unwrap_or("data-wt-preview"); // sensible default


    let tag_filter = cfg.selector.as_deref();

    let mut pdf_urls = Vec::new();

    for el in elements {
        if let Some(tag) = tag_filter {
            if el.tag != tag {
                continue; // tag does not match selector
            }
        }

        let attrs: &HashMap<String, String> = match &el.attributes {
            Some(m) => m,
            None => continue, // no attributes at all
        };

        // must have the attribute key at all
        let value = match attrs.get(pdf_attr) {
            Some(v) => v,
            None => continue,
        };

        // if the attribute has a value, enforce case-insensitive "pdf"
        if !value.eq_ignore_ascii_case("pdf") {
            continue;
        }

        let href = match attrs.get("href") {
            Some(h) => h,
            None => continue,
        };

        let url = base
            .join(href)
            .with_context(|| format!("Cannot join {base} with {href}"))?;
        pdf_urls.push(url);
    }

    Ok(pdf_urls)
}

/// Download every URL into `output_dir`.
/// The filename is the last path segment; ".pdf" is appended if missing.
pub async fn download_pdfs(urls: &[Url], output_dir: &str) -> Result<()> {
    fs::create_dir_all(output_dir)
        .with_context(|| format!("Cannot create {output_dir}"))?;

    let client = Client::builder().build()?;

    for url in urls {
        // --- derive target filename ----------------------------------------
        let mut filename = url
            .path_segments()
            .and_then(|s| s.last())
            .filter(|s| !s.is_empty())
            .unwrap_or("download")
            .to_owned();

        if !filename.to_ascii_lowercase().ends_with(".pdf") {
            filename.push_str(".pdf");
        }

        let path = Path::new(output_dir).join(&filename);
        if path.exists() {
            println!("Skip {}, already downloaded", path.display());
            continue; // idempotent: do nothing
        }

        // --- perform download ----------------------------------------------
        println!("Downloading {}", filename);
        let bytes = client
            .get(url.clone())
            .send()
            .await?
            .error_for_status()?
            .bytes()
            .await?;

        fs::write(&path, &bytes)
            .with_context(|| format!("Cannot write {:?}", path))?;
        println!("Saved {}", path.display());
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::Config;

    #[test]
    fn finds_pdf_links_with_custom_selector() {
        // ---------------------------------------------------- prepare config
        let cfg = Config {
            url: Some("https://host".into()),
            timeout: None,
            selector: Some("a".into()),          // we expect <a> tags
            pdf_selector: Some("data-wt-preview".into()),
        };

        // -------------------------------------------------- sample JSON file
        let sample = r#"[{
            "tag":"a",
            "content":"PDF",
            "attributes":{"href":"/f.pdf","data-wt-preview":"pdf"}
        }]"#;
        fs::write("tmp.json", sample).unwrap();

        // ----------------------------------------------------------- collect
        let urls = collect_pdf_links("tmp.json", &cfg).unwrap();
        assert_eq!(urls.len(), 1);
        assert_eq!(urls[0].as_str(), "https://host/f.pdf");

        fs::remove_file("tmp.json").unwrap();
    }
}