use crate::file_manager::FileState;
use crate::strategy::{ChompRange, Strategy};
use std::collections::HashMap;
use std::path::PathBuf;

/// Bisection strategy: systematically tries removing halves, quarters, eighths, etc.
pub struct BisectionStrategy;

impl Strategy for BisectionStrategy {
    fn name(&self) -> &str {
        "bisection"
    }

    fn generate_ranges(&self, files: &HashMap<PathBuf, FileState>) -> Vec<ChompRange> {
        let mut ranges = Vec::new();

        for (path, state) in files {
            let total_lines = state.total_lines();
            if total_lines == 0 {
                continue;
            }

            // Generate bisection ranges: halves, then quarters, then eighths, etc.
            let mut range_size = total_lines / 2;
            while range_size > 0 {
                let mut start = 0;
                while start < total_lines {
                    let end = (start + range_size).min(total_lines);
                    ranges.push(ChompRange {
                        file: path.clone(),
                        start_line: start,
                        end_line: end,
                    });
                    start = end;
                }
                range_size /= 2;
            }
        }

        ranges
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::file_manager::FileState;

    #[test]
    fn test_bisection_strategy() {
        let strategy = BisectionStrategy;
        let mut files = HashMap::new();

        let content = "line1\nline2\nline3\nline4".to_string();
        let path = PathBuf::from("test.txt");
        files.insert(path.clone(), FileState::new(path.clone(), content));

        let ranges = strategy.generate_ranges(&files);

        // Should generate ranges for: halves (2 ranges), quarters (4 ranges)
        assert!(!ranges.is_empty());
        assert_eq!(strategy.name(), "bisection");
    }
}
