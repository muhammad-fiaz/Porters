# Porters

> A universal C/C++ project manager and build orchestrator with GitHub-integrated package publishing

[![License: Apache-2.0](https://img.shields.io/badge/License-Apache_2.0-blue.svg)](https://opensource.org/licenses/Apache-2.0)
[![Rust](https://img.shields.io/badge/rust-2024-orange.svg)](https://www.rust-lang.org)
[![Documentation](https://img.shields.io/badge/docs-book-green.svg)](https://muhammad-fiaz.github.io/porters/)

Porters is a modern, production-ready project manager for C/C++ that simplifies dependency management, build orchestration, and package publishing. Inspired by Cargo but designed specifically for C/C++ ecosystems.

> This Project is in Active Development!

## ✨ Features

### Core Functionality
- 🚀 **Zero-Config Project Init** - Automatically detects existing C/C++ projects
- ⚡ **Direct File Execution** - `porters execute file.c` compiles and runs instantly with automatic dependency resolution
- 📦 **Unified Dependency Management** - Git (SSH/HTTPS), path, and global/local support
- 🔨 **14 Build System Support** - Auto-detect and seamlessly integrate with CMake, XMake, Meson, Make, Ninja, Autotools, SCons, Bazel, Buck2, Premake, QMake, Conan, vcpkg, or custom builds
- 🎯 **Smart Project Scaffolding** - Interactive project creation with license selection
- 🤖 **Smart Auto-Configuration** - Everything works automatically with optional manual overrides
- 🔄 **Automatic Dependency Resolution** - Platform-aware with constraint checking
- 📊 **Dependency Graph Visualization** - Understand your project dependencies
- 🔒 **Lockfile Support** - Reproducible builds with `porters.lock`
- 📤 **GitHub Publishing** - Automated package releases with artifacts

### Advanced Features
- 🔌 **Extension System** - Create and publish custom extensions to crates.io
- 🌍 **Global Package Installation** - Centralized dependency management in `~/.porters/`
- 📂 **Isolated Virtual Environments** - Project-specific dependencies in `ports/` folder
- 🔄 **Smart Sync** - `porters sync` with `--dev` and `--optional` flags
- 🛠️ **Compiler Detection** - Auto-detect GCC, Clang, MSVC, LLVM, MinGW, Emscripten
- 📝 **Build Scripts** - Pre/post build hooks for custom workflows
- 🎛️ **Enhanced Configuration** - Build flags, include paths, linking options
- 🆙 **Self-Updating** - Built-in update mechanism via GitHub releases
- ✅ **Tool Version Requirements** - Specify minimum versions for compilers and build tools (like Python's requirements.txt)
- 🔐 **Hash Verification** - SHA-256 checksums for all dependencies
- 🔗 **Transitive Dependencies** - Automatic resolution of dependencies-of-dependencies
- 🎨 **Custom Commands** - Define project-specific CLI commands in config
- 📜 **Named Scripts** - Quick shortcuts for common tasks

### Extension Ecosystem
- 📦 **Auto-Install Extensions** - Automatically install extensions from porters.toml
- 🌐 **crates.io Marketplace** - Publish and discover extensions on crates.io
- 🔧 **6 Lifecycle Hooks** - pre_build, post_build, pre_install, post_install, pre_sync, post_sync
- 🎯 **User-Made Extensions** - Anyone can create and publish Porters extensions
- 💡 **Template Generator** - `porters extension create` scaffolds extension projects

### Production-Ready Features
- 🌍 **Cross-Platform** - Windows, macOS, Linux support
- 🔧 **Platform-Specific Dependencies** - Conditional dependency resolution
- 📚 **Library & Application Projects** - Proper configuration for both types
- ⚡ **Parallel Operations** - Async dependency downloads
- 🎨 **Beautiful CLI** - Colored output with emoji indicators
- 🔍 **Build Tool Detection** - Automatic checking and installation guidance
- ⚠️ **Enhanced Error Handling** - Helpful messages with GitHub issue links
- 🚨 **Version Validation** - Pre-build checks for tool version requirements

## 📥 Installation

### Quick Install (Recommended)

**Windows:**
```powershell
# Clone and run installer
git clone https://github.com/muhammad-fiaz/porters
cd porters
.\install.ps1
```

**Linux/macOS:**
```bash
# Clone and run installer
git clone https://github.com/muhammad-fiaz/porters
cd porters
chmod +x install.sh
./install.sh
```

The installer will:
- ✅ Check Rust installation
- ✅ Install Porters via cargo
- ✅ **Automatically add to PATH** (with your permission)
- ✅ Verify the installation

### Via Cargo
```bash
cargo install porters

# After installation, add to PATH if needed:
# Windows: [Environment]::SetEnvironmentVariable("Path", $env:Path + ";$env:USERPROFILE\.cargo\bin", "User")
# Linux/macOS: export PATH="$HOME/.cargo/bin:$PATH"
```

### From Source
```bash
git clone https://github.com/muhammad-fiaz/porters
cd porters
cargo build --release
cargo install --path .
```

### Binary Downloads
Download pre-built binaries from [GitHub Releases](https://github.com/muhammad-fiaz/porters/releases)

**Note:** Porters will automatically check if cargo bin is in your PATH on first run and show setup instructions if needed.

## 🚀 Quick Start

### Create a New Project
```bash
# Interactive project creation
porters create my-awesome-project

# Quick creation with defaults (C++, CMake, Application)
porters create my-project -y
```

### Initialize Existing Project
```bash
cd your-existing-cpp-project
porters init
```

### Quick Single-File Execution (No Configuration Needed!)
```bash
# Execute any C/C++ file - works immediately!
porters execute hello.c

# With arguments
porters execute main.cpp arg1 arg2

# NO porters.toml needed - works anywhere!
```

**100% Automatic - Zero Configuration:**
- ✅ **Any C/C++ File** - `.c`, `.cpp`, `.cxx`, `.cc`, `.c++`, `.cp`
- ✅ **Compiler Auto-Detection** - Finds gcc/clang/g++/clang++
- ✅ **Dependency Resolution** - Reads `porters.toml` if present
- ✅ **Include/Lib Paths** - Automatically injected from dependencies
- ✅ **Works Standalone** - Execute files even outside a project

See [EXECUTE_GUIDE.md](./EXECUTE_GUIDE.md) for detailed documentation.

## 📋 Commands

```bash
porters create <name>         # Create new project
porters init                  # Initialize existing project
porters add <package>         # Add dependency
porters remove <package>      # Remove dependency
porters build                 # Build whole project
porters execute <file>        # Execute single C/C++ file (zero config!)
porters run [args]            # Run compiled project executable
porters test                  # Run tests
porters update                # Update dependencies
porters clean                 # Clean build artifacts
porters lock                  # Generate lockfile
porters vendor                # Vendor dependencies
porters graph                 # Show dependency graph
porters publish               # Publish to GitHub
porters upgrade               # Update porters itself
porters run-script <name>     # Run named script from config
porters <custom-command>      # Run custom command from config
```

## 📝 Configuration Example

```toml
[project]
name = "my-project"
version = "0.1.0"
authors = ["Your Name <email@example.com>"]
description = "An awesome C++ project"
license = "Apache-2.0"
repository = "https://github.com/username/my-project"
project-type = "application"  # or "library"
entry_point = "src/main"
platforms = ["windows", "macos", "linux"]
keywords = ["networking", "cpp"]

[dependencies]
fmt = { git = "https://github.com/fmtlib/fmt", tag = "10.1.1" }
mylib = { path = "../mylib" }

[dev-dependencies]
gtest = { git = "https://github.com/google/googletest" }

[build]
system = "cmake"  # Auto-detected from CMakeLists.txt

# Enhanced build configuration
[build.flags]
cflags = ["-Wall", "-Wextra", "-O2"]
cxxflags = ["-std=c++17", "-Wall"]
ldflags = ["-pthread"]
defines = ["USE_FEATURE_X"]

[build.include]
include = ["include/", "src/"]

[build.linking]
libraries = ["pthread", "m"]
library_paths = ["/usr/local/lib"]

[build.scripts]
pre-build = "scripts/pre_build.sh"
post-build = "scripts/post_build.sh"
```

## 🔌 Extension System

Create and use extensions for custom functionality:

### Install Extensions

```bash
# From crates.io
porters extension install porters-format

# From GitHub
porters extension install my-ext --git https://github.com/user/porters-ext-myext

# From local path
porters extension install my-ext --path ./my-extension
```

### Create Extensions

```bash
porters extension create my-awesome-extension
```

### Extension Example

```toml
# extension.toml
name = "porters-format"
version = "0.1.0"
description = "Code formatting extension"

[hooks]
post-build = "hooks/format.sh"

[[commands]]
name = "format"
description = "Format code"
script = "scripts/format.sh"
```

See the [Extension Guide](https://muhammad-fiaz.github.io/porters/extensions.html) for details.

## 🔨 Supported Build Systems (14)

Porters **natively supports and executes** the following build systems with auto-detection:

### Traditional Build Systems
- **Make** - Makefile-based builds
- **Ninja** - Fast, lightweight build system
- **Autotools** - configure/make (GNU Build System)
- **SCons** - Python-based build tool

### Modern Build Systems
- **CMake** - Cross-platform build generator
- **XMake** - Lua-based modern build system
- **Meson** - Fast, user-friendly build system
- **Bazel** - Google's scalable build system
- **Buck2** - Meta's fast build system

### Meta Build Systems
- **Premake** - Project file generator (Visual Studio, Makefiles, Xcode)
- **QMake** - Qt's build system

### Package Managers with Build Integration
- **Conan** - C/C++ package manager with CMake integration
- **vcpkg** - Microsoft's C++ library manager

### Custom Build
- **Custom Scripts** - Define your own build commands in `porters.toml`

### Auto-Detection Priority
1. Package managers (Conan, vcpkg)
2. Modern systems (Bazel, Buck2, CMake, XMake, Meson)
3. Meta build (Premake, QMake)
4. Traditional (Ninja, Autotools, SCons, Make)
5. Explicit configuration via `build.system` in porters.toml

## 📦 Dependency Management

### Global Installation

```bash
porters install fmt --git https://github.com/fmtlib/fmt
```

Installs to `~/.porters/packages/`

### Local Dependencies (Isolated)

```bash
porters add fmt --git https://github.com/fmtlib/fmt
```

Installs to project's `ports/` folder

### Sync Dependencies

```bash
porters sync              # Regular dependencies
porters sync --dev        # Include dev dependencies
porters sync --optional   # Include optional dependencies
```

### Lock File

```bash
porters lock  # Generate/update porters.lock
```

## 🎯 Project Types

### Application
- Generates executable
- Entry point: `src/main.cpp`
- CMake: `add_executable()`

### Library
- Generates static/shared library
- Proper include setup
- CMake: `add_library()`

## 🌍 Platform-Specific Dependencies

```toml
[dependencies]
winapi = { git = "...", platforms = ["windows"] }
pthread = { version = "*", platforms = ["linux", "macos"] }
```

## 📦 Publishing

```bash
export GITHUB_TOKEN=ghp_your_token
porters publish
```

Creates GitHub release with:
- Version tag
- Release notes
- Installation table
- CHANGELOG integration

## 🤝 Contributing

Contributions welcome! See [CONTRIBUTING.md](CONTRIBUTING.md)

## 📜 License

This project is licensed under the Apache License 2.0 - see [LICENSE](LICENSE)

## 📚 Documentation

- [📖 Complete Documentation](https://muhammad-fiaz.github.io/porters/)
- [🚀 Getting Started](https://muhammad-fiaz.github.io/porters/getting-started.html)
- [🔌 Extension Guide](https://muhammad-fiaz.github.io/porters/extensions.html)
- [📦 Dependency Management](https://muhammad-fiaz.github.io/porters/dependencies.html)
- [🔨 Build Systems](https://muhammad-fiaz.github.io/porters/building.html)
- [🛠️ Configuration Reference](https://muhammad-fiaz.github.io/porters/configuration.html)

## 🐛 Issues & Support

Found a bug or have a feature request?

**🤔 Oops! Looks like something went wrong?**

If you think this is a bug in Porters, please report it to:
[https://github.com/muhammad-fiaz/Porters/issues](https://github.com/muhammad-fiaz/Porters/issues)

## 👤 Author

**Muhammad Fiaz**
- Email: contact@muhammadfiaz.com
- GitHub: [@muhammad-fiaz](https://github.com/muhammad-fiaz)

---

⭐ Star this repo if you find it useful!
