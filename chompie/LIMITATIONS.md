# Chompie Limitations and Future Improvements

## Current Limitations

### 1. **Line-Level Blanking in Compiled Languages**

Chompie currently blanks individual lines while preserving line numbers. This works well for some scenarios but has limitations with compiled languages like Rust.

**Problem**: Blanking arbitrary lines can create syntactically invalid code that won't compile.

**Example**: If you blank a closing brace `}`, the code becomes invalid.

### 2. **Module System Constraints (Rust)**

In Rust projects with `mod declarations`, all declared modules must compile successfully, even if tests only use a subset.

**Scenario**:
- Project has: `main.rs`, `card.rs`, `deck.rs`, `hand.rs`, `game.rs`
- Test only uses: `card.rs`
- **But**: `main.rs` declares `mod deck; mod hand; mod game;`
- **Result**: All modules must compile, even if unused by the test

**Expected Reduction**: 300+ lines (remove deck, hand, game entirely)
**Actual Reduction**: ~4 lines (only safe whitespace/comments)

This means chompie achieves minimal reduction (~1%) on well-structured Rust projects, not because it's broken, but because the language requires all modules to be valid.

## Proposed Solutions

### Short Term

1. **Add Language-Specific Modes**
   - Rust mode: Comment out unused `mod` declarations
   - Python mode: Comment out unused `import` statements
   - JavaScript mode: Comment out unused `require`/`import` statements

2. **Syntax-Aware Blanking**
   - Parse code structure before blanking
   - Only blank complete syntactic units (functions, blocks, statements)
   - Never blank partial constructs (half a function, unmatched braces)

3. **Compilation Verification**
   - For compiled languages, verify code compiles before considering blank "successful"
   - Currently only checks if command output matches, not if compilation succeeded

### Long Term

1. **Smart Module Detection**
   - Analyze dependency graphs
   - Automatically identify and remove unused modules
   - Start with module-level reduction before line-level

2. **Multiple Strategies**
   - Try file-level removal first
   - Then function-level removal
   - Finally line-level bisection
   - Use the strategy that gives best results

3. **Language Plugins**
   - Plugin system for language-specific understanding
   - Each plugin knows how to safely reduce code for that language
   - Rust plugin, Python plugin, JavaScript plugin, etc.

## Workarounds

### For Better Rust Reduction

Create a standalone test file that doesn't depend on main.rs module declarations:

```rust
// tests/standalone_test.rs
#[path = "../src/card.rs"]
mod card;

#[cfg(test)]
mod tests {
    use super::card::*;

    #[test]
    fn test_card_value() {
        let card = Card::new(Suit::Hearts, Rank::Five);
        assert_eq!(card.value(), 5);
    }
}
```

Then run: `chompie -d tests "cargo test --test standalone_test"`

This would allow much more dramatic reduction since it doesn't require all modules to compile.

### For Single-File Projects

Chompie works best on:
- Single-file programs
- Scripts (Python, Ruby, etc.) where invalid syntax just fails the test
- Projects where you can isolate the code under test

## Design Philosophy

Chompie's current design prioritizes:
1. **Correctness**: Never claim a reduction if tests don't pass
2. **Simplicity**: Line-based bisection is simple and predictable
3. **Language-agnostic**: Works with any command, any language

The tradeoff is that it's conservative and may not achieve maximum reduction in all scenarios.

## Success Metrics

Chompie is successful when it:
- Identifies truly unused code and removes it
- Maintains exact test output throughout
- Never produces invalid test results

Low reduction percentage doesn't mean failure - it means the code is tightly coupled and well-tested!

## Future Vision

Ideal chompie would:
1. Understand code structure (AST-level)
2. Know language-specific rules
3. Try multiple reduction strategies
4. Achieve 70-90% reduction on focused tests
5. Still work as a simple line-blanker for unknown languages

---

**Current Status**: Chompie correctly implements bisection-based line blanking. The ~1% reduction on Rust projects is expected given module system constraints, not a bug.

**Next Step**: Implement Rust-specific module awareness to unlock dramatic reductions.
