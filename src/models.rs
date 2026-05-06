use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct LogEntry {
    pub id: u32,
    pub message: String,
    pub tags: Vec<String>,
}