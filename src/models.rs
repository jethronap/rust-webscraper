use std::collections::HashMap;

use serde::Serialize;

#[derive(Serialize)]
pub struct ExtractedElement {
    pub tag: String,
    pub content: String,
    pub attributes: Option<HashMap<String, String>>, // use a map or attributes
}

