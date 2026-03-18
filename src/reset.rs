use crate::config::Config;
use anyhow::{Result, Context, anyhow};
use std::process::Command;
use dialoguer::{theme::ColorfulTheme, Confirm};
use std::path::Path;

pub fn run(skill_name: Option<String>, dry_run: bool) -> Result<()> {
    let config = Config::load()?;
    
    if !config.vault_path.exists() {
         anyhow::bail!("Vault not found. Run 'skm init' first.");
    }

    let target_desc = match &skill_name {
        Some(name) => format!("skill '{}'", name),
        None => "the entire vault (all skills)".to_string(),
    };

    println!("⚠️  Warning: This will discard all uncommitted changes in {}.", target_desc);

    if !Confirm::with_theme(&ColorfulTheme::default())
        .with_prompt("Are you sure you want to proceed?")
        .default(false)
        .interact()? 
    {
        println!("Reset cancelled.");
        return Ok(());
    }

    if dry_run {
        println!("[Dry Run] Would reset {}.", target_desc);
        match &skill_name {
            Some(name) => {
                println!("  git checkout HEAD -- {:?}", name);
            }
            None => {
                println!("  git reset --hard HEAD");
                println!("  git clean -fd");
            }
        }
        return Ok(());
    }

    match &skill_name {
        Some(name) => {
            let skill_path = config.vault_path.join(name);
            if !skill_path.exists() {
                anyhow::bail!("Skill directory not found: {:?}", skill_path);
            }
            println!("Resetting skill: {}", name);
            run_git(&config.vault_path, &["checkout", "HEAD", "--", name])?;
        }
        None => {
            println!("Resetting entire vault...");
            run_git(&config.vault_path, &["reset", "--hard", "HEAD"])?;
            run_git(&config.vault_path, &["clean", "-fd"])?;
        }
    }

    println!("Successfully reset {}.", target_desc);
    Ok(())
}

fn run_git(dir: &Path, args: &[&str]) -> Result<()> {
    let status = Command::new("git")
        .current_dir(dir)
        .args(args)
        .status()
        .context(format!("Failed to run git {:?}", args))?;
        
    if !status.success() {
        return Err(anyhow!("Git command failed: git {:?}", args));
    }
    Ok(())
}
