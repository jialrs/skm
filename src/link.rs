use crate::config::Config;
use anyhow::{Result, Context};
use std::path::{Path, PathBuf};
use walkdir::WalkDir;
use std::fs;
use std::os::unix::fs::symlink;

pub fn run(skill_name: Option<String>, dry_run: bool, target_dir_override: Option<String>) -> Result<()> {
    let mut config = Config::load()?;
    
    if let Some(td) = target_dir_override {
        config.target_path = PathBuf::from(td);
    }

    if !config.vault_path.exists() {
        anyhow::bail!("Vault path does not exist: {:?}", config.vault_path);
    }
    
    // Create target dir if it doesn't exist
    if !dry_run {
         fs::create_dir_all(&config.target_path)?;
    }

    let skills = find_skills(&config.vault_path)?;

    for skill_path in skills {
        let name = skill_path.file_name().unwrap().to_string_lossy();
        
        if let Some(ref specific) = skill_name {
            if name != *specific {
                continue;
            }
        }

        let target_link = config.target_path.join(&*name);
        
        if dry_run {
            println!("[Dry Run] Checking {}", name);
        }
        
        link_skill(&skill_path, &target_link, dry_run)?;
    }

    Ok(())
}

pub fn find_skills(vault_path: &Path) -> Result<Vec<PathBuf>> {
    let mut skills = Vec::new();
    // Assuming structure: vault/skill_name/SKILL.md
    
    for entry in WalkDir::new(vault_path).min_depth(1).max_depth(2) {
        let entry = entry?;
        if entry.file_type().is_file() && entry.file_name() == "SKILL.md" {
            if let Some(parent) = entry.path().parent() {
                skills.push(parent.to_path_buf());
            }
        }
    }
    Ok(skills)
}

fn link_skill(source: &Path, target: &Path, dry_run: bool) -> Result<()> {
    // Check for existence of the link itself (not what it points to)
    if let Ok(metadata) = fs::symlink_metadata(target) {
        if metadata.file_type().is_symlink() {
             let current_point = fs::read_link(target)?;
             if current_point == source {
                 if !dry_run {
                     println!("Skipping {} (already linked correctly)", target.file_name().unwrap().to_string_lossy());
                 } else {
                     println!("  Would skip (already linked)");
                 }
                 return Ok(());
             }
             
             if dry_run {
                 println!("  Would remove old link: {:?}", target);
             } else {
                 println!("Removing old link: {:?}", target);
                 fs::remove_file(target).context("Failed to remove existing symlink")?;
             }
        } else {
            println!("Warning: {:?} exists and is not a symlink. Skipping.", target);
            return Ok(());
        }
    }

    if dry_run {
        println!("  Would link {:?} -> {:?}", source.file_name().unwrap(), target);
    } else {
        println!("Linking {:?} -> {:?}", source.file_name().unwrap(), target);
        symlink(source, target).context("Failed to create symlink")?;
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;
    use std::fs::File;

    #[test]
    fn test_find_skills() -> Result<()> {
        let vault = tempdir()?;
        let skill_dir = vault.path().join("my-skill");
        fs::create_dir(&skill_dir)?;
        File::create(skill_dir.join("SKILL.md"))?;
        
        let skills = find_skills(vault.path())?;
        assert_eq!(skills.len(), 1);
        assert_eq!(skills[0], skill_dir);
        Ok(())
    }

    #[test]
    fn test_link_skill() -> Result<()> {
        let vault = tempdir()?;
        let target_dir = tempdir()?;
        
        let skill_dir = vault.path().join("my-skill");
        fs::create_dir(&skill_dir)?;
        File::create(skill_dir.join("SKILL.md"))?;
        
        let target_link = target_dir.path().join("my-skill");
        
        // 1. Dry run
        link_skill(&skill_dir, &target_link, true)?;
        assert!(!target_link.exists() && !fs::symlink_metadata(&target_link).is_ok());

        // 2. Real run
        link_skill(&skill_dir, &target_link, false)?;
        assert!(fs::symlink_metadata(&target_link).unwrap().file_type().is_symlink());
        assert_eq!(fs::read_link(&target_link)?, skill_dir);

        Ok(())
    }
}
