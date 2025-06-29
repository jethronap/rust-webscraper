use std::collections::HashMap;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct ExtractedElement {
    pub tag: String,
    pub content: String,
    pub attributes: Option<HashMap<String, String>>, // use a map or attributes
}

#[derive(Debug, Serialize)]
pub struct PdfText {
    pub file: String,
    pub pages: Vec<String>,   // one string per page
}
