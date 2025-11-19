# Blackjack Chomped with Multi-Strategy

This directory demonstrates chompie's multi-strategy system running on a focused test.

## Test Command

```bash
cargo test card::tests::test_card_value_number --quiet 2>&1
```

This test only needs:
- `Card::new(Suit::Hearts, Rank::Five)`
- `card.value()` returning 5

## Multi-Strategy Results

```
📋 Using 3 strategies: bisection, random_lines, random_ranges

--- Round 1 ---
Trying strategy: bisection
  Successful chomps: 1 | Current lines: 430
Trying strategy: random_lines
  Successful chomps: 0 | Current lines: 430
Trying strategy: random_ranges
  Successful chomps: 0 | Current lines: 430

--- Round 2 ---
Trying strategy: bisection
  Successful chomps: 0 | Current lines: 430
Trying strategy: random_lines
  Successful chomps: 0 | Current lines: 430
Trying strategy: random_ranges
  Successful chomps: 0 | Current lines: 430

✅ No more progress possible. Chomping complete!

Initial lines: 434
Final lines: 430
Reduction: 0.9%
```

## Strategy Explanation

### Bisection Strategy
Systematically tries removing halves, quarters, eighths, etc. of each file.
Good for finding large contiguous blocks of removable code.

### Random Lines Strategy
Randomly tries removing individual lines.
Can find lines that bisection might miss due to dependencies.

### Random Ranges Strategy
Tries removing random ranges of varying sizes (1-25% of file).
Discovers chunks of removable code that don't align with bisection boundaries.

## Meta-Strategy

The orchestrator rotates through all strategies until no progress:
1. Run all strategies in order (Round 1)
2. If ANY made progress, run all again (Round 2)
3. Repeat until a full round with zero successful chomps
4. Done!

This ensures we find all removable code regardless of which strategy would find it.

## Why Still Minimal Reduction?

Despite using multiple strategies, reduction is still minimal (~1%) because of **Rust's module system**.

The `main.rs` file declares:
```rust
mod card;
mod deck;  // ← Forces compilation of deck.rs
mod hand;  // ← Forces compilation of hand.rs
mod game;  // ← Forces compilation of game.rs
```

Even though the test only uses `card.rs`, Rust requires ALL declared modules to compile successfully. Chompie can't blank code that would break compilation.

## Future: Language-Aware Strategies

To achieve dramatic reduction, we need language-specific strategies:
- **Rust Module Strategy**: Comment out unused `mod` declarations
- **Python Import Strategy**: Comment out unused imports
- **JavaScript Strategy**: Comment out unused requires/imports

This keeps chompie language-agnostic while allowing optional language-specific optimizations.

## Verification

```bash
cargo test card::tests::test_card_value_number --quiet
# running 1 test
# .
# test result: ok. 1 passed
```

The test still passes after chomping!
