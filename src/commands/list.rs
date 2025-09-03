use crate::storage;
use colored::*;
use std::path::Path;

pub fn list_entries(
    file_path: &Path,
    project_filter: Option<&str>,
    limit: Option<usize>,
) -> anyhow::Result<()> {
    let data = storage::load_data(file_path)?;
    
    let mut entries: Vec<_> = data.time_entries.iter().collect();
    
    // Sort by start time (newest first)
    entries.sort_by(|a, b| b.start_time.cmp(&a.start_time));
    
    // Apply project filter
    if let Some(project) = project_filter {
        entries.retain(|entry| entry.project.as_deref() == Some(project));
    }
    
    // Apply limit
    if let Some(limit) = limit {
        entries.truncate(limit);
    }
    
    if entries.is_empty() {
        println!("{}", "No time entries found.".yellow());
        return Ok(());
    }
    
    println!("{}", "📋 Time Entries".bold());
    println!("{}", "=".repeat(80));
    
    for entry in &entries {
        let status = if entry.is_active() {
            "🟢 ACTIVE".green()
        } else {
            "✅ COMPLETED".blue()
        };
        
        let hours = entry.hours().unwrap_or(0.0);
        let project = entry.project.as_deref().unwrap_or("No Project");
        
        println!("{} {} - {} ({:.2}h)", 
            status,
            entry.start_time.format("%Y-%m-%d %H:%M"),
            project.blue(),
            hours
        );
        
        if let Some(desc) = &entry.description {
            println!("    Description: {}", desc);
        }
        
        if let Some(end_time) = entry.end_time {
            println!("    Ended: {}", end_time.format("%Y-%m-%d %H:%M"));
        }
        
        println!();
    }
    
    // Summary
    let total_entries = entries.len();
    let total_hours: f64 = entries.iter().filter_map(|entry| entry.hours()).sum();
    let active_entries = entries.iter().filter(|entry| entry.is_active()).count();
    
    println!("{}", "📊 Summary".bold());
    println!("Total Entries: {}", total_entries);
    println!("Total Hours: {:.2}", total_hours);
    println!("Active Entries: {}", active_entries);
    
    Ok(())
}
