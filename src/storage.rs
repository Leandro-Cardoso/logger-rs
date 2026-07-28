use std::{
    fs::{self, OpenOptions},
    io::Write,
    path::PathBuf,
};

use chrono::Local;

use crate::{
    config::LoggerConfig,
    error::LoggerError,
};

/// Responsável pela persistência dos logs.
#[derive(Debug)]
pub struct Storage {
    config: LoggerConfig,
}

impl Storage {
    /// Cria um novo Storage.
    pub fn new(config: LoggerConfig) -> Self {
        Self { config }
    }

    /// Garante que o diretório de logs exista.
    pub fn initialize(&self) -> Result<(), LoggerError> {
        fs::create_dir_all(&self.config.directory)
            .map_err(|e| LoggerError::DirectoryCreationFailed(e.to_string()))
    }

    /// Escreve uma linha no arquivo de log do dia.
    pub fn write(
        &self,
        line: &str,
    ) -> Result<(), LoggerError> {
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

    /// Retorna o caminho do arquivo atual.
    pub fn current_log_file(&self) -> PathBuf {
        self.config
            .directory
            .join(Self::current_file_name())
    }

    /// Calcula o tamanho total dos logs.
    pub fn storage_size(&self) -> Result<u64, LoggerError> {
        if !self.config.directory.exists() {
            return Ok(0);
        }

        let mut total = 0;

        for entry in fs::read_dir(&self.config.directory)? {
            let entry = entry?;

            if entry.metadata()?.is_file() {
                total += entry.metadata()?.len();
            }
        }

        Ok(total)
    }

    fn cleanup(&self) -> Result<(), LoggerError> {
        let limit = self.config.max_storage_mb * 1024 * 1024;

        while self.storage_size()? > limit {
            let Some(oldest) = self.oldest_file()? else {
                break;
            };

            fs::remove_file(oldest)?;
        }

        Ok(())
    }

    fn oldest_file(
        &self,
    ) -> Result<Option<PathBuf>, LoggerError> {
        if !self.config.directory.exists() {
            return Ok(None);
        }

        let mut files = Vec::new();

        for entry in fs::read_dir(&self.config.directory)? {
            let entry = entry?;

            if entry.metadata()?.is_file() {
                files.push((
                    entry.metadata()?.modified()?,
                    entry.path(),
                ));
            }
        }

        files.sort_by_key(|(date, _)| *date);

        Ok(files
            .into_iter()
            .next()
            .map(|(_, path)| path))
    }

    fn current_file_name() -> String {
        format!(
            "log_{}.txt",
            Local::now().format("%Y-%m-%d")
        )
    }
}
