use crate::file_manager::FileState;
use crate::strategy::{ChompRange, Strategy};
use std::collections::HashMap;
use std::path::PathBuf;

/// Random ranges strategy: tries removing random ranges of varying sizes
/// This can discover chunks of removable code that bisection might not find
pub struct RandomRangesStrategy {
    max_attempts: usize,
    seed: u64,
}

impl RandomRangesStrategy {
    pub fn new(max_attempts: usize) -> Self {
        RandomRangesStrategy {
            max_attempts,
            seed: 54321,
        }
    }

    pub fn with_seed(max_attempts: usize, seed: u64) -> Self {
        RandomRangesStrategy { max_attempts, seed }
    }
}

impl Strategy for RandomRangesStrategy {
    fn name(&self) -> &str {
        "random_ranges"
    }

    fn generate_ranges(&self, files: &HashMap<PathBuf, FileState>) -> Vec<ChompRange> {
        let mut ranges = Vec::new();

        if files.is_empty() {
            return ranges;
        }

        // Simple LCG random number generator
        let mut rng_state = self.seed;
        let lcg_next = |state: &mut u64| -> u64 {
            *state = state.wrapping_mul(1103515245).wrapping_add(12345);
            (*state / 65536) % 32768
        };

        for _ in 0..self.max_attempts {
            // Pick a random file
            let file_index = (lcg_next(&mut rng_state) as usize) % files.len();
            if let Some((path, state)) = files.iter().nth(file_index) {
                let non_blank_indices = state.non_blank_line_indices();
                if non_blank_indices.len() < 2 {
                    continue;
                }

                let count = non_blank_indices.len();

                // Pick random start index (in the non-blank list)
                let start_idx = (lcg_next(&mut rng_state) as usize) % count;

                // Pick random range size (1 to 25% of non-blank lines, at least 1)
                let max_size = (count / 4).max(1);
                let size = ((lcg_next(&mut rng_state) as usize) % max_size) + 1;

                let end_idx = (start_idx + size).min(count);

                if end_idx > start_idx {
                    // Convert to actual line numbers
                    let start_line = non_blank_indices[start_idx];
                    let end_line = non_blank_indices[end_idx - 1] + 1;

                    ranges.push(ChompRange {
                        file: path.clone(),
                        start_line,
                        end_line,
                    });
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
    fn test_random_ranges_strategy() {
        let strategy = RandomRangesStrategy::new(10);
        let mut files = HashMap::new();

        let content = (0..20).map(|i| format!("line{}", i)).collect::<Vec<_>>().join("\n");
        let path = PathBuf::from("test.txt");
        files.insert(path.clone(), FileState::new(path.clone(), content));

        let ranges = strategy.generate_ranges(&files);

        assert!(!ranges.is_empty());
        assert!(ranges.len() <= 10);
        assert_eq!(strategy.name(), "random_ranges");

        // Verify ranges are valid
        for range in &ranges {
            assert!(range.end_line > range.start_line);
            assert!(range.end_line <= 20);
        }
    }
}
