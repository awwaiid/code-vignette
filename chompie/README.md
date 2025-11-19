# 🍽️ Chompie

**Minimize code to the smallest subset that produces the same output**

Chompie is a code minimization tool that systematically reduces your codebase to the minimal set of lines needed to produce a specific output or pass specific tests. It's perfect for creating minimal reproducible examples, understanding code dependencies, or debugging complex issues.

## 🚀 How It Works

Chompie uses multiple strategies to systematically blank out lines of code:

1. **Establish Baseline**: Runs your command once to capture the expected output
2. **Apply Strategies**: Tries different chomping strategies (bisection, random lines, random ranges)
3. **Meta-Strategy**: Rotates through all strategies until no more progress
4. **Systematic Chomping**: Tries blanking ranges, keeping changes that maintain identical output

The algorithm blanks lines instead of deleting them to preserve line numbers, which is crucial for maintaining stack traces and error messages.

### Chomping Strategies

- **Bisection**: Systematically tries removing halves, quarters, eighths, etc.
- **Random Lines**: Randomly tries removing individual lines
- **Random Ranges**: Tries removing random ranges of varying sizes (1-25% of file)

The meta-strategy orchestrator rotates through all strategies until a full round produces zero successful chomps, ensuring maximum code reduction.

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

# Use specific strategies
chompie --strategies bisection "cargo test"
chompie --strategies random_lines --random-attempts 200 "cargo test"
chompie --strategies bisection,random_lines,random_ranges "cargo test"

# Different commands
chompie "python -m pytest tests/test_feature.py"
chompie "go test ./..."
chompie "cargo check && cargo test items"
```

### Command-line Options

```
Options:
  -d, --directory <DIRECTORY>         Directory to chomp (defaults to current directory)
  -y, --yes                          Skip confirmation prompt (DANGEROUS!)
  --strategies <STRATEGIES>          Strategies to use (comma-separated)
                                      [default: bisection,random_lines,random_ranges]
                                      Options: bisection, random_lines, random_ranges
  --random-attempts <NUM>            Max attempts for random strategies [default: 100]
  -h, --help                         Print help
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

📋 Using 3 strategies: bisection, random_lines, random_ranges
📁 Scanning directory: src
Found 5 files with 434 total lines

🎯 Establishing baseline with command: 'cargo test --quiet'
Baseline established:
  Exit code: 0
  Stdout length: 114 chars
  Stderr length: 0 chars

🍽️  Starting multi-strategy chomping...

--- Round 1 ---
Trying strategy: bisection
  Successful chomps: 1 | Current lines: 430
Trying strategy: random_lines
  Successful chomps: 0 | Current lines: 430
Trying strategy: random_ranges
  Successful chomps: 0 | Current lines: 430
Round 1 complete: 1 successful chomps

--- Round 2 ---
Trying strategy: bisection
  Successful chomps: 0 | Current lines: 430
Trying strategy: random_lines
  Successful chomps: 0 | Current lines: 430
Trying strategy: random_ranges
  Successful chomps: 0 | Current lines: 430
Round 2 complete: 0 successful chomps

✅ No more progress possible. Chomping complete!

=== Final Results ===
Initial lines: 434
Final lines: 430
Reduction: 0.9%
Total successful chomps: 1
Total chomps tested: 10
Rounds: 2
Time elapsed: 3s

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

## ⚠️ Known Limitations

### Compiled Languages & Module Systems

Chompie works by blanking lines, which can create syntactically invalid code in compiled languages. Additionally, languages like Rust require all declared modules to compile, even if tests only use a subset.

**Example**: Testing only `card.rs` but `main.rs` declares `mod deck; mod hand; mod game;` means all those files must compile.

**Result**: Minimal reduction (~1-2%) on well-structured Rust projects, not due to bugs, but language constraints.

See [`LIMITATIONS.md`](./LIMITATIONS.md) for detailed analysis and proposed solutions.

### Best Use Cases

Chompie works best for:
- Single-file programs
- Python/Ruby scripts where syntax errors just fail tests
- Finding dead code in loosely-coupled modules
- Creating minimal reproducible examples from single files

## 🔮 Future Enhancements

Potential improvements:

- **Language-Aware Modes**: Rust mode that comments out unused `mod` declarations
- **Syntax-Aware Blanking**: Only blank complete syntactic units (functions, blocks)
- **Compilation Verification**: Ensure code compiles before considering blank successful
- **Module-Level Reduction**: Remove entire files before trying line-level bisection
- **Combination Testing**: Try removing multiple ranges together
- **Smart Ordering**: Use heuristics to prioritize likely-removable code
- **Parallel Execution**: Run tests in parallel for faster chomping

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
