use std::env;
use std::ffi::OsStr;
use std::fmt;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};

use clap::{Args, Parser, Subcommand, ValueEnum};

fn main() {
    let cli = Cli::parse();

    if let Err(error) = run(cli) {
        eprintln!("error: {error}");
        std::process::exit(1);
    }
}

#[derive(Debug, Parser)]
#[command(name = "oss-ws")]
#[command(version)]
#[command(about = "Automate OSS-WS standards checks for git repositories")]
struct Cli {
    #[arg(long, global = true, value_name = "PATH")]
    repo: Option<PathBuf>,

    #[command(subcommand)]
    command: Commands,
}

#[derive(Debug, Subcommand)]
enum Commands {
    #[command(about = "Inspect repository metadata and git state")]
    Git(GitCommand),
    #[command(about = "Validate or build OSS-WS commit messages")]
    Commit(CommitCommand),
    #[command(about = "Check the repository changelog")]
    Changelog(ChangelogCommand),
    #[command(about = "Run phase completion checks")]
    Phase(PhaseCommand),
    #[command(about = "Run all available checks")]
    Check(CheckCommand),
}

#[derive(Debug, Args)]
struct GitCommand {
    #[command(subcommand)]
    command: GitSubcommand,
}

#[derive(Debug, Subcommand)]
enum GitSubcommand {
    #[command(about = "Inspect .git, HEAD, branch, status, and file counts")]
    Inspect,
    #[command(about = "Print a git diff")]
    Diff(DiffArgs),
    #[command(about = "Print changed file names")]
    Files(DiffFilesArgs),
}

#[derive(Debug, Args)]
struct DiffArgs {
    #[arg(long)]
    staged: bool,

    #[arg(long, value_name = "REF")]
    base: Option<String>,

    #[arg(long)]
    stat: bool,

    #[arg(long)]
    name_only: bool,
}

#[derive(Debug, Args)]
struct DiffFilesArgs {
    #[arg(long)]
    staged: bool,

    #[arg(long, value_name = "REF")]
    base: Option<String>,
}

#[derive(Debug, Args)]
struct CommitCommand {
    #[command(subcommand)]
    command: CommitSubcommand,
}

#[derive(Debug, Subcommand)]
enum CommitSubcommand {
    #[command(about = "Validate a commit subject against OSS-WS rules")]
    Validate(CommitValidateArgs),
    #[command(about = "Build a valid lowercase scoped commit subject")]
    Build(CommitBuildArgs),
}

#[derive(Debug, Args)]
struct CommitValidateArgs {
    #[arg(value_name = "SUBJECT")]
    subject: String,
}

#[derive(Debug, Args)]
struct CommitBuildArgs {
    #[arg(long, value_enum)]
    kind: CommitKind,

    #[arg(long)]
    scope: String,

    #[arg(long)]
    subject: String,

    #[arg(long)]
    breaking: bool,
}

#[derive(Clone, Debug, ValueEnum)]
enum CommitKind {
    Feat,
    Perf,
    Docs,
    Fix,
    Refactor,
    Chore,
}

impl fmt::Display for CommitKind {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CommitKind::Feat => formatter.write_str("feat"),
            CommitKind::Perf => formatter.write_str("perf"),
            CommitKind::Docs => formatter.write_str("docs"),
            CommitKind::Fix => formatter.write_str("fix"),
            CommitKind::Refactor => formatter.write_str("refactor"),
            CommitKind::Chore => formatter.write_str("chore"),
        }
    }
}

#[derive(Debug, Args)]
struct ChangelogCommand {
    #[command(subcommand)]
    command: ChangelogSubcommand,
}

#[derive(Debug, Subcommand)]
enum ChangelogSubcommand {
    #[command(about = "Validate changelog.md structure")]
    Check,
}

#[derive(Debug, Args)]
struct PhaseCommand {
    #[command(subcommand)]
    command: PhaseSubcommand,
}

#[derive(Debug, Subcommand)]
enum PhaseSubcommand {
    #[command(about = "Check whether a phase is ready to commit")]
    Check,
}

#[derive(Debug, Args)]
struct CheckCommand {
    #[arg(long, value_name = "SUBJECT")]
    commit_subject: Option<String>,
}

fn run(cli: Cli) -> Result<(), String> {
    let repo = Repository::open(cli.repo)?;

    match cli.command {
        Commands::Git(command) => run_git(command, &repo),
        Commands::Commit(command) => run_commit(command),
        Commands::Changelog(command) => run_changelog(command, &repo),
        Commands::Phase(command) => run_phase(command, &repo),
        Commands::Check(command) => run_all_checks(command, &repo),
    }
}

fn run_git(command: GitCommand, repo: &Repository) -> Result<(), String> {
    match command.command {
        GitSubcommand::Inspect => {
            let inspection = inspect_git(repo)?;
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

fn run_commit(command: CommitCommand) -> Result<(), String> {
    match command.command {
        CommitSubcommand::Validate(args) => {
            validate_commit_subject(&args.subject)?;
            println!("valid commit subject");
            Ok(())
        }
        CommitSubcommand::Build(args) => {
            let scope = normalize_token(&args.scope);
            let subject = normalize_subject(&args.subject);
            let marker = if args.breaking { "!" } else { "" };
            let built = format!("{}({}){}: {}", args.kind, scope, marker, subject);
            validate_commit_subject(&built)?;
            println!("{built}");
            Ok(())
        }
    }
}

fn run_changelog(command: ChangelogCommand, repo: &Repository) -> Result<(), String> {
    match command.command {
        ChangelogSubcommand::Check => check_changelog(repo),
    }
}

fn run_phase(command: PhaseCommand, repo: &Repository) -> Result<(), String> {
    match command.command {
        PhaseSubcommand::Check => check_phase(repo),
    }
}

fn run_all_checks(command: CheckCommand, repo: &Repository) -> Result<(), String> {
    check_changelog(repo)?;
    check_phase(repo)?;

    if let Some(subject) = command.commit_subject {
        validate_commit_subject(&subject)?;
        println!("commit subject: valid");
    }

    Ok(())
}

#[derive(Debug)]
struct Repository {
    root: PathBuf,
}

impl Repository {
    fn open(path: Option<PathBuf>) -> Result<Self, String> {
        let start = match path {
            Some(path) => path,
            None => env::current_dir()
                .map_err(|error| format!("cannot read current directory: {error}"))?,
        };

        let output = git_output_in(&start, ["rev-parse", "--show-toplevel"])?;
        let root = PathBuf::from(output.trim());

        Ok(Self { root })
    }
}

#[derive(Debug)]
struct GitInspection {
    git_dir: PathBuf,
    head: String,
    branch: Option<String>,
    clean: bool,
    staged: usize,
    modified: usize,
    untracked: usize,
}

fn inspect_git(repo: &Repository) -> Result<GitInspection, String> {
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

#[derive(Debug, Default)]
struct StatusCounts {
    staged: usize,
    modified: usize,
    untracked: usize,
}

impl StatusCounts {
    fn from_porcelain(status: &str) -> Self {
        let mut counts = Self::default();

        for line in status.lines() {
            let mut chars = line.chars();
            let index = chars.next().unwrap_or(' ');
            let worktree = chars.next().unwrap_or(' ');

            if index == '?' && worktree == '?' {
                counts.untracked += 1;
                continue;
            }

            if index != ' ' {
                counts.staged += 1;
            }

            if worktree != ' ' {
                counts.modified += 1;
            }
        }

        counts
    }
}

fn check_changelog(repo: &Repository) -> Result<(), String> {
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

fn check_phase(repo: &Repository) -> Result<(), String> {
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

fn validate_release_heading(line: &str) -> Result<(), String> {
    let value = line
        .strip_prefix("## ")
        .ok_or_else(|| format!("invalid release heading: {line}"))?;
    let (version, date) = value
        .split_once(" - ")
        .ok_or_else(|| format!("release heading must use VERSION - YYYY-MM-DD: {line}"))?;

    if version.starts_with('v') {
        return Err(format!("release version must not start with v: {line}"));
    }

    if !is_iso_date(date) {
        return Err(format!("release date must use YYYY-MM-DD: {line}"));
    }

    Ok(())
}

fn validate_commit_subject(subject: &str) -> Result<(), String> {
    if subject.trim() != subject {
        return Err("commit subject must not start or end with whitespace".to_string());
    }

    if subject.ends_with('.') {
        return Err("commit subject must not end with a period".to_string());
    }

    if subject != subject.to_lowercase() {
        return Err("commit subject must be lowercase".to_string());
    }

    let (head, summary) = subject
        .split_once(": ")
        .ok_or_else(|| "commit subject must use type(scope): subject".to_string())?;

    if summary.is_empty() {
        return Err("commit subject summary must not be empty".to_string());
    }

    let open = head
        .find('(')
        .ok_or_else(|| "commit subject must include a scope".to_string())?;
    let close = head
        .find(')')
        .ok_or_else(|| "commit subject scope must close with ')'".to_string())?;

    if close <= open + 1 {
        return Err("commit subject scope must not be empty".to_string());
    }

    let kind = &head[..open];
    if !matches!(
        kind,
        "feat" | "perf" | "docs" | "fix" | "refactor" | "chore"
    ) {
        return Err(format!("invalid commit type: {kind}"));
    }

    let after_scope = &head[close + 1..];
    if !after_scope.is_empty() && after_scope != "!" {
        return Err("only ! may appear after the scope".to_string());
    }

    let scope = &head[open + 1..close];
    validate_token(scope, "scope")?;

    Ok(())
}

fn validate_token(value: &str, label: &str) -> Result<(), String> {
    if value != value.to_lowercase() {
        return Err(format!("{label} must be lowercase"));
    }

    if !value.chars().all(|character| {
        character.is_ascii_lowercase() || character.is_ascii_digit() || character == '-'
    }) {
        return Err(format!(
            "{label} may only contain lowercase letters, digits, and hyphens"
        ));
    }

    Ok(())
}

fn normalize_token(value: &str) -> String {
    value
        .trim()
        .to_lowercase()
        .chars()
        .map(|character| {
            if character.is_ascii_lowercase() || character.is_ascii_digit() {
                character
            } else {
                '-'
            }
        })
        .collect::<String>()
        .split('-')
        .filter(|part| !part.is_empty())
        .collect::<Vec<_>>()
        .join("-")
}

fn normalize_subject(value: &str) -> String {
    value.trim().trim_end_matches('.').to_lowercase()
}

fn is_iso_date(value: &str) -> bool {
    let bytes = value.as_bytes();
    bytes.len() == 10
        && bytes[4] == b'-'
        && bytes[7] == b'-'
        && bytes[..4].iter().all(u8::is_ascii_digit)
        && bytes[5..7].iter().all(u8::is_ascii_digit)
        && bytes[8..].iter().all(u8::is_ascii_digit)
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
    let output = git_command(&repo.root, args)?
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

fn git_output<I, S>(repo: &Repository, args: I) -> Result<String, String>
where
    I: IntoIterator<Item = S>,
    S: AsRef<OsStr>,
{
    git_output_in(&repo.root, args)
}

fn git_output_in<I, S>(directory: &Path, args: I) -> Result<String, String>
where
    I: IntoIterator<Item = S>,
    S: AsRef<OsStr>,
{
    let output = git_command(directory, args)?
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .output()
        .map_err(|error| format!("failed to run git: {error}"))?;

    if !output.status.success() {
        return Err(String::from_utf8_lossy(&output.stderr).trim().to_string());
    }

    Ok(String::from_utf8_lossy(&output.stdout).to_string())
}

fn git_command<I, S>(directory: &Path, args: I) -> Result<Command, String>
where
    I: IntoIterator<Item = S>,
    S: AsRef<OsStr>,
{
    let mut command = Command::new("git");
    command.current_dir(directory);
    command.args(args);
    Ok(command)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn accepts_valid_commit_subjects() {
        for subject in [
            "feat(phase): add draft commit standard",
            "docs(commit): document lowercase subject rules",
            "feat(template)!: require schema version",
            "refactor(skill): flatten commit rules",
        ] {
            assert!(validate_commit_subject(subject).is_ok(), "{subject}");
        }
    }

    #[test]
    fn rejects_invalid_commit_subjects() {
        for subject in [
            "Add draft commit standard",
            "feat: add draft commit standard",
            "feat(commit): Add draft commit standard",
            "feat(commit): add draft commit standard.",
            "style(commit): add formatting",
            "feat(Commit): add guide",
        ] {
            assert!(validate_commit_subject(subject).is_err(), "{subject}");
        }
    }

    #[test]
    fn parses_status_counts() {
        let counts = StatusCounts::from_porcelain("M  a\n M b\n?? c\nA  d\n");
        assert_eq!(counts.staged, 2);
        assert_eq!(counts.modified, 1);
        assert_eq!(counts.untracked, 1);
    }

    #[test]
    fn validates_release_dates() {
        assert!(validate_release_heading("## 0.1.0-draft - 2026-06-12").is_ok());
        assert!(validate_release_heading("## v0.1.0 - 2026-06-12").is_err());
        assert!(validate_release_heading("## 0.1.0 - 12/06/2026").is_err());
    }
}
