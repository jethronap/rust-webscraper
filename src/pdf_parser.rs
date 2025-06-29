use anyhow::{Context, Result};
use std::{collections::HashSet, fs, path::Path};
use crate::models::{PdfText};


// Read `output_json` if it exists and return a set of filenames already done.
fn already_parsed(output_json: &Path) -> Result<(Vec<PdfText>, HashSet<String>)> {
    if !Path::new(output_json).exists() {
        return Ok((Vec::new(), HashSet::new()));
    }
    let file = fs::File::open(output_json)
        .with_context(|| format!("Cannot open {}", output_json.display()))?;
    let vec: Vec<PdfText> = serde_json::from_reader(file)
        .with_context(|| "Cannot decode existing pdf_text.json")?;
    let set = vec.iter().map(|p| p.file.clone()).collect();
    Ok((vec, set))
}

/// Extract plain text from every NEW PDF in `dir` and merge results into
/// `output_json`.  Skips files that are already listed in the JSON.
pub fn parse_and_save(dir: &str, output_json: &Path) -> Result<()> {
    let (mut all_entries, done_set) = already_parsed(output_json)?;

    let mut new_count = 0;
    for entry in fs::read_dir(dir)? {
        let path = entry?.path();
        if path.extension().map(|e| e.eq_ignore_ascii_case("pdf")) != Some(true) {
            continue; // not a PDF
        }
        let fname = path.file_name().unwrap().to_string_lossy().into_owned();
        if done_set.contains(&fname) {
            println!("Skip {fname}, already parsed");
            continue; // idempotent: we parsed it before
        }

        // ------------------------- extract text with pdf_extract ------------
        let bytes = fs::read(&path)
            .with_context(|| format!("Cannot read {:?}", path))?;
        let raw = pdf_extract::extract_text_from_mem(&bytes)
            .with_context(|| format!("Cannot parse {:?}", path))?;

        // simple clean-up: drop leading/trailing whitespace, compress blanks
        let text = raw
            .lines()
            .map(str::trim)
            .filter(|l| !l.is_empty())
            .collect::<Vec<_>>()
            .join("\n");

        all_entries.push(PdfText { file: fname, text });
        new_count += 1;
    }

    if new_count == 0 {
        println!("No new PDFs to parse");
        return Ok(()); // nothing changed, keep old file as-is
    }

    // ------------------------------------------------------------------ save
    let file = fs::File::create(output_json)
        .with_context(|| format!("Cannot create {}", output_json.display()))?;
    serde_json::to_writer_pretty(file, &all_entries)?;
    println!("Parsed {} new PDF(s), saved to {}", new_count, output_json.display());
    Ok(())
}