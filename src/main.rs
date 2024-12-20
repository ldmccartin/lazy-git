use clap::{Parser, Subcommand};
use lib::git;
use lib::structs;
use lib::config;

#[derive(Parser)]
#[command(name = "lazy-git", version = "1.0", about = "A CLI tool for managing branches and commits")]
pub struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Creates a new branch
    B(structs::BranchArgs),

    /// Commits with a message
    C(structs::CommitArgs),

    /// Stages all changes
    A,

    /// Resets all changes
    R,

    /// Set Config for CLI tool.
    Config(structs::ConfigArgs),
}

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::Config(args) => {
            if args.branch_prefix.is_some() {
                config::set_config_branch_prefix(args.branch_prefix.unwrap());
            }

            if args.commit_extraction_regex.is_some() {
                config::set_config_commit_extraction_regex(args.commit_extraction_regex.unwrap());
            }

            if args.commit_prefix.is_some() {
                config::set_config_commit_prefix(args.commit_prefix.unwrap());
            }

            config::display_config()
        }

        Commands::B(args) => {git::checkout_new_branch(config::get_config().branch_prefix, args)}

        Commands::C(args) => {git::commit_staged_changes(config::get_config(), args)}

        Commands::A => {git::stage_all_changes()}
        
        Commands::R => {git::reset_all_staged_changes()}
    }
}
