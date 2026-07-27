use std::path::PathBuf;

#[derive(Debug, Clone)]
pub struct LoggerConfig {

    /// Diretório onde os arquivos de log serão armazenados
    pub directory: PathBuf,

    /// Limite máximo de armazenamento em MB
    pub max_storage_mb: u64,

}

impl LoggerConfig {

    /// Cria uma nova configuração personalizada
    pub fn new(
        directory: impl Into<PathBuf>,
        max_storage_mb: u64,
    ) -> Self {

        Self {
            directory: directory.into(),
            max_storage_mb,
        }

    }

    /// Cria uma configuração padrão
    pub fn default() -> Self {

        Self {
            directory: PathBuf::from("./logs"),
            max_storage_mb: 100,
        }

    }

    /// Valida se a configuração está correta
    pub fn validate(
        &self
    ) -> Result<(), String> {

        if self.max_storage_mb == 0 {
            return Err(
                "Storage limit must be greater than zero"
                .to_string()
            );
        }

        if self.directory.as_os_str().is_empty() {
            return Err(
                "Log directory cannot be empty"
                .to_string()
            );
        }

        Ok(())

    }

}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_create_config() {

        let config =
            LoggerConfig::new(
                "./test_logs",
                50
            );

        assert_eq!(
            config.max_storage_mb,
            50
        );

        assert_eq!(
            config.directory,
            PathBuf::from("./test_logs")
        );

    }

    #[test]
    fn test_default_config() {

        let config =
            LoggerConfig::default();

        assert_eq!(
            config.directory,
            PathBuf::from("./logs")
        );

        assert_eq!(
            config.max_storage_mb,
            100
        );

    }

    #[test]
    fn test_invalid_storage_limit() {

        let config =
            LoggerConfig::new(
                "./logs",
                0
            );

        assert!(
            config.validate()
            .is_err()
        );

    }

    #[test]
    fn test_valid_config() {

        let config =
            LoggerConfig::new(
                "./logs",
                100
            );

        assert!(
            config.validate()
            .is_ok()
        );

    }
    
}
