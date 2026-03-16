# 🐂 Язык программирования Taurine

**Быстрый, встраиваемый скриптовый язык, реализованный на Rust**

Taurine сочетает простоту Lua с производительностью компилируемых языков. Чистый синтаксис, мощные встроенные функции и отличная интеграция с Rust.

## ✨ Возможности

- **Быстрое выполнение** — Оптимизированный интерпретатор с Rc/RefCell для однопоточной производительности
- **Простой синтаксис** — Lua-подобный синтаксис, лёгкий в изучении
- **Современные функции** — f-строки, multi-return, деструктуризация, nil-safe операторы
- **Богатая стандартная библиотека** — JSON, HTTP, криптография, дата/время, regex и многое другое
- **Встраиваемый** — Легко интегрируется в Rust приложения

## 🚀 Быстрый старт

### Установка

#### Windows (PowerShell)

```powershell
# Быстрая установка
curl -fsSL https://raw.githubusercontent.com/ffsonDev/taurine/main/installers/install.ps1 | Invoke-Expression

# Или вручную
Invoke-WebRequest -Uri https://github.com/ffsonDev/taurine/releases/download/v1.0.0/taurine-x86_64-pc-windows-msvc.zip -OutFile taurine.zip
Expand-Archive taurine.zip -DestinationPath $env:USERPROFILE\.taurine
```

#### Linux/macOS (Bash)

```bash
# Быстрая установка
curl -fsSL https://raw.githubusercontent.com/ffsonDev/taurine/main/installers/install.sh | bash

# Или вручную
curl -fsSL https://github.com/ffsonDev/taurine/releases/download/v1.0.0/taurine-x86_64-unknown-linux-gnu.tar.gz | tar -xzf - -C $HOME/.taurine
```

#### Из исходников

```bash
git clone https://github.com/ffsonDev/taurine.git
cd taurine
cargo build --release
```

#### Через cargo-binstall

```bash
cargo install cargo-binstall
cargo binstall taurine
```

Подробные инструкции в [installers/INSTALL.md](installers/INSTALL.md).

### Использование

```bash
# Запуск скрипта
taurine script.tau

# REPL (интерактивный режим)
taurine --repl

# С оптимизацией
taurine --optimize script.tau

# Форматирование кода
taurine --format script.tau

# Показать помощь
taurine --help
```

### Hello World

```taurine
print("Привет, Taurine!")
```

## 📖 Руководство по языку

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

### Типы данных

```taurine
// Числа
let num = 42
let float = 3.14

// Строки
let str = "Привет"
let multiline = "Строка1\nСтрока2"

// Булевы
let truthy = true
let falsy = false

// Массивы
let arr = [1, 2, 3]
let first = arr[0]
arr[0] = 100

// Таблицы (объекты)
let obj = { name: "Taurine", version: "1.0" }
print(obj.name)  // "Taurine"
```

### Функции

```taurine
// Базовая функция
function add(a, b) {
    return a + b
}

// Параметры по умолчанию
function greet(name, greeting = "Привет") {
    print(f"{greeting}, {name}!")
}

// Multi-return (несколько возвращаемых значений)
function divmod(a, b) {
    return a / b, a % b
}

// Деструктуризация возвращаемых значений
let {q, r} = divmod(10, 3)
print(f"Частное: {q}, Остаток: {r}")
```

### Управляющие конструкции

```taurine
// If/else
if x > 10 {
    print("большое")
} else {
    print("маленькое")
}

// While цикл
let i = 0
while i < 10 {
    print(i)
    i = i + 1
}

// For-in с диапазоном
for i in 1..10 {
    print(i)  // 1 до 9
}

// For-in с массивом
for item in [1, 2, 3] {
    print(item)
}

// Break и continue
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

### Интерполяция строк

```taurine
let name = "Taurine"
let version = "1.0"

// f-строки
print(f"Привет от {name} v{version}!")
print(f"2 + 2 = {2 + 2}")
```

### Nil-safe операторы

```taurine
let obj = { name: "тест" }

// Безопасный доступ к свойству
print(obj?.name)  // "тест"
print(obj?.missing)  // nil (без ошибки)

let nilObj = nil
print(nilObj?.prop)  // nil
```

### Обработка ошибок

```taurine
try {
    risky_operation()
} catch (err) {
    print(f"Ошибка: {err}")
}
```

### Модули

```taurine
// Импорт стандартной библиотеки
import "std/json.tau" as json
import "std/http.tau" as http
import "std/crypto.tau" as crypto

// Использование импортированных функций
let obj = json.parse("{ \"name\": \"тест\" }")
let hash = crypto.sha256("привет")
```

## 📚 Стандартная библиотека

### JSON

```taurine
import "std/json.tau" as json

// Парсинг JSON строки
let obj = json.parse("{ \"name\": \"тест\", \"value\": 42 }")
print(obj.name)

// Сериализация в JSON
let jsonStr = json.stringify({ name: "тест", value: 42 })
```

### Криптография

```taurine
import "std/crypto.tau" as crypto

let hash = crypto.md5("привет")
let hash256 = crypto.sha256("привет")
let encoded = crypto.base64Encode("привет")
let decoded = crypto.base64Decode(encoded)
let uuid = crypto.uuid()
```

### Дата/Время

```taurine
import "std/date.tau" as date

let now = date.now()
print(date.format(now, "%Y-%m-%d %H:%M:%S"))
```

### Регулярные выражения

```taurine
import "std/regex.tau" as regex

let pattern = regex.compile("\\d+")
print(pattern:match("abc123"))  // true
let match = pattern:find("abc123")
print(match.text)  // "123"
```

### Массивы

```taurine
import "std/array.tau" as array

let arr = [1, 2, 3, 4, 5]
print(array.len(arr))  // 5
print(array.includes(arr, 3))  // true

// Функциональные методы
let doubled = array.map(arr, x => x * 2)
let evens = array.filter(arr, x => x % 2 == 0)
```

## 🔧 Продвинутые функции

### Функциональные литералы

```taurine
// Анонимная функция в таблице
let obj = {
    name: "тест",
    greet: function(self) {
        print(f"Привет, {self.name}!")
    }
}
```

### Методы таблиц

```taurine
let counter = { count: 0 }

function increment(self, amount) {
    self.count = self.count + amount
}

increment(counter, 5)
print(counter.count)  // 5
```

## 📊 Производительность

Taurine v1.0 обеспечивает значительное улучшение производительности:

- **Rc<RefCell<>>** вместо Arc<Mutex<>> для однопоточной эффективности
- **Constant folding** оптимизатор
- **Dead code elimination**
- **Кэширование поиска переменных**

## 📝 Примеры

Примеры кода доступны в папке [examples/](../examples/) и в [документации с примерами](docs/EXAMPLES_RU.md):

- `hello.tau` — Привет, мир!
- `fibonacci.tau` — Числа Фибоначчи
- `json_example.tau` — Работа с JSON
- `crypto_example.tau` — Криптография
- `regex_example.tau` — Регулярные выражения
- `arrays.tau` — Операции с массивами
- `date_example.tau` — Дата и время
- `test_example.tau` — Тестирование

## 🤝 Вклад в проект

Мы приветствуем вклад! Пожалуйста, не стесняйтесь отправлять Pull Request.

### Разработка

```bash
# Клонировать репозиторий
git clone https://github.com/ffsonDev/taurine.git
cd taurine

# Запустить тесты
cargo test

# Запустить clippy
cargo clippy

# Отформатировать код
cargo fmt

# Собрать релиз
cargo build --release
```

## 📄 Лицензия

MIT License — см. файл LICENSE для деталей.

## 🙏 Благодарности

Taurine вдохновлён:
- **Lua** — Простота и встраиваемость
- **Rust** — Безопасность и производительность
- **JavaScript** — Современные функции синтаксиса

## 🔗 Ссылки

- [GitHub Repository](https://github.com/ffsonDev/taurine)
- [Документация](docs/GUIDE_RU.md)
- [Примеры](docs/EXAMPLES_RU.md)
- [Changelog](docs/CHANGELOG_RU.md)
- [Заметки о релизе](RELEASE_NOTES_RU.md)
- [Установка](installers/INSTALL_RU.md)
- [Заметки о релизе](RELEASE_NOTES.md)

---

**Счастливого кодинга с Taurine! 🐂**
