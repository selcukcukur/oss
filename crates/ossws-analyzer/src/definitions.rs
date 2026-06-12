use crate::types::{ChangeAnalysis, ChangeArea, ChangeSummary, ChangedFile};

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
