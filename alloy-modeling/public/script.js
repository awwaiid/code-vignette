// Initialize CodeMirror editor
let editor;

document.addEventListener('DOMContentLoaded', function() {
    const textarea = document.getElementById('alloyEditor');
    editor = CodeMirror.fromTextArea(textarea, {
        lineNumbers: true,
        mode: 'text/x-java',
        theme: 'monokai',
        indentUnit: 4,
        tabSize: 4,
        lineWrapping: true,
        autoCloseBrackets: true,
        matchBrackets: true,
        extraKeys: {
            'Ctrl-Space': 'autocomplete'
        }
    });

    editor.on('change', updateLineCount);

    // Load default example
    loadExampleByName('addressBook');
});

// Example models
const examples = {
    addressBook: `// Address Book Model
// Models an address book with names, addresses, and groups

sig Name {}
sig Addr {}

sig Book {
    addr: Name -> lone Addr
}

pred add[b, b': Book, n: Name, a: Addr] {
    b'.addr = b.addr + n->a
}

pred del[b, b': Book, n: Name] {
    b'.addr = b.addr - n->Addr
}

fact traces {
    some b: Book | no b.addr
    all b: Book - last |
        let b' = b.next |
            some n: Name, a: Addr |
                add[b, b', n, a] or del[b, b', n]
}

run {} for 3`,

    fileSystem: `// File System Model
// Models a hierarchical file system

abstract sig Object {}
sig File extends Object {}
sig Dir extends Object {
    contents: set Object
}

sig FileSystem {
    root: one Dir,
    live: set Object
}

fact {
    // All live objects are reachable from root
    all fs: FileSystem |
        fs.live = fs.root.*contents

    // No cycles
    all d: Dir | d not in d.^contents

    // Each object appears in at most one directory
    all o: Object | lone contents.o
}

pred create[fs, fs': FileSystem, parent: Dir, o: Object] {
    o not in fs.live
    parent in fs.live
    fs'.root = fs.root
    fs'.live = fs.live + o
    contents' = contents + parent->o
}

run create for 4`,

    genealogy: `// Genealogy Model
// Models family relationships

abstract sig Person {
    father: lone Man,
    mother: lone Woman
}

sig Man extends Person {
    wife: lone Woman
}

sig Woman extends Person {
    husband: lone Man
}

fact {
    // Marriage is symmetric
    wife = ~husband

    // No person is their own ancestor
    no p: Person | p in p.^(mother + father)

    // Parents' marriage
    all p: Person |
        some p.mother and some p.father implies
            p.mother = p.father.wife
}

fun grandparents[p: Person]: set Person {
    p.(mother + father).(mother + father)
}

pred siblings[p1, p2: Person] {
    p1 != p2
    p1.mother = p2.mother
    p1.father = p2.father
}

run {} for 4`,

    riverCrossing: `// River Crossing Puzzle
// Farmer, Fox, Chicken, and Grain

abstract sig Object {}
one sig Farmer, Fox, Chicken, Grain extends Object {}

sig State {
    near: set Object,
    far: set Object
}

fact {
    // Every object is either near or far
    all s: State | s.near + s.far = Object

    // No object in both places
    all s: State | no s.near & s.far
}

pred safe[s: State] {
    // Fox and chicken can't be alone together
    Chicken in s.near implies
        (Fox in s.near implies Farmer in s.near)
    Chicken in s.far implies
        (Fox in s.far implies Farmer in s.far)

    // Chicken and grain can't be alone together
    Grain in s.near implies
        (Chicken in s.near implies Farmer in s.near)
    Grain in s.far implies
        (Chicken in s.far implies Farmer in s.far)
}

pred move[s, s': State, cargo: set Object] {
    // Farmer must be in the boat
    Farmer in cargo

    // At most one other object
    #cargo <= 2

    // Move from near to far
    (Farmer in s.near and
     s'.near = s.near - cargo and
     s'.far = s.far + cargo)
    or
    // Move from far to near
    (Farmer in s.far and
     s'.far = s.far - cargo and
     s'.near = s.near + cargo)
}

fact solution {
    // Start with everything near
    first.near = Object

    // End with everything far
    last.far = Object

    // Each transition is a valid move
    all s: State - last |
        let s' = next[s] |
            some cargo: set Object |
                move[s, s', cargo] and safe[s']
}

run {} for 8 State`,

    linkedList: `// Linked List Model
// Models a singly-linked list data structure

sig Node {
    next: lone Node,
    value: one Int
}

sig List {
    head: lone Node
}

fact {
    // No cycles
    all n: Node | n not in n.^next

    // Each node in at most one list
    all n: Node | lone head.n
}

pred isEmpty[l: List] {
    no l.head
}

pred contains[l: List, v: Int] {
    v in l.head.*next.value
}

pred sorted[l: List] {
    all n: l.head.*next |
        some n.next implies n.value <= n.next.value
}

run sorted for 5`
};

function switchTab(tabName) {
    // Hide all tabs
    document.querySelectorAll('.tab-content').forEach(content => {
        content.classList.remove('active');
    });

    document.querySelectorAll('.tab').forEach(tab => {
        tab.classList.remove('active');
    });

    // Show selected tab
    document.getElementById(tabName).classList.add('active');
    event.target.classList.add('active');
}

function loadExample() {
    const select = document.getElementById('exampleSelect');
    const exampleName = select.value;
    if (exampleName) {
        loadExampleByName(exampleName);
    }
}

function loadExampleByName(name) {
    if (examples[name]) {
        editor.setValue(examples[name]);
        updateStatus('Example loaded: ' + name);
        document.getElementById('exampleSelect').value = name;
    }
}

function analyzeModel() {
    const code = editor.getValue();
    const analysis = performAnalysis(code);

    document.getElementById('analysisOutput').textContent = analysis;
    switchTab('analysis');

    // Activate analysis tab
    document.querySelectorAll('.tab').forEach(tab => tab.classList.remove('active'));
    document.querySelectorAll('.tab')[2].classList.add('active');

    updateStatus('Analysis complete');
}

function performAnalysis(code) {
    let analysis = '=== ALLOY MODEL ANALYSIS ===\n\n';

    // Count signatures
    const sigMatches = code.match(/sig\s+\w+/g);
    const sigCount = sigMatches ? sigMatches.length : 0;
    analysis += `Signatures found: ${sigCount}\n`;
    if (sigMatches) {
        analysis += '  - ' + sigMatches.map(s => s.replace('sig ', '')).join('\n  - ') + '\n';
    }
    analysis += '\n';

    // Count facts
    const factMatches = code.match(/fact\s+(\w+)?/g);
    const factCount = factMatches ? factMatches.length : 0;
    analysis += `Facts found: ${factCount}\n\n`;

    // Count predicates
    const predMatches = code.match(/pred\s+\w+/g);
    const predCount = predMatches ? predMatches.length : 0;
    analysis += `Predicates found: ${predCount}\n`;
    if (predMatches) {
        analysis += '  - ' + predMatches.map(s => s.replace('pred ', '')).join('\n  - ') + '\n';
    }
    analysis += '\n';

    // Count functions
    const funMatches = code.match(/fun\s+\w+/g);
    const funCount = funMatches ? funMatches.length : 0;
    analysis += `Functions found: ${funCount}\n`;
    if (funMatches) {
        analysis += '  - ' + funMatches.map(s => s.replace('fun ', '')).join('\n  - ') + '\n';
    }
    analysis += '\n';

    // Check for run/check commands
    const runMatches = code.match(/run\s+(\w+)?/g);
    const checkMatches = code.match(/check\s+(\w+)?/g);
    analysis += `Run commands: ${runMatches ? runMatches.length : 0}\n`;
    analysis += `Check commands: ${checkMatches ? checkMatches.length : 0}\n\n`;

    // Pattern detection
    analysis += '=== PATTERNS DETECTED ===\n\n';

    if (code.includes('extends')) {
        analysis += '✓ Inheritance hierarchy detected\n';
    }

    if (code.includes('^')) {
        analysis += '✓ Transitive closure used\n';
    }

    if (code.includes('~')) {
        analysis += '✓ Relation transpose used\n';
    }

    if (code.match(/all\s+\w+:/)) {
        analysis += '✓ Universal quantification used\n';
    }

    if (code.match(/some\s+\w+:/)) {
        analysis += '✓ Existential quantification used\n';
    }

    if (code.includes('one sig')) {
        analysis += '✓ Singleton pattern detected\n';
    }

    if (code.includes('abstract sig')) {
        analysis += '✓ Abstract signatures (type hierarchy)\n';
    }

    analysis += '\n=== RECOMMENDATIONS ===\n\n';

    if (sigCount > 0 && factCount === 0) {
        analysis += '⚠ Consider adding facts to constrain your model\n';
    }

    if (predCount > 0 && !runMatches) {
        analysis += '⚠ Add run commands to explore predicates\n';
    }

    if (!code.includes('run') && !code.includes('check')) {
        analysis += '⚠ Add run or check commands to execute the model\n';
    }

    if (code.length < 50) {
        analysis += '⚠ Model appears to be very simple or empty\n';
    }

    analysis += '\n=== BEST PRACTICES ===\n\n';
    analysis += '• Use meaningful names for signatures and relations\n';
    analysis += '• Document complex constraints with comments\n';
    analysis += '• Start with small scopes and increase gradually\n';
    analysis += '• Use assertions to verify expected properties\n';
    analysis += '• Keep predicates focused and composable\n';

    return analysis;
}

function validateSyntax() {
    const code = editor.getValue();
    let errors = [];

    // Basic syntax checks
    const openBraces = (code.match(/{/g) || []).length;
    const closeBraces = (code.match(/}/g) || []).length;

    if (openBraces !== closeBraces) {
        errors.push('Mismatched braces: ' + openBraces + ' opening, ' + closeBraces + ' closing');
    }

    const openBrackets = (code.match(/\[/g) || []).length;
    const closeBrackets = (code.match(/]/g) || []).length;

    if (openBrackets !== closeBrackets) {
        errors.push('Mismatched brackets: ' + openBrackets + ' opening, ' + closeBrackets + ' closing');
    }

    const openParens = (code.match(/\(/g) || []).length;
    const closeParens = (code.match(/\)/g) || []).length;

    if (openParens !== closeParens) {
        errors.push('Mismatched parentheses: ' + openParens + ' opening, ' + closeParens + ' closing');
    }

    if (errors.length > 0) {
        alert('Syntax Issues Found:\n\n' + errors.join('\n'));
        updateStatus('Syntax errors detected');
    } else {
        alert('✓ No basic syntax errors detected!\n\nNote: For complete validation, use the Alloy Analyzer.');
        updateStatus('Syntax validation passed');
    }
}

function clearEditor() {
    if (confirm('Are you sure you want to clear the editor?')) {
        editor.setValue('');
        updateStatus('Editor cleared');
    }
}

function downloadModel() {
    const code = editor.getValue();
    const blob = new Blob([code], { type: 'text/plain' });
    const url = window.URL.createObjectURL(blob);
    const a = document.createElement('a');
    a.href = url;
    a.download = 'model.als';
    a.click();
    window.URL.revokeObjectURL(url);
    updateStatus('Model downloaded');
}

function updateLineCount() {
    const lineCount = editor.lineCount();
    document.getElementById('lineCount').textContent = 'Lines: ' + lineCount;
}

function updateStatus(message) {
    document.getElementById('modelStatus').textContent = message;
    setTimeout(() => {
        document.getElementById('modelStatus').textContent = 'Ready';
    }, 3000);
}
