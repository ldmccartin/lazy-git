use std::process::Command;

pub fn create_branch(branch_prefix: String, ticket_number: String, description: String) {
    let formatted_args = format!("{}{}-{}", branch_prefix, ticket_number, description);
    let _ = Command::new("git")
        .args(["checkout", "-b", &formatted_args])
        .output();
}