# Руководство по установке Taurine

## Быстрая установка

### Windows (PowerShell)

```powershell
# Установка для текущего пользователя
curl -fsSL https://raw.githubusercontent.com/ffsonDev/taurine/main/installers/install.ps1 | Invoke-Expression

# Или скачать и запустить вручную
Invoke-WebRequest -Uri https://raw.githubusercontent.com/ffsonDev/taurine/main/installers/install.ps1 -OutFile install.ps1
.\install.ps1 -User
```

### Linux/macOS (Bash)

```bash
# Быстрая установка
curl -fsSL https://raw.githubusercontent.com/ffsonDev/taurine/main/installers/install.sh | bash

# Или скачать и запустить вручную
curl -fsSL https://raw.githubusercontent.com/ffsonDev/taurine/main/installers/install.sh -o install.sh
chmod +x install.sh
./install.sh
```

## Способы установки

### 1. Готовый бинарный файл (Рекомендуется)

Скачайте последнюю версию со страницы [GitHub Releases](https://github.com/ffsonDev/taurine/releases):

- **Windows**: `taurine-x86_64-pc-windows-msvc.zip`
- **Linux**: `taurine-x86_64-unknown-linux-gnu.tar.gz`
- **macOS**: `taurine-aarch64-apple-darwin.tar.gz` (Apple Silicon)
- **macOS**: `taurine-x86_64-apple-darwin.tar.gz` (Intel)

Распакуйте и добавьте в PATH.

### 2. Установка из исходников

**Требования:**
- Rust 1.70 или новее
- Cargo

```bash
# Клонировать репозиторий
git clone https://github.com/ffsonDev/taurine.git
cd taurine

# Собрать релиз
cargo build --release

# Расположение бинарного файла
./target/release/taurine
```

### 3. Через cargo-binstall

```bash
# Сначала установите cargo-binstall
cargo install cargo-binstall

# Установите Taurine
cargo binstall taurine
```

### 4. Через cargo install

```bash
cargo install taurine
```

## Переменные окружения

| Переменная | Описание | По умолчанию |
|------------|----------|--------------|
| `TAURINE_INSTALL_DIR` | Каталог установки | `~/.taurine` |
| `TAURINE_REPO_URL` | URL репозитория для загрузок | GitHub releases |

## Деинсталляция

### Windows

```powershell
# Через установщик
.\install.ps1 -Uninstall

# Вручную
Remove-Item -Path "$env:USERPROFILE\.taurine" -Recurse -Force
# Или для системной установки:
Remove-Item -Path "$env:ProgramFiles\Taurine" -Recurse -Force
```

### Linux/macOS

```bash
# Через установщик
./install.sh --uninstall

# Вручную
rm -rf ~/.taurine
```

## Проверка установки

```bash
# Проверить версию
taurine --version

# Запустить REPL
taurine --repl

# Запустить скрипт
echo 'print("Привет!")' > test.tau
taurine test.tau
```

## Решение проблем

### "Команда не найдена"

Убедитесь, что Taurine добавлен в PATH:

```bash
# Linux/macOS
export PATH="$HOME/.taurine/bin:$PATH"

# Windows (PowerShell)
$env:PATH += ";$env:USERPROFILE\.taurine\bin"
```

### Permission denied (Linux/macOS)

```bash
chmod +x ~/.taurine/bin/taurine
```

### Ошибка сборки (Rust)

Обновите Rust:

```bash
rustup update
```

### Отсутствует стандартная библиотека

Скопируйте папку std в каталог установки:

```bash
# Linux/macOS
cp -r std ~/.taurine/

# Windows
Copy-Item std $env:USERPROFILE\.taurine\
```

## Системные требования

- **ОС**: Windows 10+, Linux (glibc 2.17+), macOS 10.15+
- **Архитектура**: x86_64, aarch64 (ARM64)
- **Память**: 50МБ минимум
- **Диск**: 20МБ свободного места

## Следующие шаги

После установки:

1. Выполните `taurine --help` для просмотра доступных команд
2. Попробуйте `taurine --repl` для интерактивного режима
3. Проверьте [examples/](../examples/) для примеров кода
4. Прочитайте [документацию](GUIDE_RU.md)
