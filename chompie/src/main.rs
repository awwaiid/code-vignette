mod chomper;
mod command_runner;
mod file_manager;
mod progress;
mod strategies;
mod strategy;

use anyhow::{Context, Result};
use chomper::Chomper;
use clap::Parser;
use command_runner::CommandRunner;
use file_manager::FileManager;
use strategies::{BisectionStrategy, RandomLinesStrategy, RandomRangesStrategy, SlidingWindowStrategy};
use strategy::Strategy;
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

    /// Strategies to use (comma-separated: bisection,random_lines,random_ranges,sliding_window)
    #[arg(long, default_value = "bisection,random_lines,random_ranges")]
    strategies: String,

    /// Maximum attempts for random strategies
    #[arg(long, default_value = "100")]
    random_attempts: usize,

    /// Window size for sliding_window strategy
    #[arg(long, default_value = "1")]
    window_size: usize,
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

fn parse_strategies(strategies_str: &str, random_attempts: usize, window_size: usize) -> Result<Vec<Box<dyn Strategy>>> {
    let mut strategies: Vec<Box<dyn Strategy>> = Vec::new();

    for strategy_name in strategies_str.split(',') {
        let strategy_name = strategy_name.trim();
        match strategy_name {
            "bisection" => strategies.push(Box::new(BisectionStrategy)),
            "random_lines" => strategies.push(Box::new(RandomLinesStrategy::new(random_attempts))),
            "random_ranges" => strategies.push(Box::new(RandomRangesStrategy::new(random_attempts))),
            "sliding_window" => strategies.push(Box::new(SlidingWindowStrategy::new(window_size))),
            _ => anyhow::bail!("Unknown strategy: {}", strategy_name),
        }
    }

    if strategies.is_empty() {
        anyhow::bail!("No strategies specified");
    }

    Ok(strategies)
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

    // Parse strategies
    let strategies = parse_strategies(&args.strategies, args.random_attempts, args.window_size)?;
    println!("📋 Using {} strategies: {}",
        strategies.len(),
        strategies.iter().map(|s| s.name()).collect::<Vec<_>>().join(", ")
    );

    // Set up file manager
    println!("📁 Scanning directory: {}", args.directory);
    let mut file_manager = FileManager::new();
    file_manager
        .add_directory(&args.directory)
        .context("Failed to scan directory")?;

    let file_count = file_manager.files().len();
    let initial_lines = file_manager.non_blank_lines();

    println!("Found {} files with {} lines\n", file_count, initial_lines);

    if file_count == 0 {
        println!("No files to chomp!");
        return Ok(());
    }

    // Set up command runner
    let command_runner = CommandRunner::new(args.command.clone());

    // Create chomper
    let mut chomper = Chomper::new(file_manager, command_runner);

    // Establish baseline
    println!("🎯 Establishing baseline with command: '{}'", args.command);
    let baseline = chomper.establish_baseline()?;
    println!("Baseline established:");
    println!("  Exit code: {}", baseline.exit_code);
    println!("  Stdout length: {} chars", baseline.stdout.len());
    println!("  Stderr length: {} chars", baseline.stderr.len());
    println!();

    // Meta-strategy: rotate through all strategies until no more progress
    println!("🍽️  Starting multi-strategy chomping...\n");

    let mut round = 0;
    let mut total_successful = 0;
    let start_time = std::time::Instant::now();

    loop {
        round += 1;
        let mut round_successful = 0;

        println!("--- Round {} ---", round);

        for strategy in &strategies {
            println!("Trying strategy: {}", strategy.name());

            let successful = chomper.execute_strategy(strategy.as_ref())?;
            round_successful += successful;
            total_successful += successful;

            let current_lines = chomper.file_manager().non_blank_lines();
            println!("  Successful chomps: {} | Current lines: {}", successful, current_lines);
        }

        println!("Round {} complete: {} successful chomps\n", round, round_successful);

        // If no strategy made progress, we're done
        if round_successful == 0 {
            println!("✅ No more progress possible. Chomping complete!");
            break;
        }
    }

    // Final statistics
    let final_lines = chomper.file_manager().non_blank_lines();
    let elapsed = start_time.elapsed();
    let reduction_percent = if initial_lines > 0 {
        ((initial_lines - final_lines) as f64 / initial_lines as f64) * 100.0
    } else {
        0.0
    };

    println!("\n=== Final Results ===");
    println!("Initial lines: {}", initial_lines);
    println!("Final lines: {}", final_lines);
    println!("Reduction: {:.1}%", reduction_percent);
    println!("Total successful chomps: {}", total_successful);
    println!("Total chomps tested: {}", chomper.chomps_tested());
    println!("Rounds: {}", round);
    println!("Time elapsed: {}s", elapsed.as_secs());

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
