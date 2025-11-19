use anyhow::Result;
use std::process::Command;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RunResult {
    pub stdout: String,
    pub stderr: String,
    pub exit_code: i32,
}

impl RunResult {
    pub fn is_identical(&self, other: &RunResult) -> bool {
        self.stdout == other.stdout && self.stderr == other.stderr && self.exit_code == other.exit_code
    }
}

pub struct CommandRunner {
    command: String,
    verbose: bool,
}

impl CommandRunner {
    pub fn new(command: String) -> Self {
        CommandRunner { command, verbose: false }
    }

    pub fn with_verbose(command: String, verbose: bool) -> Self {
        CommandRunner { command, verbose }
    }

    pub fn run(&self) -> Result<RunResult> {
        if self.verbose {
            println!("      🔧 Running command: {}", self.command);
        }

        let output = if cfg!(target_os = "windows") {
            Command::new("cmd")
                .args(["/C", &self.command])
                .output()?
        } else {
            Command::new("sh")
                .arg("-c")
                .arg(&self.command)
                .output()?
        };

        let result = RunResult {
            stdout: String::from_utf8_lossy(&output.stdout).to_string(),
            stderr: String::from_utf8_lossy(&output.stderr).to_string(),
            exit_code: output.status.code().unwrap_or(-1),
        };

        if self.verbose {
            println!("      ✓ Exit code: {}", result.exit_code);
            if !result.stdout.is_empty() {
                println!("      📤 Stdout ({} bytes):", result.stdout.len());
                for line in result.stdout.lines().take(10) {
                    println!("         {}", line);
                }
                if result.stdout.lines().count() > 10 {
                    println!("         ... ({} more lines)", result.stdout.lines().count() - 10);
                }
            }
            if !result.stderr.is_empty() {
                println!("      📤 Stderr ({} bytes):", result.stderr.len());
                for line in result.stderr.lines().take(10) {
                    println!("         {}", line);
                }
                if result.stderr.lines().count() > 10 {
                    println!("         ... ({} more lines)", result.stderr.lines().count() - 10);
                }
            }
        }

        Ok(result)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_run_simple_command() {
        let runner = CommandRunner::new("echo hello".to_string());
        let result = runner.run().unwrap();
        assert_eq!(result.stdout.trim(), "hello");
        assert_eq!(result.exit_code, 0);
    }

    #[test]
    fn test_run_failing_command() {
        let runner = CommandRunner::new("exit 42".to_string());
        let result = runner.run().unwrap();
        assert_eq!(result.exit_code, 42);
    }

    #[test]
    fn test_result_identical() {
        let result1 = RunResult {
            stdout: "test".to_string(),
            stderr: "".to_string(),
            exit_code: 0,
        };
        let result2 = RunResult {
            stdout: "test".to_string(),
            stderr: "".to_string(),
            exit_code: 0,
        };
        assert!(result1.is_identical(&result2));
    }

    #[test]
    fn test_result_not_identical() {
        let result1 = RunResult {
            stdout: "test1".to_string(),
            stderr: "".to_string(),
            exit_code: 0,
        };
        let result2 = RunResult {
            stdout: "test2".to_string(),
            stderr: "".to_string(),
            exit_code: 0,
        };
        assert!(!result1.is_identical(&result2));
    }
}
