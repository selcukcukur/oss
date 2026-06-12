mod definitions;
mod types;

pub use definitions::analyze_name_status;
pub use types::{ChangeAnalysis, ChangeArea, ChangeSummary, ChangedFile};

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
