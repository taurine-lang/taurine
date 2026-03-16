# Taurine Language Guide

## Table of Contents

1. [Introduction](#introduction)
2. [Getting Started](#getting-started)
3. [Basic Syntax](#basic-syntax)
4. [Data Types](#data-types)
5. [Operators](#operators)
6. [Control Flow](#control-flow)
7. [Functions](#functions)
8. [Tables and Arrays](#tables-and-arrays)
9. [Error Handling](#error-handling)
10. [Modules](#modules)

---

## Introduction

Taurine is a fast, embeddable scripting language implemented in Rust. It combines Lua's simplicity with modern language features.

### Design Goals

- **Simple** — Easy to learn and use
- **Fast** — Optimized for single-threaded performance
- **Embeddable** — Easy to integrate into Rust applications
- **Modern** — f-strings, destructuring, nil-safe operators

---

## Getting Started

### Installation

```bash
cargo build --release
```

### Running Scripts

```bash
# Run a script
./target/release/taurine script.tau

# Start REPL
./target/release/taurine --repl

# With optimizations
./target/release/taurine --optimize script.tau
```

---

## Basic Syntax

### Comments

```taurine
// Single line comment

/* Multi-line comments are not supported yet */
```

### Variables

```taurine
// Mutable variable
let x = 10
x = 100

// Constant (cannot be changed)
const PI = 3.14
// PI = 3  // Error!

// Legacy syntax (still supported)
loc y = 20
```

### Identifiers

- Start with letter or underscore
- Can contain letters, digits, underscores
- Case-sensitive

```taurine
let name = "test"
let _private = "hidden"
let camelCase = true
let snake_case = true
```

---

## Data Types

### Numbers

64-bit floating point numbers:

```taurine
let int = 42
let float = 3.14
let negative = -10
let scientific = 1e10
```

### Strings

UTF-8 strings with escape sequences:

```taurine
let str = "Hello, World!"
let multiline = "Line1\nLine2"
let tabbed = "Col1\tCol2"
let quoted = "He said \"Hello\""
```

### Booleans

```taurine
let truthy = true
let falsy = false
```

### Nil

Represents absence of value:

```taurine
let empty = nil
```

### Arrays

Ordered lists of values:

```taurine
let numbers = [1, 2, 3, 4, 5]
let mixed = [1, "two", true, nil]
let nested = [[1, 2], [3, 4]]
```

### Tables

Key-value maps (like objects/dictionaries):

```taurine
let person = { name: "John", age: 30 }
let empty = {}
let nested = { outer: { inner: "value" } }
```

### Ranges

Used in for-in loops:

```taurine
// 1 to 9 (exclusive end)
for i in 1..10 { print(i) }
```

---

## Operators

### Arithmetic

```taurine
let sum = 1 + 2       // 3
let diff = 5 - 3      // 2
let prod = 4 * 5      // 20
let quot = 10 / 2     // 5
let neg = -5          // -5
```

### Comparison

```taurine
let eq = (1 == 1)     // true
let ne = (1 != 2)     // true
let lt = (1 < 2)      // true
let gt = (2 > 1)      // true
let le = (1 <= 1)     // true
let ge = (2 >= 2)     // true
```

### Logical

```taurine
let and = (true and false)  // false
let or = (true or false)    // true
let not = not true          // false
```

### String Concatenation

```taurine
let str = "Hello" + " " + "World"  // "Hello World"
let mixed = "Value: " + 42         // "Value: 42"
```

### Length

```taurine
let len = #[1, 2, 3]    // 3
let slen = #"hello"     // 5
let tlen = #{a: 1, b: 2} // 2
```

---

## Control Flow

### If/Else

```taurine
if x > 10 {
    print("big")
} else if x > 5 {
    print("medium")
} else {
    print("small")
}
```

### While Loop

```taurine
let i = 0
while i < 10 {
    print(i)
    i = i + 1
}
```

### For-In Loop

```taurine
// Range
for i in 1..10 {
    print(i)
}

// Array
for item in [1, 2, 3] {
    print(item)
}

// String (character by character)
for ch in "hello" {
    print(ch)
}
```

### Break/Continue

```taurine
for i in 1..10 {
    if i == 5 {
        break  // Exit loop
    }
    if i == 3 {
        continue  // Skip to next iteration
    }
    print(i)
}
```

---

## Functions

### Definition

```taurine
function add(a, b) {
    return a + b
}
```

### Default Parameters

```taurine
function greet(name, greeting = "Hello") {
    print(f"{greeting}, {name}!")
}

greet("World")           // "Hello, World!"
greet("World", "Hi")     // "Hi, World!"
```

### Multi-Return

```taurine
function divmod(a, b) {
    return a / b, a % b
}

// Capture all returns
let result = divmod(10, 3)

// Destructure
let {q, r} = divmod(10, 3)
```

### Function Literals

```taurine
let add = function(a, b) {
    return a + b
}

// In tables
let obj = {
    name: "test",
    greet: function(self) {
        print(f"Hello, {self.name}!")
    }
}
```

---

## Tables and Arrays

### Array Operations

```taurine
import "std/array.tau" as array

let arr = [1, 2, 3]

// Length
let len = #arr

// Access
let first = arr[0]
arr[0] = 100

// Operations
array.push(arr, 4)
let last = array.pop(arr)
let doubled = array.map(arr, x => x * 2)
let evens = array.filter(arr, x => x % 2 == 0)
```

### Table Operations

```taurine
let obj = { name: "test", value: 42 }

// Access
print(obj.name)
print(obj["name"])

// Modify
obj.name = "new"
obj.newField = "added"

// Nil-safe access
print(obj?.missing)  // nil (no error)
```

---

## Error Handling

### Try/Catch

```taurine
try {
    risky_operation()
} catch (err) {
    print(f"Error: {err}")
}
```

### Throw

```taurine
function divide(a, b) {
    if b == 0 {
        throw "Division by zero"
    }
    return a / b
}
```

### Assertions

```taurine
import "std/test.tau" as test

assert(x > 0, "x must be positive")
assert_eq(a, b, "a and b should be equal")
```

---

## Modules

### Import

```taurine
// Import with alias
import "std/json.tau" as json

// Import without alias (uses filename)
import "std/crypto.tau" as crypto
```

### Standard Library

- `std/json.tau` — JSON parsing and stringification
- `std/http.tau` — HTTP client (GET, POST, PUT, DELETE)
- `std/crypto.tau` — MD5, SHA256, Base64, UUID
- `std/date.tau` — Date/time formatting
- `std/regex.tau` — Regular expressions
- `std/array.tau` — Array utilities
- `std/string.tau` — String utilities
- `std/io.tau` — File I/O
- `std/math.tau` — Math functions
- `std/os.tau` — OS functions
