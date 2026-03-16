# CHANGELOG

All notable changes to Taurine are documented in this file.

## [1.0.0] - 2026-03-16

### 🎉 Stable Release!

The first stable release of Taurine programming language.

### Added

- **let/const** variable declarations
- **f-strings** for string interpolation
- **Multi-return** functions with destructuring
- **break/continue** in loops
- **Nil-safe operators** (`?.`)
- **JSON-style tables** (`{key: val}`)
- **Range expressions** (`1..10`)
- **Array index assignment** (`arr[i] = value`)
- **Default parameters** in functions
- **Function literals** (anonymous functions)

### Standard Library

- `std/json.tau` — JSON parsing/stringification
- `std/http.tau` — HTTP client
- `std/crypto.tau` — Crypto functions
- `std/date.tau` — Date/time
- `std/regex.tau` — Regular expressions
- `std/array.tau` — Array utilities
- `std/string.tau` — String utilities
- `std/io.tau` — File I/O
- `std/math.tau` — Math functions
- `std/os.tau` — OS functions

### Performance

- Replaced `Arc<Mutex<>>` with `Rc<RefCell<>>` for 2-3x speedup
- Constant folding optimizer
- Dead code elimination

### Changed

- Removed package manager (focus on core language)
- Simplified CLI interface

---

## [0.12.0] - 2026-03-15

### Added

- Test framework (`taurine test`)
- Benchmark suite (`taurine bench`)
- `assert()` and `assert_eq()` functions

---

## [0.11.0] - 2026-03-14

### Added

- `let` and `const` keywords (aliases for `loc`)
- JSON-style table syntax (`{key: val}`)
- Const protection (cannot reassign)

---

## [0.10.0] - 2026-03-13

### Added

- `break` and `continue` statements
- f-strings (`f"Hello {name}!"`)
- Multi-return values
- Destructuring assignment

---

## [0.9.0] - 2026-03-12

### Performance

- Replaced `Arc<Mutex<>>` with `Rc<RefCell<>>`
- Optimized variable lookup

---

## [0.8.0] - 2026-03-11

### Added

- Array index assignment (`arr[0] = 100`)
- JSON parse/stringify
- HTTP client (GET, POST, PUT, DELETE)
- Range in for-in loops
- Nil-safe operator (`?.`)
- Regex support
- Date/time functions
- Crypto functions (MD5, SHA256, Base64, UUID)

---

## [0.7.0] - 2026-03-10

### Added

- Method call syntax (`obj:method()`)
- Default parameters
- Function literals

---

## [0.6.0] - 2026-03-09

### Added

- Initial release
- Basic language features
- Standard library stubs
