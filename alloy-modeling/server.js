const express = require('express');
const path = require('path');

const app = express();
const PORT = process.env.PORT || 3000;

// Serve static files from the public directory
app.use(express.static(path.join(__dirname, 'public')));
app.use(express.json());

// Serve the main page
app.get('/', (req, res) => {
    res.sendFile(path.join(__dirname, 'public', 'index.html'));
});

// Future: API endpoint for running Alloy analyzer
// This would require the Alloy JAR file and Java runtime
app.post('/api/analyze', (req, res) => {
    const { model } = req.body;

    // Placeholder for future Alloy analyzer integration
    // This would execute the Alloy analyzer JAR with the model
    res.json({
        status: 'info',
        message: 'Server-side Alloy analysis not yet implemented. Use the Alloy Analyzer desktop application for full analysis.',
        suggestion: 'Download Alloy from https://alloytools.org/download.html'
    });
});

app.listen(PORT, () => {
    console.log(`
╔═══════════════════════════════════════════════════════════╗
║                                                           ║
║        🔍 Alloy Modeling Expert Server Running           ║
║                                                           ║
║  Server: http://localhost:${PORT}                       ║
║                                                           ║
║  Features:                                                ║
║  • Interactive Alloy model editor                         ║
║  • Example models and patterns                            ║
║  • Syntax validation                                      ║
║  • Model analysis and recommendations                     ║
║                                                           ║
║  Next Steps:                                              ║
║  1. Open http://localhost:${PORT} in your browser       ║
║  2. Select an example or write your own model             ║
║  3. Download Alloy Analyzer for full verification:        ║
║     https://alloytools.org/download.html                  ║
║                                                           ║
╚═══════════════════════════════════════════════════════════╝
    `);
});
