use std::path::Path;
use std::fs;
use std::process::Command;
use serde::{Serialize, Deserialize};
use tauri::{Manager, AppHandle};
use anyhow::{Result, Error};
use dirs::{desktop_dir, home_dir};

#[derive(Debug, Serialize, Deserialize)]
pub struct Component {
    pub id: String,
    pub name: String,
    pub description: String,
    pub required: bool,
    pub selected: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct InstallConfig {
    pub install_path: String,
    pub components: Vec<Component>,
    pub options: InstallOptions,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct InstallOptions {
    pub create_desktop_shortcut: bool,
    pub create_start_menu_shortcut: bool,
    pub run_after_install: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Update {
    pub version: String,
    pub date: String,
    pub body: String,
}

#[cfg(feature = "updater")]
pub async fn check_update(app: AppHandle) -> Result<Option<Update>, String> {
    let update = app.check_update().await.map_err(|e| e.to_string())?;
    Ok(Some(Update {
        version: update.version,
        date: update.date,
        body: update.body,
    }))
}

#[cfg(not(feature = "updater"))]
pub async fn check_update(_app: AppHandle) -> Result<Option<Update>, String> {
    Ok(None)
}

#[tauri::command]
pub async fn install_application(app: AppHandle, config: InstallConfig) -> Result<(), String> {
    // 创建安装目录
    std::fs::create_dir_all(&config.install_path).map_err(|e| e.to_string())?;
    
    // 复制应用文件
    copy_application_files(&config.install_path).map_err(|e| e.to_string())?;
    
    // 创建快捷方式
    if config.options.create_desktop_shortcut {
        create_desktop_shortcut(&app, &config.install_path).map_err(|e| e.to_string())?;
    }
    
    if config.options.create_start_menu_shortcut {
        create_start_menu_shortcut(&app, &config.install_path).map_err(|e| e.to_string())?;
    }
    
    // 注册系统服务
    if config.components.iter().any(|c| c.id == "service" && c.selected) {
        register_system_service(&config.install_path).map_err(|e| e.to_string())?;
    }
    
    Ok(())
}

fn copy_directory(src: impl AsRef<Path>, dst: impl AsRef<Path>) -> std::io::Result<()> {
    fs::create_dir_all(&dst)?;
    for entry in fs::read_dir(src)? {
        let entry = entry?;
        let ty = entry.file_type()?;
        if ty.is_dir() {
            copy_directory(entry.path(), dst.as_ref().join(entry.file_name()))?;
        } else {
            fs::copy(entry.path(), dst.as_ref().join(entry.file_name()))?;
        }
    }
    Ok(())
}

fn get_app_menu_dir() -> String {
    let home = home_dir().unwrap();
    let menu_dir = home.join("AppData\\Roaming\\Microsoft\\Windows\\Start Menu\\Programs");
    menu_dir.to_str().unwrap().to_string()
}

fn get_desktop_dir() -> String {
    desktop_dir().unwrap().to_str().unwrap().to_string()
}

fn create_start_menu_shortcut(_app: &AppHandle, install_path: &str) -> Result<(), Error> {
    let start_menu = get_app_menu_dir();
    // TODO: 实现开始菜单快捷方式创建
    Ok(())
}

fn create_desktop_shortcut(_app: &AppHandle, install_path: &str) -> Result<(), Error> {
    let desktop_dir = get_desktop_dir();
    // TODO: 实现桌面快捷方式创建
    Ok(())
}

fn create_shortcut(target: &str, shortcut_path: &Path) -> Result<(), Error> {
    let target = format!("{}\\nginx-gui.exe", target);
    Command::new("powershell")
        .args(&[
            "-Command",
            &format!(
                "$WS = New-Object -ComObject WScript.Shell; $SC = $WS.CreateShortcut('{}'); $SC.TargetPath = '{}'; $SC.Save()",
                shortcut_path.display(),
                target
            ),
        ])
        .output()
        .map_err(|e| Error::msg(e.to_string()))?;

    Ok(())
}

fn register_as_service(install_path: &str) -> Result<(), Error> {
    let service_path = format!("{}\\nginx-gui.exe", install_path);
    Command::new("sc")
        .args(&[
            "create",
            "NginxGUI",
            "binPath=",
            &service_path,
            "start=",
            "auto",
        ])
        .output()
        .map_err(|e| Error::msg(e.to_string()))?;

    Ok(())
}

#[cfg(feature = "updater")]
pub async fn perform_update(app: AppHandle) -> Result<(), String> {
    app.check_update().await?.install().await.map_err(|e| e.to_string())?;
    Ok(())
}

#[cfg(not(feature = "updater"))]
pub async fn perform_update(_app: AppHandle) -> Result<(), String> {
    Ok(())
}

fn copy_application_files(install_path: &str) -> Result<(), Error> {
    // TODO: 实现文件复制逻辑
    Ok(())
}

fn register_system_service(install_path: &str) -> Result<(), Error> {
    // TODO: 实现系统服务注册逻辑
    Ok(())
} 