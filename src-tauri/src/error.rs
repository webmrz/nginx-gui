use std::fmt;
use std::io;
use serde_json;
use zip;

#[derive(Debug)]
pub struct AppError {
    message: String,
}

impl AppError {
    pub fn new(message: &str) -> Self {
        Self {
            message: message.to_string(),
        }
    }
}

impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl std::error::Error for AppError {}

impl From<io::Error> for AppError {
    fn from(error: io::Error) -> Self {
        Self::new(&error.to_string())
    }
}

impl From<serde_json::Error> for AppError {
    fn from(error: serde_json::Error) -> Self {
        Self::new(&error.to_string())
    }
}

impl From<zip::result::ZipError> for AppError {
    fn from(error: zip::result::ZipError) -> Self {
        Self::new(&error.to_string())
    }
} 