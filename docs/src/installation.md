# Installation

This guide will help you install Porters on your system.

## Prerequisites

Before installing Porters, ensure you have the following:

### Required

- **Rust Toolchain** (for building from source or installing via Cargo)
  - Install from [rustup.rs](https://rustup.rs/)
  - Minimum version: Rust 1.70+

### Build Tools (At least one)

Porters works with various build systems. You'll need at least one installed:

- **CMake** (Recommended)
  - Download from [cmake.org](https://cmake.org/download/)
  - Minimum version: 3.15+

- **XMake**
  - Install from [xmake.io](https://xmake.io/#/guide/installation)

- **Meson**
  - Install via pip: `pip install meson ninja`

- **Make**
  - Usually pre-installed on Linux/macOS
  - Windows: Install via [MinGW](http://www.mingw.org/) or [Cygwin](https://www.cygwin.com/)

### C/C++ Compiler

You'll need a C/C++ compiler:

- **Windows**: MSVC (Visual Studio), MinGW-w64, or Clang
- **macOS**: Xcode Command Line Tools (`xcode-select --install`)
- **Linux**: GCC or Clang (usually pre-installed)

## Installing Porters

### Via Cargo (Recommended)

The easiest way to install Porters is through Cargo:

```bash
cargo install porters
```

This will download, compile, and install the latest version of Porters.

#### Automatic PATH Setup

On first run, Porters will check if the cargo bin directory is in your PATH. If not, it will show instructions to add it automatically.

**Windows PowerShell (Run as Administrator):**
```powershell
[Environment]::SetEnvironmentVariable(
  "Path",
  [Environment]::GetEnvironmentVariable("Path", "User") + ";$env:USERPROFILE\.cargo\bin",
  "User"
)
```

**Linux/macOS (add to ~/.bashrc or ~/.zshrc):**
```bash
export PATH="$HOME/.cargo/bin:$PATH"
```

Then restart your terminal or run:
```bash
source ~/.bashrc  # or source ~/.zshrc
```

**Quick Setup (Current Session Only):**
```powershell
# Windows PowerShell
$env:Path += ";$env:USERPROFILE\.cargo\bin"
```
```bash
# Linux/macOS
export PATH="$HOME/.cargo/bin:$PATH"
```

### Using Installation Script (Windows)

For Windows users, we provide an automated installation script:

```powershell
# Download and run the installer
cd path\to\Porters
.\install.ps1
```

This script will:
1. Check Rust installation
2. Install Porters via cargo
3. Automatically add to PATH (with your permission)
4. Verify the installation

### From GitHub Releases

Download pre-built binaries from the [GitHub Releases](https://github.com/muhammad-fiaz/Porters/releases) page:

1. Download the appropriate binary for your platform
2. Extract the archive
3. Move the `porters` binary to a directory in your PATH

**For Linux/macOS:**
```bash
sudo mv porters /usr/local/bin/
chmod +x /usr/local/bin/porters
```

**For Windows:**
Move `porters.exe` to a directory in your PATH (e.g., `C:\Program Files\Porters\`)

## Building from Source

If you want to build Porters from source (for development or custom builds):

### Step 1: Clone the Repository

```bash
# Clone the repository
git clone https://github.com/muhammad-fiaz/Porters.git
cd Porters
```

### Step 2: Build the Project

```bash
# Build in release mode for optimal performance
cargo build --release
```

### Step 3: Install the Binary

After building, you need to install the binary to your system PATH:

**Linux/macOS:**
```bash
# Copy to system PATH
sudo cp target/release/porters /usr/local/bin/

# Make it executable
sudo chmod +x /usr/local/bin/porters
```

**Windows (PowerShell as Administrator):**
```powershell
# Copy to a directory in PATH (e.g., Program Files)
cp target\release\porters.exe "C:\Program Files\Porters\porters.exe"

# Or add to user PATH
$portersPath = "$env:USERPROFILE\.porters\bin"
New-Item -ItemType Directory -Force -Path $portersPath
cp target\release\porters.exe "$portersPath\porters.exe"

# Add to PATH permanently
[Environment]::SetEnvironmentVariable(
  "Path",
  [Environment]::GetEnvironmentVariable("Path", "User") + ";$portersPath",
  "User"
)
```

### Step 4: Verify Installation

```bash
porters --version
```

You should see output similar to:
```
porters 0.1.0
```

## Verify Installation

Confirm Porters is installed correctly:

```bash
porters --version
```

You should see output similar to:
```
porters 0.1.0
```

## Global Configuration

On first run, Porters will create a global configuration directory:

- **Linux/macOS**: `~/.porters/`
- **Windows**: `C:\Users\<username>\.porters\`

This directory contains:
- `config.toml` - Global settings
- `packages/` - Globally installed packages
- `cache/` - Download cache

## Updating Porters

Keep Porters up-to-date with:

```bash
porters self-update
```

Or via Cargo:

```bash
cargo install porters --force
```

## Next Steps

Now that Porters is installed, continue to the [Getting Started](./getting-started.md) guide to create your first project!
