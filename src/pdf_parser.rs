use anyhow::{Context, Result};
use std::{fs, path::Path};
use crate::models::{PdfText};

/// Scan `dir` for *.pdf, extract text, return a Vec<PdfText>.
pub fn parse_pdfs_in_dir(dir: &str) -> Result<Vec<PdfText>> {
    let mut out = Vec::new();

    for entry in fs::read_dir(dir)? {
        let path = entry?.path();
        if path.extension().map(|e| e.eq_ignore_ascii_case("pdf")) != Some(true) {
            continue;
        }
        let bytes = fs::read(&path)
            .with_context(|| format!("Cannot read {:?}", path))?;

        // --- pdf_extract API -------------------------------------------------
        let mut pages = Vec::new();
        for page in pdf_extract::extract_text_from_mem(&bytes)
            .with_context(|| format!("Cannot parse {:?}", path))?
            .split("\x0C")
        {
            pages.push(page.trim().to_owned());
        }
    out.push(PdfText {
            file: path.file_name().unwrap().to_string_lossy().into_owned(),
            pages,
        });
    }
    Ok(out)

}

/// Helper: parse all PDFs in `dir` and write pretty JSON to `output_path`.
pub fn parse_and_save(dir:&str, output_path: &Path)-> Result<()> {
    let data = parse_pdfs_in_dir(dir)?;
    let file = fs::File::create(output_path)?;
    serde_json::to_writer_pretty(file, &data)?;
    println!("Saved parsed text to {}", output_path.display());
    Ok(())
}