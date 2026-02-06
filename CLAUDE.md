# CLAUDE.md

## Project Overview

This is **code-vignette**, a collection of standalone experiments and art projects by Brock Wilcox. Each vignette lives in its own directory and is fully independent — its own language, build system, dependencies, and documentation. The root repo is just the container; all real work happens inside individual vignette directories.

Licensed under MIT.

## Repository Structure

```
code-vignette/
├── CLAUDE.md              # This file
├── README.md              # Root overview listing all vignettes
├── LICENSE                # MIT License
├── .gitignore             # Shared ignores
├── chompie/               # Rust: code minimization tool
│   ├── Cargo.toml
│   ├── README.md
│   ├── LIMITATIONS.md
│   ├── src/               # Main source
│   └── examples/          # Example projects (blackjack, blackjack_chomped)
└── recaman-visualizer.html  # Standalone HTML5 Recaman sequence visualization
```

## Current Vignettes

### chompie/ (Rust)
A code minimization tool that reduces codebases to the minimal set of lines needed to produce a specific output. Uses multiple strategies (bisection, random lines, random ranges, up-to-n-lines window).

- **Language:** Rust (edition 2021)
- **Dependencies:** `anyhow` 1.0, `clap` 4.5 (with derive)
- **Dev dependencies:** `tempfile` 3.8
- **Build:** `cargo build` / `cargo build --release`
- **Test:** `cargo test` (run from `chompie/` directory)
- **Install:** `cargo install --path .`

Key source files in `chompie/src/`:
- `main.rs` — CLI interface (clap)
- `chomper.rs` — Core execution engine, orchestrates strategies
- `command_runner.rs` — Runs shell commands, captures stdout/stderr/exit code
- `file_manager.rs` — Tracks file state, blanks/restores lines
- `strategy.rs` — Strategy trait definition
- `progress.rs` — Progress tracking and display
- `bisector.rs` — Legacy bisection utility
- `strategies/` — Strategy implementations (bisection, random_lines, random_ranges, up_to_n_lines)

### recaman-visualizer.html
A standalone HTML5/Canvas interactive Recaman sequence visualizer. No build step; open directly in a browser.

## Development Conventions

### Each vignette is standalone
- Every vignette has its own language, toolchain, and build system
- Work within the vignette's directory using its native tools
- The root repo has no global build, test, or lint command
- When adding a new vignette, create a new top-level directory and update the root `README.md`

### Rust conventions (chompie)
- Tests are inline at the bottom of each source file using `#[cfg(test)]` modules
- Uses `anyhow::Result` for error handling
- CLI parsing via clap derive macros
- No CI/CD, no rustfmt/clippy configuration — follows standard Rust defaults
- `Cargo.lock` is gitignored (it's a binary tool, not a library)

### Git
- `.gitignore` covers: `target/`, `Cargo.lock`, `test-*/`, editor files, `.DS_Store`
- Commit messages are concise and descriptive (see `git log` for style)
- PRs are merged from feature branches

## Common Commands

```bash
# Chompie — build and test
cd chompie
cargo build
cargo test
cargo run -- "cargo test"    # Run chompie itself

# Chompie — release build
cargo build --release
./target/release/chompie --help
```

## Architecture Notes (chompie)

The core loop:
1. Establish a baseline by running the user's command and capturing output
2. Rotate through enabled strategies in rounds
3. Each strategy generates `ChompRange`s (file + line range) to try blanking
4. For each range: blank lines, re-run command, keep blanking if output matches baseline, restore if not
5. Stop when a full round produces zero successful chomps

Key design decisions:
- **Blanks lines** (replaces with empty) rather than deleting — preserves line numbers for stack traces
- **Strategies are composable** via the `Strategy` trait — easy to add new ones
- **Language-agnostic** — works with any command, not tied to any AST or parser
- **Reproducible RNG** — random strategies use a fixed-seed LCG for deterministic results
- **State deduplication** — avoids re-testing identical configurations
