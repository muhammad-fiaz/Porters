<div align="center">
<img width="1536" height="1024" alt="porters_logo" src="https://github.com/user-attachments/assets/17c507aa-3131-49b6-a682-e2ae55c3841c" />

<a href="https://crates.io/crates/porters"><img src="https://img.shields.io/crates/v/porters" alt="Crates.io"></a>
<a href="https://crates.io/crates/porters"><img src="https://img.shields.io/crates/d/porters" alt="Crates.io Downloads"></a>
<a href="https://muhammad-fiaz.github.io/Porters/"><img src="https://img.shields.io/badge/docs-muhammad--fiaz.github.io-blue" alt="Documentation"></a>
<a href="https://www.rust-lang.org/"><img src="https://img.shields.io/badge/rust-%3E%3D1.70-orange.svg" alt="Rust"></a>
<a href="https://github.com/muhammad-fiaz/porters"><img src="https://img.shields.io/github/stars/muhammad-fiaz/porters" alt="GitHub stars"></a>
<a href="https://github.com/muhammad-fiaz/porters/issues"><img src="https://img.shields.io/github/issues/muhammad-fiaz/porters" alt="GitHub issues"></a>
<a href="https://github.com/muhammad-fiaz/porters/pulls"><img src="https://img.shields.io/github/issues-pr/muhammad-fiaz/porters" alt="GitHub pull requests"></a>
<a href="https://github.com/muhammad-fiaz/porters"><img src="https://img.shields.io/github/last-commit/muhammad-fiaz/porters" alt="GitHub last commit"></a>
<a href="https://github.com/muhammad-fiaz/porters/releases"><img src="https://img.shields.io/github/v/release/muhammad-fiaz/porters" alt="GitHub release"></a>
<a href="https://github.com/muhammad-fiaz/porters"><img src="https://img.shields.io/github/license/muhammad-fiaz/porters" alt="License"></a>
<a href="https://github.com/muhammad-fiaz/porters/actions"><img src="https://github.com/muhammad-fiaz/porters/workflows/CI/badge.svg" alt="CI"></a>
<a href="https://github.com/muhammad-fiaz/porters/actions"><img src="https://github.com/muhammad-fiaz/porters/workflows/Release/badge.svg" alt="Release"></a>

<p><em>A universal C/C++ project manager and build orchestrator with GitHub-integrated package publishing</em></p>

**📚 [Documentation](https://muhammad-fiaz.github.io/Porters/) | [Configurations](https://muhammad-fiaz.github.io/Porters/configuration/) | [Quick Start](https://muhammad-fiaz.github.io/Porters/getting-started/)**

</div>

Porters is a modern, production-ready project manager for C/C++ that simplifies dependency management, build orchestration, and package publishing. Inspired by Cargo but designed specifically for C/C++ ecosystems.

<details>
<summary>✨ Features</summary>

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

</details>

<details>
<summary>📍 Prerequisites</summary>

Before installing Porters, ensure you have the following tools installed:

### Core Requirements
- **[Rust & Cargo](https://www.rust-lang.org/)** - Required for Porters itself (installation and core functionality)
- **[Git](https://git-scm.com/)** - Required for dependency management and cloning repositories

### Build System Support (Install as needed)
Porters auto-detects and supports 14+ build systems. Install the ones you plan to use:

#### Modern Build Systems
- **[CMake](https://cmake.org/)** - Cross-platform build generator
- **[XMake](https://xmake.io/)** - Lua-based modern build system  
- **[Meson](https://mesonbuild.com/)** - Fast, user-friendly build system
- **[Bazel](https://bazel.build/)** - Google's scalable build system
- **[Buck2](https://buck2.build/)** - Meta's fast build system

#### Traditional Build Systems
- **[Make](https://www.gnu.org/software/make/)** - Makefile-based builds
- **[Ninja](https://ninja-build.org/)** - Fast, lightweight build system
- **[Autotools](https://www.gnu.org/software/automake/)** - configure/make (GNU Build System)
- **[SCons](https://scons.org/)** - Python-based build tool

#### Meta Build Systems
- **[Premake](https://premake.github.io/)** - Project file generator (Visual Studio, Makefiles, Xcode)
- **[QMake](https://doc.qt.io/qt-6/qmake-manual.html)** - Qt's build system

#### Package Managers with Build Integration
- **[Conan](https://conan.io/)** - C/C++ package manager with CMake integration
- **[vcpkg](https://vcpkg.io/)** - Microsoft's C++ library manager

#### Compilers
- **[GCC/G++](https://gcc.gnu.org/)** - GNU Compiler Collection
- **[Clang/Clang++](https://clang.llvm.org/)** - LLVM compiler
- **[MSVC](https://visualstudio.microsoft.com/vs/features/cplusplus/)** - Microsoft Visual C++ (Windows)
- **[MinGW](https://www.mingw-w64.org/)** - Minimalist GNU for Windows

**Note:** You don't need to install all build systems - Porters will auto-detect and use what's available for your projects.

</details>

## 📦 Installation

### Via Cargo (Recommended)
```bash
cargo install porters

# After installation, add to PATH if needed:
# Windows: [Environment]::SetEnvironmentVariable("Path", $env:Path + ";$env:USERPROFILE\.cargo\bin", "User")
# Linux/macOS: export PATH="$HOME/.cargo/bin:$PATH"
```

### Quick Install

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

See [Execute Guide](https://muhammad-fiaz.github.io/Porters/execute.html) for detailed documentation.

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

<details>
<summary>📝 Configuration Structure Example</summary>

```toml
[project]
name = "my-project"
version = "1.0.0"
description = "My awesome C++ project"
license = "Apache-2.0"
authors = ["Your Name <you@example.com>"]
repository = "https://github.com/username/my-project"
project-type = "application"  # or "library"
entry_point = "src/main"
platforms = ["windows", "macos", "linux"]
keywords = ["application", "c", "cpp"]
readme = "README.md"

# Tool version requirements (like Python's requirements.txt)
[requires]
cpp = ">=17"           # C++ standard version
cmake = ">=3.20"       # CMake version
gcc = ">=9.0"          # GCC version
clang = ">=12.0"       # Clang version (alternative to gcc)
ninja = ">=1.10"       # Ninja build tool version
make = ">=4.0"         # Make version
meson = ">=0.60"       # Meson version
bazel = ">=5.0"        # Bazel version
conan = ">=1.50"       # Conan version
vcpkg = "*"            # Any version of vcpkg

# Extensions to auto-install from crates.io
extensions = [
    "porters-format",    # Code formatter extension
    "porters-lint",      # Linting extension
    "porters-doc"        # Documentation generator
]

[dependencies]
fmt = { git = "https://github.com/fmtlib/fmt", tag = "10.1.1" }
spdlog = { git = "https://github.com/gabime/spdlog", branch = "v1.x" }
mylib = { path = "../mylib" }

[dev-dependencies]
catch2 = { git = "https://github.com/catchorg/Catch2" }
benchmark = { git = "https://github.com/google/benchmark" }

[optional-dependencies]
zlib = { git = "https://github.com/madler/zlib" }

[build]
system = "cmake"
options = ["-DBUILD_SHARED_LIBS=ON"]

[build.env]
CC = "clang"
CXX = "clang++"

# Build lifecycle scripts
[build.scripts]
pre-build = "echo Building..."
post-build = "strip build/myapp"
pre-install = "echo Installing..."
post-install = "echo Done!"

# Custom CLI commands
[[commands]]
name = "format"
description = "Format source code"
script = "clang-format -i src/**/*.cpp"

[[commands]]
name = "docs"
description = "Generate documentation"
script = "doxygen Doxyfile"
[commands.env]
DOXYGEN_OUTPUT = "docs/html"

# Named script shortcuts
[scripts]
test-all = "cargo build && cargo test"
deploy = "./deploy.sh production"

# Direct file execution configuration (OPTIONAL - works automatically)
[run]
# Extra include directories (beyond automatic dependency includes)
include-dirs = ["./include", "./extra/include"]

# Compiler flags (only if you want warnings, optimizations, etc.)
compiler-flags = ["-Wall", "-O2", "-std=c17"]

# Linker flags (only if you need extra libraries)
linker-flags = ["-lm", "-lpthread"]

# Override compiler (only if you need a specific one)
c-compiler = "clang"    # Default: auto-detect
cpp-compiler = "clang++"  # Default: auto-detect
```

</details>

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

<details>
<summary>🔌 Extension Example</summary>

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

</details>

See the [Extension Guide](https://muhammad-fiaz.github.io/Porters/extensions.html) for details.

<details>
<summary>🔨 Supported Build Systems (17+)</summary>

Porters **natively supports and executes** the following build systems with auto-detection:

### Traditional Build Systems
- **Make** - Makefile-based builds
- **Ninja** - Fast, lightweight build system
- **Autotools** - configure/make (GNU Build System)
- **Meson** - Fast, user-friendly build system
- **SCons** - Python-based build tool
- **Jam** - Boost.Build system

### CMake Ecosystem
- **CMake** - Cross-platform build generator
- **Conan** - C/C++ package manager with CMake integration
- **vcpkg** - Microsoft's C++ library manager
- **Hunter** - CMake-based package manager

### Modern Build Systems
- **XMake** - Lua-based modern build system
- **Bazel** - Google's scalable build system
- **Buck2** - Meta's fast build system
- **Premake** - Project file generator (Visual Studio, Makefiles, Xcode)
- **QMake** - Qt's build system
- **Gradle C++** - Gradle build system for C++

### Custom Build
- **Custom Scripts** - Define your own build commands in `porters.toml`

### Auto-Detection Priority
1. Package managers (Conan, vcpkg, Hunter)
2. Modern systems (Bazel, Buck2, CMake, XMake, Meson, Gradle C++)
3. Meta build (Premake, QMake)
4. Traditional (Ninja, Autotools, SCons, Jam, Make)
5. Explicit configuration via `build.system` in porters.toml

</details>

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

- [📖 Complete Documentation](https://muhammad-fiaz.github.io/Porters/)
- [🚀 Getting Started](https://muhammad-fiaz.github.io/Porters/getting-started.html)
- [🔌 Extension Guide](https://muhammad-fiaz.github.io/Porters/extensions.html)
- [📦 Dependency Management](https://muhammad-fiaz.github.io/Porters/dependencies.html)
- [🔨 Build Systems](https://muhammad-fiaz.github.io/Porters/building.html)
- [🛠️ Configuration Reference](https://muhammad-fiaz.github.io/Porters/configuration.html)

## 🐛 Issues & Support

if you found any issues please report them at [https://github.com/muhammad-fiaz/Porters/issues](https://github.com/muhammad-fiaz/Porters/issues)

## 👤 Author

**Muhammad Fiaz**
- Email: contact@muhammadfiaz.com
- GitHub: [@muhammad-fiaz](https://github.com/muhammad-fiaz)

---

⭐ Star this repo if you find it useful!
