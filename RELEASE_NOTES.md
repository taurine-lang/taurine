# Taurine v1.0.0 Release Notes

**Release Date:** March 16, 2026

## 🎉 Welcome to Taurine v1.0.0!

We're thrilled to announce the first stable release of Taurine — a fast, embeddable scripting language that combines Lua's simplicity with modern language features and Rust's performance.

## ✨ What's New

### Language Features

#### let/const Declarations
```taurine
// Mutable variable
let x = 10
x = 100

// Constant (cannot be reassigned)
const PI = 3.14
// PI = 3  // Error!
```

#### String Interpolation (f-strings)
```taurine
let name = "Taurine"
let version = "1.0"
print(f"Hello from {name} v{version}!")
```

#### Multi-Return Functions
```taurine
function divmod(a, b) {
    return a / b, a % b
}

let {q, r} = divmod(10, 3)
print(f"Quotient: {q}, Remainder: {r}")
```

#### break/continue in Loops
```taurine
for i in 1..10 {
    if i == 5 { break }
    if i == 3 { continue }
    print(i)
}
```

#### Nil-Safe Operators
```taurine
let obj = { name: "test" }
print(obj?.name)      // "test"
print(obj?.missing)   // nil (no error)

let nilObj = nil
print(nilObj?.prop)   // nil
```

#### JSON-Style Tables
```taurine
// Both styles now supported
let obj1 = { name: "test", value: 42 }  // JSON style
let obj2 = { name = "test", value = 42 }  // Legacy style
```

### Performance Improvements

- **2-3x faster** variable access with `Rc<RefCell<>>` instead of `Arc<Mutex<>>`
- **Constant folding** optimizer for compile-time evaluation
- **Dead code elimination** for unreachable code

### Standard Library

Complete standard library with:

- **JSON** — Parse and stringify
- **HTTP** — GET, POST, PUT, DELETE requests
- **Crypto** — MD5, SHA256, Base64, UUID
- **Date/Time** — Formatting and manipulation
- **Regex** — Pattern matching
- **Arrays** — Map, filter, reduce, and more
- **Strings** — Utilities and manipulation
- **I/O** — File operations
- **Math** — Mathematical functions
- **OS** — Operating system functions

### Documentation

Comprehensive documentation including:

- `README.md` — Quick start and overview
- `docs/GUIDE.md` — Complete language guide
- `docs/EXAMPLES.md` — Code examples
- `docs/CHANGELOG.md` — Version history
- `examples/` — Working code examples

## 📦 Installation

```bash
# Build from source
cargo build --release

# Run a script
./target/release/taurine script.tau

# Start REPL
./target/release/taurine --repl

# With optimizations
./target/release/taurine --optimize script.tau
```

## 🔄 Migration Guide

### From v0.x to v1.0

#### Variable Declarations
```taurine
// Old (still works)
loc x = 10

// New (recommended)
let x = 10
const PI = 3.14
```

#### Table Syntax
```taurine
// Old (still works)
let obj = { name = "test" }

// New (recommended)
let obj = { name: "test" }
```

#### Package Manager
The package manager has been removed. Use standard library modules directly:

```taurine
// Import standard library
import "std/json.tau" as json
import "std/crypto.tau" as crypto
```

## 🐛 Bug Fixes

- Fixed string escaping in lexer
- Fixed module import path resolution
- Fixed function literal parsing in tables
- Fixed environment variable lookup

## ⚠️ Breaking Changes

- Removed package manager commands (`init`, `install`, `list`, etc.)
- Removed `taurine test` and `taurine bench` commands (use std/test.tau directly)

## 📊 Statistics

- **50+** built-in functions
- **10** standard library modules
- **22** unit tests
- **Multiple** integration tests

## 🙏 Acknowledgments

Taurine is inspired by:
- **Lua** — Simplicity and embeddability
- **Rust** — Safety and performance
- **JavaScript** — Modern syntax features

## 🔮 What's Next?

Future releases may include:
- Bytecode compilation
- JIT compilation
- Pattern matching
- Pipe operator
- More standard library modules

## 📄 License

MIT License — see LICENSE file for details.

## 🔗 Links

- [GitHub Repository](https://github.com/ffsonDev/taurine)
- [Documentation](docs/GUIDE.md)
- [Examples](examples/)
- [Changelog](docs/CHANGELOG.md)

---

**Happy coding with Taurine! 🐂**
