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
    pub(crate) fn add(&mut self, area: ChangeArea) {
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
