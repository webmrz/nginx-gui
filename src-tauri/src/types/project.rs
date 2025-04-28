use serde::{Deserialize, Serialize};
use chrono::{DateTime, Local};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ProjectConfig {
    pub id: String,
    pub name: String,
    pub path: String,
    pub domain: String,
    pub port: u16,
    pub root: String,
    pub index: Vec<String>,
    pub php: bool,
    pub ssl: bool,
    pub remark: String,
    pub created_at: DateTime<Local>,
    pub updated_at: DateTime<Local>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ProjectTemplate {
    pub id: String,
    pub name: String,
    pub content: String,
    pub remark: String,
    pub created_at: DateTime<Local>,
    pub updated_at: DateTime<Local>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateProjectRequest {
    pub name: String,
    pub domain: String,
    pub port: u16,
    pub root: String,
    pub index: Vec<String>,
    pub php: bool,
    pub ssl: bool,
    pub remark: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateTemplateRequest {
    pub name: String,
    pub content: String,
    pub remark: String,
} 