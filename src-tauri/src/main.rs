// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

mod utils;
mod services;
mod install;
mod types;
mod error;

use tauri::Manager;
use tauri_plugin_dialog::DialogExt;
use services::nginx::{NginxService, get_service_info, start_nginx, stop_nginx, restart_nginx, 
                     get_nginx_version, get_nginx_status, get_nginx_logs, clear_logs, 
                     open_log_folder, check_log_exists, get_nginx_metrics};
use services::config::{get_config_files, create_config_file, read_config_file, save_nginx_config, test_config_validity, export_config_file};
use services::settings::{get_settings, save_settings, get_builtin_nginx_path};
#[tokio::main]
async fn main() {
    println!("[INFO] 启动 Nginx GUI 应用程序");
    
    tauri::Builder::default()
        .plugin(tauri_plugin_dialog::init())
        .setup(|app| {
            println!("[INFO] 初始化 Nginx 服务...");
            let nginx_service = NginxService::new(app.handle().clone());
            
            // 启动状态监控
            let nginx_service_clone = nginx_service.clone();
            tokio::spawn(async move {
                nginx_service_clone.start_status_monitor().await;
            });
            
            app.manage(nginx_service);
            println!("[INFO] Nginx 服务初始化完成");
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            get_service_info,
            start_nginx,
            stop_nginx,
            restart_nginx,
            get_nginx_version,
            get_nginx_status,
            get_nginx_logs,
            clear_logs,
            open_log_folder,
            check_log_exists,
            get_nginx_metrics,
            get_config_files,
            create_config_file,
            read_config_file,
            save_nginx_config,
            test_config_validity,
            export_config_file,
            get_settings,
            save_settings,
            get_builtin_nginx_path,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}