use std::fs::File;
use std::io::Write;
use std::collections::BTreeSet;
use anyhow::Result;
use crate::models::{EdfSummary, EdfProject};

/// Generate a concise structured summary that can be converted to PDF
pub fn generate_structured_summary(output_file: &str, summary: &EdfSummary) -> Result<()> {
    let mut file = File::create(output_file)?;
    
    // Write header
    writeln!(file, "# EDF 2024 PROJECT SUMMARY")?;
    writeln!(file, "Generated: {}", chrono::Utc::now().format("%Y-%m-%d %H:%M:%S UTC"))?;
    writeln!(file)?;
    
    // Overview statistics
    writeln!(file, "## OVERVIEW")?;
    writeln!(file, "- Total Projects: {}", summary.total_projects)?;
    writeln!(file, "- Total EU Funding: €{:.1}M", summary.total_funding / 1_000_000.0)?;
    writeln!(file, "- Unique Participants: {}", summary.unique_participants)?;
    writeln!(file)?;
    
    // Projects by call type
    writeln!(file, "## PROJECTS BY CALL TYPE")?;
    let mut projects_by_call: Vec<(&String, &usize)> = summary.projects_by_call.iter().collect();
    projects_by_call.sort_by(|a, b| b.1.cmp(a.1));
    
    for (call, count) in projects_by_call.iter() {
        writeln!(file, "- {}: {} projects", call, count)?;
    }
    writeln!(file)?;
    
    // Country participation
    writeln!(file, "## COUNTRY PARTICIPATION")?;
    let mut countries: Vec<(&String, &usize)> = summary.projects_by_country.iter().collect();
    countries.sort_by(|a, b| b.1.cmp(a.1));
    
    for (country, count) in countries.iter().take(15) { // Top 15 countries
        writeln!(file, "- {}: {} participations", country, count)?;
    }
    writeln!(file)?;
    
    // Detailed project listings by call
    writeln!(file, "## DETAILED PROJECT LISTINGS")?;
    
    for (call, _) in projects_by_call.iter() {
        writeln!(file, "\n### {}", call)?;
        writeln!(file)?;
        
        let projects: Vec<&EdfProject> = summary.projects.iter()
            .filter(|p| &p.call_title == *call)
            .collect();
            
        for project in projects {
            writeln!(file, "**{}**", project.project_name)?;
            writeln!(file, "- Topic: {}", project.topic_title)?;
            
            if let Some(duration) = project.duration_months {
                writeln!(file, "- Duration: {} months", duration)?;
            }
            
            if let Some(funding) = project.max_eu_contribution {
                writeln!(file, "- EU Funding: €{:.0}", funding)?;
            }
            
            if !project.activities.is_empty() {
                writeln!(file, "- Activities: {}", project.activities.join(", "))?;
            }
            
            // Get unique countries for this project
            let countries: BTreeSet<String> = project.consortium_members
                .iter()
                .map(|m| m.country.clone())
                .collect();
            
            if !countries.is_empty() {
                writeln!(file, "- Countries: {}", countries.into_iter().collect::<Vec<_>>().join(", "))?;
            }
            
            // Find coordinator
            if let Some(coordinator) = project.consortium_members.iter().find(|m| m.is_coordinator) {
                writeln!(file, "- Coordinator: {} ({})", coordinator.name, coordinator.country)?;
            }
            
            writeln!(file, "- Description: {}", project.description)?;
            writeln!(file)?;
        }
    }
    
    Ok(())
}

/// Generate a JSON output for easy querying
pub fn generate_json_summary(output_file: &str, summary: &EdfSummary) -> Result<()> {
    let file = File::create(output_file)?;
    serde_json::to_writer_pretty(file, summary)?;
    Ok(())
}
