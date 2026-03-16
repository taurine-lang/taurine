# Примеры кода Taurine

## Привет, мир!

```taurine
print("Привет, мир!")
```

## Переменные

```taurine
// Изменяемые
let x = 10
x = 100

// Константы
const PI = 3.14

// Legacy
loc y = 20
```

## Функции

```taurine
// Базовая
function add(a, b) {
    return a + b
}

// Параметры по умолчанию
function greet(name, greeting = "Привет") {
    print(f"{greeting}, {name}!")
}

// Multi-return
function divmod(a, b) {
    return a / b, a % b
}

let {q, r} = divmod(10, 3)
print(f"Частное: {q}, Остаток: {r}")
```

## Массивы

```taurine
let arr = [1, 2, 3, 4, 5]

// Доступ
print(arr[0])  // 1
arr[0] = 100

// Итерация
for item in arr {
    print(item)
}

// Длина
print(#arr)
```

## Таблицы

```taurine
// JSON стиль
let obj = { name: "Taurine", version: "1.0" }

// Legacy стиль
let obj2 = { name = "тест", value = 42 }

// Доступ
print(obj.name)
print(obj?.missing)  // nil (безопасно)
```

## Управляющие конструкции

```taurine
// If/else
if x > 10 {
    print("большое")
} else {
    print("маленькое")
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

## Интерполяция строк

```taurine
let name = "Taurine"
let version = "1.0"

print(f"Привет от {name} v{version}!")
print(f"2 + 2 = {2 + 2}")
```

## Обработка ошибок

```taurine
try {
    throw "Что-то пошло не так"
} catch (err) {
    print(f"Ошибка: {err}")
}
```

## Модули

```taurine
// JSON
import "std/json.tau" as json
let obj = json.parse("{ \"name\": \"тест\" }")

// Криптография
import "std/crypto.tau" as crypto
let hash = crypto.sha256("привет")

// Дата
import "std/date.tau" as date
print(date.format(date.now(), "%Y-%m-%d"))

// Regex
import "std/regex.tau" as regex
let pattern = regex.compile("\\d+")
print(pattern:match("abc123"))
```

## Числа Фибоначчи

```taurine
function fib(n) {
    if n <= 1 { return n }
    return fib(n - 1) + fib(n - 2)
}

for i in 1..10 {
    print(f"fib({i}) = {fib(i)}")
}
```

## Факториал

```taurine
function factorial(n) {
    if n <= 1 { return 1 }
    return n * factorial(n - 1)
}

print(factorial(5))  // 120
```

## Операции с массивами

```taurine
import "std/array.tau" as array

let arr = [1, 2, 3, 4, 5]

// Map
let doubled = array.map(arr, x => x * 2)

// Filter
let evens = array.filter(arr, x => x % 2 == 0)

// Reduce
let sum = array.reduce(arr, (a, b) => a + b, 0)

print(f"Удвоено: {doubled}")
print(f"Чётные: {evens}")
print(f"Сумма: {sum}")
```

## Тестирование

```taurine
import "std/test.tau" as test

test.describe("Математика", function() {
    test.it("должно складывать", function() {
        assert(1 + 2 == 3)
        assert_eq(5 * 5, 25)
    })
})

test.run_tests()
```
