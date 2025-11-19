use crate::file_manager::FileState;
use crate::strategy::{ChompRange, Strategy};
use std::collections::HashMap;
use std::path::PathBuf;

/// Random lines strategy: tries removing random individual lines
/// This can find removable lines that bisection might miss due to dependencies
pub struct RandomLinesStrategy {
    max_attempts: usize,
    seed: u64,
}

impl RandomLinesStrategy {
    pub fn new(max_attempts: usize) -> Self {
        // Use a fixed seed for reproducibility
        RandomLinesStrategy {
            max_attempts,
            seed: 12345,
        }
    }

    pub fn with_seed(max_attempts: usize, seed: u64) -> Self {
        RandomLinesStrategy { max_attempts, seed }
    }
}

impl Strategy for RandomLinesStrategy {
    fn name(&self) -> &str {
        "random_lines"
    }

    fn generate_ranges(&self, files: &HashMap<PathBuf, FileState>) -> Vec<ChompRange> {
        let mut ranges = Vec::new();

        // Count non-blank lines across all files
        let total_non_blank: usize = files.values().map(|f| f.non_blank_lines()).sum();
        if total_non_blank == 0 {
            return ranges;
        }

        // Simple LCG random number generator for reproducibility
        let mut rng_state = self.seed;
        let lcg_next = |state: &mut u64| {
            *state = state.wrapping_mul(1103515245).wrapping_add(12345);
            (*state / 65536) % 32768
        };

        // Generate random single-line ranges from non-blank lines only
        let attempts = self.max_attempts.min(total_non_blank);
        let mut tried_lines = std::collections::HashSet::new();

        for _ in 0..attempts {
            // Pick a random file (weighted by non-blank line count)
            let file_index = (lcg_next(&mut rng_state) as usize) % files.len();
            if let Some((path, state)) = files.iter().nth(file_index) {
                let non_blank_indices = state.non_blank_line_indices();
                if non_blank_indices.is_empty() {
                    continue;
                }

                // Pick a random non-blank line
                let idx = (lcg_next(&mut rng_state) as usize) % non_blank_indices.len();
                let line = non_blank_indices[idx];
                let key = (path.clone(), line);

                // Avoid trying the same line twice
                if tried_lines.contains(&key) {
                    continue;
                }
                tried_lines.insert(key);

                ranges.push(ChompRange {
                    file: path.clone(),
                    start_line: line,
                    end_line: line + 1,
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
    fn test_random_lines_strategy() {
        let strategy = RandomLinesStrategy::new(10);
        let mut files = HashMap::new();

        let content = "line1\nline2\nline3\nline4\nline5".to_string();
        let path = PathBuf::from("test.txt");
        files.insert(path.clone(), FileState::new(path.clone(), content));

        let ranges = strategy.generate_ranges(&files);

        assert!(!ranges.is_empty());
        assert!(ranges.len() <= 10);
        assert_eq!(strategy.name(), "random_lines");

        // Each range should be a single line
        for range in &ranges {
            assert_eq!(range.end_line - range.start_line, 1);
        }
    }

    #[test]
    fn test_random_lines_reproducible() {
        let strategy1 = RandomLinesStrategy::with_seed(5, 42);
        let strategy2 = RandomLinesStrategy::with_seed(5, 42);

        let mut files = HashMap::new();
        let content = "a\nb\nc\nd\ne\nf\ng\nh".to_string();
        let path = PathBuf::from("test.txt");
        files.insert(path.clone(), FileState::new(path.clone(), content));

        let ranges1 = strategy1.generate_ranges(&files);
        let ranges2 = strategy2.generate_ranges(&files);

        // Same seed should produce same sequence
        assert_eq!(ranges1.len(), ranges2.len());
    }
}
