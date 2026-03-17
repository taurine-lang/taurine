# Taurine Embedding Examples

This directory contains examples of embedding Taurine in various programming languages.

## Rust Examples

### 01_basic_rust.rs

Basic example of running Taurine code from Rust.

```bash
cargo run --example 01_basic_rust
```

### 02_getting_values.rs

Example of getting values from Taurine interpreter.

```bash
cargo run --example 02_getting_values
```

### 03_rust_functions.rs

Example of using Taurine features from Rust.

```bash
cargo run --example 03_rust_functions
```

## C/C++ Examples

### 04_c_api.c

Demonstration of the C API structure.

```bash
# Compile (requires libtaurine)
gcc -o 04_c_api 04_c_api.c -ltaurine

# Run
./04_c_api
```

### 05_cpp_api.cpp

C++ wrapper class demonstration.

```bash
# Compile (requires libtaurine)
g++ -o 05_cpp_api 05_cpp_api.cpp -ltaurine -std=c++11

# Run
./05_cpp_api
```

## Scripting Language Bindings

### 06_python.py

Demonstration of Python binding structure (mock).

```bash
python 06_python.py
```

### 07_nodejs.js

Demonstration of Node.js binding structure (mock).

```bash
node 07_nodejs.js
```

## API Reference

### Rust API

```rust
use taurine::Interpreter;

// Create interpreter
let mut interp = Interpreter::new();

// Run code
interp.run("print(\"Hello!\")")?;

// Get value
let value = interp.get("variable_name")?;
```

### C API

```c
#include "taurine.h"

// Create VM
TaurineVM* vm = taurine_new();

// Run code
taurine_run(vm, "print(\"Hello!\")");

// Get error
const char* error = taurine_get_error(vm);

// Free VM
taurine_free(vm);
```

## More Information

For detailed embedding documentation, see the [Taurine Documentation](https://ffsondev.github.io/taurinedev).
