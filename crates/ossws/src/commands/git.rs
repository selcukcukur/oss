use std::path::PathBuf;
use std::process::Stdio;

use ossws_core::StatusCounts;

use crate::definitions::{GitCommand, GitSubcommand};
use crate::types::{git_command, git_output, GitInspection, Repository};

pub fn run(command: GitCommand, repo: &Repository) -> Result<(), String> {
    match command.command {
        GitSubcommand::Inspect => {
            let inspection = inspect(repo)?;
            println!("repository: {}", repo.root.display());
            println!("git directory: {}", inspection.git_dir.display());
            println!("head: {}", inspection.head);
            println!(
                "branch: {}",
                inspection.branch.unwrap_or_else(|| "detached".to_string())
            );
            println!("clean: {}", inspection.clean);
            println!("staged files: {}", inspection.staged);
            println!("modified files: {}", inspection.modified);
            println!("untracked files: {}", inspection.untracked);
            Ok(())
        }
        GitSubcommand::Diff(args) => {
            let mut git_args = vec!["diff".to_string()];
            if args.staged {
                git_args.push("--cached".to_string());
            }
            if args.stat {
                git_args.push("--stat".to_string());
            }
            if args.name_only {
                git_args.push("--name-only".to_string());
            }
            if let Some(base) = args.base {
                git_args.push(base);
            }
            print_git(repo, git_args)
        }
        GitSubcommand::Files(args) => {
            let mut git_args = vec!["diff".to_string(), "--name-only".to_string()];
            if args.staged {
                git_args.push("--cached".to_string());
            }
            if let Some(base) = args.base {
                git_args.push(base);
            }
            print_git(repo, git_args)
        }
    }
}

fn inspect(repo: &Repository) -> Result<GitInspection, String> {
    let git_dir = git_path(repo, "rev-parse", ["--git-dir"])?;
    let head =
        git_output(repo, ["rev-parse", "--short", "HEAD"]).unwrap_or_else(|_| "unborn".to_string());
    let branch = git_output(repo, ["branch", "--show-current"])
        .ok()
        .map(|value| value.trim().to_string())
        .filter(|value| !value.is_empty());
    let status = git_output(repo, ["status", "--porcelain=v1"])?;
    let counts = StatusCounts::from_porcelain(&status);

    Ok(GitInspection {
        git_dir,
        head: head.trim().to_string(),
        branch,
        clean: status.trim().is_empty(),
        staged: counts.staged,
        modified: counts.modified,
        untracked: counts.untracked,
    })
}

fn git_path<const N: usize>(
    repo: &Repository,
    command: &str,
    args: [&str; N],
) -> Result<PathBuf, String> {
    let mut all_args = vec![command.to_string()];
    all_args.extend(args.into_iter().map(String::from));
    let path = git_output(repo, all_args)?.trim().to_string();
    let candidate = PathBuf::from(path);

    if candidate.is_absolute() {
        Ok(candidate)
    } else {
        Ok(repo.root.join(candidate))
    }
}

fn print_git(repo: &Repository, args: Vec<String>) -> Result<(), String> {
    let output = git_command(&repo.root, args)
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .output()
        .map_err(|error| format!("failed to run git: {error}"))?;

    if !output.status.success() {
        return Err(String::from_utf8_lossy(&output.stderr).trim().to_string());
    }

    print!("{}", String::from_utf8_lossy(&output.stdout));
    Ok(())
}
