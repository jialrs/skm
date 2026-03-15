use crate::config::Config;
use anyhow::{Result, Context};
use std::path::{Path, PathBuf};
use std::fs;

pub fn run(skill_name: Option<String>, dry_run: bool, target_dir_override: Option<String>) -> Result<()> {
    let mut config = Config::load()?;
    
    if let Some(td) = target_dir_override {
        config.target_path = PathBuf::from(td);
    }

    if !config.target_path.exists() {
        println!("Target directory does not exist: {:?}", config.target_path);
        return Ok(());
    }

    if let Some(name) = skill_name {
        let target_link = config.target_path.join(&name);
        unlink_skill(&target_link, dry_run)?;
    } else {
        // Unlink all skills that point to the vault
        for entry in fs::read_dir(&config.target_path)? {
            let entry = entry?;
            let path = entry.path();
            if let Ok(metadata) = fs::symlink_metadata(&path) {
                if metadata.file_type().is_symlink() {
                    if let Ok(target) = fs::read_link(&path) {
                        if target.starts_with(&config.vault_path) {
                            unlink_skill(&path, dry_run)?;
                        }
                    }
                }
            }
        }
    }

    Ok(())
}

fn unlink_skill(path: &Path, dry_run: bool) -> Result<()> {
    let name = path.file_name().unwrap().to_string_lossy();
    
    if let Ok(metadata) = fs::symlink_metadata(path) {
        if metadata.file_type().is_symlink() {
            if dry_run {
                println!("[Dry Run] Would unlink {}", name);
            } else {
                println!("Unlinking {}", name);
                fs::remove_file(path).context("Failed to remove symlink")?;
            }
        } else {
            println!("Skipping {}: Not a symlink", name);
        }
    } else {
        println!("Skipping {}: Does not exist", name);
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;
    use crate::utils::symlink_dir;

    #[test]
    fn test_unlink_skill() -> Result<()> {
        let vault = tempdir()?;
        let target_dir = tempdir()?;
        
        let skill_dir = vault.path().join("my-skill");
        fs::create_dir(&skill_dir)?;
        
        let target_link = target_dir.path().join("my-skill");
        symlink_dir(&skill_dir, &target_link)?;
        
        assert!(fs::symlink_metadata(&target_link).is_ok());

        // Dry run
        unlink_skill(&target_link, true)?;
        assert!(fs::symlink_metadata(&target_link).is_ok());

        // Real run
        unlink_skill(&target_link, false)?;
        assert!(fs::symlink_metadata(&target_link).is_err());

        Ok(())
    }
}
