# Examples

## Hello World

```taurine
print("Hello, World!")
```

## Variables

```taurine
// Mutable
let x = 10
x = 100

// Constant
const PI = 3.14

// Legacy
loc y = 20
```

## Functions

```taurine
// Basic
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

let {q, r} = divmod(10, 3)
print(f"Quotient: {q}, Remainder: {r}")
```

## Arrays

```taurine
let arr = [1, 2, 3, 4, 5]

// Access
print(arr[0])  // 1
arr[0] = 100

// Iterate
for item in arr {
    print(item)
}

// Length
print(#arr)
```

## Tables

```taurine
// JSON style
let obj = { name: "Taurine", version: "1.0" }

// Legacy style
let obj2 = { name = "test", value = 42 }

// Access
print(obj.name)
print(obj?.missing)  // nil (safe)
```

## Control Flow

```taurine
// If/else
if x > 10 {
    print("big")
} else {
    print("small")
}

// While
let i = 0
while i < 10 {
    i = i + 1
}

// For-in
for i in 1..10 {
    if i == 5 { break }
    if i == 3 { continue }
    print(i)
}
```

## String Interpolation

```taurine
let name = "Taurine"
let version = "1.0"

print(f"Hello from {name} v{version}!")
print(f"2 + 2 = {2 + 2}")
```

## Error Handling

```taurine
try {
    throw "Something went wrong"
} catch (err) {
    print(f"Error: {err}")
}
```

## Modules

```taurine
// JSON
import "std/json.tau" as json
let obj = json.parse("{ \"name\": \"test\" }")

// Crypto
import "std/crypto.tau" as crypto
let hash = crypto.sha256("hello")

// Date
import "std/date.tau" as date
print(date.format(date.now(), "%Y-%m-%d"))

// Regex
import "std/regex.tau" as regex
let pattern = regex.compile("\\d+")
print(pattern:match("abc123"))
```

## Fibonacci

```taurine
function fib(n) {
    if n <= 1 { return n }
    return fib(n - 1) + fib(n - 2)
}

for i in 1..10 {
    print(f"fib({i}) = {fib(i)}")
}
```

## Factorial

```taurine
function factorial(n) {
    if n <= 1 { return 1 }
    return n * factorial(n - 1)
}

print(factorial(5))  // 120
```

## Array Operations

```taurine
import "std/array.tau" as array

let arr = [1, 2, 3, 4, 5]

// Map
let doubled = array.map(arr, x => x * 2)

// Filter
let evens = array.filter(arr, x => x % 2 == 0)

// Reduce
let sum = array.reduce(arr, (a, b) => a + b, 0)

print(f"Doubled: {doubled}")
print(f"Evens: {evens}")
print(f"Sum: {sum}")
```
