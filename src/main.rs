mod commands;
mod engine;
mod git;

use anyhow::Result;
use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "gt")]
#[command(about = "Fast stacked Git branches and PRs", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Show the current stack
    #[command(visible_alias = "s")]
    Status,

    /// Restack (rebase) the current branch onto its parent
    #[command(visible_alias = "rs")]
    Restack {
        /// Restack all branches in the stack
        #[arg(short, long)]
        all: bool,
    },

    /// Checkout a branch in the stack
    #[command(visible_alias = "co")]
    Checkout {
        /// Branch name (interactive if not provided)
        branch: Option<String>,
    },

    /// Continue after resolving conflicts
    Continue,

    /// Branch management commands
    #[command(subcommand, visible_alias = "b")]
    Branch(BranchCommands),

    /// Log the stack (alias for status)
    #[command(visible_alias = "l")]
    Log,
}

#[derive(Subcommand)]
enum BranchCommands {
    /// Create a new branch stacked on current
    #[command(visible_alias = "c")]
    Create {
        /// Name for the new branch
        name: String,
    },

    /// Track an existing branch (set its parent)
    Track {
        /// Parent branch name
        #[arg(short, long)]
        parent: Option<String>,
    },

    /// Delete a branch and its metadata
    #[command(visible_alias = "d")]
    Delete {
        /// Branch to delete
        branch: Option<String>,
        /// Force delete even if not merged
        #[arg(short, long)]
        force: bool,
    },
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Status | Commands::Log => commands::status::run(),
        Commands::Restack { all } => commands::restack::run(all),
        Commands::Checkout { branch } => commands::checkout::run(branch),
        Commands::Continue => commands::continue_cmd::run(),
        Commands::Branch(cmd) => match cmd {
            BranchCommands::Create { name } => commands::branch::create::run(&name),
            BranchCommands::Track { parent } => commands::branch::track::run(parent),
            BranchCommands::Delete { branch, force } => {
                commands::branch::delete::run(branch, force)
            }
        },
    }
}
