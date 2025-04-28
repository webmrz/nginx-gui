use std::fs;
use std::path::Path;
use chrono::{Local, DateTime};
use uuid::Uuid;
use serde_json;
use tauri::Manager;
use reqwest;
use crate::types::nginx::{NginxVersion, NginxVersionConfig};
use crate::error::AppError;

pub struct VersionService {
    versions_dir: String,
    current_link: String,
    config_path: String,
}

impl VersionService {
    pub fn new(app_handle: &tauri::AppHandle) -> Result<Self, AppError> {
        let app_dir = app_handle
            .path()
            .app_data_dir()
            .map_err(|e| AppError::new(&format!("无法获取应用数据目录: {}", e)))?;

        let versions_dir = app_dir.join("nginx_versions").to_string_lossy().to_string();
        let current_link = app_dir.join("nginx_current").to_string_lossy().to_string();
        let config_path = app_dir.join("version_config.json").to_string_lossy().to_string();

        // 确保目录存在
        fs::create_dir_all(&versions_dir)?;

        Ok(Self {
            versions_dir,
            current_link,
            config_path,
        })
    }

    pub fn get_versions(&self) -> Result<Vec<NginxVersion>, AppError> {
        let mut versions = Vec::new();
        let entries = fs::read_dir(&self.versions_dir)?;

        for entry in entries {
            let entry = entry?;
            let path = entry.path();
            if path.is_dir() {
                let version = path.file_name()
                    .ok_or_else(|| AppError::new("无法获取版本名称"))?
                    .to_string_lossy()
                    .to_string();

                let metadata = fs::metadata(&path)?;
                let installed_at = metadata.created()
                    .map(|time| {
                        let datetime: DateTime<Local> = time.into();
                        datetime
                    })
                    .unwrap_or_else(|_| Local::now());

                let is_current = self.is_current_version(&version)?;

                versions.push(NginxVersion {
                    version,
                    path: path.to_string_lossy().to_string(),
                    is_current,
                    installed_at,
                });
            }
        }

        Ok(versions)
    }

    pub fn get_current_version(&self) -> Result<String, AppError> {
        if !Path::new(&self.current_link).exists() {
            return Ok("".to_string());
        }

        let target = fs::read_link(&self.current_link)?;
        let version = target.file_name()
            .ok_or_else(|| AppError::new("无法获取当前版本"))?
            .to_string_lossy()
            .to_string();

        Ok(version)
    }

    pub fn set_current_version(&self, version: &str) -> Result<(), AppError> {
        let version_path = Path::new(&self.versions_dir).join(version);
        if !version_path.exists() {
            return Err(AppError::new("版本不存在"));
        }

        // 删除现有的软链接
        if Path::new(&self.current_link).exists() {
            fs::remove_file(&self.current_link)?;
        }

        // 创建新的软链接
        #[cfg(target_os = "windows")]
        {
            use std::os::windows::fs::symlink_dir;
            symlink_dir(version_path, &self.current_link)?;
        }

        #[cfg(not(target_os = "windows"))]
        {
            use std::os::unix::fs::symlink;
            symlink(version_path, &self.current_link)?;
        }

        // 更新配置
        self.save_config(version)?;

        Ok(())
    }

    pub fn install_version(&self, version: &str, source_path: &str) -> Result<(), AppError> {
        let target_path = Path::new(&self.versions_dir).join(version);
        if target_path.exists() {
            return Err(AppError::new("版本已存在"));
        }

        // 复制文件
        fs::create_dir_all(&target_path)?;
        copy_dir(source_path, &target_path)?;

        // 如果是第一个版本，设置为当前版本
        if self.get_versions()?.is_empty() {
            self.set_current_version(version)?;
        }

        Ok(())
    }

    pub fn uninstall_version(&self, version: &str) -> Result<(), AppError> {
        let version_path = Path::new(&self.versions_dir).join(version);
        if !version_path.exists() {
            return Err(AppError::new("版本不存在"));
        }

        // 如果是当前版本，不允许卸载
        if self.is_current_version(version)? {
            return Err(AppError::new("不能卸载当前版本"));
        }

        // 删除版本目录
        fs::remove_dir_all(version_path)?;

        Ok(())
    }

    fn is_current_version(&self, version: &str) -> Result<bool, AppError> {
        let current = self.get_current_version()?;
        Ok(current == version)
    }

    fn save_config(&self, current_version: &str) -> Result<(), AppError> {
        let versions = self.get_versions()?;
        let config = NginxVersionConfig {
            current_version: current_version.to_string(),
            versions,
        };

        let content = serde_json::to_string_pretty(&config)?;
        fs::write(&self.config_path, content)?;

        Ok(())
    }

    pub async fn download_and_install_version(&self, version: &str, install_path: &str) -> Result<(), AppError> {
        // 构建下载 URL
        let download_url = format!(
            "http://nginx.org/download/nginx-{}.zip",
            version
        );

        // 下载文件
        let response = reqwest::get(&download_url)
            .await
            .map_err(|e| AppError::new(&format!("下载失败: {}", e)))?;

        if !response.status().is_success() {
            return Err(AppError::new("下载失败: 服务器返回错误"));
        }

        // 创建临时目录
        let temp_dir = std::env::temp_dir().join(format!("nginx-{}", Uuid::new_v4()));
        fs::create_dir_all(&temp_dir)?;

        // 保存下载的文件
        let zip_path = temp_dir.join("nginx.zip");
        let content = response.bytes()
            .await
            .map_err(|e| AppError::new(&format!("下载失败: {}", e)))?;
        fs::write(&zip_path, content)?;

        // 解压文件
        let target_path = Path::new(install_path).join(format!("nginx-{}", version));
        fs::create_dir_all(&target_path)?;

        // 使用 zip 库解压文件
        let file = std::fs::File::open(&zip_path)?;
        let mut archive = zip::ZipArchive::new(file)?;
        archive.extract(&target_path)?;

        // 安装版本
        self.install_version(version, target_path.to_string_lossy().as_ref())?;

        // 清理临时文件
        fs::remove_dir_all(temp_dir)?;

        Ok(())
    }
}

fn copy_dir(src: &str, dst: &Path) -> Result<(), AppError> {
    let src_path = Path::new(src);
    if !src_path.exists() {
        return Err(AppError::new("源目录不存在"));
    }

    if !dst.exists() {
        fs::create_dir_all(dst)?;
    }

    for entry in fs::read_dir(src_path)? {
        let entry = entry?;
        let path = entry.path();
        let target = dst.join(path.file_name().unwrap());

        if path.is_dir() {
            copy_dir(path.to_string_lossy().as_ref(), &target)?;
        } else {
            fs::copy(path, target)?;
        }
    }

    Ok(())
} 