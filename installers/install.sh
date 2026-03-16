#!/bin/bash
# Taurine Installer for Linux/macOS
# Usage: curl -fsSL https://.../install.sh | bash

set -e

TAURINE_VERSION="1.0.0"
INSTALL_DIR="${TAURINE_INSTALL_DIR:-$HOME/.taurine}"
BIN_DIR="$INSTALL_DIR/bin"
REPO_URL="${TAURINE_REPO_URL:-https://github.com/ffsonDev/taurine}"

# Colors
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
CYAN='\033[0;36m'
NC='\033[0m' # No Color

header() {
    echo -e "${CYAN}==================================${NC}"
    echo -e "${CYAN}  Taurine v${TAURINE_VERSION} Installer${NC}"
    echo -e "${CYAN}==================================${NC}"
    echo ""
}

install_taurine() {
    echo -e "${GREEN}Installing Taurine v${TAURINE_VERSION}...${NC}"
    
    # Create installation directory
    if [ ! -d "$INSTALL_DIR" ]; then
        mkdir -p "$INSTALL_DIR"
        echo -e "  ${GRAY}Created: $INSTALL_DIR${NC}"
    fi
    
    # Create bin directory
    if [ ! -d "$BIN_DIR" ]; then
        mkdir -p "$BIN_DIR"
        echo -e "  ${GRAY}Created: $BIN_DIR${NC}"
    fi
    
    # Detect OS and architecture
    OS=$(uname -s | tr '[:upper:]' '[:lower:]')
    ARCH=$(uname -m)
    
    case "$ARCH" in
        x86_64) ARCH="x86_64" ;;
        aarch64|arm64) ARCH="aarch64" ;;
        *) echo -e "${RED}Unsupported architecture: $ARCH${NC}"; exit 1 ;;
    esac
    
    # Download pre-built binary or build from source
    BINARY_URL="$REPO_URL/releases/download/v${TAURINE_VERSION}/taurine-${ARCH}-${OS}.tar.gz"
    
    echo ""
    echo -e "${YELLOW}Attempting to download pre-built binary...${NC}"
    
    if command -v curl &> /dev/null; then
        if curl -fsSL --head "$BINARY_URL" &> /dev/null; then
            echo -e "  ${GRAY}Downloading: $BINARY_URL${NC}"
            curl -fsSL "$BINARY_URL" -o /tmp/taurine.tar.gz
            tar -xzf /tmp/taurine.tar.gz -C "$BIN_DIR"
            rm /tmp/taurine.tar.gz
            echo -e "  ${GRAY}Installed: $BIN_DIR/taurine${NC}"
        else
            echo -e "  ${YELLOW}Pre-built binary not found. Building from source...${NC}"
            build_from_source
        fi
    elif command -v wget &> /dev/null; then
        if wget --spider "$BINARY_URL" &> /dev/null; then
            echo -e "  ${GRAY}Downloading: $BINARY_URL${NC}"
            wget -q "$BINARY_URL" -O /tmp/taurine.tar.gz
            tar -xzf /tmp/taurine.tar.gz -C "$BIN_DIR"
            rm /tmp/taurine.tar.gz
            echo -e "  ${GRAY}Installed: $BIN_DIR/taurine${NC}"
        else
            echo -e "  ${YELLOW}Pre-built binary not found. Building from source...${NC}"
            build_from_source
        fi
    else
        echo -e "  ${YELLOW}Neither curl nor wget found. Building from source...${NC}"
        build_from_source
    fi
    
    # Make executable
    chmod +x "$BIN_DIR/taurine"
    
    # Add to PATH
    if ! echo "$PATH" | grep -q "$BIN_DIR"; then
        SHELL_RC=""
        if [ -f "$HOME/.bashrc" ]; then
            SHELL_RC="$HOME/.bashrc"
        elif [ -f "$HOME/.zshrc" ]; then
            SHELL_RC="$HOME/.zshrc"
        elif [ -f "$HOME/.bash_profile" ]; then
            SHELL_RC="$HOME/.bash_profile"
        fi
        
        if [ -n "$SHELL_RC" ]; then
            if ! grep -q "$BIN_DIR" "$SHELL_RC"; then
                echo "" >> "$SHELL_RC"
                echo "# Taurine" >> "$SHELL_RC"
                echo "export PATH=\"$BIN_DIR:\$PATH\"" >> "$SHELL_RC"
                echo -e "  ${GRAY}Added to PATH in $SHELL_RC${NC}"
            fi
        fi
    fi
    
    echo ""
    echo -e "${GREEN}==================================${NC}"
    echo -e "${GREEN}  Taurine installed successfully!${NC}"
    echo -e "${GREEN}==================================${NC}"
    echo ""
    echo -e "Installation directory: ${CYAN}$INSTALL_DIR${NC}"
    echo -e "Binary location: ${CYAN}$BIN_DIR/taurine${NC}"
    echo ""
    echo -e "${YELLOW}To get started:${NC}"
    echo -e "  ${WHITE}taurine --help${NC}"
    echo -e "  ${WHITE}taurine --repl${NC}"
    echo ""
    echo -e "${YELLOW}Note: You may need to restart your terminal or run:${NC}"
    echo -e "  ${WHITE}source $SHELL_RC${NC}"
}

build_from_source() {
    echo -e "${YELLOW}Building Taurine from source...${NC}"
    
    # Check for Rust
    if ! command -v cargo &> /dev/null; then
        echo -e "${RED}Error: Rust/Cargo not found.${NC}"
        echo -e "${YELLOW}Please install Rust from https://rustup.rs/${NC}"
        exit 1
    fi
    
    # Build
    cargo build --release
    
    # Copy binary
    cp "target/release/taurine" "$BIN_DIR/taurine"
    echo -e "  ${GRAY}Built and installed: $BIN_DIR/taurine${NC}"
}

uninstall_taurine() {
    echo -e "${YELLOW}Uninstalling Taurine...${NC}"
    
    # Remove binary
    if [ -f "$BIN_DIR/taurine" ]; then
        rm -f "$BIN_DIR/taurine"
        echo -e "  ${GRAY}Removed: $BIN_DIR/taurine${NC}"
    fi
    
    # Remove installation directory
    if [ -d "$INSTALL_DIR" ]; then
        rm -rf "$INSTALL_DIR"
        echo -e "  ${GRAY}Removed: $INSTALL_DIR${NC}"
    fi
    
    # Remove from PATH
    for rc_file in "$HOME/.bashrc" "$HOME/.zshrc" "$HOME/.bash_profile"; do
        if [ -f "$rc_file" ]; then
            sed -i.bak '/# Taurine/,/export PATH=".*\.taurine.*:.*PATH"/d' "$rc_file" 2>/dev/null || true
            rm -f "$rc_file.bak" 2>/dev/null || true
        fi
    done
    
    echo ""
    echo -e "${GREEN}==================================${NC}"
    echo -e "${GREEN}  Taurine uninstalled!${NC}"
    echo -e "${GREEN}==================================${NC}"
}

# Main
header

if [ "$1" = "--uninstall" ] || [ "$1" = "-u" ]; then
    uninstall_taurine
else
    install_taurine
fi
