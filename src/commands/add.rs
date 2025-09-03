use crate::{models::TimeEntry, storage};
use chrono::{DateTime, Utc};
use colored::*;
use std::path::Path;

pub fn add_entry(
    file_path: &Path,
    project: Option<String>,
    description: Option<String>,
    start_time_str: &str,
    end_time_str: &str,
) -> anyhow::Result<()> {
    let start_time = parse_datetime(start_time_str)?;
    let end_time = parse_datetime(end_time_str)?;
    
    if end_time <= start_time {
        return Err(anyhow::anyhow!("End time must be after start time"));
    }
    
    let mut data = storage::load_data(file_path)?;
    
    // Create manual time entry
    let mut entry = TimeEntry::new(project, description);
    entry.start_time = start_time;
    entry.end_time = Some(end_time);
    entry.updated_at = Utc::now();
    
    let duration = entry.duration().unwrap();
    let hours = duration.num_seconds() as f64 / 3600.0;
    
    println!("{}", "✅ Manual time entry added!".green());
    println!("Start: {}", start_time.format("%Y-%m-%d %H:%M:%S"));
    println!("End: {}", end_time.format("%Y-%m-%d %H:%M:%S"));
    println!("Duration: {:.2} hours", hours);
    
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

fn parse_datetime(datetime_str: &str) -> anyhow::Result<DateTime<Utc>> {
    // Try different datetime formats
    let formats = [
        "%Y-%m-%d %H:%M:%S",
        "%Y-%m-%d %H:%M",
        "%Y-%m-%d",
        "%H:%M:%S",
        "%H:%M",
    ];
    
    for format in &formats {
        if let Ok(datetime) = chrono::NaiveDateTime::parse_from_str(datetime_str, format) {
            return Ok(DateTime::from_naive_utc_and_offset(datetime, Utc));
        }
    }
    
    // If only time is provided, assume today's date
    if let Ok(time) = chrono::NaiveTime::parse_from_str(datetime_str, "%H:%M") {
        let today = Utc::now().date_naive();
        let datetime = today.and_time(time);
        return Ok(DateTime::from_naive_utc_and_offset(datetime, Utc));
    }
    
    if let Ok(time) = chrono::NaiveTime::parse_from_str(datetime_str, "%H:%M:%S") {
        let today = Utc::now().date_naive();
        let datetime = today.and_time(time);
        return Ok(DateTime::from_naive_utc_and_offset(datetime, Utc));
    }
    
    Err(anyhow::anyhow!(
        "Invalid datetime format: {}. Use formats like: 2024-01-15 14:30, 14:30, 2024-01-15",
        datetime_str
    ))
}
