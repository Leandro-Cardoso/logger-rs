use std::io;

use thiserror::Error;

#[derive(Debug, Error)]
pub enum LoggerError {
    #[error("Failed to create log directory: {0}")]
    DirectoryCreationFailed(String),

    #[error("Failed to create log file: {0}")]
    FileCreationFailed(String),

    #[error("Failed to write log: {0}")]
    WriteFailed(String),

    #[error("Failed to read log: {0}")]
    ReadFailed(String),

    #[error("Invalid configuration: {0}")]
    InvalidConfiguration(String),

    #[error("IO error: {0}")]
    Io(#[from] io::Error),
}

impl LoggerError {
    pub fn invalid_configuration(message: impl Into<String>) -> Self {
        LoggerError::InvalidConfiguration(message.into())
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_invalid_configuration_error() {
        let error = LoggerError::invalid_configuration("Invalid storage size");

        assert_eq!(
            error.to_string(),
            "Invalid configuration: Invalid storage size"
        );
    }

    #[test]
    fn test_directory_creation_error() {
        let error = LoggerError::DirectoryCreationFailed("Permission denied".into());

        assert_eq!(
            error.to_string(),
            "Failed to create log directory: Permission denied"
        );
    }

    #[test]
    fn test_io_error_conversion() {
        let io_error = io::Error::new(io::ErrorKind::NotFound, "File missing");

        let logger_error: LoggerError = io_error.into();

        assert!(logger_error.to_string().contains("IO error"));
    }
}
