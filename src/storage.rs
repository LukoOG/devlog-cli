use crate::models::LogEntry;
use std::{fs, error::Error};

pub fn load_logs() -> Vec<LogEntry> {
    let contents = fs::read_to_string("devlog.json");

    let parsed = match contents {
        Ok(val) => match serde_json::from_str(val.as_str()) {
            Ok(logs) => logs,
            Err(e) => {
                eprintln!("Failed to parse logs: {}", e);
                Vec::new()
            }
        },

        Err(_) => return Vec::new(),
    };

    parsed
}

pub fn save_logs(logs: &[LogEntry]) -> Result<(), Box<dyn Error>> {
    let json = serde_json::to_string_pretty(logs)?;
    fs::write("devlog.json", json)?;
    Ok(())
}
