use crate::file_manager::FileState;
use crate::strategy::{ChompRange, Strategy};
use std::collections::HashMap;
use std::path::PathBuf;

/// Sliding window strategy: exhaustively tries removing windows of N consecutive lines
///
/// For window_size=1: tries every individual line (exhaustive single-line removal)
/// For window_size=2: tries every pair of consecutive lines
/// For window_size=3: tries every triple of consecutive lines, etc.
///
/// This is more exhaustive than bisection or random, but can be slow for large files.
pub struct SlidingWindowStrategy {
    window_size: usize,
}

impl SlidingWindowStrategy {
    pub fn new(window_size: usize) -> Self {
        SlidingWindowStrategy {
            window_size: window_size.max(1),
        }
    }
}

impl Strategy for SlidingWindowStrategy {
    fn name(&self) -> &str {
        "sliding_window"
    }

    fn generate_ranges(&self, files: &HashMap<PathBuf, FileState>) -> Vec<ChompRange> {
        let mut ranges = Vec::new();

        for (path, state) in files {
            let non_blank_indices = state.non_blank_line_indices();
            if non_blank_indices.is_empty() {
                continue;
            }

            let count = non_blank_indices.len();
            if count < self.window_size {
                // If fewer non-blank lines than window size, try removing all of them
                if count > 0 {
                    ranges.push(ChompRange {
                        file: path.clone(),
                        start_line: non_blank_indices[0],
                        end_line: non_blank_indices[count - 1] + 1,
                    });
                }
                continue;
            }

            // Slide a window of size window_size across the non-blank lines
            for start_idx in 0..=(count - self.window_size) {
                let end_idx = start_idx + self.window_size;

                // Convert from indices in non-blank list to actual line numbers
                let start_line = non_blank_indices[start_idx];
                let end_line = non_blank_indices[end_idx - 1] + 1;

                ranges.push(ChompRange {
                    file: path.clone(),
                    start_line,
                    end_line,
                });
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
    fn test_sliding_window_size_1() {
        let strategy = SlidingWindowStrategy::new(1);
        let mut files = HashMap::new();

        let content = "line1\nline2\nline3\nline4".to_string();
        let path = PathBuf::from("test.txt");
        files.insert(path.clone(), FileState::new(path.clone(), content));

        let ranges = strategy.generate_ranges(&files);

        // Should generate 4 ranges (one for each line)
        assert_eq!(ranges.len(), 4);
        assert_eq!(strategy.name(), "sliding_window");

        // Each range should be a single line
        for (i, range) in ranges.iter().enumerate() {
            assert_eq!(range.start_line, i);
            assert_eq!(range.end_line, i + 1);
        }
    }

    #[test]
    fn test_sliding_window_size_2() {
        let strategy = SlidingWindowStrategy::new(2);
        let mut files = HashMap::new();

        let content = "line1\nline2\nline3\nline4".to_string();
        let path = PathBuf::from("test.txt");
        files.insert(path.clone(), FileState::new(path.clone(), content));

        let ranges = strategy.generate_ranges(&files);

        // Should generate 3 ranges: [0-2), [1-3), [2-4)
        assert_eq!(ranges.len(), 3);

        assert_eq!(ranges[0].start_line, 0);
        assert_eq!(ranges[0].end_line, 2);

        assert_eq!(ranges[1].start_line, 1);
        assert_eq!(ranges[1].end_line, 3);

        assert_eq!(ranges[2].start_line, 2);
        assert_eq!(ranges[2].end_line, 4);
    }

    #[test]
    fn test_sliding_window_size_3() {
        let strategy = SlidingWindowStrategy::new(3);
        let mut files = HashMap::new();

        let content = "a\nb\nc\nd\ne".to_string();
        let path = PathBuf::from("test.txt");
        files.insert(path.clone(), FileState::new(path.clone(), content));

        let ranges = strategy.generate_ranges(&files);

        // Should generate 3 ranges: [0-3), [1-4), [2-5)
        assert_eq!(ranges.len(), 3);
    }

    #[test]
    fn test_sliding_window_with_blanked_lines() {
        let strategy = SlidingWindowStrategy::new(1);
        let mut files = HashMap::new();

        let content = "line1\nline2\nline3\nline4".to_string();
        let path = PathBuf::from("test.txt");
        let mut state = FileState::new(path.clone(), content);

        // Blank line 1 (index 1)
        state.blank_lines(&[1]);

        files.insert(path.clone(), state);

        let ranges = strategy.generate_ranges(&files);

        // Should only generate ranges for non-blank lines (3 ranges)
        assert_eq!(ranges.len(), 3);

        // Should skip the blanked line (index 1)
        assert!(ranges.iter().all(|r| r.start_line != 1));
    }
}
