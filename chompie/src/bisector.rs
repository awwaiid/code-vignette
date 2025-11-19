use crate::command_runner::{CommandRunner, RunResult};
use crate::file_manager::FileManager;
use anyhow::Result;
use std::collections::HashSet;
use std::path::PathBuf;

#[derive(Debug, Clone)]
pub struct ChompRange {
    pub file: PathBuf,
    pub start_line: usize,
    pub end_line: usize,
}

pub struct Bisector {
    file_manager: FileManager,
    command_runner: CommandRunner,
    baseline_result: Option<RunResult>,
    tested_states: HashSet<String>,
}

impl Bisector {
    pub fn new(file_manager: FileManager, command_runner: CommandRunner) -> Self {
        Bisector {
            file_manager,
            command_runner,
            baseline_result: None,
            tested_states: HashSet::new(),
        }
    }

    pub fn establish_baseline(&mut self) -> Result<RunResult> {
        let result = self.command_runner.run()?;
        self.baseline_result = Some(result.clone());
        Ok(result)
    }

    pub fn baseline_result(&self) -> Option<&RunResult> {
        self.baseline_result.as_ref()
    }

    fn get_state_key(&self) -> String {
        let mut keys: Vec<_> = self
            .file_manager
            .files()
            .iter()
            .map(|(path, state)| {
                let blanked: Vec<_> = state.blanked_lines.iter().copied().collect();
                format!("{:?}:{:?}", path, blanked)
            })
            .collect();
        keys.sort();
        keys.join("|")
    }

    fn is_state_tested(&self) -> bool {
        let key = self.get_state_key();
        self.tested_states.contains(&key)
    }

    fn mark_state_tested(&mut self) {
        let key = self.get_state_key();
        self.tested_states.insert(key);
    }

    pub fn try_blank_range(&mut self, range: &ChompRange) -> Result<bool> {
        // Check if we've already tested this state
        if self.is_state_tested() {
            return Ok(false);
        }

        // Blank the lines in the range
        let lines_to_blank: Vec<usize> = (range.start_line..range.end_line).collect();

        if let Some(file_state) = self.file_manager.get_file_mut(&range.file) {
            file_state.blank_lines(&lines_to_blank);
        } else {
            anyhow::bail!("File not found: {:?}", range.file);
        }

        // Write the changes
        self.file_manager.write_all()?;

        // Run the command
        let result = self.command_runner.run()?;

        // Mark this state as tested
        self.mark_state_tested();

        // Check if result matches baseline
        let matches = if let Some(baseline) = &self.baseline_result {
            result.is_identical(baseline)
        } else {
            false
        };

        // If it doesn't match, restore the lines
        if !matches {
            if let Some(file_state) = self.file_manager.get_file_mut(&range.file) {
                file_state.unblank_lines(&lines_to_blank);
            }
            self.file_manager.write_all()?;
        }

        Ok(matches)
    }

    pub fn generate_ranges(&self) -> Vec<ChompRange> {
        let mut ranges = Vec::new();

        for (path, state) in self.file_manager.files() {
            let total_lines = state.total_lines();
            if total_lines == 0 {
                continue;
            }

            // Generate bisection ranges
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

    pub fn file_manager(&self) -> &FileManager {
        &self.file_manager
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;
    use tempfile::TempDir;

    #[test]
    fn test_bisector_creation() {
        let manager = FileManager::new();
        let runner = CommandRunner::new("echo test".to_string());
        let bisector = Bisector::new(manager, runner);
        assert!(bisector.baseline_result().is_none());
    }

    #[test]
    fn test_establish_baseline() {
        let manager = FileManager::new();
        let runner = CommandRunner::new("echo hello".to_string());
        let mut bisector = Bisector::new(manager, runner);
        let result = bisector.establish_baseline().unwrap();
        assert_eq!(result.stdout.trim(), "hello");
        assert!(bisector.baseline_result().is_some());
    }

    #[test]
    fn test_generate_ranges() {
        let mut manager = FileManager::new();
        let content = "line1\nline2\nline3\nline4".to_string();
        let path = PathBuf::from("test.txt");
        manager.add_file_from_content(path.clone(), content);

        let runner = CommandRunner::new("echo test".to_string());
        let bisector = Bisector::new(manager, runner);
        let ranges = bisector.generate_ranges();

        // Should generate ranges for bisection
        assert!(!ranges.is_empty());
    }

    #[test]
    fn test_try_blank_range_matching() {
        let temp_dir = TempDir::new().unwrap();
        let file_path = temp_dir.path().join("test.txt");
        std::fs::write(&file_path, "line1\nline2\nline3").unwrap();

        let mut manager = FileManager::new();
        manager.add_file(&file_path).unwrap();

        // Command that doesn't depend on file content
        let runner = CommandRunner::new("echo constant".to_string());
        let mut bisector = Bisector::new(manager, runner);

        bisector.establish_baseline().unwrap();

        let range = ChompRange {
            file: file_path.clone(),
            start_line: 0,
            end_line: 1,
        };

        let result = bisector.try_blank_range(&range).unwrap();
        // Should match since command output is constant
        assert!(result);
    }
}
