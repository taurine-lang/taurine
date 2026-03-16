# 🐂 Taurine Programming Language

**Fast, embeddable scripting language implemented in Rust**

Taurine combines the simplicity of Lua with the performance of compiled languages. It features a clean syntax, powerful built-in functions, and excellent Rust integration.

## ✨ Features

- **Fast execution** — Optimized interpreter with Rc/RefCell for single-threaded performance
- **Simple syntax** — Lua-like syntax that's easy to learn
- **Modern features** — f-strings, multi-return, destructuring, nil-safe operators
- **Rich standard library** — JSON, HTTP, crypto, date/time, regex, and more
- **Embeddable** — Easy to integrate into Rust applications

## 🚀 Quick Start

### Installation

#### Windows (PowerShell)

```powershell
# Quick install
curl -fsSL https://raw.githubusercontent.com/ffsonDev/taurine/main/installers/install.ps1 | Invoke-Expression

# Or manual
Invoke-WebRequest -Uri https://github.com/ffsonDev/taurine/releases/download/v1.0.0/taurine-x86_64-pc-windows-msvc.zip -OutFile taurine.zip
Expand-Archive taurine.zip -DestinationPath $env:USERPROFILE\.taurine
```

#### Linux/macOS (Bash)

```bash
# Quick install
curl -fsSL https://raw.githubusercontent.com/ffsonDev/taurine/main/installers/install.sh | bash

# Or manual
curl -fsSL https://github.com/ffsonDev/taurine/releases/download/v1.0.0/taurine-x86_64-unknown-linux-gnu.tar.gz | tar -xzf - -C $HOME/.taurine
```

#### From Source

```bash
git clone https://github.com/ffsonDev/taurine.git
cd taurine
cargo build --release
```

#### Using cargo-binstall

```bash
cargo install cargo-binstall
cargo binstall taurine
```

See [installers/INSTALL.md](installers/INSTALL.md) for detailed instructions.

### Usage

```bash
# Run a script
taurine script.tau

# Start REPL
taurine --repl

# With optimizations
taurine --optimize script.tau

# Format code
taurine --format script.tau

# Show help
taurine --help
```

### Hello World

```taurine
print("Hello, Taurine!")
```

## 📖 Language Guide

### Variables

```taurine
// Mutable variable
let x = 10
x = 100

// Constant (cannot be reassigned)
const PI = 3.14
// PI = 3  // Error!

// Legacy syntax (still supported)
loc y = 20
```

### Data Types

```taurine
// Numbers
let num = 42
let float = 3.14

// Strings
let str = "Hello"
let multiline = "Line1\nLine2"

// Booleans
let truthy = true
let falsy = false

// Arrays
let arr = [1, 2, 3]
let first = arr[0]
arr[0] = 100

// Tables (objects)
let obj = { name: "Taurine", version: "1.0" }
print(obj.name)  // "Taurine"
```

### Functions

```taurine
// Basic function
function add(a, b) {
    return a + b
}

// Default parameters
function greet(name, greeting = "Hello") {
    print(f"{greeting}, {name}!")
}

// Multi-return
function divmod(a, b) {
    return a / b, a % b
}

// Destructure return values
let {q, r} = divmod(10, 3)
print(f"Quotient: {q}, Remainder: {r}")
```

### Control Flow

```taurine
// If/else
if x > 10 {
    print("big")
} else {
    print("small")
}

// While loop
let i = 0
while i < 10 {
    print(i)
    i = i + 1
}

// For-in with range
for i in 1..10 {
    print(i)  // 1 to 9
}

// For-in with array
for item in [1, 2, 3] {
    print(item)
}

// Break and continue
for i in 1..10 {
    if i == 5 {
        break
    }
    if i == 3 {
        continue
    }
    print(i)
}
```

### String Interpolation

```taurine
let name = "Taurine"
let version = "1.0"

// f-strings
print(f"Hello from {name} v{version}!")
print(f"2 + 2 = {2 + 2}")
```

### Nil-Safe Operators

```taurine
let obj = { name: "test" }

// Safe property access
print(obj?.name)  // "test"
print(obj?.missing)  // nil (no error)

let nilObj = nil
print(nilObj?.prop)  // nil
```

### Error Handling

```taurine
try {
    risky_operation()
} catch (err) {
    print(f"Error: {err}")
}
```

### Modules

```taurine
// Import standard library
import "std/json.tau" as json
import "std/http.tau" as http
import "std/crypto.tau" as crypto

// Use imported functions
let obj = json.parse("{ \"name\": \"test\" }")
let hash = crypto.sha256("hello")
```

## 📚 Standard Library

### JSON

```taurine
import "std/json.tau" as json

// Parse JSON string
let obj = json.parse("{ \"name\": \"test\", \"value\": 42 }")
print(obj.name)

// Stringify to JSON
let jsonStr = json.stringify({ name: "test", value: 42 })
```

### Crypto

```taurine
import "std/crypto.tau" as crypto

let hash = crypto.md5("hello")
let hash256 = crypto.sha256("hello")
let encoded = crypto.base64Encode("hello")
let decoded = crypto.base64Decode(encoded)
let uuid = crypto.uuid()
```

### Date/Time

```taurine
import "std/date.tau" as date

let now = date.now()
print(date.format(now, "%Y-%m-%d %H:%M:%S"))
```

### Regex

```taurine
import "std/regex.tau" as regex

let pattern = regex.compile("\\d+")
print(pattern:match("abc123"))  // true
let match = pattern:find("abc123")
print(match.text)  // "123"
```

### Arrays

```taurine
import "std/array.tau" as array

let arr = [1, 2, 3, 4, 5]
print(array.len(arr))  // 5
print(array.includes(arr, 3))  // true

// Functional methods
let doubled = array.map(arr, x => x * 2)
let evens = array.filter(arr, x => x % 2 == 0)
```

## 🔧 Advanced Features

### Function Literals

```taurine
// Anonymous function in table
let obj = {
    name: "test",
    greet: function(self) {
        print(f"Hello, {self.name}!")
    }
}
```

### Table Methods

```taurine
let counter = { count: 0 }

function increment(self, amount) {
    self.count = self.count + amount
}

increment(counter, 5)
print(counter.count)  // 5
```

## 📊 Performance

Taurine v1.0 features significant performance improvements:

- **Rc<RefCell<>>** instead of Arc<Mutex<>> for single-threaded efficiency
- **Constant folding** optimizer
- **Dead code elimination**
- **Variable lookup caching**

## 🤝 Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

## 📄 License

MIT License - see LICENSE file for details.

## 🙏 Acknowledgments

Taurine is inspired by:
- Lua — Simple and embeddable
- Rust — Safety and performance
- JavaScript — Modern syntax features
