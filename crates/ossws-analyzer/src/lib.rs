#[derive(Debug)]
pub struct ChangeAnalysis {
    pub files: Vec<ChangedFile>,
    pub summary: ChangeSummary,
}

#[derive(Debug)]
pub struct ChangedFile {
    pub status: String,
    pub path: String,
    pub area: ChangeArea,
}

#[derive(Clone, Copy, Debug)]
pub enum ChangeArea {
    Specs,
    Docs,
    Skills,
    Evals,
    Changelog,
    Cli,
    Repo,
}

impl ChangeArea {
    pub fn from_path(path: &str) -> Self {
        if path == "changelog.md" {
            Self::Changelog
        } else if path.starts_with("specs/") {
            Self::Specs
        } else if path.starts_with("docs/") {
            Self::Docs
        } else if path.contains("/evals/") {
            Self::Evals
        } else if path.starts_with("skills/") {
            Self::Skills
        } else if path.starts_with("src/")
            || path.starts_with("crates/")
            || path == "Cargo.toml"
            || path == "Cargo.lock"
        {
            Self::Cli
        } else {
            Self::Repo
        }
    }

    pub fn label(self) -> &'static str {
        match self {
            Self::Specs => "specs",
            Self::Docs => "docs",
            Self::Skills => "skills",
            Self::Evals => "evals",
            Self::Changelog => "changelog",
            Self::Cli => "cli",
            Self::Repo => "repo",
        }
    }
}

#[derive(Debug, Default)]
pub struct ChangeSummary {
    pub specs: usize,
    pub docs: usize,
    pub skills: usize,
    pub evals: usize,
    pub changelog: usize,
    pub cli: usize,
    pub repo: usize,
}

impl ChangeSummary {
    fn add(&mut self, area: ChangeArea) {
        match area {
            ChangeArea::Specs => self.specs += 1,
            ChangeArea::Docs => self.docs += 1,
            ChangeArea::Skills => self.skills += 1,
            ChangeArea::Evals => self.evals += 1,
            ChangeArea::Changelog => self.changelog += 1,
            ChangeArea::Cli => self.cli += 1,
            ChangeArea::Repo => self.repo += 1,
        }
    }
}

pub fn analyze_name_status(output: &str) -> ChangeAnalysis {
    let mut files = Vec::new();
    let mut summary = ChangeSummary::default();

    for line in output.lines() {
        if let Some((status, path)) = line.split_once('\t') {
            let path = path.replace('\\', "/");
            let area = ChangeArea::from_path(&path);
            summary.add(area);
            files.push(ChangedFile {
                status: status.to_string(),
                path,
                area,
            });
        }
    }

    files.sort_by(|left, right| left.path.cmp(&right.path));

    ChangeAnalysis { files, summary }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn classifies_changed_paths() {
        assert!(matches!(
            ChangeArea::from_path("specs/oss-cli.md"),
            ChangeArea::Specs
        ));
        assert!(matches!(
            ChangeArea::from_path("docs/cli-guide.md"),
            ChangeArea::Docs
        ));
        assert!(matches!(
            ChangeArea::from_path("skills/oss-ws/evals/evals.json"),
            ChangeArea::Evals
        ));
        assert!(matches!(
            ChangeArea::from_path("crates/ossws/src/main.rs"),
            ChangeArea::Cli
        ));
        assert!(matches!(
            ChangeArea::from_path("changelog.md"),
            ChangeArea::Changelog
        ));
    }

    #[test]
    fn analyzes_name_status_output() {
        let analysis = analyze_name_status("M\tchangelog.md\nA\tcrates/ossws/src/main.rs\n");
        assert_eq!(analysis.files.len(), 2);
        assert_eq!(analysis.summary.changelog, 1);
        assert_eq!(analysis.summary.cli, 1);
    }
}
