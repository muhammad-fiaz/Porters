#!/usr/bin/env bash
# Porters Installation Script for Linux/macOS
# This script installs Porters and automatically adds it to PATH

set -e

# Colors
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
CYAN='\033[0;36m'
NC='\033[0m' # No Color

echo -e "${CYAN}==================================${NC}"
echo -e "${CYAN}  Porters Installation Script${NC}"
echo -e "${CYAN}==================================${NC}"
echo ""

# Check if Rust is installed
echo -e "${YELLOW}[1/4] Checking Rust installation...${NC}"
if ! command -v cargo &> /dev/null; then
    echo -e "${RED}❌ Rust is not installed!${NC}"
    echo -e "${YELLOW}Please install Rust from: https://rustup.rs/${NC}"
    echo -e "${YELLOW}Run: curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh${NC}"
    echo -e "${YELLOW}After installing Rust, run this script again.${NC}"
    exit 1
fi

RUST_VERSION=$(cargo --version)
echo -e "${GREEN}✓ Rust detected: $RUST_VERSION${NC}"
echo ""

# Install Porters
echo -e "${YELLOW}[2/4] Installing Porters...${NC}"
if cargo install --path . 2>&1 | grep -q "Installed\|Updated"; then
    echo -e "${GREEN}✓ Porters installed successfully!${NC}"
else
    echo -e "${RED}❌ Failed to install Porters${NC}"
    exit 1
fi
echo ""

# Get cargo bin directory
if [ -n "$CARGO_HOME" ]; then
    CARGO_BIN="$CARGO_HOME/bin"
else
    CARGO_BIN="$HOME/.cargo/bin"
fi

echo -e "Cargo bin directory: ${CYAN}$CARGO_BIN${NC}"

# Check if already in PATH
echo -e "${YELLOW}[3/4] Checking PATH configuration...${NC}"
if echo "$PATH" | grep -q "$CARGO_BIN"; then
    echo -e "${GREEN}✓ Cargo bin directory already in PATH${NC}"
    SKIP_PATH_UPDATE=true
else
    echo -e "${YELLOW}⚠ Cargo bin directory not found in PATH${NC}"
    SKIP_PATH_UPDATE=false
fi
echo ""

# Add to PATH if needed
if [ "$SKIP_PATH_UPDATE" = false ]; then
    echo -e "${YELLOW}[4/4] Adding Porters to PATH...${NC}"
    echo -e "${CYAN}This will add: $CARGO_BIN${NC}"
    echo -e "${CYAN}To your shell configuration file.${NC}"
    echo ""
    
    # Detect shell
    SHELL_NAME=$(basename "$SHELL")
    if [ "$SHELL_NAME" = "bash" ]; then
        SHELL_RC="$HOME/.bashrc"
    elif [ "$SHELL_NAME" = "zsh" ]; then
        SHELL_RC="$HOME/.zshrc"
    elif [ "$SHELL_NAME" = "fish" ]; then
        SHELL_RC="$HOME/.config/fish/config.fish"
    else
        SHELL_RC="$HOME/.profile"
    fi
    
    echo -e "Detected shell: ${CYAN}$SHELL_NAME${NC}"
    echo -e "Configuration file: ${CYAN}$SHELL_RC${NC}"
    echo ""
    
    read -p "Do you want to add Porters to PATH automatically? (Y/n) " -n 1 -r
    echo
    if [[ $REPLY =~ ^[Yy]$ ]] || [[ -z $REPLY ]]; then
        # Add to shell config
        if [ "$SHELL_NAME" = "fish" ]; then
            PATH_EXPORT="set -gx PATH $CARGO_BIN \$PATH"
        else
            PATH_EXPORT="export PATH=\"$CARGO_BIN:\$PATH\""
        fi
        
        # Check if already in config file
        if ! grep -q "$CARGO_BIN" "$SHELL_RC" 2>/dev/null; then
            echo "" >> "$SHELL_RC"
            echo "# Added by Porters installer" >> "$SHELL_RC"
            echo "$PATH_EXPORT" >> "$SHELL_RC"
            echo -e "${GREEN}✓ Successfully added to $SHELL_RC!${NC}"
        else
            echo -e "${GREEN}✓ Already present in $SHELL_RC${NC}"
        fi
        
        # Update current session
        export PATH="$CARGO_BIN:$PATH"
        
        echo ""
        echo -e "${YELLOW}⚠ IMPORTANT:${NC}"
        echo -e "  ${NC}- Current terminal: PATH updated (porters available now)${NC}"
        echo -e "  ${NC}- Other terminals: Run 'source $SHELL_RC' or restart terminal${NC}"
        echo ""
    else
        echo -e "${YELLOW}Skipped PATH update.${NC}"
        echo ""
        echo -e "${YELLOW}To use 'porters' command, add to PATH manually:${NC}"
        echo -e "  ${CYAN}$PATH_EXPORT${NC}"
        echo -e "${YELLOW}Add to: $SHELL_RC${NC}"
        echo ""
    fi
else
    echo -e "${YELLOW}[4/4] PATH configuration${NC}"
    echo -e "${GREEN}✓ Already configured${NC}"
    echo ""
fi

# Verify installation
echo -e "${CYAN}==================================${NC}"
echo -e "${CYAN}  Verifying Installation${NC}"
echo -e "${CYAN}==================================${NC}"
echo ""

if command -v porters &> /dev/null; then
    PORTERS_VERSION=$(porters --version 2>&1 || echo "unknown")
    echo -e "${GREEN}✓ Porters is ready!${NC}"
    echo -e "  ${CYAN}Version: $PORTERS_VERSION${NC}"
    echo ""
    echo -e "${YELLOW}Try it now:${NC}"
    echo -e "  ${CYAN}porters --help${NC}"
    echo -e "  ${CYAN}porters init${NC}"
    echo -e "  ${CYAN}porters execute myfile.c${NC}"
    echo ""
else
    echo -e "${YELLOW}⚠ Installation complete, but 'porters' command not found${NC}"
    echo ""
    echo -e "${YELLOW}Please:${NC}"
    echo -e "  ${NC}1. Restart your terminal${NC}"
    echo -e "  ${NC}2. Or run: source $SHELL_RC${NC}"
    echo -e "  ${NC}3. Or run: export PATH=\"$CARGO_BIN:\$PATH\"${NC}"
    echo -e "  ${NC}4. Then try: porters --help${NC}"
    echo ""
fi

echo -e "${CYAN}==================================${NC}"
echo -e "${CYAN}  Installation Complete! 🎉${NC}"
echo -e "${CYAN}==================================${NC}"
echo ""
echo -e "${CYAN}Documentation: https://porters.dev${NC}"
echo -e "${CYAN}Issues: https://github.com/muhammad-fiaz/Porters/issues${NC}"
echo ""
