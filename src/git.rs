
use std::io::{self, BufRead};
use std::process::{Stdio, Command};

use regex::Regex;

use crate::config::LazyGitConfig;
use crate::structs;

pub fn checkout_new_branch(branch_prefix: String, args: structs::BranchArgs) {
  let formatted_args = format!("{}{}-{}", branch_prefix, args.ticket_number, args.description);
  let _ = Command::new("git")
    .args(["checkout", "-b", &formatted_args])
    .output();
}

pub fn stage_all_changes() {
  let _ = Command::new("git")
    .args(["add", "."])
    .output();
}

pub fn reset_all_staged_changes() {
  let _ = Command::new("git")
    .args(["reset", "."])
    .output();
}

pub fn commit_staged_changes(config: LazyGitConfig, args: structs::CommitArgs) {
  let branch = Command::new("git")
  .args(["symbolic-ref", "--short", "HEAD"])
  .output()
  .expect("Failed to get current git branch name, cannot attempt commit.")
  .stdout;

  let branch_string: String = String::from_utf8_lossy(&branch).into_owned();
  let commit_extraction_value = Regex::new(config.commit_extraction_regex.as_str()).unwrap().find(&branch_string)
    .unwrap()
    .as_str();

  let formatted_args = format!("{}{}:{}", config.commit_prefix, commit_extraction_value, args.message);
  let mut child = Command::new("git")
    .args(["commit", "-m", &formatted_args])
    .stdout(Stdio::piped())
    .stderr(Stdio::piped())
    .spawn()
    .unwrap();

  let stdout = child.stdout.take().expect("Failed to capture stdout");
  let stderr = child.stderr.take().expect("Failed to capture stderr");

  // Create threads to pipe stdout and stderr to the terminal in real-time
  let stdout_thread = std::thread::spawn(move || {
    let reader = io::BufReader::new(stdout);
    for line in reader.lines() {
        if let Ok(line) = line {
            println!("{}", line); // Print each line of stdout
        }
    }
  });

  let stderr_thread = std::thread::spawn(move || {
    let reader = io::BufReader::new(stderr);
    for line in reader.lines() {
        if let Ok(line) = line {
            eprintln!("{}", line); // Print each line of stderr
        }
    }
  });

  // Wait for the command to finish
  child.wait().expect("Failed to wait on child process");

  // Join stdout and stderr threads to ensure all output is printed
  stdout_thread.join().expect("Failed to join stdout thread");
  stderr_thread.join().expect("Failed to join stderr thread");
}