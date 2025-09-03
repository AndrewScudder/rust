use crate::models::TimeCardData;
use std::fs;
use std::path::Path;

pub fn load_data(file_path: &Path) -> anyhow::Result<TimeCardData> {
    if !file_path.exists() {
        return Ok(TimeCardData::default());
    }
    
    let content = fs::read_to_string(file_path)?;
    let data: TimeCardData = serde_json::from_str(&content)?;
    Ok(data)
}

pub fn save_data(file_path: &Path, data: &TimeCardData) -> anyhow::Result<()> {
    let content = serde_json::to_string_pretty(data)?;
    
    // Create parent directory if it doesn't exist
    if let Some(parent) = file_path.parent() {
        if !parent.exists() {
            fs::create_dir_all(parent)?;
        }
    }
    
    fs::write(file_path, content)?;
    Ok(())
}

pub fn backup_data(file_path: &Path) -> anyhow::Result<()> {
    if !file_path.exists() {
        return Ok(());
    }
    
    let backup_path = file_path.with_extension("json.backup");
    fs::copy(file_path, backup_path)?;
    Ok(())
}
