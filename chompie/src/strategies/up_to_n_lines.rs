use crate::file_manager::FileState;
use crate::strategy::{ChompRange, Strategy};
use std::collections::HashMap;
use std::path::PathBuf;

/// Up to N lines strategy: tries removing lines in blocks of increasing sizes
///
/// This strategy systematically tries to remove lines in blocks of size 1, then 2, then 3,
/// up to a maximum window size. For each window size, it uses overlapping windows that only
/// target non-blank lines.
///
/// For example, with max_window_size=3:
/// - Size 1: Try removing line 0, then line 1, then line 2, etc. (only non-blank lines)
/// - Size 2: Try removing lines [0,1], then [1,2], then [2,3], etc. (only non-blank lines)
/// - Size 3: Try removing lines [0,1,2], then [1,2,3], etc. (only non-blank lines)
pub struct UpToNLinesStrategy {
    max_window_size: usize,
}

impl UpToNLinesStrategy {
    pub fn new(max_window_size: usize) -> Self {
        UpToNLinesStrategy {
            max_window_size: max_window_size.max(1),
        }
    }
}

impl Strategy for UpToNLinesStrategy {
    fn name(&self) -> &str {
        "up_to_n_lines"
    }

    fn generate_ranges(&self, files: &HashMap<PathBuf, FileState>) -> Vec<ChompRange> {
        let mut ranges = Vec::new();

        // For each window size from 1 to max_window_size
        for window_size in 1..=self.max_window_size {
            // For each file
            for (path, state) in files {
                let non_blank_indices = state.non_blank_line_indices();

                if non_blank_indices.is_empty() {
                    continue;
                }

                // Generate overlapping windows of non-blank lines
                // For window_size=1: just each individual non-blank line
                // For window_size=2: consecutive pairs of non-blank lines
                // etc.
                if window_size <= non_blank_indices.len() {
                    for i in 0..=(non_blank_indices.len() - window_size) {
                        let start_line = non_blank_indices[i];
                        let end_line = non_blank_indices[i + window_size - 1] + 1;

                        ranges.push(ChompRange {
                            file: path.clone(),
                            start_line,
                            end_line,
                        });
                    }
                }
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
    fn test_up_to_n_lines_single_file() {
        let strategy = UpToNLinesStrategy::new(2);
        let mut files = HashMap::new();

        let content = "line1\nline2\nline3\nline4".to_string();
        let path = PathBuf::from("test.txt");
        files.insert(path.clone(), FileState::new(path.clone(), content));

        let ranges = strategy.generate_ranges(&files);

        assert_eq!(strategy.name(), "up_to_n_lines");

        // With window size 1-2 and 4 lines:
        // Size 1: [0-1], [1-2], [2-3], [3-4] = 4 ranges
        // Size 2: [0-2], [1-3], [2-4] = 3 ranges
        // Total: 7 ranges
        assert_eq!(ranges.len(), 7);

        // Check first few ranges
        assert_eq!(ranges[0].start_line, 0);
        assert_eq!(ranges[0].end_line, 1); // Window size 1

        assert_eq!(ranges[1].start_line, 1);
        assert_eq!(ranges[1].end_line, 2); // Window size 1

        assert_eq!(ranges[4].start_line, 0);
        assert_eq!(ranges[4].end_line, 2); // Window size 2
    }

    #[test]
    fn test_up_to_n_lines_with_blanked_lines() {
        let strategy = UpToNLinesStrategy::new(2);
        let mut files = HashMap::new();

        let content = "line1\nline2\nline3\nline4\nline5".to_string();
        let path = PathBuf::from("test.txt");
        let mut state = FileState::new(path.clone(), content);

        // Blank out line 1 (0-indexed), so we have lines 0, 2, 3, 4 as non-blank
        state.blank_lines(&[1]);

        files.insert(path.clone(), state);

        let ranges = strategy.generate_ranges(&files);

        // Non-blank lines are at indices: 0, 2, 3, 4 (4 lines)
        // Size 1: 4 ranges
        // Size 2: 3 ranges (pairs of consecutive non-blank lines)
        // Total: 7 ranges
        assert_eq!(ranges.len(), 7);

        // First range should be line 0 (first non-blank)
        assert_eq!(ranges[0].start_line, 0);
        assert_eq!(ranges[0].end_line, 1);

        // Second range should be line 2 (second non-blank, skipping blanked line 1)
        assert_eq!(ranges[1].start_line, 2);
        assert_eq!(ranges[1].end_line, 3);
    }

    #[test]
    fn test_up_to_n_lines_window_larger_than_file() {
        let strategy = UpToNLinesStrategy::new(10);
        let mut files = HashMap::new();

        let content = "line1\nline2\nline3".to_string();
        let path = PathBuf::from("test.txt");
        files.insert(path.clone(), FileState::new(path.clone(), content));

        let ranges = strategy.generate_ranges(&files);

        // With 3 lines and max window 10:
        // Size 1: 3 ranges
        // Size 2: 2 ranges
        // Size 3: 1 range
        // Total: 6 ranges
        assert_eq!(ranges.len(), 6);
    }

    #[test]
    fn test_up_to_n_lines_empty_file() {
        let strategy = UpToNLinesStrategy::new(3);
        let mut files = HashMap::new();

        let content = String::new();
        let path = PathBuf::from("test.txt");
        files.insert(path.clone(), FileState::new(path.clone(), content));

        let ranges = strategy.generate_ranges(&files);

        assert_eq!(ranges.len(), 0);
    }

    #[test]
    fn test_up_to_n_lines_multiple_files() {
        let strategy = UpToNLinesStrategy::new(2);
        let mut files = HashMap::new();

        let content1 = "a\nb".to_string();
        let path1 = PathBuf::from("test1.txt");
        files.insert(path1.clone(), FileState::new(path1.clone(), content1));

        let content2 = "x\ny\nz".to_string();
        let path2 = PathBuf::from("test2.txt");
        files.insert(path2.clone(), FileState::new(path2.clone(), content2));

        let ranges = strategy.generate_ranges(&files);

        // File 1: 2 lines
        //   Size 1: 2 ranges
        //   Size 2: 1 range
        // File 2: 3 lines
        //   Size 1: 3 ranges
        //   Size 2: 2 ranges
        // Total: 8 ranges
        assert_eq!(ranges.len(), 8);
    }

    #[test]
    fn test_up_to_n_lines_zero_window() {
        // Should default to at least 1
        let strategy = UpToNLinesStrategy::new(0);
        let mut files = HashMap::new();

        let content = "line1\nline2".to_string();
        let path = PathBuf::from("test.txt");
        files.insert(path.clone(), FileState::new(path.clone(), content));

        let ranges = strategy.generate_ranges(&files);

        // Should still try window size 1
        assert_eq!(ranges.len(), 2);
    }
}
