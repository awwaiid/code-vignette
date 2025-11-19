use std::time::Instant;

pub struct ProgressTracker {
    total_ranges: usize,
    chomps_performed: usize,
    successful_chomps: usize,
    start_time: Instant,
}

impl ProgressTracker {
    pub fn new(total_ranges: usize) -> Self {
        ProgressTracker {
            total_ranges,
            chomps_performed: 0,
            successful_chomps: 0,
            start_time: Instant::now(),
        }
    }

    pub fn record_chomp(&mut self, success: bool) {
        self.chomps_performed += 1;
        if success {
            self.successful_chomps += 1;
        }
    }

    pub fn display(&self) {
        let elapsed = self.start_time.elapsed();
        let elapsed_secs = elapsed.as_secs();

        let percent = if self.total_ranges > 0 {
            (self.chomps_performed as f64 / self.total_ranges as f64) * 100.0
        } else {
            0.0
        };

        println!(
            "Chomps: {}/{} ({:.1}%) | Successful: {} | Time: {}s",
            self.chomps_performed, self.total_ranges, percent, self.successful_chomps, elapsed_secs
        );
    }

    pub fn summary(&self, initial_lines: usize, final_lines: usize) {
        let elapsed = self.start_time.elapsed();
        let elapsed_secs = elapsed.as_secs();
        let reduction_percent = if initial_lines > 0 {
            ((initial_lines - final_lines) as f64 / initial_lines as f64) * 100.0
        } else {
            0.0
        };

        println!("\n=== Chomping Complete ===");
        println!("Initial lines: {}", initial_lines);
        println!("Final lines: {}", final_lines);
        println!("Reduction: {:.1}%", reduction_percent);
        println!("Total chomps: {}", self.chomps_performed);
        println!("Successful chomps: {}", self.successful_chomps);
        println!("Time elapsed: {}s", elapsed_secs);
    }

    pub fn chomps_performed(&self) -> usize {
        self.chomps_performed
    }

    pub fn successful_chomps(&self) -> usize {
        self.successful_chomps
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_progress_tracker_creation() {
        let tracker = ProgressTracker::new(10);
        assert_eq!(tracker.chomps_performed(), 0);
        assert_eq!(tracker.successful_chomps(), 0);
    }

    #[test]
    fn test_record_chomp() {
        let mut tracker = ProgressTracker::new(10);
        tracker.record_chomp(true);
        tracker.record_chomp(false);
        assert_eq!(tracker.chomps_performed(), 2);
        assert_eq!(tracker.successful_chomps(), 1);
    }
}
