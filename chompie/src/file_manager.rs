use anyhow::{Context, Result};
use std::collections::{HashMap, HashSet};
use std::fs;
use std::path::{Path, PathBuf};

#[derive(Debug, Clone)]
pub struct FileState {
    pub path: PathBuf,
    pub original_lines: Vec<String>,
    pub blanked_lines: HashSet<usize>,
}

impl FileState {
    pub fn new(path: PathBuf, content: String) -> Self {
        let original_lines: Vec<String> = content.lines().map(|s| s.to_string()).collect();
        FileState {
            path,
            original_lines,
            blanked_lines: HashSet::new(),
        }
    }

    pub fn blank_lines(&mut self, lines: &[usize]) {
        for &line in lines {
            if line < self.original_lines.len() {
                self.blanked_lines.insert(line);
            }
        }
    }

    pub fn unblank_lines(&mut self, lines: &[usize]) {
        for &line in lines {
            self.blanked_lines.remove(&line);
        }
    }

    pub fn current_content(&self) -> String {
        self.original_lines
            .iter()
            .enumerate()
            .map(|(i, line)| {
                if self.blanked_lines.contains(&i) {
                    String::new()
                } else {
                    line.clone()
                }
            })
            .collect::<Vec<_>>()
            .join("\n")
    }

    pub fn total_lines(&self) -> usize {
        self.original_lines.len()
    }

    pub fn non_blank_lines(&self) -> usize {
        self.original_lines.len() - self.blanked_lines.len()
    }

    /// Get list of line indices that are not currently blanked
    pub fn non_blank_line_indices(&self) -> Vec<usize> {
        (0..self.original_lines.len())
            .filter(|i| !self.blanked_lines.contains(i))
            .collect()
    }
}

pub struct FileManager {
    files: HashMap<PathBuf, FileState>,
}

impl FileManager {
    pub fn new() -> Self {
        FileManager {
            files: HashMap::new(),
        }
    }

    pub fn add_file<P: AsRef<Path>>(&mut self, path: P) -> Result<()> {
        let path = path.as_ref().to_path_buf();
        let content = fs::read_to_string(&path)
            .with_context(|| format!("Failed to read file: {:?}", path))?;
        self.files.insert(path.clone(), FileState::new(path, content));
        Ok(())
    }

    pub fn add_directory<P: AsRef<Path>>(&mut self, dir: P) -> Result<()> {
        let dir = dir.as_ref();
        if !dir.is_dir() {
            anyhow::bail!("Not a directory: {:?}", dir);
        }

        self.visit_directory(dir)?;
        Ok(())
    }

    fn visit_directory(&mut self, dir: &Path) -> Result<()> {
        for entry in fs::read_dir(dir)? {
            let entry = entry?;
            let path = entry.path();

            // Skip hidden files/directories and common build artifacts
            if let Some(name) = path.file_name() {
                let name_str = name.to_string_lossy();
                if name_str.starts_with('.') || name_str == "target" || name_str == "node_modules" {
                    continue;
                }
            }

            if path.is_dir() {
                self.visit_directory(&path)?;
            } else if path.is_file() {
                // Only add text files (simple heuristic: common code extensions)
                if let Some(ext) = path.extension() {
                    let ext = ext.to_string_lossy();
                    if matches!(
                        ext.as_ref(),
                        "rs" | "py" | "js" | "ts" | "java" | "c" | "cpp" | "h" | "rb" | "go"
                    ) {
                        self.add_file(&path)?;
                    }
                }
            }
        }
        Ok(())
    }

    pub fn write_all(&self) -> Result<()> {
        for file_state in self.files.values() {
            let content = file_state.current_content();
            fs::write(&file_state.path, content)
                .with_context(|| format!("Failed to write file: {:?}", file_state.path))?;
        }
        Ok(())
    }

    pub fn restore_all(&self) -> Result<()> {
        for file_state in self.files.values() {
            let content = file_state.original_lines.join("\n");
            fs::write(&file_state.path, content)
                .with_context(|| format!("Failed to restore file: {:?}", file_state.path))?;
        }
        Ok(())
    }

    pub fn get_file_mut(&mut self, path: &Path) -> Option<&mut FileState> {
        self.files.get_mut(path)
    }

    pub fn files(&self) -> &HashMap<PathBuf, FileState> {
        &self.files
    }

    pub fn total_lines(&self) -> usize {
        self.files.values().map(|f| f.total_lines()).sum()
    }

    pub fn non_blank_lines(&self) -> usize {
        self.files.values().map(|f| f.non_blank_lines()).sum()
    }

    #[cfg(test)]
    pub fn add_file_from_content(&mut self, path: PathBuf, content: String) {
        self.files.insert(path.clone(), FileState::new(path, content));
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use tempfile::TempDir;

    #[test]
    fn test_file_state_creation() {
        let content = "line1\nline2\nline3".to_string();
        let state = FileState::new(PathBuf::from("test.txt"), content);
        assert_eq!(state.total_lines(), 3);
        assert_eq!(state.non_blank_lines(), 3);
    }

    #[test]
    fn test_file_state_blank_lines() {
        let content = "line1\nline2\nline3\nline4".to_string();
        let mut state = FileState::new(PathBuf::from("test.txt"), content);
        state.blank_lines(&[0, 2]);
        assert_eq!(state.non_blank_lines(), 2);
        let output = state.current_content();
        assert_eq!(output, "\nline2\n\nline4");
    }

    #[test]
    fn test_file_state_unblank_lines() {
        let content = "line1\nline2\nline3".to_string();
        let mut state = FileState::new(PathBuf::from("test.txt"), content);
        state.blank_lines(&[0, 1, 2]);
        state.unblank_lines(&[1]);
        assert_eq!(state.non_blank_lines(), 1);
    }

    #[test]
    fn test_file_manager_add_file() {
        let temp_dir = TempDir::new().unwrap();
        let file_path = temp_dir.path().join("test.txt");
        fs::write(&file_path, "content").unwrap();

        let mut manager = FileManager::new();
        manager.add_file(&file_path).unwrap();
        assert_eq!(manager.files().len(), 1);
    }

    #[test]
    fn test_file_manager_write_blanked() {
        let temp_dir = TempDir::new().unwrap();
        let file_path = temp_dir.path().join("test.txt");
        fs::write(&file_path, "line1\nline2\nline3").unwrap();

        let mut manager = FileManager::new();
        manager.add_file(&file_path).unwrap();

        let file_state = manager.get_file_mut(&file_path).unwrap();
        file_state.blank_lines(&[1]);

        manager.write_all().unwrap();

        let content = fs::read_to_string(&file_path).unwrap();
        assert_eq!(content, "line1\n\nline3");
    }

    #[test]
    fn test_file_manager_restore() {
        let temp_dir = TempDir::new().unwrap();
        let file_path = temp_dir.path().join("test.txt");
        let original = "line1\nline2\nline3";
        fs::write(&file_path, original).unwrap();

        let mut manager = FileManager::new();
        manager.add_file(&file_path).unwrap();

        let file_state = manager.get_file_mut(&file_path).unwrap();
        file_state.blank_lines(&[0, 1, 2]);
        manager.write_all().unwrap();

        manager.restore_all().unwrap();

        let content = fs::read_to_string(&file_path).unwrap();
        assert_eq!(content, original);
    }
}
