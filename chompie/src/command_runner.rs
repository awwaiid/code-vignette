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
}

impl CommandRunner {
    pub fn new(command: String) -> Self {
        CommandRunner { command }
    }

    pub fn run(&self) -> Result<RunResult> {
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

        Ok(RunResult {
            stdout: String::from_utf8_lossy(&output.stdout).to_string(),
            stderr: String::from_utf8_lossy(&output.stderr).to_string(),
            exit_code: output.status.code().unwrap_or(-1),
        })
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
