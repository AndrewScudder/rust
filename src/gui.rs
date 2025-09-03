use crate::{models::TimeCardData, storage};
use chrono::{DateTime, Duration, Utc, Datelike};
use eframe::egui;
use std::path::PathBuf;

pub struct TimeCardApp {
    data_file: PathBuf,
    data: TimeCardData,
    current_project: String,
    current_description: String,
    selected_period: String,
    show_add_manual: bool,
    manual_start: String,
    manual_end: String,
    manual_project: String,
    manual_description: String,
}

impl TimeCardApp {
    pub fn new(data_file: PathBuf) -> Self {
        let data = storage::load_data(&data_file).unwrap_or_default();
        
        Self {
            data_file,
            data,
            current_project: String::new(),
            current_description: String::new(),
            selected_period: "today".to_string(),
            show_add_manual: false,
            manual_start: String::new(),
            manual_end: String::new(),
            manual_project: String::new(),
            manual_description: String::new(),
        }
    }
    
    fn save_data(&mut self) {
        if let Err(e) = storage::save_data(&self.data_file, &self.data) {
            eprintln!("Error saving data: {}", e);
        }
    }
    
    fn clock_in(&mut self) {
        if self.data.get_active_entry().is_some() {
            return; // Already clocked in
        }
        
        let entry = crate::models::TimeEntry::new(
            if self.current_project.is_empty() { None } else { Some(self.current_project.clone()) },
            if self.current_description.is_empty() { None } else { Some(self.current_description.clone()) }
        );
        
        self.data.add_time_entry(entry);
        self.save_data();
        
        // Clear input fields
        self.current_project.clear();
        self.current_description.clear();
    }
    
    fn clock_out(&mut self) {
        if let Some(active_entry) = self.data.time_entries.iter_mut().find(|entry| entry.is_active()) {
            let end_time = Utc::now();
            active_entry.end_time = Some(end_time);
            active_entry.updated_at = end_time;
            
            if !self.current_description.is_empty() {
                active_entry.description = Some(self.current_description.clone());
            }
            
            self.save_data();
            self.current_description.clear();
        }
    }
    
    fn add_manual_entry(&mut self) {
        if self.manual_start.is_empty() || self.manual_end.is_empty() {
            return;
        }
        
        // Parse datetime strings (simplified - you might want to add better parsing)
        let start_time = match self.parse_datetime(&self.manual_start) {
            Ok(time) => time,
            Err(_) => return,
        };
        
        let end_time = match self.parse_datetime(&self.manual_end) {
            Ok(time) => time,
            Err(_) => return,
        };
        
        if end_time <= start_time {
            return;
        }
        
        let mut entry = crate::models::TimeEntry::new(
            if self.manual_project.is_empty() { None } else { Some(self.manual_project.clone()) },
            if self.manual_description.is_empty() { None } else { Some(self.manual_description.clone()) }
        );
        
        entry.start_time = start_time;
        entry.end_time = Some(end_time);
        entry.updated_at = Utc::now();
        
        self.data.add_time_entry(entry);
        self.save_data();
        
        // Clear manual entry fields
        self.manual_start.clear();
        self.manual_end.clear();
        self.manual_project.clear();
        self.manual_description.clear();
        self.show_add_manual = false;
    }
    
    fn parse_datetime(&self, datetime_str: &str) -> Result<DateTime<Utc>, ()> {
        // Try different formats
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
        
        Err(())
    }
    
    fn get_period_entries(&self) -> Vec<&crate::models::TimeEntry> {
        let (start_date, end_date, _) = match self.parse_period(&self.selected_period) {
            Ok(period) => period,
            Err(_) => return Vec::new(),
        };
        
        self.data.get_entries_by_period(start_date, end_date)
    }
    
    fn parse_period(&self, period: &str) -> Result<(DateTime<Utc>, DateTime<Utc>, String), ()> {
        let now = Utc::now();
        let today = now.date_naive();
        
        match period {
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
            "week" => {
                let week_start = today - Duration::days(today.weekday().num_days_from_monday() as i64);
                let week_end = week_start + Duration::days(6);
                let start = DateTime::from_naive_utc_and_offset(week_start.and_hms_opt(0, 0, 0).unwrap(), Utc);
                let end = DateTime::from_naive_utc_and_offset(week_end.and_hms_opt(23, 59, 59).unwrap(), Utc);
                Ok((start, end, "This Week".to_string()))
            }
            "month" => {
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
            _ => Err(()),
        }
    }
}

impl eframe::App for TimeCardApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("🕐 TimeCard - Time Tracking");
            ui.add_space(10.0);
            
            // Status section
            ui.group(|ui| {
                ui.heading("📊 Status");
                
                if let Some(active_entry) = self.data.get_active_entry() {
                    let now = Utc::now();
                    let duration = now - active_entry.start_time;
                    let hours = duration.num_seconds() as f64 / 3600.0;
                    
                    ui.label(format!("🟢 Currently Clocked In"));
                    ui.label(format!("Started: {}", active_entry.start_time.format("%Y-%m-%d %H:%M:%S")));
                    ui.label(format!("Duration: {:.2} hours", hours));
                    
                    if let Some(proj) = &active_entry.project {
                        ui.label(format!("Project: {}", proj));
                    }
                    if let Some(desc) = &active_entry.description {
                        ui.label(format!("Description: {}", desc));
                    }
                    
                    ui.add_space(5.0);
                    ui.text_edit_singleline(&mut self.current_description);
                    if ui.button("🛑 Clock Out").clicked() {
                        self.clock_out();
                    }
                } else {
                    ui.label("🔴 Not Clocked In");
                    ui.add_space(5.0);
                    
                    ui.label("Project:");
                    ui.text_edit_singleline(&mut self.current_project);
                    ui.label("Description:");
                    ui.text_edit_singleline(&mut self.current_description);
                    
                    if ui.button("🟢 Clock In").clicked() {
                        self.clock_in();
                    }
                }
            });
            
            ui.add_space(10.0);
            
            // Quick stats
            ui.group(|ui| {
                ui.heading("📈 Quick Stats");
                
                let today = Utc::now().date_naive();
                let today_entries = self.data.get_entries_by_date(today);
                let today_hours: f64 = today_entries.iter().filter_map(|entry| entry.hours()).sum();
                
                ui.label(format!("Today: {:.2} hours ({} entries)", today_hours, today_entries.len()));
                
                let total_hours: f64 = self.data.time_entries.iter().filter_map(|entry| entry.hours()).sum();
                ui.label(format!("Total: {:.2} hours ({} entries)", total_hours, self.data.time_entries.len()));
            });
            
            ui.add_space(10.0);
            
            // Reports section
            ui.group(|ui| {
                ui.heading("📊 Reports");
                
                ui.horizontal(|ui| {
                    ui.label("Period:");
                    egui::ComboBox::from_id_source("period")
                        .selected_text(&self.selected_period)
                        .show_ui(ui, |ui| {
                            ui.selectable_value(&mut self.selected_period, "today".to_string(), "Today");
                            ui.selectable_value(&mut self.selected_period, "yesterday".to_string(), "Yesterday");
                            ui.selectable_value(&mut self.selected_period, "week".to_string(), "This Week");
                            ui.selectable_value(&mut self.selected_period, "month".to_string(), "This Month");
                        });
                });
                
                let period_entries = self.get_period_entries();
                let period_hours: f64 = period_entries.iter().filter_map(|entry| entry.hours()).sum();
                
                ui.label(format!("Total Hours: {:.2}", period_hours));
                ui.label(format!("Entries: {}", period_entries.len()));
                
                if !period_entries.is_empty() {
                    ui.add_space(5.0);
                    ui.label("Recent Entries:");
                    
                    egui::ScrollArea::vertical().max_height(150.0).show(ui, |ui| {
                        for entry in period_entries.iter().take(10) {
                            let hours = entry.hours().unwrap_or(0.0);
                            let project = entry.project.as_deref().unwrap_or("No Project");
                            let status = if entry.is_active() { "🟢 ACTIVE" } else { "✅ COMPLETED" };
                            
                            ui.label(format!("{} {} - {} ({:.2}h)", 
                                status,
                                entry.start_time.format("%m-%d %H:%M"),
                                project,
                                hours
                            ));
                            
                            if let Some(desc) = &entry.description {
                                ui.label(format!("  Description: {}", desc));
                            }
                        }
                    });
                }
            });
            
            ui.add_space(10.0);
            
            // Manual entry section
            ui.group(|ui| {
                ui.heading("➕ Manual Entry");
                
                if ui.button("Add Manual Entry").clicked() {
                    self.show_add_manual = !self.show_add_manual;
                }
                
                if self.show_add_manual {
                    ui.add_space(5.0);
                    
                    ui.label("Start Time:");
                    ui.text_edit_singleline(&mut self.manual_start);
                    ui.label("End Time:");
                    ui.text_edit_singleline(&mut self.manual_end);
                    ui.label("Project:");
                    ui.text_edit_singleline(&mut self.manual_project);
                    ui.label("Description:");
                    ui.text_edit_singleline(&mut self.manual_description);
                    
                    if ui.button("Add Entry").clicked() {
                        self.add_manual_entry();
                    }
                }
            });
        });
    }
}
