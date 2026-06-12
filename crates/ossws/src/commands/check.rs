use ossws_core::validate_commit_subject;

use crate::commands::{changelog, phase};
use crate::definitions::CheckCommand;
use crate::types::Repository;

pub fn run(command: CheckCommand, repo: &Repository) -> Result<(), String> {
    changelog::check(repo)?;
    phase::check(repo)?;

    if let Some(subject) = command.commit_subject {
        validate_commit_subject(&subject)?;
        println!("commit subject: valid");
    }

    Ok(())
}
