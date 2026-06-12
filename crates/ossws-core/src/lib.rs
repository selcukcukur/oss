mod definitions;
mod types;

pub use definitions::{
    is_iso_date, normalize_subject, normalize_token, validate_commit_subject,
    validate_release_heading, validate_token,
};
pub use types::{CommitKind, StatusCounts};

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
