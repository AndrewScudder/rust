use chrono::{DateTime, Utc, Datelike};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TimeEntry {
    pub id: Uuid,
    pub project: Option<String>,
    pub description: Option<String>,
    pub start_time: DateTime<Utc>,
    pub end_time: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl TimeEntry {
    pub fn new(project: Option<String>, description: Option<String>) -> Self {
        let now = Utc::now();
        Self {
            id: Uuid::new_v4(),
            project,
            description,
            start_time: now,
            end_time: None,
            created_at: now,
            updated_at: now,
        }
    }
    
    pub fn duration(&self) -> Option<chrono::Duration> {
        self.end_time.map(|end| end - self.start_time)
    }
    
    pub fn is_active(&self) -> bool {
        self.end_time.is_none()
    }
    
    pub fn hours(&self) -> Option<f64> {
        self.duration().map(|d| d.num_seconds() as f64 / 3600.0)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Project {
    pub id: Uuid,
    pub name: String,
    pub description: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl Project {
    pub fn new(name: String, description: Option<String>) -> Self {
        let now = Utc::now();
        Self {
            id: Uuid::new_v4(),
            name,
            description,
            created_at: now,
            updated_at: now,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TimeCardData {
    pub time_entries: Vec<TimeEntry>,
    pub projects: Vec<Project>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl Default for TimeCardData {
    fn default() -> Self {
        let now = Utc::now();
        Self {
            time_entries: Vec::new(),
            projects: Vec::new(),
            created_at: now,
            updated_at: now,
        }
    }
}

impl TimeCardData {
    pub fn add_time_entry(&mut self, entry: TimeEntry) {
        self.time_entries.push(entry);
        self.updated_at = Utc::now();
    }
    
    pub fn get_active_entry(&self) -> Option<&TimeEntry> {
        self.time_entries.iter().find(|entry| entry.is_active())
    }
    
    pub fn get_entries_by_project(&self, project: &str) -> Vec<&TimeEntry> {
        self.time_entries
            .iter()
            .filter(|entry| entry.project.as_deref() == Some(project))
            .collect()
    }
    
    pub fn get_entries_by_date(&self, date: chrono::NaiveDate) -> Vec<&TimeEntry> {
        self.time_entries
            .iter()
            .filter(|entry| entry.start_time.date_naive() == date)
            .collect()
    }
    
    pub fn get_entries_by_period(&self, start: DateTime<Utc>, end: DateTime<Utc>) -> Vec<&TimeEntry> {
        self.time_entries
            .iter()
            .filter(|entry| entry.start_time >= start && entry.start_time <= end)
            .collect()
    }
    
    pub fn total_hours(&self) -> f64 {
        self.time_entries
            .iter()
            .filter_map(|entry| entry.hours())
            .sum()
    }
    
    pub fn total_hours_by_project(&self, project: &str) -> f64 {
        self.get_entries_by_project(project)
            .iter()
            .filter_map(|entry| entry.hours())
            .sum()
    }
    
    pub fn total_hours_by_period(&self, start: DateTime<Utc>, end: DateTime<Utc>) -> f64 {
        self.get_entries_by_period(start, end)
            .iter()
            .filter_map(|entry| entry.hours())
            .sum()
    }
}
