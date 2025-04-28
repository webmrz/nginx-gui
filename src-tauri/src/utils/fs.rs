use std::path::Path;
use std::fs;
use std::io::{self, Read};

pub fn ensure_dir_exists(path: &Path) -> io::Result<()> {
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)?;
    }
    Ok(())
}

pub fn read_file_content(path: &Path) -> io::Result<String> {
    let mut file = fs::File::open(path)?;
    let mut content = String::new();
    file.read_to_string(&mut content)?;
    Ok(content)
}

pub fn write_file_content(path: &Path, content: &str) -> io::Result<()> {
    ensure_dir_exists(path)?;
    fs::write(path, content)?;
    Ok(())
}

pub fn delete_file(path: &Path) -> io::Result<()> {
    if path.exists() {
        fs::remove_file(path)?;
    }
    Ok(())
}

pub fn list_files_in_dir(dir: &Path) -> io::Result<Vec<String>> {
    let mut files = Vec::new();
    for entry in fs::read_dir(dir)? {
        let entry = entry?;
        let path = entry.path();
        if path.is_file() {
            if let Some(name) = path.file_name().and_then(|n| n.to_str()) {
                files.push(name.to_string());
            }
        }
    }
    Ok(files)
} 