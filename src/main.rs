use clap::{Parser, Subcommand, CommandFactory};
use anyhow::Result;

mod config;
mod init;
mod onboard;
mod link;
mod sync;
mod list;
mod check;

#[derive(Parser)]
#[command(name = "skm")]
#[command(about = "Skill Manager for Claude Agents", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,

    /// Run commands without modifying the file system
    #[arg(short, long, global = true)]
    dry_run: bool,

    /// Override the target directory for skills
    #[arg(long, global = true)]
    target_dir: Option<String>,
}

#[derive(Subcommand)]
enum Commands {
    /// Interactive setup for Vault path, Target path, and Git URL
    Onboard,
    /// Non-interactive initialization (clones vault and saves URL)
    Init {
        /// The Git repository URL for the skill vault
        repo_url: String,
    },
    /// Pulls latest changes from Git and re-validates links
    Sync,
    /// Lists all available skills and their current status
    List,
    /// Symlinks specific or all skills
    Link {
        /// The name of the skill to link (optional, links all if not provided)
        skill_name: Option<String>,
    },
    /// View or modify the current configuration
    Config,
    /// Generate shell completion scripts
    Completion {
        /// The shell to generate the script for
        shell: clap_complete::Shell,
    },
    /// Validates if uv and other required runtimes are installed
    Check,
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Onboard => {
            onboard::run()?;
        }
        Commands::Init { repo_url } => {
            init::run(repo_url.clone())?;
        }
        Commands::Sync => {
            sync::run(cli.dry_run, cli.target_dir.clone())?;
        }
        Commands::List => {
            list::run()?;
        }
        Commands::Link { skill_name } => {
            link::run(skill_name.clone(), cli.dry_run, cli.target_dir.clone())?;
        }
        Commands::Config => {
            let config = config::Config::load()?;
            println!("{:#?}", config);
        }
        Commands::Completion { shell } => {
            clap_complete::generate(*shell, &mut Cli::command(), "skm", &mut std::io::stdout());
        }
        Commands::Check => {
            check::run()?;
        }
    }

    Ok(())
}
