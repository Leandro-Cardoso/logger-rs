use std::{
    fs::{self, OpenOptions},
    io::Write,
    path::PathBuf,
};

use chrono::Local;

use crate::{config::LoggerConfig, error::LoggerError};

/// Responsável pelo armazenamento dos logs.
#[derive(Debug)]
pub struct Storage {
    config: LoggerConfig,
}

impl Storage {
    /// Cria um novo Storage.
    pub fn new(config: LoggerConfig) -> Self {
        Self { config }
    }

    /// Inicializa o armazenamento.
    pub fn initialize(&self) -> Result<(), LoggerError> {
        if !self.config.directory.exists() {
            fs::create_dir_all(&self.config.directory)
                .map_err(|e| LoggerError::DirectoryCreation(e.to_string()))?;
        }

        Ok(())
    }

    /// Escreve uma mensagem no arquivo do dia.
    pub fn write(&self, message: &str) -> Result<(), LoggerError> {
        self.initialize()?;

        let mut file = OpenOptions::new()
            .create(true)
            .append(true)
            .open(self.current_log_file())
            .map_err(|e| LoggerError::WriteFailed(e.to_string()))?;

        writeln!(file, "{message}").map_err(|e| LoggerError::WriteFailed(e.to_string()))?;

        Ok(())
    }

    /// Retorna o caminho do arquivo de log atual.
    fn current_log_file(&self) -> PathBuf {
        self.config
            .directory
            .join(format!("log_{}.txt", Local::now().format("%Y-%m-%d")))
    }

    /// Retorna o tamanho total ocupado pelos logs.
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

    /// Futuramente removerá os arquivos antigos.
    pub fn cleanup(&self) -> Result<(), LoggerError> {
        let limit = self.config.max_storage_mb * 1024 * 1024;

        if self.storage_size()? <= limit {
            return Ok(());
        }

        // Implementação será adicionada posteriormente.

        Ok(())
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    use crate::levels::LogLevel;
    use tempfile::tempdir;

    #[test]
    fn test_create_storage() {
        let dir = tempdir().unwrap();

        let config = LoggerConfig::new(dir.path(), 100, LogLevel::Debug);

        let storage = Storage::new(config);

        assert_eq!(storage.config.directory, dir.path());
    }

    #[test]
    fn test_initialize_directory() {
        let dir = tempdir().unwrap();

        let log_dir = dir.path().join("logs");

        let config = LoggerConfig::new(&log_dir, 100, LogLevel::Debug);

        let storage = Storage::new(config);

        storage.initialize().unwrap();

        assert!(log_dir.exists());
    }

    #[test]
    fn test_write_log() {
        let dir = tempdir().unwrap();

        let config = LoggerConfig::new(dir.path(), 100, LogLevel::Debug);

        let storage = Storage::new(config);

        storage.write("Primeira linha").unwrap();

        let files: Vec<_> = fs::read_dir(dir.path()).unwrap().collect();

        assert_eq!(files.len(), 1);
    }

    #[test]
    fn test_storage_size() {
        let dir = tempdir().unwrap();

        let config = LoggerConfig::new(dir.path(), 100, LogLevel::Debug);

        let storage = Storage::new(config);

        storage.write("Linha de teste").unwrap();

        let size = storage.storage_size().unwrap();

        assert!(size > 0);
    }

    #[test]
    fn test_cleanup_without_limit() {
        let dir = tempdir().unwrap();

        let config = LoggerConfig::new(dir.path(), 100, LogLevel::Debug);

        let storage = Storage::new(config);

        storage.cleanup().unwrap();
    }
}
