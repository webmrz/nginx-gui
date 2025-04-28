use std::path::{Path, PathBuf};
use std::fs;
use serde::{Serialize, Deserialize};
use chrono::{DateTime, Local};
use tauri::{AppHandle, Manager};
use crate::utils::fs::{ensure_dir_exists, read_file_content, write_file_content, delete_file};
use std::process::Command;

#[derive(Debug, Serialize, Deserialize)]
pub struct NginxConfig {
    pub name: String,
    pub path: String,
    pub content: String,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum ConfigFileStatus {
    Valid,
    Invalid,
    Unknown,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ConfigFile {
    pub name: String,
    #[serde(rename = "type")]
    pub type_: String, // 'main' | 'site' | 'custom'
    pub path: String,
    pub size: String,
    #[serde(rename = "lastModified")]
    pub last_modified: String,
    pub status: ConfigFileStatus,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content: Option<String>,
}

/// 获取配置文件基础目录
fn get_config_base_dir(app_handle: &AppHandle) -> PathBuf {
    let resource_path = app_handle.path().resource_dir()
        .expect("Failed to get resource directory");
    
    let manifest_dir = std::env::var("CARGO_MANIFEST_DIR").unwrap_or_default();
    let manifest_dir = Path::new(&manifest_dir);
    let nginx_path = manifest_dir.join("resources").join("nginx-1.24.0");
    
    nginx_path.join("conf")
}

/// 根据配置类型获取目录
fn get_config_dir(app_handle: &AppHandle, config_type: &str) -> PathBuf {
    let base_dir = get_config_base_dir(app_handle);
    
    match config_type {
        "main" => base_dir,
        "site" => base_dir.join("sites-available"),
        "custom" => base_dir.join("conf.d"),
        _ => base_dir
    }
}

/// 测试配置文件是否有效
fn test_config_file(nginx_path: &Path, config_path: &Path) -> bool {
    let nginx_exe = nginx_path.join("nginx.exe");
    
    if !nginx_exe.exists() {
        return false;
    }
    
    let output = Command::new(&nginx_exe)
        .current_dir(nginx_path)
        .arg("-t")
        .arg("-c")
        .arg(config_path)
        .output();
    
    match output {
        Ok(output) => output.status.success(),
        Err(_) => false
    }
}

/// 格式化文件大小
fn format_file_size(size_in_bytes: u64) -> String {
    const KB: u64 = 1024;
    const MB: u64 = KB * 1024;
    
    if size_in_bytes < KB {
        format!("{} B", size_in_bytes)
    } else if size_in_bytes < MB {
        format!("{:.2} KB", size_in_bytes as f64 / KB as f64)
    } else {
        format!("{:.2} MB", size_in_bytes as f64 / MB as f64)
    }
}

#[tauri::command]
pub fn get_config_files(app_handle: tauri::AppHandle) -> Result<Vec<ConfigFile>, String> {
    // 确保目录存在
    let main_dir = get_config_base_dir(&app_handle);
    let sites_dir = get_config_dir(&app_handle, "site");
    let custom_dir = get_config_dir(&app_handle, "custom");
    
    // 创建目录（如果不存在）
    ensure_dir_exists(&sites_dir).ok();
    ensure_dir_exists(&custom_dir).ok();
    
    let mut configs = Vec::new();
    let nginx_path = app_handle.path().resource_dir()
        .map(|p| p.parent().unwrap_or(&p).to_path_buf())
        .unwrap_or_default();
    
    // 获取所有配置文件
    let dirs = vec![
        (main_dir, "main"),
        (sites_dir, "site"),
        (custom_dir, "custom")
    ];
    
    for (dir, config_type) in dirs {
        if !dir.exists() {
            continue;
        }
        
        for entry in fs::read_dir(&dir).map_err(|e| e.to_string())? {
            let entry = entry.map_err(|e| e.to_string())?;
            let path = entry.path();
            
            if path.is_file() && path.extension().map_or(false, |ext| ext == "conf") {
                let name = path.file_name()
                    .and_then(|n| n.to_str())
                    .unwrap_or("")
                    .to_string();
                
                let metadata = fs::metadata(&path).map_err(|e| e.to_string())?;
                let size = format_file_size(metadata.len());
                
                let last_modified = metadata.modified()
                    .map(|time| {
                        let datetime: DateTime<Local> = time.into();
                        datetime.format("%Y-%m-%d %H:%M:%S").to_string()
                    })
                    .unwrap_or_else(|_| "未知".to_string());
                
                let is_valid = test_config_file(&nginx_path, &path);
                let status = if is_valid { ConfigFileStatus::Valid } else { ConfigFileStatus::Invalid };
                
                configs.push(ConfigFile {
                    name,
                    type_: config_type.to_string(),
                    path: path.to_string_lossy().to_string(),
                    size,
                    last_modified,
                    status,
                    content: None,
                });
            }
        }
    }
    
    Ok(configs)
}

#[tauri::command]
pub fn create_config_file(
    app_handle: tauri::AppHandle,
    config_type: String,
    name: String
) -> Result<ConfigFile, String> {
    let config_dir = get_config_dir(&app_handle, &config_type);
    
    // 确保目录存在
    ensure_dir_exists(&config_dir).map_err(|e| e.to_string())?;
    
    // 创建文件名
    let file_name = if name.ends_with(".conf") {
        name
    } else {
        format!("{}.conf", name)
    };
    
    let file_path = config_dir.join(&file_name);
    
    // 检查文件是否已存在
    if file_path.exists() {
        return Err(format!("配置文件 {} 已存在", file_name));
    }
    
    // 创建默认模板
    let template = match config_type.as_str() {
        "main" => include_str!("../templates/main.conf"),
        "site" => include_str!("../templates/site.conf"),
        "custom" => include_str!("../templates/custom.conf"),
        _ => "# 自定义配置文件\n"
    };
    
    // 写入文件
    fs::write(&file_path, template).map_err(|e| e.to_string())?;
    
    // 获取文件元数据
    let metadata = fs::metadata(&file_path).map_err(|e| e.to_string())?;
    let size = format_file_size(metadata.len());
    
    let last_modified = metadata.modified()
        .map(|time| {
            let datetime: DateTime<Local> = time.into();
            datetime.format("%Y-%m-%d %H:%M:%S").to_string()
        })
        .unwrap_or_else(|_| "未知".to_string());
    
    // 返回创建的文件信息
    Ok(ConfigFile {
        name: file_name,
        type_: config_type,
        path: file_path.to_string_lossy().to_string(),
        size,
        last_modified,
        status: ConfigFileStatus::Valid,
        content: None,
    })
}

#[tauri::command]
pub fn read_config_file(path: String) -> Result<String, String> {
    let path = Path::new(&path);
    read_file_content(path).map_err(|e| e.to_string())
}

#[tauri::command]
pub fn save_nginx_config(config: NginxConfig) -> Result<bool, String> {
    let path = Path::new(&config.path);
    
    // 确保父目录存在
    ensure_dir_exists(path).map_err(|e| e.to_string())?;

    // 写入配置文件
    write_file_content(path, &config.content).map_err(|e| e.to_string())?;

    Ok(true)
}

#[tauri::command]
pub fn test_config_validity(path: String) -> Result<bool, String> {
    let path = Path::new(&path);
    let parent = path.parent().ok_or("无法获取父目录")?;
    
    // 使用 nginx -t -c 命令测试配置
    let output = Command::new("nginx")
        .arg("-t")
        .arg("-c")
        .arg(path)
        .current_dir(parent)
        .output()
        .map_err(|e| e.to_string())?;
    
    Ok(output.status.success())
}

#[tauri::command]
pub fn export_config_file(path: String) -> Result<String, String> {
    // 这里实际上只是返回文件内容，前端负责下载
    read_config_file(path)
}

#[tauri::command]
pub fn import_config_file(
    app_handle: tauri::AppHandle,
    file_name: String,
    config_type: String,
    content: String
) -> Result<ConfigFile, String> {
    let config_dir = get_config_dir(&app_handle, &config_type);
    
    // 确保目录存在
    ensure_dir_exists(&config_dir).map_err(|e| e.to_string())?;
    
    // 创建文件名
    let file_name = if file_name.ends_with(".conf") {
        file_name
    } else {
        format!("{}.conf", file_name)
    };
    
    let file_path = config_dir.join(&file_name);
    
    // 写入文件
    fs::write(&file_path, &content).map_err(|e| e.to_string())?;
    
    // 获取文件元数据
    let metadata = fs::metadata(&file_path).map_err(|e| e.to_string())?;
    let size = format_file_size(metadata.len());
    
    let last_modified = metadata.modified()
        .map(|time| {
            let datetime: DateTime<Local> = time.into();
            datetime.format("%Y-%m-%d %H:%M:%S").to_string()
        })
        .unwrap_or_else(|_| "未知".to_string());
    
    // 返回创建的文件信息
    Ok(ConfigFile {
        name: file_name,
        type_: config_type,
        path: file_path.to_string_lossy().to_string(),
        size,
        last_modified,
        status: ConfigFileStatus::Valid,
        content: Some(content),
    })
}

#[tauri::command]
pub fn delete_nginx_config(path: String) -> Result<bool, String> {
    let path = Path::new(&path);
    delete_file(path).map_err(|e| e.to_string())?;
    Ok(true)
} 