use std::path::PathBuf;

/// Represents a range of lines to attempt chomping in a file
#[derive(Debug, Clone)]
pub struct ChompRange {
    pub file: PathBuf,
    pub start_line: usize,
    pub end_line: usize,
}

/// A strategy for generating chomp attempts
pub trait Strategy {
    /// Name of this strategy for display purposes
    fn name(&self) -> &str;

    /// Generate chomp ranges to try
    /// Returns a list of ranges to attempt, in order
    fn generate_ranges(&self, files: &std::collections::HashMap<PathBuf, crate::file_manager::FileState>) -> Vec<ChompRange>;
}
