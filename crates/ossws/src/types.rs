use std::ffi::OsStr;
use std::path::{Path, PathBuf};
use std::process::Command;

#[derive(Debug)]
pub struct Repository {
    pub root: PathBuf,
}

impl Repository {
    pub fn open(path: Option<PathBuf>) -> Result<Self, String> {
        let start = match path {
            Some(path) => path,
            None => std::env::current_dir()
                .map_err(|error| format!("cannot read current directory: {error}"))?,
        };

        let output = git_output_in(&start, ["rev-parse", "--show-toplevel"])?;
        let root = PathBuf::from(output.trim());

        Ok(Self { root })
    }
}

#[derive(Debug)]
pub struct GitInspection {
    pub git_dir: PathBuf,
    pub head: String,
    pub branch: Option<String>,
    pub clean: bool,
    pub staged: usize,
    pub modified: usize,
    pub untracked: usize,
}

pub fn git_output<I, S>(repo: &Repository, args: I) -> Result<String, String>
where
    I: IntoIterator<Item = S>,
    S: AsRef<OsStr>,
{
    git_output_in(&repo.root, args)
}

pub fn git_output_in<I, S>(directory: &Path, args: I) -> Result<String, String>
where
    I: IntoIterator<Item = S>,
    S: AsRef<OsStr>,
{
    let output = git_command(directory, args)
        .stdout(std::process::Stdio::piped())
        .stderr(std::process::Stdio::piped())
        .output()
        .map_err(|error| format!("failed to run git: {error}"))?;

    if !output.status.success() {
        return Err(String::from_utf8_lossy(&output.stderr).trim().to_string());
    }

    Ok(String::from_utf8_lossy(&output.stdout).to_string())
}

pub fn git_command<I, S>(directory: &Path, args: I) -> Command
where
    I: IntoIterator<Item = S>,
    S: AsRef<OsStr>,
{
    let mut command = Command::new("git");
    command.current_dir(directory);
    command.args(args);
    command
}
