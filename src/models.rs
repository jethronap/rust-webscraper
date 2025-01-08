use serde::Serialize;

#[derive(Serialize)]
pub struct ExtractedElement {
    pub tag: String,
    pub content: String,
    pub attributes: Option<Vec<(String, String)>>, // e.g., vec![("id", "header")]
}

