use std::{
    fs,
    path::{Path, PathBuf},
};

use chrono::Local;

use crate::{
    config::LoggerConfig,
    error::LoggerError,
};

pub struct Reader {
    config: LoggerConfig,
}

impl Reader {
    pub fn new(config: LoggerConfig) -> Self {
        Self { config }
    }

    /// Lê o arquivo de log do dia.
    pub fn read_today(&self) -> Result<String, LoggerError> {
        let file = self.current_file();

        if !file.exists() {
            return Ok(String::new());
        }

        fs::read_to_string(file).map_err(|e| LoggerError::ReadFailed(e.to_string()))
    }

    /// Lê um arquivo específico.
    pub fn read_file(
        &self,
        file_name: impl AsRef<Path>,
    ) -> Result<String, LoggerError> {
        let path = self.config.directory.join(file_name);

        fs::read_to_string(path)
            .map_err(|e| LoggerError::ReadFailed(e.to_string()))
    }

    /// Lê todos os logs.
    pub fn read_all(&self) -> Result<String, LoggerError> {
        let mut result = String::new();

        let mut files = self.list_files()?;

        files.sort();

        for file in files {
            result.push_str(&fs::read_to_string(file)?);
        }

        Ok(result)
    }

    /// Retorna as últimas N linhas.
    pub fn read_last_lines(
        &self,
        amount: usize,
    ) -> Result<Vec<String>, LoggerError> {
        let logs = self.read_all()?;

        let lines = logs
            .lines()
            .rev()
            .take(amount)
            .map(String::from)
            .collect::<Vec<_>>();

        Ok(lines.into_iter().rev().collect())
    }

    /// Lista todos os arquivos de log.
    pub fn list_files(&self) -> Result<Vec<PathBuf>, LoggerError> {
        let mut files = Vec::new();

        if !self.config.directory.exists() {
            return Ok(files);
        }

        for entry in fs::read_dir(&self.config.directory)? {
            let entry = entry?;

            if entry.metadata()?.is_file() {
                files.push(entry.path());
            }
        }

        Ok(files)
    }

    fn current_file(&self) -> PathBuf {
        self.config.directory.join(format!(
            "log_{}.txt",
            Local::now().format("%Y-%m-%d")
        ))
    }
}

#[cfg(test)]
mod tests {

    use tempfile::tempdir;

    use crate::{
        config::LoggerConfig,
        storage::Storage,
    };

    use super::*;

    #[test]
    fn test_read_today() {

        let dir = tempdir().unwrap();

        let config = LoggerConfig::new(
            dir.path(),
            10,
        );

        let storage = Storage::new(config.clone());

        storage.write("Primeiro log").unwrap();

        let reader = Reader::new(config);

        let content = reader.read_today().unwrap();

        assert!(content.contains("Primeiro log"));
    }

    #[test]
    fn test_read_last_lines() {

        let dir = tempdir().unwrap();

        let config = LoggerConfig::new(
            dir.path(),
            10,
        );

        let storage = Storage::new(config.clone());

        storage.write("1").unwrap();
        storage.write("2").unwrap();
        storage.write("3").unwrap();

        let reader = Reader::new(config);

        let lines = reader.read_last_lines(2).unwrap();

        assert_eq!(lines.len(), 2);

        assert_eq!(lines[0], "2");
        assert_eq!(lines[1], "3");
    }

}
