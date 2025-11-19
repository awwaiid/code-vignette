use crate::command_runner::{CommandRunner, RunResult};
use crate::file_manager::FileManager;
use crate::strategy::{ChompRange, Strategy};
use anyhow::Result;
use std::collections::HashSet;

/// The Chomper executes chomp attempts using any strategy
pub struct Chomper {
    file_manager: FileManager,
    command_runner: CommandRunner,
    baseline_result: Option<RunResult>,
    tested_states: HashSet<String>,
}

impl Chomper {
    pub fn new(file_manager: FileManager, command_runner: CommandRunner) -> Self {
        Chomper {
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

    /// Try to blank a range of lines and see if tests still pass
    /// Returns true if the range was successfully blanked
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

    /// Execute a strategy and return number of successful chomps
    pub fn execute_strategy(&mut self, strategy: &dyn Strategy) -> Result<usize> {
        let ranges = strategy.generate_ranges(self.file_manager.files());
        let mut successful = 0;

        for range in &ranges {
            match self.try_blank_range(range) {
                Ok(true) => successful += 1,
                Ok(false) => {},
                Err(e) => eprintln!("Error during chomp: {}", e),
            }
        }

        Ok(successful)
    }

    pub fn file_manager(&self) -> &FileManager {
        &self.file_manager
    }

    pub fn chomps_tested(&self) -> usize {
        self.tested_states.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::strategies::BisectionStrategy;
    use std::path::PathBuf;
    use tempfile::TempDir;

    #[test]
    fn test_chomper_creation() {
        let manager = crate::file_manager::FileManager::new();
        let runner = CommandRunner::new("echo test".to_string());
        let chomper = Chomper::new(manager, runner);
        assert!(chomper.baseline_result().is_none());
    }

    #[test]
    fn test_establish_baseline() {
        let manager = crate::file_manager::FileManager::new();
        let runner = CommandRunner::new("echo hello".to_string());
        let mut chomper = Chomper::new(manager, runner);
        let result = chomper.establish_baseline().unwrap();
        assert_eq!(result.stdout.trim(), "hello");
        assert!(chomper.baseline_result().is_some());
    }

    #[test]
    fn test_execute_strategy() {
        let temp_dir = TempDir::new().unwrap();
        let file_path = temp_dir.path().join("test.txt");
        std::fs::write(&file_path, "line1\nline2\nline3").unwrap();

        let mut manager = crate::file_manager::FileManager::new();
        manager.add_file(&file_path).unwrap();

        // Command that doesn't depend on file content
        let runner = CommandRunner::new("echo constant".to_string());
        let mut chomper = Chomper::new(manager, runner);

        chomper.establish_baseline().unwrap();

        let strategy = BisectionStrategy;
        let successful = chomper.execute_strategy(&strategy).unwrap();

        // Should successfully chomp since command output is constant
        assert!(successful > 0);
    }
}
