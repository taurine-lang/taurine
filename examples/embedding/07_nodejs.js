// examples/embedding/07_nodejs.js
// Example: Using Taurine from Node.js
//
// Note: This is a demonstration of the Node.js binding structure.
// Full Node.js bindings would use napi-rs.
//
// Run with:
//   node 07_nodejs.js

console.log("=== Node.js Embedding Example ===\n");

// This is a mock demonstration
// In a real implementation, you would use:
//   const taurine = require('taurine');
//   const vm = new taurine.Interpreter();
//   vm.run("print('Hello from Node.js!')");

console.log("Node.js bindings would work like this:");
console.log(`
    const taurine = require('taurine');

    // Create interpreter
    const vm = new taurine.Interpreter();

    // Run Taurine code
    vm.run(\`
        print("Hello from Node.js!")
        let x = 42
        print(f"x = {x}")
    \`);

    // Get values
    const x = vm.get("x");
    console.log(\`Got x = \${x}\`);
`);

console.log("\n=== Success! ===");
console.log("\nNote: Full Node.js bindings are planned for a future release.");
