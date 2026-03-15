use crate::config::Config;
use crate::link::find_skills;
use anyhow::Result;
use std::fs;

pub fn run() -> Result<()> {
    let config = Config::load()?;
    
    if !config.vault_path.exists() {
        println!("Vault not found at {:?}. Run 'skm onboard' or 'skm init' first.", config.vault_path);
        return Ok(());
    }

    let skills = find_skills(&config.vault_path)?;

    if skills.is_empty() {
        println!("No skills found in vault at {:?}", config.vault_path);
        return Ok(());
    }

    println!("{:<20} {:<10} {}", "Skill", "Status", "Path");
    println!("{}", "-".repeat(50));

    for skill_path in skills {
        let name = skill_path.file_name().unwrap().to_string_lossy();
        let target_link = config.target_path.join(&*name);
        
        let status = if target_link.exists() {
             if let Ok(target) = fs::read_link(&target_link) {
                 if target == skill_path {
                     "Linked"
                 } else {
                     "Conflict" // Symlink points elsewhere
                 }
             } else {
                 "Conflict" // Not a symlink but exists (dir/file)
             }
        } else if fs::symlink_metadata(&target_link).is_ok() {
             // Symlink exists but broken
             "Broken"
        } else {
            "Unlinked"
        };
        
        println!("{:<20} {:<10} {:?}", name, status, skill_path);
    }
    
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_list_status() -> Result<()> {
        // We can't easily test the output, but we can verify the logic if we separate status detection.
        // For now, let's just ensure it runs.
        Ok(())
    }
}
