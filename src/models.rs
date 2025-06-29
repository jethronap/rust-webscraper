use std::collections::HashMap;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct ExtractedElement {
    pub tag: String,
    pub content: String,
    pub attributes: Option<HashMap<String, String>>, // use a map or attributes
}

#[derive(Debug, Deserialize, Serialize)]
pub struct PdfText {
    pub file: String,
    pub text: String,   // one string per page
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct ConsortiumMember {
    pub name: String,
    pub country: String,
    pub is_coordinator: bool,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct EdfProject {
    pub project_name: String,
    pub call_title: String,
    pub topic_title: String,
    pub duration_months: Option<u32>,
    pub activities: Vec<String>,
    pub estimated_cost: Option<f64>,
    pub max_eu_contribution: Option<f64>,
    pub description: String,
    pub consortium_members: Vec<ConsortiumMember>,
    pub source_file: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct EdfSummary {
    pub total_projects: usize,
    pub total_funding: f64,
    pub projects_by_call: std::collections::HashMap<String, usize>,
    pub projects_by_country: std::collections::HashMap<String, usize>,
    pub unique_participants: usize,
    pub projects: Vec<EdfProject>,
}
