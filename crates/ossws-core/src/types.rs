use std::fmt;

#[derive(Clone, Debug)]
pub enum CommitKind {
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

#[derive(Debug, Default)]
pub struct StatusCounts {
    pub staged: usize,
    pub modified: usize,
    pub untracked: usize,
}

impl StatusCounts {
    pub fn from_porcelain(status: &str) -> Self {
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
