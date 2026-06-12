use ossws_analyzer::{analyze_name_status, ChangeAnalysis};

use crate::definitions::AnalyzeCommand;
use crate::types::{git_output, Repository};

pub fn run(command: AnalyzeCommand, repo: &Repository) -> Result<(), String> {
    let analysis = analyze_changes(repo, command.staged, command.base)?;
    print_analysis(analysis);
    Ok(())
}

pub fn analyze_changes(
    repo: &Repository,
    staged: bool,
    base: Option<String>,
) -> Result<ChangeAnalysis, String> {
    let output = if let Some(base) = base {
        git_output(repo, ["diff", "--name-status", &base])?
    } else if staged {
        git_output(repo, ["diff", "--cached", "--name-status"])?
    } else {
        let mut output = git_output(repo, ["diff", "--name-status"])?;
        let untracked = git_output(repo, ["ls-files", "--others", "--exclude-standard"])?;
        for path in untracked.lines().filter(|line| !line.trim().is_empty()) {
            output.push_str("??\t");
            output.push_str(path);
            output.push('\n');
        }
        output
    };

    Ok(analyze_name_status(&output))
}

fn print_analysis(analysis: ChangeAnalysis) {
    println!("files: {}", analysis.files.len());
    println!("specs: {}", analysis.summary.specs);
    println!("docs: {}", analysis.summary.docs);
    println!("skills: {}", analysis.summary.skills);
    println!("evals: {}", analysis.summary.evals);
    println!("changelog: {}", analysis.summary.changelog);
    println!("cli: {}", analysis.summary.cli);
    println!("repo: {}", analysis.summary.repo);

    if !analysis.files.is_empty() {
        println!();
        println!("changed files:");
        for file in analysis.files {
            println!("- {} {} ({})", file.status, file.path, file.area.label());
        }
    }
}
