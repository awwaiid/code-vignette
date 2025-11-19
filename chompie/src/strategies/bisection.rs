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
            let non_blank_indices = state.non_blank_line_indices();
            if non_blank_indices.is_empty() {
                continue;
            }

            // Generate bisection ranges on non-blank lines
            // We bisect the count of non-blank lines, not the indices
            let count = non_blank_indices.len();
            let mut range_size = count / 2;

            while range_size > 0 {
                let mut start_idx = 0;
                while start_idx < count {
                    let end_idx = (start_idx + range_size).min(count);

                    // Convert from count-based indices to actual line numbers
                    let start_line = non_blank_indices[start_idx];
                    let end_line = non_blank_indices[end_idx - 1] + 1;

                    ranges.push(ChompRange {
                        file: path.clone(),
                        start_line,
                        end_line,
                    });
                    start_idx = end_idx;
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
