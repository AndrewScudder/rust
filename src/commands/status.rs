use crate::storage;
use chrono::{DateTime, Duration, Utc, Datelike};
use colored::*;
use std::path::Path;

pub fn show_status(file_path: &Path) -> anyhow::Result<()> {
    let data = storage::load_data(file_path)?;
    
    println!("{}", "📊 TimeCard Status".bold());
    println!("{}", "=".repeat(30));
    
    // Check if currently clocked in
    if let Some(active_entry) = data.get_active_entry() {
        let now = Utc::now();
        let duration = now - active_entry.start_time;
        let hours = duration.num_seconds() as f64 / 3600.0;
        
        println!("{}", "🟢 Currently Clocked In".green().bold());
        println!("Started: {}", active_entry.start_time.format("%Y-%m-%d %H:%M:%S"));
        println!("Duration: {:.2} hours", hours);
        
        if let Some(proj) = &active_entry.project {
            println!("Project: {}", proj.blue());
        }
        if let Some(desc) = &active_entry.description {
            println!("Description: {}", desc);
        }
    } else {
        println!("{}", "🔴 Not Clocked In".red().bold());
    }
    
    println!();
    
    // Today's summary
    let today = Utc::now().date_naive();
    let today_entries = data.get_entries_by_date(today);
    let today_hours: f64 = today_entries
        .iter()
        .filter_map(|entry| entry.hours())
        .sum();
    
    println!("{}", "📅 Today's Summary".bold());
    println!("Total Hours: {:.2}", today_hours);
    println!("Entries: {}", today_entries.len());
    
    // This week's summary
    let week_start = today - Duration::days(today.weekday().num_days_from_monday() as i64);
    let week_end = week_start + Duration::days(6);
    let week_entries = data.get_entries_by_period(
        DateTime::from_naive_utc_and_offset(week_start.and_hms_opt(0, 0, 0).unwrap(), Utc),
        DateTime::from_naive_utc_and_offset(week_end.and_hms_opt(23, 59, 59).unwrap(), Utc),
    );
    let week_hours: f64 = week_entries
        .iter()
        .filter_map(|entry| entry.hours())
        .sum();
    
    println!();
    println!("{}", "📊 This Week's Summary".bold());
    println!("Total Hours: {:.2}", week_hours);
    println!("Entries: {}", week_entries.len());
    
    // Project breakdown for today
    if !today_entries.is_empty() {
        println!();
        println!("{}", "🏷️  Today's Projects".bold());
        
        let mut project_hours = std::collections::HashMap::new();
        for entry in today_entries {
            let project = entry.project.as_deref().unwrap_or("No Project");
            *project_hours.entry(project).or_insert(0.0) += entry.hours().unwrap_or(0.0);
        }
        
        for (project, hours) in project_hours {
            println!("  {}: {:.2} hours", project.blue(), hours);
        }
    }
    
    Ok(())
}
