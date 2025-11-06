# Installation

This guide will help you install Porters on your system.

## Prerequisites

Before installing Porters, ensure you have the following:

### Required

- **Rust Toolchain** (for building from source or installing via Cargo)
  - Install from [rustup.rs](https://rustup.rs/)
  - Minimum version: Rust 1.70+

### C/C++ Compiler (At least one required)

**Porters will check for these automatically on first run.**

- **Windows**: 
  - MSVC (Visual Studio Build Tools)
  - MinGW-w64
  - Clang/LLVM
  
- **macOS**: 
  - Xcode Command Line Tools: `xcode-select --install`
  - Clang (included with Xcode)
  
- **Linux**: 
  - GCC: `sudo apt install gcc g++` (Debian/Ubuntu)
  - Clang: `sudo apt install clang` (Debian/Ubuntu)

### Build Tools (Optional but recommended)

**At least one build system is recommended for project management:**

- **CMake** (Recommended)
  - Download from [cmake.org](https://cmake.org/download/)
  - Minimum version: 3.15+
  - Install: 
    - Windows: `choco install cmake`
    - Linux: `sudo apt install cmake`
    - macOS: `brew install cmake`

- **XMake**
  - Install from [xmake.io](https://xmake.io/#/guide/installation)
  - Cross-platform build utility

- **Meson**
  - Install via pip: `pip install meson ninja`
  - Fast and user-friendly build system

- **Make**
  - Usually pre-installed on Linux/macOS
  - Windows: Install via [MinGW](http://www.mingw.org/) or [Cygwin](https://www.cygwin.com/)

**Note:** Build tools are only required for project-based workflows (`porters build`, `porters create`). The `porters execute` command works without any build system!

## Installing Porters

### Via Cargo (Recommended)

The easiest way to install Porters is through Cargo:

```bash
cargo install porters
```

This will download, compile, and install the latest version of Porters.

### First Run System Check

**After installation, when you run Porters for the first time**, it will automatically:

1. **Check for C/C++ compilers** (gcc, g++, clang, MSVC, MinGW)
2. **Check for build systems** (CMake, Make, XMake, Meson, Ninja)
3. **Display what's found** with version numbers
4. **Show installation instructions** for missing tools
5. **Save detected tools** to `~/.porters/config.toml`
6. **Block execution** if no C/C++ compiler is found

**Example First Run:**
```bash
$ porters --version

╭──────────────────────────────────────────────────╮
│  System Requirements Check                       │
╰──────────────────────────────────────────────────╯

Compilers
─────────
✅ g++ (version 11.4.0)
✅ gcc (version 11.4.0)
❌ clang++ (not found)

Build Systems
─────────────
✅ cmake (version 3.22.1)
✅ make (version 4.3)
❌ xmake (not found)

Status: ✅ System ready!

Installation Instructions:
──────────────────────────

To install missing tools on Linux:
  sudo apt-get install clang xmake

Porters version 0.1.0
```

**Manual System Check:**

You can re-run the system check anytime:
```bash
porters --check-system
```

This is useful after installing new compilers or build tools to update the global configuration.

#### Automatic PATH Setup

Porters can automatically add the Cargo bin directory to your system PATH.

**Using the built-in command:**
```bash
# Automatically add ~/.cargo/bin to PATH
porters add-to-path
```

This command:
- **Windows**: Modifies User PATH via registry (requires admin privileges)
- **Linux/macOS**: Appends to your shell profile (~/.bashrc, ~/.zshrc, etc.)
- Detects your shell automatically
- Creates backup before modifying

**To remove from PATH:**
```bash
porters remove-from-path
```

**Manual PATH Setup:**

If you prefer to do it manually:

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
