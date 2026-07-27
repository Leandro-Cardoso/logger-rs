use chrono::Local;
use colored::Colorize;

use crate::levels::LogLevel;

pub struct Formatter;

impl Formatter {

    /// Formata uma mensagem para armazenamento em arquivo
    pub fn format(
        level: LogLevel,
        message: &str,
    ) -> String {

        let timestamp =
            Self::timestamp();

        format!(
            "[{}] [{}] {}",
            timestamp,
            level.as_str(),
            message
        )

    }

    /// Formata uma mensagem para exibição no console
    pub fn format_console(
        level: LogLevel,
        message: &str,
    ) -> String {

        let timestamp =
            Self::timestamp();

        let level_text =
            level
                .as_str()
                .color(level.color());

        format!(
            "[{}] [{}] {}",
            timestamp,
            level_text,
            message
        )

    }

    /// Retorna data e hora atual
    fn timestamp() -> String {

        Local::now()
            .format("%Y-%m-%d %H:%M:%S")
            .to_string()

    }

}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_format_log_message() {

        let result =
            Formatter::format(
                LogLevel::Info,
                "Sistema iniciado"
            );

        assert!(
            result.contains("[INFO]")
        );

        assert!(
            result.contains("Sistema iniciado")
        );

    }

    #[test]
    fn test_format_error_message() {

        let result =
            Formatter::format(
                LogLevel::Error,
                "Falha no banco"
            );

        assert!(
            result.contains("[ERROR]")
        );

        assert!(
            result.contains("Falha no banco")
        );

    }

    #[test]
    fn test_timestamp_format() {

        let result =
            Formatter::format(
                LogLevel::Debug,
                "Teste"
            );

        let parts:
            Vec<&str> =
            result.split(']').collect();

        assert!(
            parts[0].starts_with("[20")
        );

    }

}
