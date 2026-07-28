use chrono::{DateTime, Local};

use crate::levels::LogLevel;

/// Representa uma entrada de log.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LogEntry {
    /// Data e hora em que o log foi criado.
    pub timestamp: DateTime<Local>,

    /// Nível do log.
    pub level: LogLevel,

    /// Mensagem do log.
    pub message: String,
}

impl LogEntry {
    /// Cria uma nova entrada de log.
    pub fn new(level: LogLevel, message: impl Into<String>) -> Self {
        Self {
            timestamp: Local::now(),
            level,
            message: message.into(),
        }
    }

    /// Cria uma entrada de log com timestamp personalizado.
    ///
    /// Este método é útil para testes.
    pub fn with_timestamp(
        timestamp: DateTime<Local>,
        level: LogLevel,
        message: impl Into<String>,
    ) -> Self {
        Self {
            timestamp,
            level,
            message: message.into(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_entry() {
        let entry = LogEntry::new(LogLevel::Info, "Teste");

        assert_eq!(entry.level, LogLevel::Info);
        assert_eq!(entry.message, "Teste");
    }

    #[test]
    fn test_clone_entry() {
        let entry = LogEntry::new(LogLevel::Warning, "Mensagem");

        let clone = entry.clone();

        assert_eq!(entry, clone);
    }

    #[test]
    fn test_message_from_string() {
        let message = String::from("Olá");

        let entry = LogEntry::new(LogLevel::Debug, message);

        assert_eq!(entry.message, "Olá");
    }

    #[test]
    fn test_message_from_str() {
        let entry = LogEntry::new(LogLevel::Error, "Erro");

        assert_eq!(entry.message, "Erro");
    }
}
