use crate::config::Config;
use anyhow::{Result, Context, anyhow};
use std::process::Command;
use dialoguer::{theme::ColorfulTheme, Input, Confirm};

pub fn run(dry_run: bool) -> Result<()> {
    let config = Config::load()?;
    
    if !config.vault_path.exists() {
         anyhow::bail!("Vault not found. Run 'skm init' first.");
    }

    println!("Checking for changes in {:?}", config.vault_path);
    
    // 1. Get status
    let output = Command::new("git")
        .current_dir(&config.vault_path)
        .arg("status")
        .arg("--short")
        .output()
        .context("Failed to run git status")?;
        
    let status_text = String::from_utf8_lossy(&output.stdout);
    
    if status_text.trim().is_empty() {
        println!("No changes to push.");
        return Ok(());
    }

    println!("Changed files:\n{}", status_text);

    // 2. Sanity Check: Check for nested .git directories
    // Git will fail to 'add .' if subdirectories have their own .git (and are not submodules)
    check_nested_git(&config.vault_path, dry_run)?;

    // 3. Ask for confirmation to proceed
    if !Confirm::with_theme(&ColorfulTheme::default())
        .with_prompt("Do you want to commit and push these changes?")
        .default(true)
        .interact()? 
    {
        println!("Push cancelled.");
        return Ok(());
    }

    // 3. Ask for commit message
    let message: String = Input::with_theme(&ColorfulTheme::default())
        .with_prompt("Commit message")
        .interact_text()?;

    if dry_run {
        println!("[Dry Run] Would run:");
        println!("  git add .");
        println!("  git commit -m \"{}\"", message);
        println!("  git push");
        return Ok(());
    }

    // 4. Git operations
    println!("Staging changes...");
    run_git(&config.vault_path, &["add", "."])?;

    println!("Committing...");
    run_git(&config.vault_path, &["commit", "-m", &message])?;

    println!("Pushing to remote...");
    run_git(&config.vault_path, &["push"])?;

    println!("Successfully pushed changes to vault.");
    
    Ok(())
}

fn check_nested_git(vault_path: &std::path::Path, dry_run: bool) -> Result<()> {
    use walkdir::WalkDir;
    
    let mut nested_git_dirs = Vec::new();
    
    for entry in WalkDir::new(vault_path).min_depth(2).into_iter().filter_map(|e| e.ok()) {
        if entry.file_name() == ".git" && entry.file_type().is_dir() {
             nested_git_dirs.push(entry.path().to_path_buf());
        }
    }
    
    if !nested_git_dirs.is_empty() {
        println!("⚠️  Found nested .git directories that will prevent staging:");
        for dir in &nested_git_dirs {
            println!("  - {:?}", dir);
        }
        
        if Confirm::with_theme(&ColorfulTheme::default())
            .with_prompt("Do you want to remove these nested .git directories to allow tracking in the Vault?")
            .default(false)
            .interact()? 
        {
            if dry_run {
                println!("[Dry Run] Would remove nested .git directories.");
            } else {
                for dir in nested_git_dirs {
                    println!("Removing {:?}", dir);
                    std::fs::remove_dir_all(dir).context("Failed to remove nested .git directory")?;
                }
            }
        } else {
            anyhow::bail!("Staging cancelled. Please remove nested .git directories manually.");
        }
    }
    
    Ok(())
}

fn run_git(dir: &std::path::Path, args: &[&str]) -> Result<()> {
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
