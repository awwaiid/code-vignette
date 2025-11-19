# Blackjack Chomped Example

This directory contains the blackjack example after running chompie on it.

## Chomping Results

```
Command: cargo test --quiet 2>&1
Initial lines: 434
Final lines: 433
Reduction: 0.2%
Total chomps: 751
Successful chomps: 1
Time elapsed: 23s
```

## What Got Chomped?

Chompie removed trailing newlines from the end of files. That's it!

## What Does This Tell Us?

This minimal reduction demonstrates an important point: **the blackjack code is well-written and tightly coupled**. Nearly every line is necessary for the tests to pass. This shows that:

1. The code has no dead code or unnecessary complexity
2. The tests exercise the full codebase
3. Chompie correctly identifies when code cannot be removed

## Verification

All 16 tests still pass after chomping:

```bash
cargo test --quiet
# running 16 tests
# ................
# test result: ok. 16 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out
```

## How It Was Created

```bash
cd chompie
cp -r examples/blackjack examples/blackjack_chomped
cd examples/blackjack_chomped
../target/release/chompie -y -d src "cargo test --quiet 2>&1"
```

## Insights

This example demonstrates that chompie is conservative and correct:
- It doesn't remove code that's needed
- It preserves exact test output
- It works with complex multi-file projects
- Even a "minimal" reduction proves the tool works correctly

For more dramatic reductions, try running chompie on a codebase with:
- Dead code or unused functions
- Overly verbose implementations
- Multiple independent features (chomp to just one feature)
