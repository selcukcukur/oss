use std::path::PathBuf;

use clap::{Args, Parser, Subcommand, ValueEnum};
use ossws_core::CommitKind as CoreCommitKind;

#[derive(Debug, Parser)]
#[command(name = "ossws")]
#[command(version)]
#[command(about = "Automate OSS-WS standards checks for git repositories")]
pub struct Cli {
    #[arg(long, global = true, value_name = "PATH")]
    pub repo: Option<PathBuf>,

    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Debug, Subcommand)]
pub enum Commands {
    #[command(about = "Inspect repository metadata and git state")]
    Git(GitCommand),
    #[command(about = "Validate or build OSS-WS commit messages")]
    Commit(CommitCommand),
    #[command(about = "Check the repository changelog")]
    Changelog(ChangelogCommand),
    #[command(about = "Analyze changed files by OSS-WS area")]
    Analyze(AnalyzeCommand),
    #[command(about = "Run phase completion checks")]
    Phase(PhaseCommand),
    #[command(about = "Run all available checks")]
    Check(CheckCommand),
}

#[derive(Debug, Args)]
pub struct GitCommand {
    #[command(subcommand)]
    pub command: GitSubcommand,
}

#[derive(Debug, Subcommand)]
pub enum GitSubcommand {
    #[command(about = "Inspect .git, HEAD, branch, status, and file counts")]
    Inspect,
    #[command(about = "Print a git diff")]
    Diff(DiffArgs),
    #[command(about = "Print changed file names")]
    Files(DiffFilesArgs),
}

#[derive(Debug, Args)]
pub struct DiffArgs {
    #[arg(long)]
    pub staged: bool,

    #[arg(long, value_name = "REF")]
    pub base: Option<String>,

    #[arg(long)]
    pub stat: bool,

    #[arg(long)]
    pub name_only: bool,
}

#[derive(Debug, Args)]
pub struct DiffFilesArgs {
    #[arg(long)]
    pub staged: bool,

    #[arg(long, value_name = "REF")]
    pub base: Option<String>,
}

#[derive(Debug, Args)]
pub struct CommitCommand {
    #[command(subcommand)]
    pub command: CommitSubcommand,
}

#[derive(Debug, Subcommand)]
pub enum CommitSubcommand {
    #[command(about = "Validate a commit subject against OSS-WS rules")]
    Validate(CommitValidateArgs),
    #[command(about = "Build a valid lowercase scoped commit subject")]
    Build(CommitBuildArgs),
}

#[derive(Debug, Args)]
pub struct CommitValidateArgs {
    #[arg(value_name = "SUBJECT")]
    pub subject: String,
}

#[derive(Debug, Args)]
pub struct CommitBuildArgs {
    #[arg(long, value_enum)]
    pub kind: CommitKind,

    #[arg(long)]
    pub scope: String,

    #[arg(long)]
    pub subject: String,

    #[arg(long)]
    pub breaking: bool,
}

#[derive(Clone, Debug, ValueEnum)]
pub enum CommitKind {
    Feat,
    Perf,
    Docs,
    Fix,
    Refactor,
    Chore,
}

impl From<CommitKind> for CoreCommitKind {
    fn from(kind: CommitKind) -> Self {
        match kind {
            CommitKind::Feat => Self::Feat,
            CommitKind::Perf => Self::Perf,
            CommitKind::Docs => Self::Docs,
            CommitKind::Fix => Self::Fix,
            CommitKind::Refactor => Self::Refactor,
            CommitKind::Chore => Self::Chore,
        }
    }
}

#[derive(Debug, Args)]
pub struct ChangelogCommand {
    #[command(subcommand)]
    pub command: ChangelogSubcommand,
}

#[derive(Debug, Subcommand)]
pub enum ChangelogSubcommand {
    #[command(about = "Validate changelog.md structure")]
    Check,
}

#[derive(Debug, Args)]
pub struct AnalyzeCommand {
    #[arg(long)]
    pub staged: bool,

    #[arg(long, value_name = "REF")]
    pub base: Option<String>,
}

#[derive(Debug, Args)]
pub struct PhaseCommand {
    #[command(subcommand)]
    pub command: PhaseSubcommand,
}

#[derive(Debug, Subcommand)]
pub enum PhaseSubcommand {
    #[command(about = "Check whether a phase is ready to commit")]
    Check,
}

#[derive(Debug, Args)]
pub struct CheckCommand {
    #[arg(long, value_name = "SUBJECT")]
    pub commit_subject: Option<String>,
}
