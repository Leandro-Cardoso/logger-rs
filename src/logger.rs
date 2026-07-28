use crate::{
    builder::LoggerBuilder, config::LoggerConfig, entry::LogEntry, error::LoggerError,
    formatter::Formatter, levels::LogLevel, reader::Reader, storage::Storage,
};

use std::path::PathBuf;

/// Estrutura principal da biblioteca.
///
/// O Logger é responsável por coordenar:
///
/// - criação dos logs;
/// - formatação;
/// - armazenamento;
/// - leitura.
#[derive(Debug)]
pub struct Logger {
    storage: Storage,

    reader: Reader,

    config: LoggerConfig,
}

impl Logger {
    /// Cria um novo Logger utilizando uma configuração.
    pub fn new(config: LoggerConfig) -> Result<Self, LoggerError> {
        config.validate()?;

        let storage = Storage::new(config.clone());

        storage.initialize()?;

        let reader = Reader::new(config.clone());

        Ok(Self {
            storage,

            reader,

            config,
        })
    }

    /// Cria um Builder para configuração do Logger.
    ///
    /// Exemplo:
    ///
    /// ```
    /// use logger_rs::Logger;
    ///
    /// let logger =
    ///     Logger::builder()
    ///         .max_storage_mb(200)
    ///         .build();
    /// ```
    pub fn builder() -> LoggerBuilder {
        LoggerBuilder::new()
    }

    /// Método interno responsável pelo fluxo de criação do log.
    fn log(&self, level: LogLevel, message: impl Into<String>) -> Result<(), LoggerError> {
        /*
            Verifica se o nível do log
            deve ser registrado.
        */
        if level < self.config.minimum_level {
            return Ok(());
        }

        let entry = LogEntry::new(level, message);

        let file_message = Formatter::format(&entry);

        let console_message = Formatter::format_console(&entry);

        println!("{}", console_message);

        self.storage.write(&file_message)?;

        Ok(())
    }

    // ==========================
    // Métodos de Log
    // ==========================

    /// Registra uma mensagem DEBUG.
    pub fn debug(&self, message: impl Into<String>) -> Result<(), LoggerError> {
        self.log(LogLevel::Debug, message)
    }

    /// Registra uma mensagem INFO.
    pub fn info(&self, message: impl Into<String>) -> Result<(), LoggerError> {
        self.log(LogLevel::Info, message)
    }

    /// Registra uma mensagem de sucesso.
    pub fn success(&self, message: impl Into<String>) -> Result<(), LoggerError> {
        self.log(LogLevel::Success, message)
    }

    /// Registra uma mensagem WARNING.
    pub fn warning(&self, message: impl Into<String>) -> Result<(), LoggerError> {
        self.log(LogLevel::Warning, message)
    }

    /// Registra uma mensagem ERROR.
    pub fn error(&self, message: impl Into<String>) -> Result<(), LoggerError> {
        self.log(LogLevel::Error, message)
    }

    /// Registra uma mensagem de progresso.
    pub fn progress(&self, message: impl Into<String>) -> Result<(), LoggerError> {
        self.log(LogLevel::Progress, message)
    }

    // ==========================
    // Leitura dos Logs
    // ==========================

    /// Lê o arquivo de log do dia atual.
    pub fn read_today(&self) -> Result<String, LoggerError> {
        self.reader.read_today()
    }

    /// Lê todos os arquivos de log.
    pub fn read_all(&self) -> Result<String, LoggerError> {
        self.reader.read_all()
    }

    /// Retorna as últimas linhas dos logs.
    pub fn tail(&self, amount: usize) -> Result<Vec<String>, LoggerError> {
        self.reader.tail(amount)
    }

    /// Lista os arquivos de log existentes.
    pub fn files(&self) -> Result<Vec<PathBuf>, LoggerError> {
        self.reader.list_files()
    }
}
