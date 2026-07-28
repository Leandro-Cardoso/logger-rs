use crate::{
    builder::LoggerBuilder, config::LoggerConfig, entry::LogEntry, error::LoggerError,
    formatter::Formatter, levels::LogLevel, reader::Reader, storage::Storage,
};

#[derive(Debug)]
pub struct Logger {
    storage: Storage,

    reader: Reader,
}

impl Logger {
    /// Cria um novo Logger.
    pub fn new(config: LoggerConfig) -> Result<Self, LoggerError> {
        config.validate()?;

        let storage = Storage::new(config.clone());

        storage.initialize()?;

        let reader = Reader::new(config);

        Ok(Self { storage, reader })
    }

    /// Construtor.
    pub fn builder() -> LoggerBuilder {
        LoggerBuilder::new()
    }

    /// Registra uma mensagem de log.
    fn log(&self, level: LogLevel, message: impl Into<String>) -> Result<(), LoggerError> {
        let entry = LogEntry::new(level, message);

        let file_message = Formatter::format(&entry);

        let console_message = Formatter::format_console(&entry);

        println!("{}", console_message);

        self.storage.write(&file_message)?;

        Ok(())
    }

    /// Registra mensagem de debug.
    pub fn debug(&self, message: impl Into<String>) -> Result<(), LoggerError> {
        self.log(LogLevel::Debug, message)
    }

    /// Registra mensagem informativa.
    pub fn info(&self, message: impl Into<String>) -> Result<(), LoggerError> {
        self.log(LogLevel::Info, message)
    }

    /// Registra mensagem de sucesso.
    pub fn success(&self, message: impl Into<String>) -> Result<(), LoggerError> {
        self.log(LogLevel::Success, message)
    }

    /// Registra alerta.
    pub fn warning(&self, message: impl Into<String>) -> Result<(), LoggerError> {
        self.log(LogLevel::Warning, message)
    }

    /// Registra erro.
    pub fn error(&self, message: impl Into<String>) -> Result<(), LoggerError> {
        self.log(LogLevel::Error, message)
    }

    /// Registra progresso.
    pub fn progress(&self, message: impl Into<String>) -> Result<(), LoggerError> {
        self.log(LogLevel::Progress, message)
    }

    // ============================
    // Leitura
    // ============================

    /// Lê os logs do dia atual.
    pub fn read_today(&self) -> Result<String, LoggerError> {
        self.reader.read_today()
    }

    /// Lê todos os logs.
    pub fn read_all(&self) -> Result<String, LoggerError> {
        self.reader.read_all()
    }

    /// Retorna as últimas linhas dos logs.
    pub fn tail(&self, amount: usize) -> Result<Vec<String>, LoggerError> {
        self.reader.tail(amount)
    }

    /// Lista os arquivos existentes.
    pub fn files(&self) -> Result<Vec<std::path::PathBuf>, LoggerError> {
        self.reader.list_files()
    }
}

#[test]
fn test_logger_full_flow() {
    let dir = tempfile::tempdir().unwrap();

    let config = LoggerConfig::new(dir.path(), 10);

    let logger = Logger::new(config).unwrap();

    logger.info("Teste completo").unwrap();

    let logs = logger.read_today().unwrap();

    assert!(logs.contains("Teste completo"));
}
