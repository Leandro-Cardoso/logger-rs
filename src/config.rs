use std::path::PathBuf;

use crate::{error::LoggerError, levels::LogLevel};

/// Configurações utilizadas pelo Logger.
#[derive(Debug, Clone)]
pub struct LoggerConfig {
    /// Diretório onde os arquivos de log serão armazenados.
    pub directory: PathBuf,

    /// Limite máximo de armazenamento em MB.
    ///
    /// Quando o limite for atingido,
    /// arquivos antigos serão removidos.
    pub max_storage_mb: u64,

    /// Menor nível de log que será registrado.
    ///
    /// Logs abaixo desse nível serão ignorados.
    pub minimum_level: LogLevel,
}

impl LoggerConfig {
    /// Cria uma nova configuração personalizada.
    pub fn new(
        directory: impl Into<PathBuf>,
        max_storage_mb: u64,
        minimum_level: LogLevel,
    ) -> Self {
        Self {
            directory: directory.into(),

            max_storage_mb,

            minimum_level,
        }
    }

    /// Valida se a configuração está correta.
    pub fn validate(&self) -> Result<(), LoggerError> {
        if self.max_storage_mb == 0 {
            return Err(LoggerError::InvalidConfiguration(
                "Storage limit must be greater than zero".to_string(),
            ));
        }

        if self.directory.as_os_str().is_empty() {
            return Err(LoggerError::InvalidConfiguration(
                "Log directory cannot be empty".to_string(),
            ));
        }

        Ok(())
    }
}

impl Default for LoggerConfig {
    /// Cria uma configuração padrão.
    fn default() -> Self {
        Self {
            directory: PathBuf::from("./logs"),

            max_storage_mb: 100,

            minimum_level: LogLevel::Debug,
        }
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_create_config() {
        let config = LoggerConfig::new("./test_logs", 50, LogLevel::Info);

        assert_eq!(config.max_storage_mb, 50);

        assert_eq!(config.directory, PathBuf::from("./test_logs"));

        assert_eq!(config.minimum_level, LogLevel::Info);
    }

    #[test]
    fn test_default_config() {
        let config = LoggerConfig::default();

        assert_eq!(config.directory, PathBuf::from("./logs"));

        assert_eq!(config.max_storage_mb, 100);

        assert_eq!(config.minimum_level, LogLevel::Debug);
    }

    #[test]
    fn test_invalid_storage_limit() {
        let config = LoggerConfig::new("./logs", 0, LogLevel::Info);

        match config.validate() {
            Err(LoggerError::InvalidConfiguration(message)) => {
                assert!(message.contains("Storage"));
            }

            _ => panic!("Expected InvalidConfiguration error"),
        }
    }

    #[test]
    fn test_invalid_directory() {
        let config = LoggerConfig {
            directory: PathBuf::new(),

            max_storage_mb: 100,

            minimum_level: LogLevel::Info,
        };

        assert!(matches!(
            config.validate(),
            Err(LoggerError::InvalidConfiguration(_))
        ));
    }

    #[test]
    fn test_valid_config() {
        let config = LoggerConfig::new("./logs", 100, LogLevel::Info);

        assert!(config.validate().is_ok());
    }

    #[test]
    fn test_clone_config() {
        let config = LoggerConfig::new("./logs", 100, LogLevel::Error);

        let cloned = config.clone();

        assert_eq!(cloned.max_storage_mb, 100);

        assert_eq!(cloned.minimum_level, LogLevel::Error);
    }
}
