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
                config::set_config(args.branch_prefix.unwrap())
            } else {
                config::display_conifg()
            }
        }

        Commands::B(args) => {git::checkout_new_branch(config::get_config().branch_prefix, args);}

        Commands::C(args) => {git::commit_staged_changes(args);}

        Commands::A => {git::stage_all_changes();}
        
        Commands::R => {git::reset_all_staged_changes()}
    }
}
