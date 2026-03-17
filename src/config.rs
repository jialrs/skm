use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use anyhow::{Result, Context};
use std::fs;
use home::home_dir;

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct Config {
    pub vault_path: PathBuf,
    pub target_path: PathBuf,
    pub remote_url: Option<String>,
}

impl Config {
    pub fn default_config() -> Result<Self> {
        let home = home_dir().context("Could not determine home directory")?;
        
        Ok(Self {
            vault_path: home.join("skm/vault"),
            target_path: home.join(".claude/skills"),
            remote_url: None,
        })
    }

    pub fn load() -> Result<Self> {
        let config_path = Self::config_file_path()?;
        if !config_path.exists() {
            return Self::default_config();
        }

        let content = fs::read_to_string(&config_path)?;
        let config: Config = toml::from_str(&content)
            .context("Failed to parse configuration file")?;
        
        Ok(config)
    }

    pub fn save(&self) -> Result<()> {
        let config_path = Self::config_file_path()?;
        if let Some(parent) = config_path.parent() {
            fs::create_dir_all(parent)?;
        }

        let content = toml::to_string_pretty(self)?;
        fs::write(config_path, content)?;
        Ok(())
    }

    fn config_file_path() -> Result<PathBuf> {
        let home = home_dir().context("Could not determine home directory")?;
        Ok(home.join("skm/config.toml"))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_config() {
        let config = Config::default_config();
        assert!(config.is_ok());
        let config = config.unwrap();
        assert!(config.vault_path.ends_with("skm/vault"));
        assert!(config.target_path.ends_with(".claude/skills"));
        assert!(config.remote_url.is_none());
    }

    // Note: Testing load/save involves file system access. 
    // Ideally we'd mock the config path, but for now we can rely on integration tests 
    // or careful unit testing if we can redirect the config path.
    // Since config_file_path uses home_dir(), we can't easily change it without mocking home.
    // However, we can test serialization at least.

    #[test]
    fn test_serialization() {
        let config = Config {
            vault_path: PathBuf::from("/tmp/vault"),
            target_path: PathBuf::from("/tmp/target"),
            remote_url: Some("https://github.com/test/repo".to_string()),
        };

        let toml_string = toml::to_string(&config).unwrap();
        let loaded_config: Config = toml::from_str(&toml_string).unwrap();

        assert_eq!(config, loaded_config);
    }
}
