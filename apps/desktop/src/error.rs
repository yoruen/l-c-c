//! Error types for the core library

use thiserror::Error;

/// Core error type
#[derive(Error, Debug)]
pub enum CoreError {
    #[error("Plugin error: {0}")]
    Plugin(String),
    
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    
    #[error("Database error: {0}")]
    Database(String),
    
    #[error("Configuration error: {0}")]
    Config(String),
    
    #[error("Permission denied: {0}")]
    PermissionDenied(String),
    
    #[error("Command execution failed: {0}")]
    CommandFailed(String),
    
    #[error("Not found: {0}")]
    NotFound(String),
    
    #[error("Invalid argument: {0}")]
    InvalidArgument(String),
    
    #[error("Internal error: {0}")]
    Internal(String),
}

impl From<sqlx::Error> for CoreError {
    fn from(err: sqlx::Error) -> Self {
        CoreError::Database(err.to_string())
    }
}

pub type CoreResult<T> = Result<T, CoreError>;
