use std::{
    fs::{self, OpenOptions},
    io::Write,
    path::{PathBuf},
};

use chrono::Local;

use crate::{
    config::LoggerConfig,
    error::LoggerError,
};

pub struct Storage {
    config: LoggerConfig,
}

impl Storage {
    pub fn new(config: LoggerConfig) -> Self {
        Self { config }
    }

    /// Inicializa o armazenamento
    pub fn initialize(&self) -> Result<(), LoggerError> {
        fs::create_dir_all(&self.config.directory)
            .map_err(|e| LoggerError::DirectoryCreationFailed(e.to_string()))
    }

    /// Retorna o caminho do arquivo de log do dia.
    pub fn current_log_file(&self) -> PathBuf {
        self.config.directory.join(Self::daily_file_name())
    }

    /// Escreve uma linha no arquivo diário.
    pub fn write(&self, line: &str) -> Result<(), LoggerError> {
        self.initialize()?;

        let path = self.current_log_file();

        let mut file = OpenOptions::new()
            .create(true)
            .append(true)
            .open(&path)
            .map_err(|e| LoggerError::FileCreationFailed(e.to_string()))?;

        writeln!(file, "{line}")
            .map_err(|e| LoggerError::WriteFailed(e.to_string()))?;

        self.cleanup()?;

        Ok(())
    }

    /// Retorna o tamanho total ocupado pelos logs.
    pub fn storage_size(&self) -> Result<u64, LoggerError> {
        let mut total = 0;

        if !self.config.directory.exists() {
            return Ok(0);
        }

        for entry in fs::read_dir(&self.config.directory)? {
            let entry = entry?;
            let metadata = entry.metadata()?;

            if metadata.is_file() {
                total += metadata.len();
            }
        }

        Ok(total)
    }

    fn cleanup(&self) -> Result<(), LoggerError> {
        let limit = self.config.max_storage_mb * 1024 * 1024;

        while self.storage_size()? > limit {
            let Some(file) = self.oldest_file()? else {
                break;
            };

            fs::remove_file(file)?;
        }

        Ok(())
    }

    fn oldest_file(&self) -> Result<Option<PathBuf>, LoggerError> {
        if !self.config.directory.exists() {
            return Ok(None);
        }

        let mut files = Vec::new();

        for entry in fs::read_dir(&self.config.directory)? {
            let entry = entry?;
            let metadata = entry.metadata()?;

            if metadata.is_file() {
                files.push((
                    metadata.modified()?,
                    entry.path(),
                ));
            }
        }

        files.sort_by_key(|(date, _)| *date);

        Ok(files.into_iter().next().map(|(_, path)| path))
    }

    fn daily_file_name() -> String {
        format!(
            "log_{}.txt",
            Local::now().format("%Y-%m-%d")
        )
    }
}
