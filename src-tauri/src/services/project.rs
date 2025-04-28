use std::fs;
use std::path::Path;
use chrono::Local;
use uuid::Uuid;
use serde_json;
use tauri::Manager;
use crate::types::project::{ProjectConfig, ProjectTemplate, CreateProjectRequest, CreateTemplateRequest};
use crate::error::AppError;

pub struct ProjectService {
    projects_dir: String,
    templates_dir: String,
}

impl ProjectService {
    pub fn new(app_handle: &tauri::AppHandle) -> Result<Self, AppError> {
        let app_dir = app_handle
            .path()
            .app_data_dir()
            .map_err(|e| AppError::new(&format!("无法获取应用数据目录: {}", e)))?;

        let projects_dir = app_dir.join("projects").to_string_lossy().to_string();
        let templates_dir = app_dir.join("templates").to_string_lossy().to_string();

        // 确保目录存在
        fs::create_dir_all(&projects_dir)?;
        fs::create_dir_all(&templates_dir)?;

        Ok(Self {
            projects_dir,
            templates_dir,
        })
    }

    pub fn get_projects(&self) -> Result<Vec<ProjectConfig>, AppError> {
        let mut projects = Vec::new();
        let entries = fs::read_dir(&self.projects_dir)?;

        for entry in entries {
            let entry = entry?;
            let path = entry.path();
            if path.is_file() && path.extension().map_or(false, |ext| ext == "json") {
                let content = fs::read_to_string(path)?;
                let project: ProjectConfig = serde_json::from_str(&content)?;
                projects.push(project);
            }
        }

        Ok(projects)
    }

    pub fn create_project(&self, request: CreateProjectRequest) -> Result<ProjectConfig, AppError> {
        let id = Uuid::new_v4().to_string();
        let now = Local::now();
        let config_path = format!("{}/{}.conf", self.projects_dir, id);
        let project_path = format!("{}/{}.json", self.projects_dir, id);

        let project = ProjectConfig {
            id,
            name: request.name,
            path: config_path.clone(),
            domain: request.domain,
            port: request.port,
            root: request.root,
            index: request.index,
            php: request.php,
            ssl: request.ssl,
            remark: request.remark,
            created_at: now,
            updated_at: now,
        };

        // 保存项目配置
        let content = serde_json::to_string_pretty(&project)?;
        fs::write(&project_path, content)?;

        // 生成 Nginx 配置
        self.generate_nginx_config(&project)?;

        Ok(project)
    }

    pub fn delete_project(&self, id: &str) -> Result<(), AppError> {
        let project_path = format!("{}/{}.json", self.projects_dir, id);
        let config_path = format!("{}/{}.conf", self.projects_dir, id);

        // 删除项目配置文件和 Nginx 配置文件
        if Path::new(&project_path).exists() {
            fs::remove_file(project_path)?;
        }
        if Path::new(&config_path).exists() {
            fs::remove_file(config_path)?;
        }

        Ok(())
    }

    pub fn get_templates(&self) -> Result<Vec<ProjectTemplate>, AppError> {
        let mut templates = Vec::new();
        let entries = fs::read_dir(&self.templates_dir)?;

        for entry in entries {
            let entry = entry?;
            let path = entry.path();
            if path.is_file() && path.extension().map_or(false, |ext| ext == "json") {
                let content = fs::read_to_string(path)?;
                let template: ProjectTemplate = serde_json::from_str(&content)?;
                templates.push(template);
            }
        }

        Ok(templates)
    }

    pub fn create_template(&self, request: CreateTemplateRequest) -> Result<ProjectTemplate, AppError> {
        let id = Uuid::new_v4().to_string();
        let now = Local::now();
        let path = format!("{}/{}.json", self.templates_dir, id);

        let template = ProjectTemplate {
            id,
            name: request.name,
            content: request.content,
            remark: request.remark,
            created_at: now,
            updated_at: now,
        };

        // 保存模板
        let content = serde_json::to_string_pretty(&template)?;
        fs::write(path, content)?;

        Ok(template)
    }

    pub fn delete_template(&self, id: &str) -> Result<(), AppError> {
        let path = format!("{}/{}.json", self.templates_dir, id);
        if Path::new(&path).exists() {
            fs::remove_file(path)?;
        }
        Ok(())
    }

    fn generate_nginx_config(&self, project: &ProjectConfig) -> Result<(), AppError> {
        let mut config = String::new();

        // 基本配置
        config.push_str(&format!("server {{\n"));
        config.push_str(&format!("    listen {};\n", project.port));
        config.push_str(&format!("    server_name {};\n", project.domain));
        config.push_str(&format!("    root {};\n", project.root));

        // 默认首页
        config.push_str("    index ");
        for (i, index) in project.index.iter().enumerate() {
            if i > 0 {
                config.push_str(" ");
            }
            config.push_str(index);
        }
        config.push_str(";\n\n");

        // PHP 支持
        if project.php {
            config.push_str("    location ~ \\.php$ {\n");
            config.push_str("        fastcgi_pass unix:/var/run/php/php-fpm.sock;\n");
            config.push_str("        fastcgi_index index.php;\n");
            config.push_str("        fastcgi_param SCRIPT_FILENAME $document_root$fastcgi_script_name;\n");
            config.push_str("        include fastcgi_params;\n");
            config.push_str("    }\n\n");
        }

        // SSL 支持
        if project.ssl {
            config.push_str("    ssl_certificate /etc/nginx/ssl/cert.pem;\n");
            config.push_str("    ssl_certificate_key /etc/nginx/ssl/key.pem;\n");
            config.push_str("    ssl_protocols TLSv1.2 TLSv1.3;\n");
            config.push_str("    ssl_ciphers HIGH:!aNULL:!MD5;\n\n");
        }

        config.push_str("}\n");

        // 保存配置文件
        fs::write(&project.path, config)?;

        Ok(())
    }
} 