# Taurine Installer for Windows
# Run as Administrator for system-wide installation

param(
    [switch]$Uninstall,
    [switch]$User
)

$ErrorActionPreference = "Stop"
$TAURINE_VERSION = "1.0.0"
$INSTALL_DIR = if ($User) {
    "$env:USERPROFILE\.taurine"
} else {
    "$env:ProgramFiles\Taurine"
}
$BIN_DIR = if ($User) {
    "$env:USERPROFILE\.taurine\bin"
} else {
    "$env:ProgramFiles\Taurine\bin"
}

function Write-Header {
    Write-Host "==================================" -ForegroundColor Cyan
    Write-Host "  Taurine v$TAURINE_VERSION Installer" -ForegroundColor Cyan
    Write-Host "==================================" -ForegroundColor Cyan
    Write-Host ""
}

function Test-Admin {
    $currentUser = New-Object Security.Principal.WindowsPrincipal([Security.Principal.WindowsIdentity]::GetCurrent())
    return $currentUser.IsInRole([Security.Principal.WindowsBuiltInRole]::Administrator)
}

function Install-Taurine {
    Write-Host "Installing Taurine v$TAURINE_VERSION..." -ForegroundColor Green
    
    # Create installation directory
    if (!(Test-Path $INSTALL_DIR)) {
        New-Item -ItemType Directory -Path $INSTALL_DIR | Out-Null
        Write-Host "  Created: $INSTALL_DIR" -ForegroundColor Gray
    }
    
    # Create bin directory
    if (!(Test-Path $BIN_DIR)) {
        New-Item -ItemType Directory -Path $BIN_DIR | Out-Null
        Write-Host "  Created: $BIN_DIR" -ForegroundColor Gray
    }
    
    # Build from source
    Write-Host ""
    Write-Host "Building Taurine from source..." -ForegroundColor Yellow
    cargo build --release
    
    # Copy binary
    $source = "target\release\taurine.exe"
    if (Test-Path $source) {
        Copy-Item $source "$BIN_DIR\taurine.exe" -Force
        Write-Host "  Copied: $BIN_DIR\taurine.exe" -ForegroundColor Gray
    } else {
        Write-Host "Error: Binary not found at $source" -ForegroundColor Red
        Write-Host "Make sure you have built Taurine first: cargo build --release" -ForegroundColor Yellow
        exit 1
    }
    
    # Add to PATH (user-wide)
    $currentPath = [Environment]::GetEnvironmentVariable("Path", "User")
    if ($currentPath -notlike "*$BIN_DIR*") {
        [Environment]::SetEnvironmentVariable("Path", "$currentPath;$BIN_DIR", "User")
        Write-Host "  Added to PATH: $BIN_DIR" -ForegroundColor Gray
    }
    
    # Copy std library
    if (Test-Path "std") {
        Copy-Item "std" "$INSTALL_DIR\std" -Recurse -Force
        Write-Host "  Copied: $INSTALL_DIR\std" -ForegroundColor Gray
    }
    
    Write-Host ""
    Write-Host "==================================" -ForegroundColor Green
    Write-Host "  Taurine installed successfully!" -ForegroundColor Green
    Write-Host "==================================" -ForegroundColor Green
    Write-Host ""
    Write-Host "Installation directory: $INSTALL_DIR" -ForegroundColor Cyan
    Write-Host "Binary location: $BIN_DIR\taurine.exe" -ForegroundColor Cyan
    Write-Host ""
    Write-Host "To get started:" -ForegroundColor Yellow
    Write-Host "  taurine --help" -ForegroundColor White
    Write-Host "  taurine --repl" -ForegroundColor White
    Write-Host ""
}

function Uninstall-Taurine {
    Write-Host "Uninstalling Taurine..." -ForegroundColor Yellow
    
    # Remove binary
    if (Test-Path "$BIN_DIR\taurine.exe") {
        Remove-Item "$BIN_DIR\taurine.exe" -Force
        Write-Host "  Removed: $BIN_DIR\taurine.exe" -ForegroundColor Gray
    }
    
    # Remove installation directory
    if (Test-Path $INSTALL_DIR) {
        Remove-Item $INSTALL_DIR -Recurse -Force
        Write-Host "  Removed: $INSTALL_DIR" -ForegroundColor Gray
    }
    
    # Remove from PATH
    $currentPath = [Environment]::GetEnvironmentVariable("Path", "User")
    if ($currentPath -like "*$BIN_DIR*") {
        $newPath = ($currentPath -split ';' | Where-Object { $_ -ne $BIN_DIR }) -join ';'
        [Environment]::SetEnvironmentVariable("Path", $newPath, "User")
        Write-Host "  Removed from PATH: $BIN_DIR" -ForegroundColor Gray
    }
    
    Write-Host ""
    Write-Host "==================================" -ForegroundColor Green
    Write-Host "  Taurine uninstalled!" -ForegroundColor Green
    Write-Host "==================================" -ForegroundColor Green
}

# Main
Write-Header

if ($Uninstall) {
    Uninstall-Taurine
} else {
    # Check for admin rights if installing system-wide
    if (!$User -and !(Test-Admin)) {
        Write-Host "Warning: Not running as Administrator. Installing for current user only." -ForegroundColor Yellow
        $User = $true
    }
    
    Install-Taurine
}
