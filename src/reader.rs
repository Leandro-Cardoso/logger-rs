use std::{
    fs,
    path::{Path, PathBuf},
};

use chrono::Local;

use crate::{
    config::LoggerConfig,
    error::LoggerError,
};

/// Responsável pela leitura dos arquivos de log.
#[derive(Debug)]
pub struct Reader {
    config: LoggerConfig,
}


impl Reader {

    /// Cria um novo Reader.
    pub fn new(
        config: LoggerConfig,
    ) -> Self {

        Self {
            config,
        }

    }

    /// Lê o arquivo de log do dia atual.
    pub fn read_today(
        &self,
    ) -> Result<String, LoggerError> {

        let file =
            self.current_log_file();

        if !file.exists() {

            return Ok(
                String::new()
            );

        }

        self.read_file_path(
            file
        )

    }

    /// Lê um arquivo específico pelo nome.
    pub fn read_file(
        &self,
        file_name: impl AsRef<Path>,
    ) -> Result<String, LoggerError> {

        let path =
            self.config
                .directory
                .join(file_name);

        self.read_file_path(
            path
        )

    }

    /// Lê todos os arquivos de log.
    pub fn read_all(
        &self,
    ) -> Result<String, LoggerError> {

        let mut content =
            String::new();

        let mut files =
            self.list_files()?;

        files.sort();

        for file in files {

            content.push_str(
                &self.read_file_path(file)?
            );

        }

        Ok(content)

    }

    /// Retorna as últimas N linhas dos logs.
    pub fn tail(
        &self,
        amount: usize,
    ) -> Result<Vec<String>, LoggerError> {

        let content =
            self.read_all()?;

        let lines =
            content
                .lines()
                .rev()
                .take(amount)
                .map(String::from)
                .collect::<Vec<_>>();

        Ok(
            lines
                .into_iter()
                .rev()
                .collect()
        )

    }

    /// Lista todos os arquivos de log existentes.
    pub fn list_files(
        &self,
    ) -> Result<Vec<PathBuf>, LoggerError> {

        let mut files =
            Vec::new();

        if !self.config.directory.exists() {

            return Ok(files);

        }

        for entry in fs::read_dir(
            &self.config.directory
        )? {

            let entry =
                entry?;

            if entry.metadata()?.is_file() {

                files.push(
                    entry.path()
                );

            }

        }

        Ok(files)

    }

    fn read_file_path(
        &self,
        path: PathBuf,
    ) -> Result<String, LoggerError> {

        fs::read_to_string(path)
            .map_err(
                |error| {
                    LoggerError::ReadFailed(
                        error.to_string()
                    )
                }
            )

    }

    fn current_log_file(
        &self,
    ) -> PathBuf {

        self.config
            .directory
            .join(
                format!(
                    "log_{}.txt",
                    Local::now()
                        .format("%Y-%m-%d")
                )
            )

    }

}
