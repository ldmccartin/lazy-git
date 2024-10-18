use clap::Parser;

#[derive(Parser)]
pub struct BranchArgs {
    /// The ticket number
    pub ticket_number: String,

    /// A description for the branch
    pub description: String,
}

#[derive(Parser)]
pub struct ConfigArgs {
    /// Set the parent folder where all Rente projects are contained.
    #[arg(short = 'p', long)]
    pub branch_prefix: Option<String>,
}

#[derive(Parser)]
pub struct CommitArgs {
    /// The commit message
    pub message: String,
}