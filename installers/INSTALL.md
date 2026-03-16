# Taurine Installation Guide

## Quick Install

### Windows (PowerShell)

```powershell
# Install for current user
curl -fsSL https://raw.githubusercontent.com/ffsonDev/taurine/main/installers/install.ps1 | Invoke-Expression

# Or download and run manually
Invoke-WebRequest -Uri https://raw.githubusercontent.com/ffsonDev/taurine/main/installers/install.ps1 -OutFile install.ps1
.\install.ps1 -User
```

### Linux/macOS (Bash)

```bash
# Quick install
curl -fsSL https://raw.githubusercontent.com/ffsonDev/taurine/main/installers/install.sh | bash

# Or download and run manually
curl -fsSL https://raw.githubusercontent.com/ffsonDev/taurine/main/installers/install.sh -o install.sh
chmod +x install.sh
./install.sh
```

## Installation Methods

### 1. Pre-built Binary (Recommended)

Download the latest release from [GitHub Releases](https://github.com/ffsonDev/taurine/releases):

- **Windows**: `taurine-x86_64-pc-windows-msvc.zip`
- **Linux**: `taurine-x86_64-unknown-linux-gnu.tar.gz`
- **macOS**: `taurine-aarch64-apple-darwin.tar.gz` (Apple Silicon)
- **macOS**: `taurine-x86_64-apple-darwin.tar.gz` (Intel)

Extract and add to PATH.

### 2. Install from Source

**Requirements:**
- Rust 1.70 or later
- Cargo

```bash
# Clone repository
git clone https://github.com/ffsonDev/taurine.git
cd taurine

# Build release
cargo build --release

# Binary location
./target/release/taurine
```

### 3. Using cargo-binstall

```bash
# Install cargo-binstall first
cargo install cargo-binstall

# Install Taurine
cargo binstall taurine
```

### 4. Using cargo install

```bash
cargo install taurine
```

## Environment Variables

| Variable | Description | Default |
|----------|-------------|---------|
| `TAURINE_INSTALL_DIR` | Installation directory | `~/.taurine` |
| `TAURINE_REPO_URL` | Repository URL for downloads | GitHub releases |

## Uninstall

### Windows

```powershell
# Using installer
.\install.ps1 -Uninstall

# Manual
Remove-Item -Path "$env:USERPROFILE\.taurine" -Recurse -Force
# Or for system-wide:
Remove-Item -Path "$env:ProgramFiles\Taurine" -Recurse -Force
```

### Linux/macOS

```bash
# Using installer
./install.sh --uninstall

# Manual
rm -rf ~/.taurine
```

## Verify Installation

```bash
# Check version
taurine --version

# Run REPL
taurine --repl

# Run a script
echo 'print("Hello!")' > test.tau
taurine test.tau
```

## Troubleshooting

### "Command not found"

Make sure Taurine is in your PATH:

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

### Build fails (Rust)

Update Rust:

```bash
rustup update
```

### Missing std library

Copy std folder to installation directory:

```bash
# Linux/macOS
cp -r std ~/.taurine/

# Windows
Copy-Item std $env:USERPROFILE\.taurine\
```

## System Requirements

- **OS**: Windows 10+, Linux (glibc 2.17+), macOS 10.15+
- **Architecture**: x86_64, aarch64 (ARM64)
- **Memory**: 50MB RAM minimum
- **Disk**: 20MB free space

## Next Steps

After installation:

1. Run `taurine --help` to see available commands
2. Try `taurine --repl` for interactive mode
3. Check out [examples/](../examples/) for sample code
4. Read the [documentation](../docs/GUIDE.md)
