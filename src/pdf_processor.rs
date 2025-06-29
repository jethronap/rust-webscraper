use anyhow::Result;
use regex::Regex;
use std::collections::{HashMap, HashSet};
use crate::models::{PdfText, EdfProject, ConsortiumMember, EdfSummary};

// Note: Future versions could include a configurable ExtractionConfig struct
// for different document types, but current implementation uses fixed patterns

/// Extract structured project data from raw PDF text
pub fn extract_project_from_text(pdf_text: &PdfText) -> Result<Option<EdfProject>> {
    let text = &pdf_text.text;
    
    // Skip summary/overview documents
    if text.contains("KEY FIGURES OF EDF 2024") || 
       text.contains("HIGHLIGHTS OF EDF 2024 FUNDING") ||
       text.len() < 500 {
        return Ok(None);
    }

    let project_name = extract_project_name(text)?;
    let call_title = extract_call_title(text);
    let topic_title = extract_topic_title(text);
    let duration_months = extract_duration(text);
    let activities = extract_activities(text);
    let estimated_cost = extract_estimated_cost(text);
    let max_eu_contribution = extract_max_eu_contribution(text);
    let description = extract_description(text);
    let consortium_members = extract_consortium_members(text);

    Ok(Some(EdfProject {
        project_name,
        call_title,
        topic_title,
        duration_months,
        activities,
        estimated_cost,
        max_eu_contribution,
        description,
        consortium_members,
        source_file: pdf_text.file.clone(),
    }))
}

fn extract_project_name(text: &str) -> Result<String> {
    let lines: Vec<&str> = text.lines().collect();
    
    // Find the project name which appears right after "credit is given and any changes are indicated."
    for (i, line) in lines.iter().enumerate() {
        if line.trim() == "credit is given and any changes are indicated." {
            // The project name is the next non-empty line
            if i + 1 < lines.len() {
                let project_name = lines[i + 1].trim();
                if !project_name.is_empty() {
                    return Ok(project_name.to_string());
                }
            }
        }
    }
    
    Ok("Unknown Project".to_string())
}

fn extract_call_title(text: &str) -> String {
    let lines: Vec<&str> = text.lines().collect();
    
    // Find the call title which appears after the project name
    for (i, line) in lines.iter().enumerate() {
        if line.trim() == "credit is given and any changes are indicated." && i + 2 < lines.len() {
            let call_title = lines[i + 2].trim();
            if !call_title.is_empty() {
                return call_title.to_string();
            }
        }
    }
    
    "Unknown Call".to_string()
}

fn extract_topic_title(text: &str) -> String {
    let lines: Vec<&str> = text.lines().collect();
    
    // The topic title is usually after project description and before duration
    // Look for a line that contains descriptive text about the topic
    for (i, line) in lines.iter().enumerate() {
        // Skip until we find project description area
        if line.trim() == "credit is given and any changes are indicated." {
            // Look ahead for topic title - it's usually a longer descriptive line
            for j in 3..15 {
                if i + j < lines.len() {
                    let candidate = lines[i + j].trim();
                    
                    // Topic titles are descriptive and longer than 20 chars but less than 100
                    if candidate.len() > 20 && candidate.len() < 100 && 
                       !candidate.contains("Months") &&
                       !candidate.contains("€") &&
                       !candidate.contains("NAME") &&
                       !candidate.contains("COUNTRY") &&
                       (candidate.contains("for") || 
                        candidate.contains("and") ||
                        candidate.contains("of") ||
                        candidate.contains("in")) {
                        return candidate.to_string();
                    }
                }
            }
            break;
        }
    }
    
    "Unknown Topic".to_string()
}

fn extract_duration(text: &str) -> Option<u32> {
    let re = Regex::new(r"(\d+)\s+Months?").ok()?;
    if let Some(captures) = re.captures(text) {
        captures[1].parse().ok()
    } else {
        None
    }
}

fn extract_activities(text: &str) -> Vec<String> {
    if let Some(start) = text.find("TYPE(S) OF ACTIVITIES:") {
        if let Some(line_start) = text[start..].find('\n') {
            if let Some(line_end) = text[start + line_start + 1..].find('\n') {
                let activities_text = text[start + line_start + 1..start + line_start + 1 + line_end].trim();
                return activities_text
                    .split(',')
                    .map(|s| s.trim().to_string())
                    .filter(|s| !s.is_empty())
                    .collect();
            }
        }
    }
    Vec::new()
}

fn extract_estimated_cost(text: &str) -> Option<f64> {
    // Look for patterns like "3,938,942.86" or "160,087,115.24"
    let patterns = [
        r"ESTIMATED TOTAL COST:[^€\d]*€?\s*([\d,]+\.\d+)",
        r"€\s*([\d,]+\.\d+)\s*€\s*([\d,]+\.\d+)", // Two amounts on same line
        r"([\d,]+\.\d+)\s+([\d,]+\.\d+)\s+[A-Z]", // Numbers before project description
    ];
    
    for pattern in &patterns {
        if let Ok(re) = Regex::new(pattern) {
            if let Some(captures) = re.captures(text) {
                let cost_str = captures[1].replace(',', "");
                if let Ok(cost) = cost_str.parse::<f64>() {
                    return Some(cost);
                }
            }
        }
    }
    None
}

fn extract_max_eu_contribution(text: &str) -> Option<f64> {
    // Look for the second number in funding patterns
    let patterns = [
        r"MAXIMUM EU CONTRIBUTION[^€\d]*€?\s*([\d,]+\.\d+)",
        r"€\s*[\d,]+\.\d+\s*€\s*([\d,]+\.\d+)", // Second amount
        r"[\d,]+\.\d+\s+([\d,]+\.\d+)\s+[A-Z]", // Second number before project description
    ];
    
    for pattern in &patterns {
        if let Ok(re) = Regex::new(pattern) {
            if let Some(captures) = re.captures(text) {
                let cost_str = captures[1].replace(',', "");
                if let Ok(cost) = cost_str.parse::<f64>() {
                    return Some(cost);
                }
            }
        }
    }
    None
}

fn extract_description(text: &str) -> String {
    if let Some(start) = text.find("SHORT DESCRIPTION OF THE PROJECT:") {
        if let Some(desc_start) = text[start..].find('\n') {
            if let Some(members_start) = text[start + desc_start..].find("Members of the consortium") {
                let description = text[start + desc_start + 1..start + desc_start + members_start].trim();
                return clean_description(description);
            } else if let Some(name_start) = text[start + desc_start..].find("NAME") {
                let description = text[start + desc_start + 1..start + desc_start + name_start].trim();
                return clean_description(description);
            }
        }
    }
    "No description available".to_string()
}

fn clean_description(description: &str) -> String {
    description
        .lines()
        .filter(|line| {
            let trimmed = line.trim();
            !trimmed.is_empty() && 
            !trimmed.starts_with("©") &&
            !trimmed.contains("European Union, 202") &&
            !trimmed.contains("Reuse of this document")
        })
        .map(|line| line.trim())
        .collect::<Vec<_>>()
        .join(" ")
        .trim()
        .to_string()
}

fn extract_consortium_members(text: &str) -> Vec<ConsortiumMember> {
    let mut members = Vec::new();
    
    if let Some(start) = text.find("Members of the consortium") {
        let members_section = &text[start..];
        
        // Look for pattern: NAME followed by COUNTRY
        let lines: Vec<&str> = members_section.lines().collect();
        let mut i = 0;
        
        while i < lines.len() {
            let line = lines[i].trim();
            
            // Skip header lines and empty lines
            if line.is_empty() || 
               line.contains("Members of the consortium") ||
               line.contains("NAME") ||
               line.contains("OF THE ENTITY") ||
               line.contains("COUNTRY") ||
               line.contains("SELECTED PROJECTS") ||
               line.contains("EUROPEAN DEFENCE FUND") {
                i += 1;
                continue;
            }
            
            // Extract member name and country
            if let Some((name, country)) = parse_member_line(line) {
                let is_coordinator = name.contains("(Coordinator)");
                let clean_name = name.replace("(Coordinator)", "").trim().to_string();
                
                members.push(ConsortiumMember {
                    name: clean_name,
                    country,
                    is_coordinator,
                });
            }
            
            i += 1;
        }
    }
    
    members
}

fn parse_member_line(line: &str) -> Option<(String, String)> {
    // Dynamic country detection using multiple strategies
    
    // Strategy 1: Look for capitalized words at the end (likely country names)
    let words: Vec<&str> = line.split_whitespace().collect();
    if words.len() >= 2 {
        let last_word = words[words.len() - 1];
        let second_last = words[words.len() - 2];
        
        // Check if the last word looks like a country (capitalized, reasonable length)
        if is_likely_country(last_word) {
            let name_part = words[..words.len() - 1].join(" ").trim().to_string();
            if !name_part.is_empty() && !name_part.contains("COUNTRY") {
                return Some((name_part, last_word.to_string()));
            }
        }
        
        // Check for two-word countries like "Czech Republic", "The Netherlands"
        if words.len() >= 3 && is_likely_country(last_word) && is_likely_country_part(second_last) {
            let country = format!("{} {}", second_last, last_word);
            let name_part = words[..words.len() - 2].join(" ").trim().to_string();
            if !name_part.is_empty() && !name_part.contains("COUNTRY") {
                return Some((name_part, country));
            }
        }
    }
    
    // Strategy 2: Fallback to known patterns for edge cases
    if let Some(result) = parse_with_known_patterns(line) {
        return Some(result);
    }
    
    None
}

fn is_likely_country(word: &str) -> bool {
    // A word is likely a country if it:
    // 1. Starts with a capital letter
    // 2. Is between 3-20 characters (reasonable country name length)
    // 3. Contains only letters (and possibly hyphens/apostrophes)
    // 4. Is not a common organization word
    
    if word.len() < 3 || word.len() > 20 {
        return false;
    }
    
    let first_char = word.chars().next().unwrap_or(' ');
    if !first_char.is_uppercase() {
        return false;
    }
    
    // Check if it contains only valid country name characters
    let valid_chars = word.chars().all(|c| {
        c.is_alphabetic() || c == '-' || c == '\'' || c == '.'
    });
    
    if !valid_chars {
        return false;
    }
    
    // Exclude common organization words that might be capitalized
    let excluded_words = [
        "LTD", "LLC", "INC", "CORP", "SA", "SPA", "SRL", "GMBH", "AS", "OY", "AB",
        "SYSTEMS", "TECHNOLOGIES", "SOLUTIONS", "SERVICES", "RESEARCH", "INSTITUTE",
        "UNIVERSITY", "CENTRE", "CENTER", "GROUP", "COMPANY", "ENTERPRISES",
        "FOUNDATION", "ASSOCIATION", "ORGANIZATION", "DEFENCE", "DEFENSE"
    ];
    
    !excluded_words.contains(&word.to_uppercase().as_str())
}

fn is_likely_country_part(word: &str) -> bool {
    // Words that commonly appear as the first part of country names
    let country_prefixes = ["The", "United", "Czech", "New", "South", "North", "West", "East"];
    country_prefixes.contains(&word)
}

fn parse_with_known_patterns(line: &str) -> Option<(String, String)> {
    // Fallback patterns for known multi-word countries or special cases
    let known_countries = [
        "Austria", "Belgium", "Bulgaria", "Croatia", "Cyprus", "Czechia", "Denmark",
        "Estonia", "Finland", "France", "Germany", "Greece", "Hungary", "Ireland",
        "Italy", "Latvia", "Lithuania", "Luxembourg", "Malta", "Netherlands",
        "Poland", "Portugal", "Romania", "Slovakia", "Slovenia", "Spain", "Sweden",
        "Norway", "Israel", "Switzerland", "Ukraine", "Turkey", "Iceland",
        "The Netherlands", "Czech Republic", "United Kingdom", "United States",
        "New Zealand", "South Korea", "North Korea", "South Africa"
    ];
    
    // Sort by length (longest first) to match multi-word countries first
    let mut sorted_countries = known_countries.to_vec();
    sorted_countries.sort_by(|a, b| b.len().cmp(&a.len()));
    
    for country in &sorted_countries {
        if line.ends_with(country) {
            let name_part = line[..line.len() - country.len()].trim();
            if !name_part.is_empty() && !name_part.contains("COUNTRY") {
                return Some((name_part.to_string(), country.to_string()));
            }
        }
    }
    
    None
}

/// Process all PDF text entries and extract structured project data
pub fn process_pdf_texts(pdf_texts: &[PdfText]) -> Result<EdfSummary> {
    let mut projects = Vec::new();
    let mut projects_by_call = HashMap::new();
    let mut projects_by_country = HashMap::new();
    let mut unique_participants = HashSet::new();
    
    for pdf_text in pdf_texts {
        if let Some(project) = extract_project_from_text(pdf_text)? {
            // Count projects by call title
            *projects_by_call.entry(project.call_title.clone()).or_insert(0) += 1;
            
            // Count participants by country
            for member in &project.consortium_members {
                *projects_by_country.entry(member.country.clone()).or_insert(0) += 1;
                unique_participants.insert(member.name.clone());
            }
            
            projects.push(project);
        }
    }
    
    let total_funding = projects
        .iter()
        .filter_map(|p| p.max_eu_contribution)
        .sum();
    
    Ok(EdfSummary {
        total_projects: projects.len(),
        total_funding,
        projects_by_call,
        projects_by_country,
        unique_participants: unique_participants.len(),
        projects,
    })
}
