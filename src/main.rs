use clap::{Parser, Subcommand};
use std::{fmt::format, process::Command};

#[derive(Parser)]
#[command(name = "git-helper", version = "1.0", about = "A CLI tool for managing branches and commits")]
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

    /// Adds all changes with no additional options
    AA,
    RA,
}

#[derive(Parser)]
struct BranchArgs {
    /// The title of the branch
    title: String,

    /// A description for the branch
    description: String,
}

#[derive(Parser)]
struct CommitArgs {
    /// The commit message
    message: String,
}

fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Commands::B(args) => {
            let formatted_args = format!("feature/VERREN-{}-{}", args.title, args.description);
            let _ = Command::new("git")
                .args(["checkout", "-b", &formatted_args])
                .output();
        }
        Commands::C(args) => {
            // let formatted_args = format!("VERREN-{}-{}", args.title, args.description);
            // let _ = Command::new("git")
            //     .args(["commit", "-m", &args.message])
            //     .output();
            let branch = Command::new("git")
                .args(["symbolic-ref", "--short", "HEAD"])
                .output();
            print!("{:?}", branch.unwrap());
        }
        Commands::AA => {
            let _ = Command::new("git")
                .arg("add .")
                .output();
        }
        Commands::RA => {
            let _ = Command::new("git")
                .arg("reset .")
                .output();
        }
    }
}
