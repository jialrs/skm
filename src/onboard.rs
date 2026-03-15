use crate::config::Config;
use anyhow::{Result, Context};
use dialoguer::{theme::ColorfulTheme, Input};
use std::path::PathBuf;

pub fn run() -> Result<()> {
    // 1. Load defaults
    let default_config = Config::default_config()?;
    
    // 2. Interactive Prompts
    let repo_url: String = Input::with_theme(&ColorfulTheme::default())
        .with_prompt("Git Repository URL")
        .interact_text()
        .context("Failed to read Git URL")?;

    let vault_path_str: String = Input::with_theme(&ColorfulTheme::default())
        .with_prompt("Vault Path (Local Git Clone)")
        .default(default_config.vault_path.to_string_lossy().to_string())
        .interact_text()
        .context("Failed to read Vault Path")?;

    let target_path_str: String = Input::with_theme(&ColorfulTheme::default())
        .with_prompt("Target Path (Claude Skills)")
        .default(default_config.target_path.to_string_lossy().to_string())
        .interact_text()
        .context("Failed to read Target Path")?;
        
    // 3. Save Configuration
    let config = Config {
        vault_path: PathBuf::from(expand_tilde(&vault_path_str)),
        target_path: PathBuf::from(expand_tilde(&target_path_str)),
        remote_url: Some(repo_url.clone()),
    };
    
    config.save()?;
    
    // 4. Trigger Init
    crate::init::run(repo_url)?;
    
    Ok(())
}

fn expand_tilde(path: &str) -> String {
    if path.starts_with("~") {
        if let Some(home) = home::home_dir() {
             return path.replacen("~", &home.to_string_lossy(), 1);
        }
    }
    path.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_expand_tilde() {
        let expanded = expand_tilde("~/test");
        assert!(expanded.contains("test"));
        assert!(!expanded.starts_with("~"));
        
        let no_tilde = "/usr/bin";
        assert_eq!(expand_tilde(no_tilde), "/usr/bin");
    }
}
