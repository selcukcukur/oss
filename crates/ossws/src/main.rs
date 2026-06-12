mod commands;
mod definitions;
mod types;

use clap::Parser;

use commands::{run_analyze, run_changelog, run_check, run_commit, run_git, run_phase};
use definitions::{Cli, Commands};
use types::Repository;

fn main() {
    let cli = Cli::parse();

    if let Err(error) = run(cli) {
        eprintln!("error: {error}");
        std::process::exit(1);
    }
}

fn run(cli: Cli) -> Result<(), String> {
    let repo = Repository::open(cli.repo)?;

    match cli.command {
        Commands::Git(command) => run_git(command, &repo),
        Commands::Commit(command) => run_commit(command),
        Commands::Changelog(command) => run_changelog(command, &repo),
        Commands::Analyze(command) => run_analyze(command, &repo),
        Commands::Phase(command) => run_phase(command, &repo),
        Commands::Check(command) => run_check(command, &repo),
    }
}
