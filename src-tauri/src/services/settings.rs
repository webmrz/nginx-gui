use std::fs;
use std::path::Path;
use serde::{Serialize, Deserialize};
use tauri::Manager;
use crate::error::AppError;
use std::path::PathBuf;

#[derive(Debug, Serialize, Deserialize)]
pub struct AppSettings {
    pub nginx_path: String,
    pub config_path: String,
    pub auto_start: bool,
    pub log_level: String,
    pub theme: String,
}

impl Default for AppSettings {
    fn default() -> Self {
        Self {
            nginx_path: String::new(),
            config_path: String::new(),
            auto_start: false,
            log_level: "info".to_string(),
            theme: "light".to_string(),
        }
    }
}

pub struct SettingsService {
    settings_path: String,
    app_handle: tauri::AppHandle,
}

impl SettingsService {
    pub fn new(app_handle: &tauri::AppHandle) -> Result<Self, AppError> {
        let app_dir = app_handle
            .path()
            .app_data_dir()
            .map_err(|e| AppError::new(&format!("无法获取应用数据目录: {}", e)))?;

        let settings_path = app_dir.join("settings.json").to_string_lossy().to_string();

        // 确保目录存在
        if let Some(parent) = Path::new(&settings_path).parent() {
            fs::create_dir_all(parent)?;
        }

        Ok(Self {
            settings_path,
            app_handle: app_handle.clone(),
        })
    }

    fn get_builtin_nginx_path(&self) -> String {
        let manifest_dir = std::env::var("CARGO_MANIFEST_DIR").unwrap_or_default();
        let manifest_dir = std::path::Path::new(&manifest_dir);
        let nginx_path = manifest_dir.join("resources").join("nginx-1.24.0");
        
        nginx_path.to_string_lossy().to_string()
    }

    fn get_builtin_config_path(&self) -> String {
        let nginx_path = self.get_builtin_nginx_path();
        let config_path = PathBuf::from(&nginx_path).join("conf");
        
        config_path.to_string_lossy().to_string()
    }

    pub fn get_settings(&self) -> Result<AppSettings, AppError> {
        println!("[DEBUG] 开始获取设置");
        println!("[DEBUG] 设置文件路径: {}", self.settings_path);

        let mut settings = if !Path::new(&self.settings_path).exists() {
            println!("[INFO] 设置文件不存在，使用默认设置");
            AppSettings::default()
        } else {
            match fs::read_to_string(&self.settings_path) {
                Ok(content) => {
                    println!("[DEBUG] 读取设置文件内容成功");
                    match serde_json::from_str(&content) {
                        Ok(settings) => {
                            println!("[DEBUG] 解析设置文件成功");
                            settings
                        },
                        Err(e) => {
                            println!("[ERROR] 解析设置文件失败: {}", e);
                            return Err(AppError::new(&format!("解析设置文件失败: {}", e)));
                        }
                    }
                },
                Err(e) => {
                    println!("[ERROR] 读取设置文件失败: {}", e);
                    return Err(AppError::new(&format!("读取设置文件失败: {}", e)));
                }
            }
        };

        // 如果nginx_path为空，则使用内置Nginx的路径
        if settings.nginx_path.is_empty() {
            println!("[INFO] nginx_path为空，使用内置Nginx路径");
            settings.nginx_path = self.get_builtin_nginx_path();
        }

        // 如果config_path为空，则使用内置Nginx的配置路径
        if settings.config_path.is_empty() {
            println!("[INFO] config_path为空，使用内置Nginx配置路径");
            settings.config_path = self.get_builtin_config_path();
        }

        println!("[DEBUG] 获取设置成功: {:?}", settings);
        Ok(settings)
    }

    pub fn save_settings(&self, settings: &AppSettings) -> Result<(), AppError> {
        let content = serde_json::to_string_pretty(settings)?;
        fs::write(&self.settings_path, content)?;
        Ok(())
    }
}

#[tauri::command]
pub fn get_settings(app_handle: tauri::AppHandle) -> Result<AppSettings, String> {
    let settings_service = SettingsService::new(&app_handle)
        .map_err(|e| e.to_string())?;
    settings_service.get_settings().map_err(|e| e.to_string())
}

#[tauri::command]
pub fn save_settings(app_handle: tauri::AppHandle, settings: AppSettings) -> Result<(), String> {
    let settings_service = SettingsService::new(&app_handle)
        .map_err(|e| e.to_string())?;
    settings_service.save_settings(&settings).map_err(|e| e.to_string())
}

#[tauri::command]
pub fn get_builtin_nginx_path(app_handle: tauri::AppHandle) -> Result<String, String> {
    let settings_service = SettingsService::new(&app_handle)
        .map_err(|e| e.to_string())?;
    Ok(settings_service.get_builtin_nginx_path())
} 