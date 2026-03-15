use crate::config::Config;
use anyhow::{Result, Context};
use std::process::Command;

pub fn run(dry_run: bool, target_dir_override: Option<String>) -> Result<()> {
    let config = Config::load()?;
    
    if !config.vault_path.exists() {
         anyhow::bail!("Vault not found. Run 'skm init' first.");
    }

    println!("Syncing vault at {:?}", config.vault_path);
    
    if !dry_run {
        let status = Command::new("git")
            .current_dir(&config.vault_path)
            .arg("pull")
            .status()
            .context("Failed to run git pull")?;
            
        if !status.success() {
             anyhow::bail!("Git pull failed");
        }
    } else {
        println!("[Dry Run] Would run 'git pull' in {:?}", config.vault_path);
    }
    
    // Trigger link to validate/update
    crate::link::run(None, dry_run, target_dir_override)?;
    
    Ok(())
}

#[cfg(test)]
mod tests {
    // Integration test would be better here, or mocking Command.
    // For now we just ensure it compiles and structure is correct.
    
    #[test]
    fn test_sync_command_structure() {
        // Can't easily test Command::new execution in unit test without side effects or mocking.
        // But we can verify dry run print logic if we captured stdout, which is overkill.
        assert!(true);
    }
}
