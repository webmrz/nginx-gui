use serde::{Deserialize, Serialize};
use chrono::{DateTime, Local};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct NginxVersion {
    pub version: String,
    pub path: String,
    pub is_current: bool,
    pub installed_at: DateTime<Local>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct NginxVersionConfig {
    pub current_version: String,
    pub versions: Vec<NginxVersion>,
} 