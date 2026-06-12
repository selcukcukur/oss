use crate::definitions::{PhaseCommand, PhaseSubcommand};
use crate::types::{git_output, Repository};

pub fn run(command: PhaseCommand, repo: &Repository) -> Result<(), String> {
    match command.command {
        PhaseSubcommand::Check => check(repo),
    }
}

pub fn check(repo: &Repository) -> Result<(), String> {
    let status = git_output(repo, ["status", "--porcelain=v1"])?;

    if status.trim().is_empty() {
        println!("phase: clean working tree");
        return Ok(());
    }

    let changed: Vec<&str> = status.lines().collect();
    let changelog_changed = changed
        .iter()
        .any(|line| line.ends_with(" changelog.md") || line.ends_with("changelog.md"));

    if !changelog_changed {
        return Err("phase has changes but changelog.md is not updated".to_string());
    }

    println!("phase: changelog updated");
    println!("phase files: {}", changed.len());
    Ok(())
}
