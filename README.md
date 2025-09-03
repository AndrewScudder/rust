# 🕐 TimeCard - Simple Time Tracking

A lightweight, command-line time tracking application built in Rust. Track your work hours, manage projects, and generate reports with ease.

## ✨ Features

- **🕐 Simple Time Tracking**: Clock in/out with project and description support
- **📊 Status Overview**: See current status and daily/weekly summaries
- **📈 Detailed Reports**: Generate reports for different time periods
- **📋 Entry Management**: List, filter, and manage time entries
- **➕ Manual Entries**: Add time entries manually with flexible datetime formats
- **📄 CSV Export**: Export reports to CSV for further analysis
- **💾 Local Storage**: All data stored locally in JSON format

## 🚀 Quick Start

### Installation

1. Clone the repository:
```bash
git clone <repository-url>
cd timecard
```

2. Build the application:
```bash
cargo build --release
```

3. Run the application:
```bash
./target/release/timecard --help
```

### Basic Usage

#### Clock In/Out
```bash
# Clock in
timecard in --project "Web Development" --description "Working on frontend"

# Clock out
timecard out --description "Completed frontend work"
```

#### Check Status
```bash
# Show current status and summaries
timecard status
```

#### Generate Reports
```bash
# Today's report
timecard report --period today

# This week's report with CSV export
timecard report --period week --csv

# Project-specific report
timecard report --period month --project "Web Development"
```

#### List Entries
```bash
# List all entries
timecard list

# List entries for specific project
timecard list --project "Web Development"

# List last 10 entries
timecard list --limit 10
```

#### Add Manual Entry
```bash
# Add entry with full datetime
timecard add --start "2024-01-15 09:00" --end "2024-01-15 17:00" --project "Meeting" --description "Team standup"

# Add entry with just time (assumes today)
timecard add --start "09:00" --end "17:00" --project "Development"
```

## 📖 Command Reference

### Global Options
- `--data-file <FILE>`: Specify data file location (default: `timecard.json`)

### Commands

#### `in` - Clock In
Start tracking time for a new session.

**Options:**
- `-p, --project <PROJECT>`: Project name
- `-d, --description <DESCRIPTION>`: Description of work

**Examples:**
```bash
timecard in
timecard in -p "Development" -d "Working on new feature"
```

#### `out` - Clock Out
Stop tracking time for the current session.

**Options:**
- `-d, --description <DESCRIPTION>`: Description of completed work

**Examples:**
```bash
timecard out
timecard out -d "Feature completed"
```

#### `status` - Show Status
Display current tracking status and summaries.

**Examples:**
```bash
timecard status
```

#### `report` - Generate Report
Generate time reports for different periods.

**Options:**
- `-p, --period <PERIOD>`: Time period (today, yesterday, week, last-week, month, last-month)
- `--project <PROJECT>`: Filter by project
- `--csv`: Export to CSV file

**Examples:**
```bash
timecard report --period today
timecard report --period week --project "Development" --csv
```

#### `list` - List Entries
Display time entries with filtering options.

**Options:**
- `--project <PROJECT>`: Filter by project
- `--limit <LIMIT>`: Limit number of entries

**Examples:**
```bash
timecard list
timecard list --project "Development" --limit 5
```

#### `add` - Add Manual Entry
Add a manual time entry.

**Options:**
- `-p, --project <PROJECT>`: Project name
- `-d, --description <DESCRIPTION>`: Description
- `-s, --start <START>`: Start time
- `-e, --end <END>`: End time

**Examples:**
```bash
timecard add --start "09:00" --end "17:00" --project "Meeting"
timecard add --start "2024-01-15 09:00" --end "2024-01-15 17:00" --project "Development" --description "Full day of coding"
```

## 📅 Time Periods

The following time periods are supported for reports:

- `today`: Current day
- `yesterday`: Previous day
- `week` or `this-week`: Current week (Monday to Sunday)
- `last-week`: Previous week
- `month` or `this-month`: Current month
- `last-month`: Previous month

## 🕐 DateTime Formats

When adding manual entries, you can use various datetime formats:

- `2024-01-15 14:30:00` - Full datetime
- `2024-01-15 14:30` - Date and time
- `2024-01-15` - Date only
- `14:30:00` - Time only (assumes today)
- `14:30` - Time only (assumes today)

## 📁 Data Storage

All time tracking data is stored locally in a JSON file. By default, this is `timecard.json` in the current directory. You can specify a different location using the `--data-file` option.

**Data Structure:**
```json
{
  "time_entries": [
    {
      "id": "uuid",
      "project": "Project Name",
      "description": "Work description",
      "start_time": "2024-01-15T09:00:00Z",
      "end_time": "2024-01-15T17:00:00Z",
      "created_at": "2024-01-15T09:00:00Z",
      "updated_at": "2024-01-15T17:00:00Z"
    }
  ],
  "projects": [],
  "created_at": "2024-01-15T09:00:00Z",
  "updated_at": "2024-01-15T17:00:00Z"
}
```

## 🔧 Configuration

The application uses minimal configuration and stores all settings in the data file. You can:

- Use different data files for different contexts
- Backup your data file for safekeeping
- Share data files between machines

## 🚀 Advanced Usage

### Multiple Data Files
```bash
# Work projects
timecard --data-file work.json in -p "Development"

# Personal projects
timecard --data-file personal.json in -p "Learning"
```

### Backup and Restore
```bash
# Backup your data
cp timecard.json timecard_backup.json

# Restore from backup
cp timecard_backup.json timecard.json
```

### CSV Export for Analysis
```bash
# Export this month's data
timecard report --period month --csv

# Import into spreadsheet applications
# The CSV file will be named: timecard_report_month.csv
```

## 🛠️ Development

### Building from Source
```bash
# Clone the repository
git clone <repository-url>
cd timecard

# Build in debug mode
cargo build

# Build in release mode
cargo build --release

# Run tests
cargo test
```

### Project Structure
```
src/
├── main.rs          # Application entry point
├── models.rs        # Data models and structures
├── storage.rs       # File I/O operations
└── commands/        # Command implementations
    ├── mod.rs
    ├── clock.rs     # Clock in/out functionality
    ├── status.rs    # Status display
    ├── report.rs    # Report generation
    ├── list.rs      # Entry listing
    └── add.rs       # Manual entry addition
```

## 🤝 Contributing

1. Fork the repository
2. Create a feature branch
3. Make your changes
4. Add tests if applicable
5. Submit a pull request

## 📄 License

This project is licensed under the MIT License - see the LICENSE file for details.

## 🙏 Acknowledgments

- Built with Rust for performance and reliability
- Uses minimal dependencies for simplicity
- Inspired by the need for a simple, local time tracking solution

## 📞 Support

If you encounter any issues or have questions:

1. Check the command help: `timecard --help`
2. Review the examples in this README
3. Open an issue on the project repository

---

**Happy Time Tracking! 🕐✨**
