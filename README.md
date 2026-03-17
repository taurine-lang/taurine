# 🐂 Taurine

**Fast, embeddable scripting language implemented in Rust**

[![Version](https://img.shields.io/crates/v/taurine.svg)](https://crates.io/crates/taurine)
[![License](https://img.shields.io/crates/l/taurine.svg)](LICENSE)
[![Rust](https://img.shields.io/badge/rust-1.70+-orange.svg)](https://www.rust-lang.org)

[📚 Documentation](https://ffsondev.github.io/taurinedev) | [🦀 Crates.io](https://crates.io/crates/taurine) | [💬 Discord](https://discord.gg/taurine)

---

Taurine combines the simplicity of Lua with the performance of compiled languages. It features a clean syntax, powerful built-in functions, and excellent Rust integration.

## ✨ Features

- ⚡ **Fast execution** — Optimized interpreter with Rc/RefCell
- 📝 **Simple syntax** — Lua-like, easy to learn
- 🚀 **Modern features** — f-strings, multi-return, destructuring, nil-safe operators
- 📦 **Rich stdlib** — JSON, HTTP, crypto, date/time, regex
- 🔌 **Embeddable** — Rust API, C API, Python/Node.js bindings

## 🚀 Quick Start

### Install

```bash
# From crates.io
cargo install taurine

# Or build from source
git clone https://github.com/ffsonDev/taurine.git
cd taurine
cargo build --release
```

### Usage

```bash
# Run a script
taurine script.tau

# Start REPL
taurine --repl

# With optimizations
taurine --optimize script.tau
```

### Hello World

```taurine
print("Hello, Taurine!")

let x = 10
let y = 20
print(f"x + y = {x + y}")
```

## 📖 Example

```taurine
// Variables
let name = "Taurine"
const VERSION = "1.0.5"

// Functions with multi-return
function divmod(a, b) {
    return a / b, a % b
}

let {q, r} = divmod(10, 3)
print(f"Quotient: {q}, Remainder: {r}")

// Arrays and loops
let arr = [1, 2, 3, 4, 5]
for i in 1..10 {
    if i == 5 { break }
    print(f"i = {i}")
}

// Tables
let obj = { name: "Taurine", version: VERSION }
print(obj?.name)  // nil-safe access
```

## 📚 Standard Library

| Module | Description |
|--------|-------------|
| `std/json.tau` | JSON parsing and stringification |
| `std/http.tau` | HTTP client (GET, POST, PUT, DELETE) |
| `std/crypto.tau` | MD5, SHA256, Base64, UUID |
| `std/date.tau` | Date/time formatting |
| `std/regex.tau` | Regular expressions |
| `std/array.tau` | Array utilities (map, filter, reduce) |

## 🔌 Embedding

### Rust

```rust
use taurine::Interpreter;

fn main() -> Result<(), String> {
    let mut interp = Interpreter::new();
    interp.run(r#"print("Hello from Rust!")"#)?;
    Ok(())
}
```

### C

```c
#include "taurine.h"

int main() {
    TaurineVM* vm = taurine_new();
    taurine_run(vm, "print(\"Hello from C!\")");
    taurine_free(vm);
    return 0;
}
```

See `examples/embedding/` for more examples.

## ⚙️ System Requirements

- **OS**: Windows 10+, Linux (glibc 2.17+), macOS 10.15+
- **Architecture**: x86_64, aarch64 (ARM64)
- **Memory**: 50MB RAM minimum
- **Disk**: 20MB free space

## 🤝 Contributing

Contributions are welcome! Please see our [Contributing Guide](https://ffsondev.github.io/taurinedev/contributing.html) for details.

## 📄 License

This project is licensed under the [MIT License](LICENSE).

## 🙏 Acknowledgments

Taurine is inspired by:
- **Lua** — Simple and embeddable
- **Rust** — Safety and performance
- **JavaScript** — Modern syntax features

---

**📚 Full documentation available at [https://ffsondev.github.io/taurinedev](https://ffsondev.github.io/taurinedev)**

**Happy coding with Taurine! 🐂**
