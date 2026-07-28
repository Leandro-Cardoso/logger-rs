use std::io;

/// Erros possíveis da biblioteca Logger.
#[derive(Debug, thiserror::Error)]
pub enum LoggerError {
    /// Configuração inválida.
    #[error("Invalid configuration: {0}")]
    InvalidConfiguration(String),

    /// Erro de criação de diretório.
    #[error("Directory creation failed: {0}")]
    DirectoryCreation(String),

    /// Erro ao escrever arquivo de log.
    #[error("Write failed: {0}")]
    WriteFailed(String),

    /// Erro ao ler arquivo de log.
    #[error("Read failed: {0}")]
    ReadFailed(String),

    /// Erro genérico de IO.
    #[error("IO error: {0}")]
    Io(#[from] io::Error),

    /// Erro inesperado interno.
    #[error("Internal error: {0}")]
    Internal(String),
}

impl LoggerError {
    /// Cria erro de configuração.
    pub fn invalid_configuration(message: impl Into<String>) -> Self {
        Self::InvalidConfiguration(message.into())
    }

    /// Cria erro de escrita.
    pub fn write_failed(message: impl Into<String>) -> Self {
        Self::WriteFailed(message.into())
    }

    /// Cria erro de leitura.
    pub fn read_failed(message: impl Into<String>) -> Self {
        Self::ReadFailed(message.into())
    }

    /// Cria erro interno.
    pub fn internal(message: impl Into<String>) -> Self {
        Self::Internal(message.into())
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_invalid_configuration_error() {
        let error = LoggerError::InvalidConfiguration("Invalid path".to_string());

        assert_eq!(error.to_string(), "Invalid configuration: Invalid path");
    }

    #[test]
    fn test_write_error() {
        let error = LoggerError::write_failed("Cannot write file");

        assert_eq!(error.to_string(), "Write failed: Cannot write file");
    }

    #[test]
    fn test_read_error() {
        let error = LoggerError::read_failed("Cannot read file");

        assert_eq!(error.to_string(), "Read failed: Cannot read file");
    }

    #[test]
    fn test_internal_error() {
        let error = LoggerError::internal("Unexpected error");

        assert_eq!(error.to_string(), "Internal error: Unexpected error");
    }

    #[test]
    fn test_io_error_conversion() {
        let io_error = io::Error::new(io::ErrorKind::NotFound, "File not found");

        let logger_error: LoggerError = io_error.into();

        assert!(logger_error.to_string().contains("IO error"));
    }

    #[test]
    fn test_directory_creation_error() {
        let error = LoggerError::DirectoryCreation("Permission denied".to_string());

        assert_eq!(
            error.to_string(),
            "Directory creation failed: Permission denied"
        );
    }
}
