mod bisector;
mod command_runner;
mod file_manager;
mod progress;

use anyhow::{Context, Result};
use bisector::Bisector;
use clap::Parser;
use command_runner::CommandRunner;
use file_manager::FileManager;
use progress::ProgressTracker;
use std::io::{self, Write};

#[derive(Parser, Debug)]
#[command(name = "chompie")]
#[command(about = "Minimize code to the smallest subset that produces the same output", long_about = None)]
struct Args {
    /// The command to run (e.g., 'cargo test', 'npm test')
    #[arg(value_name = "COMMAND")]
    command: String,

    /// Directory to chomp (defaults to current directory)
    #[arg(short, long, default_value = ".")]
    directory: String,

    /// Skip confirmation prompt (DANGEROUS!)
    #[arg(short = 'y', long)]
    yes: bool,
}

fn confirm_chomp() -> Result<bool> {
    print!("⚠️  WARNING: This will destructively modify files in the current directory!\n");
    print!("Make sure you have a backup or are using version control.\n");
    print!("Continue? [y/N]: ");
    io::stdout().flush()?;

    let mut input = String::new();
    io::stdin().read_line(&mut input)?;

    Ok(input.trim().eq_ignore_ascii_case("y"))
}

fn run_chomp(args: Args) -> Result<()> {
    // Confirm with user
    if !args.yes {
        if !confirm_chomp()? {
            println!("Chomping cancelled.");
            return Ok(());
        }
    }

    println!("🍴 Starting chomp process...\n");

    // Set up file manager
    println!("📁 Scanning directory: {}", args.directory);
    let mut file_manager = FileManager::new();
    file_manager
        .add_directory(&args.directory)
        .context("Failed to scan directory")?;

    let file_count = file_manager.files().len();
    let initial_lines = file_manager.total_lines();

    println!("Found {} files with {} total lines\n", file_count, initial_lines);

    if file_count == 0 {
        println!("No files to chomp!");
        return Ok(());
    }

    // Set up command runner
    let command_runner = CommandRunner::new(args.command.clone());

    // Create bisector
    let mut bisector = Bisector::new(file_manager, command_runner);

    // Establish baseline
    println!("🎯 Establishing baseline with command: '{}'", args.command);
    let baseline = bisector.establish_baseline()?;
    println!("Baseline established:");
    println!("  Exit code: {}", baseline.exit_code);
    println!("  Stdout length: {} chars", baseline.stdout.len());
    println!("  Stderr length: {} chars", baseline.stderr.len());
    println!();

    // Generate chomp ranges
    let ranges = bisector.generate_ranges();
    println!("Generated {} chomp ranges\n", ranges.len());

    // Set up progress tracker
    let mut progress = ProgressTracker::new(ranges.len());

    // Perform chomping
    println!("🍽️  Starting to chomp...\n");
    for (i, range) in ranges.iter().enumerate() {
        if i % 10 == 0 || i == ranges.len() - 1 {
            progress.display();
        }

        match bisector.try_blank_range(range) {
            Ok(success) => {
                progress.record_chomp(success);
            }
            Err(e) => {
                eprintln!("Error during chomp: {}", e);
                progress.record_chomp(false);
            }
        }
    }

    // Final statistics
    let final_lines = bisector.file_manager().non_blank_lines();
    progress.summary(initial_lines, final_lines);

    println!("\n✅ Chomping complete!");
    println!("Files have been modified in place.");

    Ok(())
}

fn main() {
    let args = Args::parse();

    if let Err(e) = run_chomp(args) {
        eprintln!("Error: {:#}", e);
        std::process::exit(1);
    }
}
