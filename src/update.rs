use anyhow::{Result, Context};
use self_update::backends::github::Update;

pub fn run() -> Result<()> {
    let current_version = env!("CARGO_PKG_VERSION");
    println!("Current version: v{}", current_version);
    println!("Checking for updates on GitHub...");

    let status = Update::configure()
        .repo_owner("jialrs")
        .repo_name("skm")
        .bin_name("skm")
        .show_download_progress(true)
        .current_version(current_version)
        .build()
        .context("Failed to configure self-update")?
        .update()
        .context("Failed to perform update")?;

    if status.updated() {
        println!("Successfully updated to v{}!", status.version());
    } else {
        println!("Already up to date.");
    }

    Ok(())
}
