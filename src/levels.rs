use colored::Color;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum LogLevel {
    Debug,

    Info,

    Success,

    Warning,

    Error,

    Progress,
}

impl LogLevel {
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

    pub fn color(&self) -> Color {
        match self {
            LogLevel::Debug => Color::BrightBlack,

            LogLevel::Info => Color::BrightBlue,

            LogLevel::Success => Color::BrightGreen,

            LogLevel::Warning => Color::BrightYellow,

            LogLevel::Error => Color::BrightRed,

            LogLevel::Progress => Color::BrightCyan,
        }
    }

    pub fn priority(&self) -> u8 {
        match self {
            LogLevel::Debug => 0,

            LogLevel::Info => 1,

            LogLevel::Success => 2,

            LogLevel::Progress => 3,

            LogLevel::Warning => 4,

            LogLevel::Error => 5,
        }
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_level_to_string() {
        assert_eq!(LogLevel::Info.as_str(), "INFO");

        assert_eq!(LogLevel::Error.as_str(), "ERROR");
    }

    #[test]
    fn test_level_priority() {
        assert!(LogLevel::Error.priority() > LogLevel::Info.priority());

        assert!(LogLevel::Warning.priority() > LogLevel::Success.priority());
    }

    #[test]
    fn test_level_comparison() {
        assert!(LogLevel::Error > LogLevel::Debug);
    }
}
