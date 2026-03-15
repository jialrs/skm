use assert_cmd::Command;
use predicates::prelude::*;
use tempfile::tempdir;
use std::fs;

#[test]
fn test_help_command() {
    let mut cmd = Command::cargo_bin("skm").unwrap();
    cmd.arg("--help");
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("Skill Manager for Claude Agents"));
}

#[test]
fn test_check_command() {
    let mut cmd = Command::cargo_bin("skm").unwrap();
    cmd.arg("check");
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("Checking environment"));
}

#[test]
fn test_config_command() {
    let tmp_dir = tempdir().unwrap();
    let mut cmd = Command::cargo_bin("skm").unwrap();
    
    // Override HOME to use temp dir for config
    cmd.env("HOME", tmp_dir.path())
       .arg("config");
    
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("vault_path"))
        .stdout(predicate::str::contains("target_path"));
}

#[test]
fn test_link_dry_run() {
    let tmp_dir = tempdir().unwrap();
    let home = tmp_dir.path();
    let vault = home.join(".config/skm/vault");
    let target = home.join(".claude/skills");
    
    fs::create_dir_all(&vault).unwrap();
    fs::create_dir_all(&target).unwrap();
    
    // Create a dummy skill
    let skill_dir = vault.join("test-skill");
    fs::create_dir(&skill_dir).unwrap();
    fs::write(skill_dir.join("SKILL.md"), "test").unwrap();
    
    let mut cmd = Command::cargo_bin("skm").unwrap();
    cmd.env("HOME", home)
       .arg("--dry-run")
       .arg("link");
    
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("[Dry Run] Checking test-skill"))
        .stdout(predicate::str::contains("Would link \"test-skill\""));
}

#[test]
fn test_list_uninitialized() {
    let tmp_dir = tempdir().unwrap();
    let mut cmd = Command::cargo_bin("skm").unwrap();
    cmd.env("HOME", tmp_dir.path())
       .arg("list");
    
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("Vault not found"));
}
