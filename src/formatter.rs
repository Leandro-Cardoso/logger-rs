use colored::Colorize;

use crate::entry::LogEntry;

/// Responsável por formatar uma entrada de log.
pub struct Formatter;

impl Formatter {
    /// Formata uma entrada para ser gravada em arquivo.
    pub fn format(entry: &LogEntry) -> String {
        format!(
            "[{}] [{}] {}",
            Self::format_timestamp(entry),
            entry.level.as_str(),
            entry.message
        )
    }

    /// Formata uma entrada para ser exibida no console.
    pub fn format_console(entry: &LogEntry) -> String {
        let level = entry
            .level
            .as_str()
            .color(entry.level.color());

        format!(
            "[{}] [{}] {}",
            Self::format_timestamp(entry),
            level,
            entry.message
        )
    }

    /// Formata apenas o timestamp.
    fn format_timestamp(entry: &LogEntry) -> String {
        entry
            .timestamp
            .format("%Y-%m-%d %H:%M:%S")
            .to_string()
    }
}

#[cfg(test)]
mod tests {
    use chrono::{Local, TimeZone};

    use super::*;
    use crate::{
        entry::LogEntry,
        levels::LogLevel,
    };

    fn create_entry() -> LogEntry {
        LogEntry::with_timestamp(
            Local
                .with_ymd_and_hms(
                    2026,
                    7,
                    27,
                    15,
                    30,
                    45,
                )
                .unwrap(),
            LogLevel::Info,
            "Sistema iniciado",
        )
    }

    #[test]
    fn test_format() {
        let entry = create_entry();

        let text = Formatter::format(&entry);

        assert_eq!(
            text,
            "[2026-07-27 15:30:45] [INFO] Sistema iniciado"
        );
    }

    #[test]
    fn test_console_format_contains_message() {
        let entry = create_entry();

        let text = Formatter::format_console(&entry);

        assert!(text.contains("Sistema iniciado"));
    }

    #[test]
    fn test_console_format_contains_level() {
        let entry = create_entry();

        let text = Formatter::format_console(&entry);

        assert!(text.contains("INFO"));
    }

    #[test]
    fn test_timestamp() {
        let entry = create_entry();

        assert_eq!(
            Formatter::format_timestamp(&entry),
            "2026-07-27 15:30:45"
        );
    }
}
