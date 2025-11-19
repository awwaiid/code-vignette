# 🍽️ Chompie

**Minimize code to the smallest subset that produces the same output**

Chompie is a code minimization tool that systematically reduces your codebase to the minimal set of lines needed to produce a specific output or pass specific tests. It's perfect for creating minimal reproducible examples, understanding code dependencies, or debugging complex issues.

## 🚀 How It Works

Chompie uses a bisection algorithm to systematically blank out lines of code:

1. **Establish Baseline**: Runs your command once to capture the expected output
2. **Generate Ranges**: Creates bisection ranges across all files
3. **Systematic Chomping**: Tries blanking each range, keeping changes that maintain identical output
4. **Progress Tracking**: Shows real-time progress and statistics

The algorithm blanks lines instead of deleting them to preserve line numbers, which is crucial for maintaining stack traces and error messages.

## 📦 Installation

```bash
cargo install --path .
```

Or build from source:

```bash
cargo build --release
# Binary will be at ./target/release/chompie
```

## 🎯 Usage

### Basic Usage

```bash
chompie "cargo test"
```

This will:
1. Ask for confirmation (it's destructive!)
2. Scan the current directory for source files
3. Run `cargo test` to establish baseline
4. Systematically chomp down the code while maintaining test output

### Advanced Options

```bash
# Specify directory to chomp
chompie -d src "cargo test"

# Skip confirmation (DANGEROUS!)
chompie -y "npm test"

# Different commands
chompie "python -m pytest tests/test_feature.py"
chompie "go test ./..."
chompie "cargo check && cargo test items"
```

### Command-line Options

```
Options:
  -d, --directory <DIRECTORY>  Directory to chomp (defaults to current directory)
  -y, --yes                    Skip confirmation prompt (DANGEROUS!)
  -h, --help                   Print help
```

## ⚠️ Important Warnings

**CHOMPIE IS DESTRUCTIVE!**

- It will modify files in place
- Always use version control (git) before running
- Or work on a copy of your code
- The tool asks for confirmation by default

Example safe workflow:

```bash
# Create a copy to work on
cp -r my-project my-project-chomp
cd my-project-chomp

# Run chompie
chompie "cargo test"

# Check the results
git diff
```

## 📊 Example Output

```
🍴 Starting chomp process...

📁 Scanning directory: src
Found 5 files with 434 total lines

🎯 Establishing baseline with command: 'cargo test --quiet'
Baseline established:
  Exit code: 0
  Stdout length: 131 chars
  Stderr length: 0 chars

Generated 751 chomp ranges

🍽️  Starting to chomp...

Chomps: 750/751 (99.9%) | Successful: 145 | Time: 13s

=== Chomping Complete ===
Initial lines: 434
Final lines: 289
Reduction: 33.4%
Total chomps: 751
Successful chomps: 145
Time elapsed: 13s

✅ Chomping complete!
```

## 🎮 Example Project

The repository includes a blackjack game implementation in `examples/blackjack/` that you can practice on:

```bash
# Make a copy to test on
cp -r examples/blackjack test-blackjack
cd test-blackjack

# Run chompie to minimize while keeping tests passing
../target/release/chompie -y -d src "cargo test --quiet 2>&1"

# See what got chomped
git diff src/
```

## 🏗️ Architecture

Chompie is built with a clean, testable architecture:

- **`command_runner.rs`**: Executes commands and captures output
- **`file_manager.rs`**: Manages file state and blanking operations
- **`bisector.rs`**: Core bisection algorithm
- **`progress.rs`**: Progress tracking and display
- **`main.rs`**: CLI interface

All modules are thoroughly tested with unit tests.

## 🧪 Running Tests

```bash
cargo test
```

All core functionality is tested, including:
- Command execution and output capture
- File blanking and restoration
- Bisection algorithm
- Progress tracking

## 💡 Use Cases

1. **Minimal Reproducible Examples**: Reduce failing test to minimal code
2. **Bug Isolation**: Find the exact lines causing an issue
3. **Code Understanding**: See what code is actually necessary
4. **Test Coverage**: Identify dead code in tested modules

## 🔮 Future Enhancements

Potential improvements:

- **Combination Testing**: Try removing multiple ranges together
- **Smart Ordering**: Use heuristics to prioritize likely-removable code
- **Backup/Restore**: Built-in backup before chomping
- **Incremental**: Resume from previous chomp session
- **Parallel Execution**: Run tests in parallel for faster chomping
- **Delete Mode**: Actually remove lines instead of just blanking

## 📝 Technical Details

### File Selection

Currently supports these extensions:
- Rust: `.rs`
- Python: `.py`
- JavaScript/TypeScript: `.js`, `.ts`
- Java: `.java`
- C/C++: `.c`, `.cpp`, `.h`
- Ruby: `.rb`
- Go: `.go`

### Skipped Directories

- Hidden directories (starting with `.`)
- `target/` (Rust build artifacts)
- `node_modules/` (Node.js dependencies)

### Output Matching

Considers output identical when ALL of these match:
- Standard output (stdout)
- Standard error (stderr)
- Exit code

## 📄 License

See LICENSE file in repository root.

## 🤝 Contributing

This is an experimental vignette project. Feel free to:
- Report issues
- Suggest improvements
- Submit pull requests

---

**Remember**: Always use version control or work on copies. Chompie is destructive by design! 🍴
