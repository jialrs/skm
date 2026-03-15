use anyhow::{Result, Context};
use crate::config::Config;
use std::process::Command;

pub fn run(repo_url: String) -> Result<()> {
    let mut config = Config::load()?;
    
    // Update remote url if provided
    config.remote_url = Some(repo_url.clone());
    config.save()?;

    if config.vault_path.exists() {
        if config.vault_path.join(".git").exists() {
            println!("Vault already exists and is a valid Git repository at {:?}", config.vault_path);
            return Ok(());
        } else {
            anyhow::bail!("Vault path exists at {:?} but is not a Git repository. Please remove it and try again.", config.vault_path);
        }
    }

    println!("Cloning {} into {:?}", repo_url, config.vault_path);

    let status = Command::new("git")
        .arg("clone")
        .arg(&repo_url)
        .arg(&config.vault_path)
        .status()
        .context("Failed to execute git clone")?;

    if !status.success() {
        anyhow::bail!("Git clone failed");
    }

    println!("Vault initialized successfully.");
    Ok(())
}

#[cfg(test)]
mod tests {
    // Unit testing git clone is hard without mocking or network.
    // We can test the config update part if we separate logic.
}
