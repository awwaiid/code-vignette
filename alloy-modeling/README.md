# 🔍 Alloy Modeling Expert

An interactive web-based expert system for creating, analyzing, and learning about Alloy formal specifications. This vignette provides a comprehensive environment for working with the Alloy specification language, complete with examples, patterns, and analysis tools.

## What is Alloy?

Alloy is a declarative specification language for expressing complex structural constraints and behavior in software systems. Developed at MIT, it's used for:

- **Formal Verification** - Prove properties about your systems
- **Design Exploration** - Find edge cases and design flaws early
- **Documentation** - Precisely specify system requirements
- **Education** - Learn formal methods and logic

## Features

### 🎨 Interactive Editor
- Syntax highlighting for Alloy code
- Line numbers and bracket matching
- Auto-indentation
- Download models as `.als` files

### 📚 Rich Example Library
- **Address Book** - Classic data management example
- **File System** - Hierarchical structures and constraints
- **Genealogy** - Relationship modeling
- **River Crossing Puzzle** - Classic logic puzzle
- **Linked List** - Data structure verification

### 🔬 Model Analysis
- Signature counting and detection
- Fact and predicate identification
- Pattern recognition (inheritance, transitive closure, etc.)
- Best practice recommendations
- Syntax validation

### 📖 Comprehensive Guide
- Quick start tutorial
- Operator reference
- Quantifier explanations
- Common modeling patterns
- Best practices

## Installation

```bash
cd alloy-modeling
npm install
npm start
```

Then open http://localhost:3000 in your browser.

## Usage

### Quick Start

1. **Select an Example** - Choose from the dropdown menu
2. **Explore the Code** - Read through the model structure
3. **Analyze** - Click "Analyze Model" to see insights
4. **Modify** - Edit the model to learn
5. **Download** - Save your models for use with Alloy Analyzer

### Writing Your First Model

```alloy
// Define a simple graph
sig Node {
    edges: set Node
}

// No self-loops
fact NoSelfLoops {
    no n: Node | n in n.edges
}

// Find instances
run {} for 5
```

### Running Full Analysis

For complete verification and visualization, download the official Alloy Analyzer:
- Visit: https://alloytools.org/download.html
- Download the latest Alloy JAR file
- Run: `java -jar alloy.jar`
- Open your downloaded `.als` files

## Alloy Language Basics

### Signatures (Types)
```alloy
sig Person {
    friends: set Person,
    age: one Int
}
```

### Facts (Invariants)
```alloy
fact NoSelfFriendship {
    no p: Person | p in p.friends
}
```

### Predicates (Parameterized Constraints)
```alloy
pred CanVote[p: Person] {
    p.age >= 18
}
```

### Functions (Computed Values)
```alloy
fun adults: set Person {
    {p: Person | p.age >= 18}
}
```

### Commands (Analysis)
```alloy
run CanVote for 5        // Find instances
check NoLoops for 10     // Verify property
```

## Common Patterns

### Tree Structure
```alloy
sig Node {
    children: set Node
}

fact Acyclic {
    no n: Node | n in n.^children
}

fact SingleRoot {
    one n: Node | no n.~children
}
```

### Ordered List
```alloy
sig Element {
    next: lone Element
}

fact Linear {
    all e: Element | lone e.~next
    no e: Element | e in e.^next
}
```

### State Machines
```alloy
sig State {
    transitions: set State
}

one sig Initial extends State {}

fact Reachable {
    State = Initial.*transitions
}
```

## Operators Reference

### Set Operators
- `+` - Union
- `&` - Intersection
- `-` - Difference

### Relation Operators
- `.` - Join (relational composition)
- `~` - Transpose
- `^` - Transitive closure
- `*` - Reflexive transitive closure

### Quantifiers
- `all x: S | P` - Universal (for all)
- `some x: S | P` - Existential (exists)
- `no x: S | P` - None (doesn't exist)
- `lone x: S | P` - At most one
- `one x: S | P` - Exactly one

## Best Practices

1. **Start Simple** - Begin with basic signatures and add constraints gradually
2. **Use Meaningful Names** - Make your model self-documenting
3. **Small Scopes First** - Start with `run {} for 3` and increase
4. **Test Incrementally** - Add one constraint at a time
5. **Document Complex Logic** - Use comments for non-obvious constraints
6. **Verify Properties** - Use assertions and check commands
7. **Explore Counterexamples** - Learn from instances that violate properties

## Integration with Alloy Analyzer

This web tool is designed to complement the official Alloy Analyzer:

1. **Write & Explore** - Use this web interface for learning and drafting
2. **Validate** - Use basic syntax checking in the browser
3. **Download** - Export your model as `.als` file
4. **Analyze** - Open in Alloy Analyzer for full verification
5. **Visualize** - Use Alloy's powerful instance visualizer

## Educational Use

Perfect for:
- Learning formal methods
- Teaching software verification
- Exploring system designs
- Rapid prototyping of specifications
- Understanding complex constraints

## Advanced Topics

### Temporal Logic (Alloy 6+)
```alloy
var sig Counter {
    var value: one Int
}

fact {
    always (value' = plus[value, 1])
}
```

### Module System
```alloy
module filesystem

open util/ordering[Time]

sig File {}
sig Dir {
    contents: File -> Time
}
```

## Resources

- **Official Website**: https://alloytools.org/
- **Documentation**: https://alloytools.org/documentation.html
- **Book**: "Software Abstractions" by Daniel Jackson
- **Tutorial**: https://alloytools.org/tutorials/online/
- **Community**: https://github.com/AlloyTools/org.alloytools.alloy

## Future Enhancements

Planned features:
- [ ] Server-side Alloy analyzer integration
- [ ] Real-time collaboration
- [ ] More example models
- [ ] Custom theme editor
- [ ] Export to multiple formats
- [ ] Integration with Sterling visualizer
- [ ] Syntax error highlighting
- [ ] Auto-completion for Alloy keywords

## Contributing

This is an educational tool. Contributions welcome:
- Additional example models
- Better syntax highlighting
- Enhanced analysis algorithms
- Documentation improvements

## License

MIT License - See LICENSE file for details

## Acknowledgments

- Alloy development team at MIT
- Daniel Jackson for creating Alloy
- The Alloy community for examples and patterns
- Sterling project for web-based visualization inspiration

---

**Note**: This is an educational and exploration tool. For production formal verification, always use the official Alloy Analyzer with the complete SAT solver backend.
