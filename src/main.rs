use clap::{Parser, Subcommand};
use colored::*;
use std::path::PathBuf;

mod models;
mod storage;
mod commands;
mod gui;

use commands::{clock, report, status};
use gui::TimeCardApp;
use egui::ViewportBuilder;

#[derive(Parser)]
#[command(name = "timecard")]
#[command(about = "Simple time card management application")]
#[command(version = "0.1.0")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
    
    #[arg(short, long, default_value = "timecard.json")]
    data_file: PathBuf,
}

#[derive(Subcommand)]
enum Commands {
    /// Clock in to start tracking time
    In {
        #[arg(short, long)]
        project: Option<String>,
        
        #[arg(short, long)]
        description: Option<String>,
    },
    
    /// Clock out to stop tracking time
    Out {
        #[arg(short, long)]
        description: Option<String>,
    },
    
    /// Show current status
    Status,
    
    /// Generate reports
    Report {
        #[arg(short, long, default_value = "today")]
        period: String,
        
        #[arg(short, long)]
        project: Option<String>,
        
        #[arg(short, long)]
        csv: bool,
    },
    
    /// List all time entries
    List {
        #[arg(short, long)]
        project: Option<String>,
        
        #[arg(short, long)]
        limit: Option<usize>,
    },
    
    /// Add manual time entry
    Add {
        #[arg(short, long)]
        project: Option<String>,
        
        #[arg(short, long)]
        description: Option<String>,
        
        #[arg(short, long)]
        start: String,
        
        #[arg(short, long)]
        end: String,
    },
    
    /// Launch GUI interface
    Gui,
}

fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();
    
    println!("{}", "🕐 TimeCard - Simple Time Tracking".green().bold());
    println!();
    
    match cli.command {
        Commands::In { project, description } => {
            clock::clock_in(&cli.data_file, project, description)?;
        }
        Commands::Out { description } => {
            clock::clock_out(&cli.data_file, description)?;
        }
        Commands::Status => {
            status::show_status(&cli.data_file)?;
        }
        Commands::Report { period, project, csv } => {
            report::generate_report(&cli.data_file, &period, project.as_deref(), csv)?;
        }
        Commands::List { project, limit } => {
            commands::list::list_entries(&cli.data_file, project.as_deref(), limit)?;
        }
        Commands::Add { project, description, start, end } => {
            commands::add::add_entry(&cli.data_file, project, description, &start, &end)?;
        }
        Commands::Gui => {
            launch_gui(&cli.data_file)?;
        }
    }
    
    Ok(())
}

fn launch_gui(data_file: &std::path::Path) -> anyhow::Result<()> {
    let data_file = data_file.to_path_buf();
    let options = eframe::NativeOptions {
        viewport: ViewportBuilder::default()
            .with_inner_size([800.0, 600.0]),
        ..Default::default()
    };
    
    eframe::run_native(
        "TimeCard",
        options,
        Box::new(move |_cc| Box::new(TimeCardApp::new(data_file.clone()))),
    ).map_err(|e| anyhow::anyhow!("GUI error: {}", e))?;
    
    Ok(())
}
