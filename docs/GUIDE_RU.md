# Руководство по языку Taurine

## Содержание

1. [Введение](#введение)
2. [Начало работы](#начало-работы)
3. [Базовый синтаксис](#базовый-синтаксис)
4. [Типы данных](#типы-данных)
5. [Операторы](#операторы)
6. [Управляющие конструкции](#управляющие-конструкции)
7. [Функции](#функции)
8. [Таблицы и массивы](#таблицы-и-массивы)
9. [Обработка ошибок](#обработка-ошибок)
10. [Модули](#модули)

---

## Введение

Taurine — быстрый, встраиваемый скриптовый язык, реализованный на Rust. Он сочетает простоту Lua с современными функциями языка.

### Цели дизайна

- **Простой** — Лёгкий в изучении и использовании
- **Быстрый** — Оптимизирован для однопоточной производительности
- **Встраиваемый** — Легко интегрируется в Rust приложения
- **Современный** — f-строки, деструктуризация, nil-safe операторы

---

## Начало работы

### Установка

```bash
cargo build --release
```

### Запуск скриптов

```bash
# Запуск скрипта
./target/release/taurine script.tau

# Запуск REPL
./target/release/taurine --repl

# С оптимизациями
./target/release/taurine --optimize script.tau
```

---

## Базовый синтаксис

### Комментарии

```taurine
// Однострочный комментарий

/* Многострочные комментарии пока не поддерживаются */
```

### Переменные

```taurine
// Изменяемая переменная
let x = 10
x = 100

// Константа (нельзя изменить)
const PI = 3.14
// PI = 3  // Ошибка!

// Legacy синтаксис (всё ещё работает)
loc y = 20
```

### Идентификаторы

- Начинаются с буквы или подчёркивания
- Могут содержать буквы, цифры, подчёркивания
- Чувствительны к регистру

```taurine
let name = "тест"
let _private = "скрытое"
let camelCase = true
let snake_case = true
```

---

## Типы данных

### Числа

64-битные числа с плавающей точкой:

```taurine
let int = 42
let float = 3.14
let negative = -10
let scientific = 1e10
```

### Строки

UTF-8 строки с escape-последовательностями:

```taurine
let str = "Привет, Мир!"
let multiline = "Строка1\nСтрока2"
let tabbed = "Кол1\tКол2"
let quoted = "Он сказал \"Привет\""
```

### Булевы значения

```taurine
let truthy = true
let falsy = false
```

### Nil

Представляет отсутствие значения:

```taurine
let empty = nil
```

### Массивы

Упорядоченные списки значений:

```taurine
let numbers = [1, 2, 3, 4, 5]
let mixed = [1, "два", true, nil]
let nested = [[1, 2], [3, 4]]
```

### Таблицы

Ассоциативные массивы (как объекты/словари):

```taurine
let person = { name: "Иван", age: 30 }
let empty = {}
let nested = { outer: { inner: "значение" } }
```

### Диапазоны

Используются в циклах for-in:

```taurine
// 1 до 9 (исключая 10)
for i in 1..10 { print(i) }
```

---

## Операторы

### Арифметические

```taurine
let sum = 1 + 2       // 3
let diff = 5 - 3      // 2
let prod = 4 * 5      // 20
let quot = 10 / 2     // 5
let neg = -5          // -5
```

### Сравнения

```taurine
let eq = (1 == 1)     // true
let ne = (1 != 2)     // true
let lt = (1 < 2)      // true
let gt = (2 > 1)      // true
let le = (1 <= 1)     // true
let ge = (2 >= 2)     // true
```

### Логические

```taurine
let and = (true and false)  // false
let or = (true or false)    // true
let not = not true          // false
```

### Конкатенация строк

```taurine
let str = "Привет" + " " + "Мир"  // "Привет Мир"
let mixed = "Значение: " + 42     // "Значение: 42"
```

### Длина

```taurine
let len = #[1, 2, 3]    // 3
let slen = #"привет"    // 6
let tlen = #{a: 1, b: 2} // 2
```

---

## Управляющие конструкции

### If/Else

```taurine
if x > 10 {
    print("большое")
} else if x > 5 {
    print("среднее")
} else {
    print("маленькое")
}
```

### While цикл

```taurine
let i = 0
while i < 10 {
    print(i)
    i = i + 1
}
```

### For-In цикл

```taurine
// Диапазон
for i in 1..10 {
    print(i)
}

// Массив
for item in [1, 2, 3] {
    print(item)
}

// Строка (посимвольно)
for ch in "привет" {
    print(ch)
}
```

### Break/Continue

```taurine
for i in 1..10 {
    if i == 5 {
        break  // Выход из цикла
    }
    if i == 3 {
        continue  // Переход к следующей итерации
    }
    print(i)
}
```

---

## Функции

### Определение

```taurine
function add(a, b) {
    return a + b
}
```

### Параметры по умолчанию

```taurine
function greet(name, greeting = "Привет") {
    print(f"{greeting}, {name}!")
}

greet("Мир")           // "Привет, Мир!"
greet("Мир", "Hi")     // "Hi, Мир!"
```

### Multi-Return

```taurine
function divmod(a, b) {
    return a / b, a % b
}

// Захват всех возвращаемых значений
let result = divmod(10, 3)

// Деструктуризация
let {q, r} = divmod(10, 3)
```

### Функциональные литералы

```taurine
let add = function(a, b) {
    return a + b
}

// В таблицах
let obj = {
    name: "тест",
    greet: function(self) {
        print(f"Привет, {self.name}!")
    }
}
```

---

## Таблицы и массивы

### Операции с массивами

```taurine
import "std/array.tau" as array

let arr = [1, 2, 3]

// Длина
let len = #arr

// Доступ
let first = arr[0]
arr[0] = 100

// Операции
array.push(arr, 4)
let last = array.pop(arr)
let doubled = array.map(arr, x => x * 2)
let evens = array.filter(arr, x => x % 2 == 0)
```

### Операции с таблицами

```taurine
let obj = { name: "тест", value: 42 }

// Доступ
print(obj.name)
print(obj["name"])

// Изменение
obj.name = "новый"
obj.newField = "добавлено"

// Nil-safe доступ
print(obj?.missing)  // nil (без ошибки)
```

---

## Обработка ошибок

### Try/Catch

```taurine
try {
    risky_operation()
} catch (err) {
    print(f"Ошибка: {err}")
}
```

### Throw

```taurine
function divide(a, b) {
    if b == 0 {
        throw "Деление на ноль"
    }
    return a / b
}
```

### Assertions

```taurine
import "std/test.tau" as test

assert(x > 0, "x должно быть положительным")
assert_eq(a, b, "a и b должны быть равны")
```

---

## Модули

### Импорт

```taurine
// Импорт с псевдонимом
import "std/json.tau" as json

// Импорт без псевдонима (используется имя файла)
import "std/crypto.tau" as crypto
```

### Стандартная библиотека

- `std/json.tau` — Парсинг и сериализация JSON
- `std/http.tau` — HTTP клиент (GET, POST, PUT, DELETE)
- `std/crypto.tau` — Криптографические функции
- `std/date.tau` — Дата/время
- `std/regex.tau` — Регулярные выражения
- `std/array.tau` — Утилиты для массивов
- `std/string.tau` — Строковые утилиты
- `std/io.tau` — Файловый ввод/вывод
- `std/math.tau` — Математические функции
- `std/os.tau` — Функции ОС
