use std::ffi::OsStr;

use ossws_core::validate_release_heading;

use crate::definitions::{ChangelogCommand, ChangelogSubcommand};
use crate::types::Repository;

pub fn run(command: ChangelogCommand, repo: &Repository) -> Result<(), String> {
    match command.command {
        ChangelogSubcommand::Check => check(repo),
    }
}

pub fn check(repo: &Repository) -> Result<(), String> {
    let changelog = repo.root.join("changelog.md");

    if !has_exact_root_file(repo, "changelog.md")? {
        return Err("missing root changelog.md".to_string());
    }

    if has_exact_root_file(repo, "CHANGELOG.md")? {
        return Err("CHANGELOG.md is not the canonical OSS-WS changelog file".to_string());
    }

    let content = std::fs::read_to_string(&changelog)
        .map_err(|error| format!("cannot read changelog.md: {error}"))?;
    let first = content
        .lines()
        .find(|line| !line.trim().is_empty())
        .unwrap_or("");

    if first != "# Changelog" {
        return Err("changelog.md must start with # Changelog".to_string());
    }

    for line in content.lines().filter(|line| line.starts_with("## ")) {
        validate_release_heading(line)?;
    }

    println!("changelog: valid");
    Ok(())
}

fn has_exact_root_file(repo: &Repository, name: &str) -> Result<bool, String> {
    let entries = std::fs::read_dir(&repo.root)
        .map_err(|error| format!("cannot read repository root: {error}"))?;

    for entry in entries {
        let entry = entry.map_err(|error| format!("cannot read repository entry: {error}"))?;
        if entry.file_name() == OsStr::new(name) {
            return Ok(true);
        }
    }

    Ok(false)
}
