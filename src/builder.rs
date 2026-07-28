use std::path::PathBuf;

use crate::{config::LoggerConfig, error::LoggerError, levels::LogLevel, logger::Logger};

/// Builder responsável por criar um Logger.
///
/// Permite configurar a biblioteca de forma fluente.
///
/// Exemplo:
///
/// ```
/// use logger_rs::{
///     Logger,
///     LogLevel,
/// };
///
/// let logger = Logger::builder()
///     .directory("./logs")
///     .max_storage_mb(500)
///     .minimum_level(LogLevel::Info)
///     .build();
/// ```
#[derive(Debug, Clone)]
pub struct LoggerBuilder {
    /// Diretório dos arquivos de log.
    directory: PathBuf,

    /// Limite máximo de armazenamento.
    max_storage_mb: u64,

    /// Nível mínimo de log.
    minimum_level: LogLevel,
}

impl LoggerBuilder {
    /// Cria um Builder com configurações padrão.
    pub fn new() -> Self {
        Self {
            directory: PathBuf::from("./logs"),

            max_storage_mb: 100,

            minimum_level: LogLevel::Debug,
        }
    }

    /// Define o diretório onde os logs serão salvos.
    pub fn directory(mut self, path: impl Into<PathBuf>) -> Self {
        self.directory = path.into();

        self
    }

    /// Define o limite máximo de armazenamento em MB.
    pub fn max_storage_mb(mut self, size: u64) -> Self {
        self.max_storage_mb = size;

        self
    }

    /// Define o menor nível de log permitido.
    ///
    /// Logs abaixo desse nível serão ignorados.
    pub fn minimum_level(mut self, level: LogLevel) -> Self {
        self.minimum_level = level;

        self
    }

    /// Cria uma instância do Logger.
    pub fn build(self) -> Result<Logger, LoggerError> {
        let config = LoggerConfig::new(self.directory, self.max_storage_mb, self.minimum_level);

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

        assert_eq!(builder.directory, PathBuf::from("./logs"));

        assert_eq!(builder.max_storage_mb, 100);

        assert_eq!(builder.minimum_level, LogLevel::Debug);
    }

    #[test]
    fn test_custom_directory() {
        let builder = LoggerBuilder::new().directory("./custom_logs");

        assert_eq!(builder.directory, PathBuf::from("./custom_logs"));
    }

    #[test]
    fn test_custom_storage_limit() {
        let builder = LoggerBuilder::new().max_storage_mb(500);

        assert_eq!(builder.max_storage_mb, 500);
    }

    #[test]
    fn test_custom_minimum_level() {
        let builder = LoggerBuilder::new().minimum_level(LogLevel::Warning);

        assert_eq!(builder.minimum_level, LogLevel::Warning);
    }

    #[test]
    fn test_build_logger() {
        let result = LoggerBuilder::new()
            .directory("./test_logs")
            .max_storage_mb(50)
            .minimum_level(LogLevel::Info)
            .build();

        assert!(result.is_ok());
    }
}
