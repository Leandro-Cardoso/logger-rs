use colored::Color;
use std::cmp::Ordering;

/// Representa os níveis de log disponíveis.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LogLevel {
    /// Informações detalhadas para desenvolvimento.
    Debug,

    /// Informações gerais.
    Info,

    /// Operação concluída com sucesso.
    Success,

    /// Situação que merece atenção.
    Warning,

    /// Erro ocorrido.
    Error,

    /// Atualização de progresso.
    Progress,
}

impl LogLevel {
    /// Nome do nível.
    pub fn as_str(&self) -> &'static str {
        match self {
            LogLevel::Debug => "DEBUG",
            LogLevel::Info => "INFO",
            LogLevel::Success => "SUCCESS",
            LogLevel::Warning => "WARNING",
            LogLevel::Error => "ERROR",
            LogLevel::Progress => "PROGRESS",
        }
    }

    /// Prioridade utilizada para comparação.
    pub const fn priority(&self) -> u8 {
        match self {
            LogLevel::Debug => 0,
            LogLevel::Info => 1,
            LogLevel::Success => 2,
            LogLevel::Warning => 3,
            LogLevel::Error => 4,
            LogLevel::Progress => 5,
        }
    }

    /// Cor utilizada no console.
    pub const fn color(&self) -> Color {
        match self {
            LogLevel::Debug => Color::BrightBlack,
            LogLevel::Info => Color::Blue,
            LogLevel::Success => Color::Green,
            LogLevel::Warning => Color::Yellow,
            LogLevel::Error => Color::Red,
            LogLevel::Progress => Color::Cyan,
        }
    }

    /// Verifica se o nível atende ao nível mínimo configurado.
    pub fn enabled(self, minimum_level: LogLevel) -> bool {
        self >= minimum_level
    }
}

impl Ord for LogLevel {
    fn cmp(&self, other: &Self) -> Ordering {
        self.priority().cmp(&other.priority())
    }
}

impl PartialOrd for LogLevel {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_level_to_string() {
        assert_eq!(LogLevel::Debug.as_str(), "DEBUG");
        assert_eq!(LogLevel::Info.as_str(), "INFO");
        assert_eq!(LogLevel::Success.as_str(), "SUCCESS");
        assert_eq!(LogLevel::Warning.as_str(), "WARNING");
        assert_eq!(LogLevel::Error.as_str(), "ERROR");
        assert_eq!(LogLevel::Progress.as_str(), "PROGRESS");
    }

    #[test]
    fn test_priority() {
        assert!(LogLevel::Info > LogLevel::Debug);
        assert!(LogLevel::Success > LogLevel::Info);
        assert!(LogLevel::Warning > LogLevel::Success);
        assert!(LogLevel::Error > LogLevel::Warning);
        assert!(LogLevel::Progress > LogLevel::Error);
    }

    #[test]
    fn test_enabled() {
        let minimum = LogLevel::Info;

        assert!(!LogLevel::Debug.enabled(minimum));
        assert!(LogLevel::Info.enabled(minimum));
        assert!(LogLevel::Warning.enabled(minimum));
        assert!(LogLevel::Error.enabled(minimum));
    }

    #[test]
    fn test_copy_clone() {
        let level = LogLevel::Error;

        let cloned = level;
        let copied = cloned;

        assert_eq!(level, cloned);
        assert_eq!(cloned, copied);
    }

    #[test]
    fn test_ord() {
        assert!(LogLevel::Error > LogLevel::Info);
        assert!(LogLevel::Debug < LogLevel::Progress);
    }
}
