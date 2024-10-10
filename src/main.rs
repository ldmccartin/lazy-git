use clap::{Parser, Subcommand};
use regex::Regex;
use std::process::Command;
use lib::config;
use lib::commands;

#[derive(Parser)]
#[command(name = "lazy-git", version = "1.0", about = "A CLI tool for managing branches and commits")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Creates a new branch
    B(BranchArgs),

    /// Commits with a message
    C(CommitArgs),

    /// Stages all changes
    A,

    /// Resets all changes
    R,

    /// Set Config for CLI tool.
    Config(ConfigArgs),
}

#[derive(Parser)]
struct BranchArgs {
    /// The ticket number
    ticket_number: String,

    /// A description for the branch
    description: String,
}

#[derive(Parser)]
struct ConfigArgs {
    /// Set the parent folder where all Rente projects are contained.
    #[arg(short = 'p', long)]
    branch_prefix: Option<String>,
}

#[derive(Parser)]
struct CommitArgs {
    /// The commit message
    message: String,
}

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::Config(args) => {
            if args.branch_prefix.is_some() {
                config::set_config(args.branch_prefix.unwrap())
            } else {
                config::display_conifg()
            }
        }

        Commands::B(args) => { 
            commands::create_branch(config::get_config().branch_prefix, args.ticket_number, args.description);
        }

        Commands::C(args) => {
            let branch = Command::new("git")
                .args(["symbolic-ref", "--short", "HEAD"])
                .output()
                .expect("Failed to get current git branch name, cannot attempt commit.")
                .stdout;

            let branch_string = String::from_utf8_lossy(&branch).into_owned();
            let ticket_number = Regex::new(r"\d+").unwrap().find(&branch_string)
                .expect("Branch does not contain number, likely does not conform to convention.")
                .as_str();

            let formatted_args = format!("VERREN-{}:{}", ticket_number, args.message);
            let _ = Command::new("git")
                .args(["commit", "-m", &formatted_args])
                .output();
        }

        Commands::A => {
            let _ = Command::new("git")
                .args(["add", "."])
                .output();
        }

        Commands::R => {
            let _ = Command::new("git")
                .args(["reset", "."])
                .output();
        }
    }
}
