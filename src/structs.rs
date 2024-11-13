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
    #[arg(short = 'b', long, help = "string prefix for all branches when creating a new branch via this tool.")]
    pub branch_prefix: Option<String>,
    #[arg(short = 'e', long, help = "regex for use in commit message - extracts part of branch and prepends to all commit messages. (e.g., r\"\\d+\") for extracting digits.")]
    pub commit_extraction_regex: Option<String>,
    #[arg(short = 'c', long, help = "string prefix for all commit messages created via this tool.")]
    pub commit_prefix: Option<String>
}

#[derive(Parser)]
pub struct CommitArgs {
    /// The commit message
    pub message: String,
}