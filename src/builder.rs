use std::path::PathBuf;

use crate::{config::LoggerConfig, error::LoggerError, logger::Logger};

#[derive(Debug, Clone)]
pub struct LoggerBuilder {
    directory: PathBuf,

    max_storage_mb: u64,
}

impl LoggerBuilder {
    pub fn new() -> Self {
        Self {
            directory: PathBuf::from("./logs"),

            max_storage_mb: 100,
        }
    }

    pub fn directory(mut self, path: impl Into<PathBuf>) -> Self {
        self.directory = path.into();

        self
    }

    pub fn max_storage_mb(mut self, size: u64) -> Self {
        self.max_storage_mb = size;

        self
    }

    pub fn build(self) -> Result<Logger, LoggerError> {
        let config = LoggerConfig::new(self.directory, self.max_storage_mb);

        Logger::new(config)
    }
}

impl Default for LoggerBuilder {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_default_builder() {
        let builder = LoggerBuilder::new();

        assert_eq!(builder.max_storage_mb, 100);
    }

    #[test]
    fn test_custom_directory() {
        let builder = LoggerBuilder::new().directory("./custom_logs");

        assert_eq!(builder.directory, PathBuf::from("./custom_logs"));
    }

    #[test]
    fn test_build_logger() {
        let result = LoggerBuilder::new()
            .directory("./test_logs")
            .max_storage_mb(50)
            .build();

        assert!(result.is_ok());
    }
}
