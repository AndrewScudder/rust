use crate::storage;
use chrono::{DateTime, Duration, Utc, Datelike};
use colored::*;
use csv::Writer;
use std::path::Path;

pub fn generate_report(
    file_path: &Path,
    period: &str,
    project_filter: Option<&str>,
    csv_export: bool,
) -> anyhow::Result<()> {
    let data = storage::load_data(file_path)?;
    
    let (start_date, end_date, period_name) = parse_period(period)?;
    
    println!("{}", format!("📊 Time Report - {}", period_name).bold());
    println!("{}", "=".repeat(50));
    println!("Period: {} to {}", 
        start_date.format("%Y-%m-%d"), 
        end_date.format("%Y-%m-%d")
    );
    println!();
    
    let entries = data.get_entries_by_period(start_date, end_date);
    let filtered_entries: Vec<_> = if let Some(project) = project_filter {
        entries.into_iter().filter(|entry| {
            entry.project.as_deref() == Some(project)
        }).collect()
    } else {
        entries
    };
    
    if filtered_entries.is_empty() {
        println!("{}", "No time entries found for this period.".yellow());
        return Ok(());
    }
    
    let total_hours: f64 = filtered_entries
        .iter()
        .filter_map(|entry| entry.hours())
        .sum();
    
    println!("{}", "📈 Summary".bold());
    println!("Total Hours: {:.2}", total_hours);
    println!("Total Entries: {}", filtered_entries.len());
    println!();
    
    // Project breakdown
    let mut project_hours = std::collections::HashMap::new();
    for entry in &filtered_entries {
        let project = entry.project.as_deref().unwrap_or("No Project");
        *project_hours.entry(project).or_insert(0.0) += entry.hours().unwrap_or(0.0);
    }
    
    if project_hours.len() > 1 {
        println!("{}", "🏷️  Project Breakdown".bold());
        for (project, hours) in project_hours {
            let percentage = (hours / total_hours) * 100.0;
            println!("  {}: {:.2} hours ({:.1}%)", project.blue(), hours, percentage);
        }
        println!();
    }
    
    // Detailed entries
    println!("{}", "📝 Detailed Entries".bold());
    for entry in &filtered_entries {
        let hours = entry.hours().unwrap_or(0.0);
        let project = entry.project.as_deref().unwrap_or("No Project");
        
        println!("  {} - {} ({:.2}h)", 
            entry.start_time.format("%Y-%m-%d %H:%M"),
            project.blue(),
            hours
        );
        
        if let Some(desc) = &entry.description {
            println!("    Description: {}", desc);
        }
    }
    
    // CSV export
    if csv_export {
        export_to_csv(&filtered_entries, period)?;
    }
    
    Ok(())
}

fn parse_period(period: &str) -> anyhow::Result<(DateTime<Utc>, DateTime<Utc>, String)> {
    let now = Utc::now();
    let today = now.date_naive();
    
    match period.to_lowercase().as_str() {
        "today" => {
            let start = DateTime::from_naive_utc_and_offset(today.and_hms_opt(0, 0, 0).unwrap(), Utc);
            let end = DateTime::from_naive_utc_and_offset(today.and_hms_opt(23, 59, 59).unwrap(), Utc);
            Ok((start, end, "Today".to_string()))
        }
        "yesterday" => {
            let yesterday = today - Duration::days(1);
            let start = DateTime::from_naive_utc_and_offset(yesterday.and_hms_opt(0, 0, 0).unwrap(), Utc);
            let end = DateTime::from_naive_utc_and_offset(yesterday.and_hms_opt(23, 59, 59).unwrap(), Utc);
            Ok((start, end, "Yesterday".to_string()))
        }
        "week" | "this-week" => {
            let week_start = today - Duration::days(today.weekday().num_days_from_monday() as i64);
            let week_end = week_start + Duration::days(6);
            let start = DateTime::from_naive_utc_and_offset(week_start.and_hms_opt(0, 0, 0).unwrap(), Utc);
            let end = DateTime::from_naive_utc_and_offset(week_end.and_hms_opt(23, 59, 59).unwrap(), Utc);
            Ok((start, end, "This Week".to_string()))
        }
        "last-week" => {
            let week_start = today - Duration::days(today.weekday().num_days_from_monday() as i64);
            let last_week_start = week_start - Duration::days(7);
            let last_week_end = last_week_start + Duration::days(6);
            let start = DateTime::from_naive_utc_and_offset(last_week_start.and_hms_opt(0, 0, 0).unwrap(), Utc);
            let end = DateTime::from_naive_utc_and_offset(last_week_end.and_hms_opt(23, 59, 59).unwrap(), Utc);
            Ok((start, end, "Last Week".to_string()))
        }
        "month" | "this-month" => {
            let month_start = today.with_day(1).unwrap();
            let next_month = if today.month() == 12 {
                today.with_year(today.year() + 1).unwrap().with_month(1).unwrap()
            } else {
                today.with_month(today.month() + 1).unwrap()
            };
            let month_end = next_month.with_day(1).unwrap() - Duration::days(1);
            let start = DateTime::from_naive_utc_and_offset(month_start.and_hms_opt(0, 0, 0).unwrap(), Utc);
            let end = DateTime::from_naive_utc_and_offset(month_end.and_hms_opt(23, 59, 59).unwrap(), Utc);
            Ok((start, end, "This Month".to_string()))
        }
        "last-month" => {
            let month_start = today.with_day(1).unwrap();
            let last_month_start = if today.month() == 1 {
                today.with_year(today.year() - 1).unwrap().with_month(12).unwrap()
            } else {
                today.with_month(today.month() - 1).unwrap()
            };
            let last_month_start = last_month_start.with_day(1).unwrap();
            let last_month_end = month_start - Duration::days(1);
            let start = DateTime::from_naive_utc_and_offset(last_month_start.and_hms_opt(0, 0, 0).unwrap(), Utc);
            let end = DateTime::from_naive_utc_and_offset(last_month_end.and_hms_opt(23, 59, 59).unwrap(), Utc);
            Ok((start, end, "Last Month".to_string()))
        }
        _ => {
            Err(anyhow::anyhow!("Invalid period: {}. Use: today, yesterday, week, last-week, month, last-month", period))
        }
    }
}

fn export_to_csv(entries: &[&crate::models::TimeEntry], period: &str) -> anyhow::Result<()> {
    let filename = format!("timecard_report_{}.csv", period.replace("-", "_"));
    let mut wtr = Writer::from_path(&filename)?;
    
    // Write header
    wtr.write_record(&["Date", "Start Time", "End Time", "Duration (hours)", "Project", "Description"])?;
    
    // Write data
    for entry in entries {
        let start_time = entry.start_time.format("%Y-%m-%d %H:%M:%S").to_string();
        let end_time = entry.end_time
            .map(|t| t.format("%Y-%m-%d %H:%M:%S").to_string())
            .unwrap_or_else(|| "".to_string());
        let duration = entry.hours().unwrap_or(0.0);
        let project = entry.project.as_deref().unwrap_or("");
        let description = entry.description.as_deref().unwrap_or("");
        
        wtr.write_record(&[
            &entry.start_time.format("%Y-%m-%d").to_string(),
            &start_time,
            &end_time,
            &format!("{:.2}", duration),
            project,
            description,
        ])?;
    }
    
    wtr.flush()?;
    println!("{}", format!("📄 CSV exported to: {}", filename).green());
    
    Ok(())
}
