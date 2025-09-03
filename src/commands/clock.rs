use crate::{models::TimeEntry, storage};
use chrono::Utc;
use colored::*;
use std::path::Path;

pub fn clock_in(
    file_path: &Path,
    project: Option<String>,
    description: Option<String>,
) -> anyhow::Result<()> {
    let mut data = storage::load_data(file_path)?;
    
    // Check if already clocked in
    if let Some(active_entry) = data.get_active_entry() {
        println!("{}", "❌ Already clocked in!".red());
        println!("Started: {}", active_entry.start_time.format("%Y-%m-%d %H:%M:%S"));
        if let Some(proj) = &active_entry.project {
            println!("Project: {}", proj);
        }
        if let Some(desc) = &active_entry.description {
            println!("Description: {}", desc);
        }
        return Ok(());
    }
    
    // Create new time entry
    let entry = TimeEntry::new(project, description);
    
    println!("{}", "✅ Clocked in!".green());
    println!("Started: {}", entry.start_time.format("%Y-%m-%d %H:%M:%S"));
    if let Some(proj) = &entry.project {
        println!("Project: {}", proj.blue());
    }
    if let Some(desc) = &entry.description {
        println!("Description: {}", desc);
    }
    
    data.add_time_entry(entry);
    storage::save_data(file_path, &data)?;
    
    Ok(())
}

pub fn clock_out(file_path: &Path, description: Option<String>) -> anyhow::Result<()> {
    let mut data = storage::load_data(file_path)?;
    
    // Find active entry
    let active_entry = data
        .time_entries
        .iter_mut()
        .find(|entry| entry.is_active());
    
    match active_entry {
        Some(entry) => {
            let end_time = Utc::now();
            entry.end_time = Some(end_time);
            entry.updated_at = end_time;
            
            if let Some(desc) = description {
                entry.description = Some(desc);
            }
            
            let duration = entry.duration().unwrap();
            let hours = duration.num_seconds() as f64 / 3600.0;
            
            println!("{}", "✅ Clocked out!".green());
            println!("Started: {}", entry.start_time.format("%Y-%m-%d %H:%M:%S"));
            println!("Ended: {}", end_time.format("%Y-%m-%d %H:%M:%S"));
            println!("Duration: {:.2} hours", hours);
            
            if let Some(proj) = &entry.project {
                println!("Project: {}", proj.blue());
            }
            if let Some(desc) = &entry.description {
                println!("Description: {}", desc);
            }
            
            storage::save_data(file_path, &data)?;
        }
        None => {
            println!("{}", "❌ Not clocked in!".red());
        }
    }
    
    Ok(())
}
