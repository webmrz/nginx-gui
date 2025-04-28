use std::process::Command;
use std::path::PathBuf;
use tauri::{Manager, AppHandle};
use serde::{Serialize, Deserialize};
use std::fs;
use std::io::Write;
use chrono::Local;
use tokio::task;
use reqwest;

#[derive(Debug, Serialize, Deserialize)]
pub struct NginxStatus {
    pub is_running: bool,
    pub version: String,
    pub config_test: bool,
    pub error_message: Option<String>,
}

#[derive(Debug)]
pub struct NginxService {
    app_handle: AppHandle,
}

impl NginxService {
    pub fn new(app_handle: AppHandle) -> Self {
        let service = Self { app_handle };
        
        // 初始化日志目录
        if let Err(e) = service.init_log_dirs() {
            println!("[ERROR] 初始化日志目录失败: {}", e);
        }
        
        service
    }

    fn init_log_dirs(&self) -> Result<(), String> {
        let nginx_path = self.get_nginx_path();
        let logs_dir = nginx_path.join("logs");
        
        // 创建日志目录
        if !logs_dir.exists() {
            println!("[INFO] 创建日志目录: {:?}", logs_dir);
            fs::create_dir_all(&logs_dir).map_err(|e| {
                let error_msg = format!("创建日志目录失败: {}", e);
                println!("[ERROR] {}", error_msg);
                error_msg
            })?;
        }

        // 创建并初始化日志文件
        let log_files = [
            "nginx-service.log",
            "error.log",
            "access.log"
        ];

        for log_file in log_files.iter() {
            let log_path = logs_dir.join(log_file);
            if !log_path.exists() {
                println!("[INFO] 创建日志文件: {:?}", log_path);
                fs::write(&log_path, "").map_err(|e| {
                    let error_msg = format!("创建日志文件失败: {}", e);
                    println!("[ERROR] {}", error_msg);
                    error_msg
                })?;
            }
        }

        Ok(())
    }

    fn get_nginx_path(&self) -> PathBuf {
        let resource_path = self.app_handle.path().resource_dir()
            .expect("Failed to get resource directory");
        
        // println!("[INFO] 资源目录: {:?}", resource_path);
        
        // 获取项目根目录
        let manifest_dir = std::env::var("CARGO_MANIFEST_DIR").unwrap();
        let manifest_dir = std::path::Path::new(&manifest_dir);
        let nginx_path = manifest_dir.join("resources").join("nginx-1.24.0");
        
        // println!("[INFO] Nginx 目录: {:?}", nginx_path);
        
        if !nginx_path.exists() {
            println!("[ERROR] Nginx 目录不存在: {:?}", nginx_path);
            panic!("Nginx 目录不存在。请确保 nginx-1.24.0 目录存在于 resources 文件夹中");
        }
        
        nginx_path
    }

    fn log_operation(&self, operation: &str, result: &str) {
        let log_path = self.get_nginx_path().join("logs").join("nginx-service.log");
        let timestamp = Local::now().format("%Y-%m-%d %H:%M:%S").to_string();
        let log_entry = format!("[{}] {}: {}\n", timestamp, operation, result);
        
        if let Ok(mut file) = fs::OpenOptions::new()
            .create(true)
            .append(true)
            .open(log_path) 
        {
            let _ = file.write_all(log_entry.as_bytes());
        }
    }

    fn emit_status_change(&self) {
        println!("[INFO] Nginx status changed");
        // 先注释掉事件通知代码，以确保应用能启动
        // 未来可以研究正确的 Tauri v2 事件API实现
        // if let Err(e) = self.app_handle.emit_to("main", "nginx-status-change", ()) {
        //     println!("[WARN] Failed to emit status change event: {}", e);
        // }
    }

    pub async fn start(&self) -> Result<(), String> {
        let nginx_path = self.get_nginx_path();
        let nginx_exe = nginx_path.join("nginx.exe");
        
        println!("[DEBUG] Starting Nginx service...");
        println!("[DEBUG] Nginx path: {:?}", nginx_path);
        println!("[DEBUG] Nginx executable: {:?}", nginx_exe);
        
        if !nginx_exe.exists() {
            let error_msg = "Nginx executable not found. Please ensure nginx.exe is placed in the nginx-1.24.0 directory";
            println!("[ERROR] {}", error_msg);
            println!("[INFO] Please ensure nginx.exe is placed in the nginx-1.24.0 directory");
            self.log_operation("start", error_msg);
            return Err(error_msg.to_string());
        }

        println!("[DEBUG] Checking if Nginx is already running...");
        let result = task::spawn_blocking(move || {
            Command::new("tasklist")
                .arg("/FI")
                .arg("IMAGENAME eq nginx.exe")
                .output()
        }).await.map_err(|e| {
            let error_msg = format!("Failed to spawn task: {}", e);
            println!("[ERROR] {}", error_msg);
            error_msg
        })?
        .map_err(|e| {
            let error_msg = format!("Failed to check Nginx process: {}", e);
            println!("[ERROR] {}", error_msg);
            error_msg
        })?;

        let output_str = String::from_utf8_lossy(&result.stdout).to_string();
        if output_str.contains("nginx.exe") {
            println!("[WARN] Nginx is already running");
            self.log_operation("start", "Nginx is already running");
            return Ok(());
        }

        println!("[DEBUG] Starting Nginx process...");
        let nginx_path_clone = nginx_path.clone();
        let nginx_exe_clone = nginx_exe.clone();
        let status = task::spawn_blocking(move || {
            Command::new(&nginx_exe_clone)
                .current_dir(&nginx_path_clone)
                .status()
        }).await.map_err(|e| {
            let error_msg = format!("Failed to spawn task: {}", e);
            println!("[ERROR] {}", error_msg);
            error_msg
        })?
        .map_err(|e| {
            let error_msg = format!("Failed to start Nginx: {}", e);
            println!("[ERROR] {}", error_msg);
            error_msg
        })?;

        if !status.success() {
            let error_msg = format!("Nginx failed to start with status: {:?}", status.code());
            println!("[ERROR] {}", error_msg);
            self.log_operation("start", &error_msg);
            return Err(error_msg);
        }

        let pid_file = nginx_path.join("logs").join("nginx.pid");
        if pid_file.exists() {
            match fs::remove_file(&pid_file) {
                Ok(_) => println!("[DEBUG] Removed nginx.pid file"),
                Err(e) => println!("[WARN] Failed to remove nginx.pid file: {}", e)
            }
        }

        println!("[INFO] Nginx started successfully");
        self.log_operation("start", "Nginx started successfully");
        self.emit_status_change();
        Ok(())
    }

    pub async fn stop(&self) -> Result<(), String> {
        let nginx_path = self.get_nginx_path();
        let nginx_exe = nginx_path.join("nginx.exe");
        
        if !nginx_exe.exists() {
            self.log_operation("stop", "Nginx executable not found");
            return Err("Nginx executable not found".to_string());
        }

        let nginx_path_clone = nginx_path.clone();
        let nginx_exe_clone = nginx_exe.clone();
        let result = task::spawn_blocking(move || {
            Command::new(nginx_exe_clone)
                .arg("-s")
                .arg("stop")
                .current_dir(nginx_path_clone)
                .output()
        }).await.map_err(|e| format!("Failed to spawn task: {}", e))?
          .map_err(|e| format!("Failed to stop Nginx: {}", e))?;

        if result.status.success() {
            self.log_operation("stop", "Service stopped successfully");
            self.emit_status_change();
            Ok(())
        } else {
            let error = String::from_utf8_lossy(&result.stderr).to_string();
            self.log_operation("stop", &format!("Failed to stop service: {}", error));
            Err(error)
        }
    }

    pub async fn restart(&self) -> Result<(), String> {
        let nginx_path = self.get_nginx_path();
        let nginx_exe = nginx_path.join("nginx.exe");
        
        if !nginx_exe.exists() {
            self.log_operation("restart", "Nginx executable not found");
            return Err("Nginx executable not found".to_string());
        }

        let nginx_path_clone = nginx_path.clone();
        let nginx_exe_clone = nginx_exe.clone();
        let result = task::spawn_blocking(move || {
            Command::new(nginx_exe_clone)
                .arg("-s")
                .arg("reload")
                .current_dir(nginx_path_clone)
                .output()
        }).await.map_err(|e| format!("Failed to spawn task: {}", e))?
          .map_err(|e| format!("Failed to restart Nginx: {}", e))?;

        if result.status.success() {
            self.log_operation("restart", "Service restarted successfully");
            self.emit_status_change();
            Ok(())
        } else {
            let error = String::from_utf8_lossy(&result.stderr).to_string();
            self.log_operation("restart", &format!("Failed to restart service: {}", error));
            Err(error)
        }
    }

    pub async fn test_config(&self) -> Result<(), String> {
        let nginx_path = self.get_nginx_path();
        let nginx_exe = nginx_path.join("nginx.exe");
        
        if !nginx_exe.exists() {
            self.log_operation("test_config", "Nginx executable not found");
            return Err("Nginx executable not found".to_string());
        }

        let nginx_path_clone = nginx_path.clone();
        let nginx_exe_clone = nginx_exe.clone();
        let result = task::spawn_blocking(move || {
            Command::new(nginx_exe_clone)
                .arg("-t")
                .current_dir(nginx_path_clone)
                .output()
        }).await.map_err(|e| format!("Failed to spawn task: {}", e))?
          .map_err(|e| format!("Failed to test Nginx config: {}", e))?;

        if result.status.success() {
            self.log_operation("test_config", "Configuration test successful");
            Ok(())
        } else {
            let error = String::from_utf8_lossy(&result.stderr).to_string();
            self.log_operation("test_config", &format!("Configuration test failed: {}", error));
            Err(error)
        }
    }

    pub async fn get_status(&self) -> Result<String, String> {
        let nginx_path = self.get_nginx_path();
        let nginx_exe = nginx_path.join("nginx.exe");
        
        if !nginx_exe.exists() {
            println!("[ERROR] Nginx executable not found at {:?}", nginx_exe);
            self.log_operation("get_status", "Nginx executable not found");
            return Err("Nginx executable not found".to_string());
        }

        println!("[DEBUG] Checking Nginx status...");
        
        // 使用spawn_blocking执行阻塞操作
        let result = task::spawn_blocking(move || {
            Command::new("tasklist")
                .arg("/FI")
                .arg("IMAGENAME eq nginx.exe")
                .output()
        }).await.map_err(|e| format!("Failed to spawn task: {}", e))?
          .map_err(|e| format!("Failed to check Nginx status: {}", e))?;

        let output_str = String::from_utf8_lossy(&result.stdout).to_string();
        let is_running = output_str.contains("nginx.exe");
        
        println!("[DEBUG] Nginx process check result: {}", is_running);
        self.log_operation("get_status", if is_running { "Service is running" } else { "Service is stopped" });
        
        Ok(if is_running { "running".to_string() } else { "stopped".to_string() })
    }

    pub async fn get_version(&self) -> Result<String, String> {
        let nginx_path = self.get_nginx_path();
        let nginx_exe = nginx_path.join("nginx.exe");
        
        if !nginx_exe.exists() {
            self.log_operation("get_version", "Nginx executable not found");
            return Err("Nginx executable not found".to_string());
        }

        // 使用spawn_blocking执行阻塞操作
        let nginx_path_clone = nginx_path.clone();
        let nginx_exe_clone = nginx_exe.clone();
        let result = task::spawn_blocking(move || {
            Command::new(nginx_exe_clone)
                .arg("-v")
                .current_dir(nginx_path_clone)
                .output()
        }).await.map_err(|e| format!("Failed to spawn task: {}", e))?
          .map_err(|e| format!("Failed to get Nginx version: {}", e))?;

        if result.status.success() {
            let full_version = String::from_utf8_lossy(&result.stderr).to_string();
            
            // 提取版本号 "nginx/x.x.x" 部分
            let version = if let Some(version_part) = full_version.split_whitespace().last() {
                version_part.to_string()
            } else {
                full_version
            };
            
            self.log_operation("get_version", &format!("Version: {}", version));
            Ok(version)
        } else {
            let error = String::from_utf8_lossy(&result.stderr).to_string();
            self.log_operation("get_version", &format!("Failed to get version: {}", error));
            Err(error)
        }
    }

    pub async fn get_logs(&self, log_type: &str, num_lines: usize, search: Option<String>, level: Option<String>) -> Result<String, String> {
        println!("[INFO] 正在获取日志，类型: {}, 行数: {}", log_type, num_lines);
        
        let nginx_path = self.get_nginx_path();
        let log_file = match log_type {
            "service" => nginx_path.join("logs").join("nginx-service.log"),
            "error" => nginx_path.join("logs").join("error.log"),
            "access" => nginx_path.join("logs").join("access.log"),
            _ => {
                let error_msg = format!("无效的日志类型: {}，支持的日志类型: access, error, service", log_type);
                println!("[ERROR] {}", error_msg);
                return Err(error_msg);
            }
        };

        // 读取文件内容
        println!("[INFO] 读取日志文件: {:?}", log_file);
        
        let log_file_clone = log_file.clone();
        let search_clone = search.clone();
        let level_clone = level.clone();
        let num_lines_clone = num_lines;
        
        // 使用spawn_blocking读取和处理文件
        let lines_result = task::spawn_blocking(move || -> Result<Vec<String>, String> {
            let content = fs::read_to_string(&log_file_clone).map_err(|e| {
                let error_msg = format!("读取日志文件失败: {}", e);
                println!("[ERROR] {}", error_msg);
                error_msg
            })?;

            // 处理日志内容
            let mut lines: Vec<&str> = content.lines().collect();
            
            // 如果需要，从末尾开始获取指定行数
            if lines.len() > num_lines_clone {
                lines = lines[lines.len() - num_lines_clone..].to_vec();
            }

            // 应用搜索过滤
            if let Some(search_str) = &search_clone {
                if !search_str.is_empty() {
                    println!("[INFO] 应用搜索过滤: {}", search_str);
                    lines = lines.into_iter()
                        .filter(|line| line.contains(search_str))
                        .collect();
                }
            }

            // 应用日志级别过滤
            if let Some(level_str) = &level_clone {
                if !level_str.is_empty() {
                    println!("[INFO] 应用级别过滤: {}", level_str);
                    lines = lines.into_iter()
                        .filter(|line| line.contains(level_str))
                        .collect();
                }
            }
            
            Ok(lines.into_iter().map(String::from).collect())
        }).await.map_err(|e| format!("Failed to process logs: {}", e))?;

        // 从Result中提取Vec<String>
        let lines = lines_result?;
        
        println!("[INFO] 成功获取日志，返回 {} 行", lines.len());
        Ok(lines.join("\n"))
    }

    pub async fn clear_log(&self, log_type: &str) -> Result<(), String> {
        println!("[DEBUG] 开始清空日志，类型: {}", log_type);
        
        let nginx_path = self.get_nginx_path();
        println!("[DEBUG] Nginx 路径: {:?}", nginx_path);
        
        let log_file = match log_type {
            "service" => nginx_path.join("logs").join("nginx-service.log"),
            "error" => nginx_path.join("logs").join("error.log"),
            "access" => nginx_path.join("logs").join("access.log"),
            _ => {
                let error_msg = format!("无效的日志类型: {}，支持的日志类型: access, error, service", log_type);
                println!("[ERROR] {}", error_msg);
                return Err(error_msg);
            }
        };
        
        println!("[DEBUG] 目标日志文件: {:?}", log_file);
        
        // 确保日志目录存在
        if let Some(parent) = log_file.parent() {
            if !parent.exists() {
                println!("[INFO] 创建日志目录: {:?}", parent);
                fs::create_dir_all(parent).map_err(|e| {
                    let error_msg = format!("创建日志目录失败: {}", e);
                    println!("[ERROR] {}", error_msg);
                    error_msg
                })?;
            }
        }
        
        if !log_file.exists() {
            println!("[INFO] 日志文件不存在，创建新文件: {:?}", log_file);
            fs::write(&log_file, "").map_err(|e| {
                let error_msg = format!("创建日志文件失败: {}", e);
                println!("[ERROR] {}", error_msg);
                error_msg
            })?;
        }
        
        // 检查文件权限
        if let Err(e) = fs::metadata(&log_file) {
            let error_msg = format!("无法访问日志文件: {}", e);
            println!("[ERROR] {}", error_msg);
            return Err(error_msg);
        }
        
        // 使用spawn_blocking执行文件操作
        let log_file_clone = log_file.clone();
        let result = task::spawn_blocking(move || {
            // 尝试获取文件锁并清空文件
            match fs::OpenOptions::new()
                .write(true)
                .truncate(true)
                .create(true)
                .open(&log_file_clone) 
            {
                Ok(_) => Ok(()),
                Err(e) => {
                    let error_msg = format!("无法获取文件锁: {}", e);
                    println!("[ERROR] {}", error_msg);
                    Err(error_msg)
                }
            }
        }).await.map_err(|e| format!("Failed to spawn task: {}", e))?;
        
        match result {
            Ok(_) => {
                println!("[INFO] 成功清空日志文件: {:?}", log_file);
                self.log_operation("clear_log", &format!("清空{}日志成功", log_type));
                Ok(())
            },
            Err(e) => {
                println!("[ERROR] 清空日志文件失败: {}", e);
                Err(e)
            }
        }
    }

    pub async fn get_cpu_usage(&self) -> Result<f64, String> {
        // 使用 wmic 获取 Nginx 进程的 CPU 使用率
        let output = task::spawn_blocking(|| {
            Command::new("wmic")
                .args(&[
                    "path",
                    "Win32_PerfFormattedData_PerfProc_Process",
                    "where",
                    "Name='nginx.exe'",
                    "get",
                    "PercentProcessorTime"
                ])
                .output()
        }).await.map_err(|e| format!("Failed to spawn task: {}", e))?
          .map_err(|e| format!("Failed to get CPU usage: {}", e))?;

        let output_str = String::from_utf8_lossy(&output.stdout).to_string();
        let lines: Vec<&str> = output_str.lines().collect();
        
        if lines.len() >= 2 {
            if let Ok(cpu) = lines[1].trim().parse::<f64>() {
                return Ok(cpu);
            }
        }
        
        Ok(0.0)
    }

    pub async fn get_memory_usage(&self) -> Result<f64, String> {
        // 获取系统总内存
        let total_mem_output = task::spawn_blocking(|| {
            Command::new("wmic")
                .args(&["computersystem", "get", "TotalPhysicalMemory"])
                .output()
        }).await.map_err(|e| format!("Failed to spawn task: {}", e))?
          .map_err(|e| format!("Failed to get total memory: {}", e))?;
        
        let total_mem_str = String::from_utf8_lossy(&total_mem_output.stdout).to_string();
        let total_mem: u64 = total_mem_str
            .lines()
            .nth(1)
            .and_then(|line| line.trim().parse().ok())
            .unwrap_or(0);

        // 获取 Nginx 进程内存使用
        let output = task::spawn_blocking(|| {
            Command::new("wmic")
                .args(&[
                    "process",
                    "where",
                    "name='nginx.exe'",
                    "get",
                    "WorkingSetSize"
                ])
                .output()
        }).await.map_err(|e| format!("Failed to spawn task: {}", e))?
          .map_err(|e| format!("Failed to get memory usage: {}", e))?;

        let output_str = String::from_utf8_lossy(&output.stdout).to_string();
        let lines: Vec<&str> = output_str.lines().collect();
        
        if lines.len() >= 2 {
            if let Ok(mem) = lines[1].trim().parse::<u64>() {
                if total_mem > 0 {
                    // 计算内存使用百分比
                    let percentage = (mem as f64 / total_mem as f64) * 100.0;
                    return Ok(percentage);
                }
            }
        }
        
        Ok(0.0)
    }

    pub async fn get_performance_metrics(&self) -> Result<(String, String), String> {
        // 获取 CPU 使用率
        let cpu_usage = match self.get_cpu_usage().await {
            Ok(usage) => format!("{:.1}", usage),
            Err(e) => {
                println!("[ERROR] 获取 CPU 使用率失败: {}", e);
                "0.0".to_string()
            }
        };

        // 获取内存使用率
        let memory_usage = match self.get_memory_usage().await {
            Ok(usage) => format!("{:.1}", usage),
            Err(e) => {
                println!("[ERROR] 获取内存使用率失败: {}", e);
                "0.0".to_string()
            }
        };

        Ok((cpu_usage, memory_usage))
    }

    pub async fn get_connection_info(&self) -> Result<(usize, usize, usize), String> {
        // 获取活动连接数
        let active_connections = match self.get_active_connections().await {
            Ok(connections) => connections,
            Err(e) => {
                println!("[ERROR] 获取活动连接数失败: {}", e);
                0
            }
        };

        // 获取总连接数
        let total_connections = match self.get_total_connections().await {
            Ok(connections) => connections,
            Err(e) => {
                println!("[ERROR] 获取总连接数失败: {}", e);
                0
            }
        };

        // 获取每秒请求数
        let requests_per_second = match self.get_requests_per_second().await {
            Ok(rps) => rps,
            Err(e) => {
                println!("[ERROR] 获取每秒请求数失败: {}", e);
                0
            }
        };

        Ok((active_connections, total_connections, requests_per_second))
    }

    pub async fn get_active_connections(&self) -> Result<usize, String> {
        // 首先检查 nginx 是否运行
        let status_output = Command::new("tasklist")
            .arg("/FI")
            .arg("IMAGENAME eq nginx.exe")
            .output()
            .map_err(|e| e.to_string())?;
        
        let status_str = String::from_utf8_lossy(&status_output.stdout).to_string();
        if !status_str.contains("nginx.exe") {
            return Ok(0);
        }
        
        // 尝试从 stub_status 页面获取数据
        match reqwest::get("http://localhost/nginx_status").await {
            Ok(response) => {
                if response.status().is_success() {
                    match response.text().await {
                        Ok(text) => {
                            // 解析 stub_status 输出
                            for line in text.lines() {
                                if line.starts_with("Active connections:") {
                                    if let Some(count_str) = line.split(':').nth(1) {
                                        if let Ok(count) = count_str.trim().parse::<usize>() {
                                            return Ok(count);
                                        }
                                    }
                                }
                            }
                            println!("[ERROR] Failed to parse active connections from stub_status");
                            Ok(0)
                        },
                        Err(e) => {
                            println!("[ERROR] Failed to read stub_status response: {}", e);
                            Ok(0)
                        }
                    }
                } else {
                    println!("[ERROR] Failed to get stub_status, status: {}", response.status());
                    Ok(0)
                }
            },
            Err(e) => {
                println!("[ERROR] Failed to connect to stub_status: {}", e);
                Ok(0)
            }
        }
    }

    pub async fn get_total_connections(&self) -> Result<usize, String> {
        // 首先检查 nginx 是否运行
        let status_output = Command::new("tasklist")
            .arg("/FI")
            .arg("IMAGENAME eq nginx.exe")
            .output()
            .map_err(|e| e.to_string())?;
        
        let status_str = String::from_utf8_lossy(&status_output.stdout).to_string();
        if !status_str.contains("nginx.exe") {
            return Ok(0);
        }
        
        // 尝试从 stub_status 页面获取数据
        match reqwest::get("http://localhost/nginx_status").await {
            Ok(response) => {
                if response.status().is_success() {
                    match response.text().await {
                        Ok(text) => {
                            // 解析 stub_status 输出，查找总连接数
                            for line in text.lines() {
                                if line.contains("accepts") {
                                    if let Some(count_str) = line.split_whitespace().nth(1) {
                                        if let Ok(count) = count_str.parse::<usize>() {
                                            return Ok(count);
                                        }
                                    }
                                }
                            }
                            println!("[ERROR] Failed to parse total connections from stub_status");
                            Ok(0)
                        },
                        Err(e) => {
                            println!("[ERROR] Failed to read stub_status response: {}", e);
                            Ok(0)
                        }
                    }
                } else {
                    println!("[ERROR] Failed to get stub_status, status: {}", response.status());
                    Ok(0)
                }
            },
            Err(e) => {
                println!("[ERROR] Failed to connect to stub_status: {}", e);
                Ok(0)
            }
        }
    }

    pub async fn get_requests_per_second(&self) -> Result<usize, String> {
        // 首先检查 nginx 是否运行
        let status_output = Command::new("tasklist")
            .arg("/FI")
            .arg("IMAGENAME eq nginx.exe")
            .output()
            .map_err(|e| e.to_string())?;
        
        let status_str = String::from_utf8_lossy(&status_output.stdout).to_string();
        if !status_str.contains("nginx.exe") {
            return Ok(0);
        }
        
        // 从 stub_status 页面获取数据
        match reqwest::get("http://localhost/nginx_status").await {
            Ok(response) => {
                if response.status().is_success() {
                    match response.text().await {
                        Ok(text) => {
                            let mut total_requests = 0;
                            // 解析 stub_status 输出，查找总请求数
                            for line in text.lines() {
                                if line.trim().split_whitespace().count() == 3 {
                                    let numbers: Vec<&str> = line.trim().split_whitespace().collect();
                                    if let Ok(requests) = numbers[2].parse::<usize>() {
                                        total_requests = requests;
                                        break;
                                    }
                                }
                            }
                            // 计算每秒请求数（这里简化处理，返回一个估计值）
                            Ok(if total_requests > 0 { total_requests / 60 } else { 0 })
                        },
                        Err(e) => {
                            println!("[ERROR] Failed to read stub_status response: {}", e);
                            Ok(0)
                        }
                    }
                } else {
                    println!("[ERROR] Failed to get stub_status, status: {}", response.status());
                    Ok(0)
                }
            },
            Err(e) => {
                println!("[ERROR] Failed to connect to stub_status: {}", e);
                Ok(0)
            }
        }
    }

    pub async fn get_uptime(&self) -> Result<String, String> {
        // 首先检查 nginx 是否运行
        let status_output = Command::new("tasklist")
            .arg("/FI")
            .arg("IMAGENAME eq nginx.exe")
            .output()
            .map_err(|e| e.to_string())?;
        
        let status_str = String::from_utf8_lossy(&status_output.stdout).to_string();
        if !status_str.contains("nginx.exe") {
            return Ok("0s".to_string());
        }
        
        // 获取进程创建时间
        let output = Command::new("wmic")
            .args(&[
                "process",
                "where",
                "name='nginx.exe'",
                "get",
                "creationdate"
            ])
            .output()
            .map_err(|e| e.to_string())?;

        let output_str = String::from_utf8_lossy(&output.stdout).to_string();
        let lines: Vec<&str> = output_str.lines().filter(|line| !line.trim().is_empty()).collect();
        
        if lines.len() < 2 {
            return Ok("未知".to_string());
        }
        
        let creation_date = lines[1].trim();
        if creation_date.len() < 14 {
            return Ok("格式错误".to_string());
        }
        
        use chrono::{Local, NaiveDateTime, TimeZone};
        
        let year = &creation_date[0..4];
        let month = &creation_date[4..6];
        let day = &creation_date[6..8];
        let hour = &creation_date[8..10];
        let minute = &creation_date[10..12];
        let second = &creation_date[12..14];
        
        let date_str = format!("{}-{}-{} {}:{}:{}", year, month, day, hour, minute, second);
        
        match NaiveDateTime::parse_from_str(&date_str, "%Y-%m-%d %H:%M:%S") {
            Ok(dt) => {
                let start_time = Local.from_local_datetime(&dt)
                                   .single()
                                   .unwrap_or_else(|| Local::now());
                let now = Local::now();
                let duration = now.signed_duration_since(start_time);
                
                let hours = duration.num_hours();
                let minutes = duration.num_minutes() % 60;
                let seconds = duration.num_seconds() % 60;
                
                Ok(format!("{}:{:02}:{:02}", hours, minutes, seconds))
            },
            Err(_) => Ok("解析错误".to_string()),
        }
    }

    pub async fn get_service_info(&self) -> Result<ServiceInfo, String> {
        println!("[DEBUG] 开始获取服务信息");
        
        // 检查 Nginx 是否正在运行
        let status = match self.get_status().await {
            Ok(s) => s,
            Err(e) => {
                println!("[ERROR] 检查 Nginx 状态失败: {}", e);
                return Ok(ServiceInfo {
                    status: "stopped".to_string(),
                    version: None,
                    uptime: None,
                    cpu_usage: None,
                    memory_usage: None,
                    active_connections: None,
                    total_connections: None,
                    requests_per_second: None,
                });
            }
        };

        println!("[DEBUG] Nginx 状态: {}", status);
        
        if status != "running" {
            println!("[INFO] Nginx 未运行，返回默认状态");
            return Ok(ServiceInfo {
                status: "stopped".to_string(),
                version: None,
                uptime: None,
                cpu_usage: None,
                memory_usage: None,
                active_connections: None,
                total_connections: None,
                requests_per_second: None,
            });
        }

        // 获取版本信息
        let version = match self.get_version().await {
            Ok(v) => Some(v),
            Err(e) => {
                println!("[ERROR] 获取 Nginx 版本失败: {}", e);
                None
            }
        };

        // 获取运行时间
        let uptime = match self.get_uptime().await {
            Ok(u) => Some(u),
            Err(e) => {
                println!("[ERROR] 获取 Nginx 运行时间失败: {}", e);
                None
            }
        };

        // 获取性能指标
        let (cpu_usage, memory_usage) = match self.get_performance_metrics().await {
            Ok((cpu, mem)) => (Some(cpu), Some(mem)),
            Err(e) => {
                println!("[ERROR] 获取 Nginx 性能指标失败: {}", e);
                (None, None)
            }
        };

        // 获取连接信息
        let (active_connections, total_connections, requests_per_second) = match self.get_connection_info().await {
            Ok((active, total, rps)) => (Some(active), Some(total), Some(rps)),
            Err(e) => {
                println!("[ERROR] 获取 Nginx 连接信息失败: {}", e);
                (None, None, None)
            }
        };

        println!("[DEBUG] 服务信息获取完成");
        
        Ok(ServiceInfo {
            status: "running".to_string(),
            version,
            uptime,
            cpu_usage,
            memory_usage,
            active_connections,
            total_connections,
            requests_per_second,
        })
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ServiceInfo {
    pub status: String,
    pub version: Option<String>,
    pub uptime: Option<String>,
    pub cpu_usage: Option<String>,
    pub memory_usage: Option<String>,
    pub active_connections: Option<usize>,
    pub total_connections: Option<usize>,
    pub requests_per_second: Option<usize>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DashboardStats {
    pub active_connections: usize,
    pub requests_per_second: usize,
    pub total_requests: usize,
    pub memory_usage: String,
    pub cpu_usage: String,
    pub uptime: String,
}

#[tauri::command]
pub async fn get_service_info(nginx_service: tauri::State<'_, NginxService>) -> Result<ServiceInfo, String> {
    nginx_service.get_service_info().await
}

#[tauri::command]
pub async fn start_nginx(nginx_service: tauri::State<'_, NginxService>) -> Result<(), String> {
    nginx_service.start().await
}

#[tauri::command]
pub async fn stop_nginx(nginx_service: tauri::State<'_, NginxService>) -> Result<(), String> {
    nginx_service.stop().await
}

#[tauri::command]
pub async fn restart_nginx(nginx_service: tauri::State<'_, NginxService>) -> Result<(), String> {
    nginx_service.restart().await
}

#[tauri::command]
pub async fn get_nginx_version(nginx_service: tauri::State<'_, NginxService>) -> Result<String, String> {
    nginx_service.get_version().await
}

#[tauri::command]
pub async fn get_nginx_status(nginx_service: tauri::State<'_, NginxService>) -> Result<String, String> {
    nginx_service.get_status().await
}

#[tauri::command]
pub async fn get_nginx_logs(
    nginx_service: tauri::State<'_, NginxService>,
    log_type: &str,
    lines: usize,
    search: Option<String>,
    level: Option<String>
) -> Result<String, String> {
    nginx_service.get_logs(log_type, lines, search, level).await
}

#[tauri::command]
pub async fn clear_logs(nginx_service: tauri::State<'_, NginxService>, log_type: &str) -> Result<(), String> {
    nginx_service.clear_log(log_type).await
}

#[tauri::command]
pub async fn open_log_folder(nginx_service: tauri::State<'_, NginxService>) -> Result<(), String> {
    let nginx_path = nginx_service.get_nginx_path();
    let logs_dir = nginx_path.join("logs");
    
    if !logs_dir.exists() {
        return Err("日志文件夹不存在".to_string());
    }
    
    let logs_dir_str = logs_dir.to_string_lossy().to_string();
    println!("[INFO] 打开日志文件夹: {}", logs_dir_str);
    
    #[cfg(target_os = "windows")]
    task::spawn_blocking(move || {
        Command::new("explorer")
            .arg(logs_dir_str)
            .spawn()
    }).await.map_err(|e| format!("无法启动文件浏览器: {}", e))?
      .map_err(|e| format!("无法打开日志文件夹: {}", e))?;
    
    #[cfg(target_os = "macos")]
    task::spawn_blocking(move || {
        Command::new("open")
            .arg(logs_dir_str)
            .spawn()
    }).await.map_err(|e| format!("无法启动文件浏览器: {}", e))?
      .map_err(|e| format!("无法打开日志文件夹: {}", e))?;
    
    #[cfg(target_os = "linux")]
    task::spawn_blocking(move || {
        Command::new("xdg-open")
            .arg(logs_dir_str)
            .spawn()
    }).await.map_err(|e| format!("无法启动文件浏览器: {}", e))?
      .map_err(|e| format!("无法打开日志文件夹: {}", e))?;
    
    Ok(())
}

#[tauri::command]
pub async fn check_log_exists(nginx_service: tauri::State<'_, NginxService>, log_type: &str) -> Result<bool, String> {
    let nginx_path = nginx_service.get_nginx_path();
    let log_file = match log_type {
        "service" => nginx_path.join("logs").join("nginx-service.log"),
        "error" => nginx_path.join("logs").join("error.log"),
        "access" => nginx_path.join("logs").join("access.log"),
        _ => return Err(format!("无效的日志类型: {}", log_type))
    };
    
    Ok(log_file.exists())
} 