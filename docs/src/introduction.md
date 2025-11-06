# Porters Documentation

Welcome to the official documentation for **Porters** - a modern, universal package manager for C and C++ projects.

## What is Porters?

Porters is a comprehensive project management tool designed specifically for C/C++ developers. It simplifies the entire development workflow from project creation to building, dependency management, and publishing.

### Key Features

- 🚀 **Universal Build System Support** - Works with CMake, XMake, Meson, Make, and custom build systems
- 📦 **Smart Dependency Management** - Supports Git, local paths, and remote registries
- 💾 **Global Cache System** - Share dependencies across projects with `~/.porters/cache/`
- 🔌 **Offline Mode** - Work without network access using cached dependencies
- � **Remote Registry** - Discover packages from GitHub-based package index
- �🌍 **Global and Local Dependencies** - Install packages globally or isolate them per-project
- 🔄 **Lock File Support** - Ensures reproducible builds across environments
- 🎯 **Auto-Detection** - Automatically detects existing build systems and project structure
- 🔧 **Interactive Project Creation** - Step-by-step project setup with customizable options
- 🟣 **Hybrid C/C++ Support** - Seamlessly combine C and C++ code with `extern "C"` scaffolding
- ⚡ **Zero-Config Single File Execution** - Run any C/C++ file instantly with `porters execute` - no project needed!
- 🪟 **External Terminal Support** - Open programs in new terminal windows with `--external` flag
- 📄 **Automatic License Generation** - Creates LICENSE files from 9+ SPDX templates (MIT, Apache-2.0, GPL, BSD, etc.)
- 📝 **Comprehensive README Generation** - Auto-creates README with badges, usage examples, and project structure
- 🏗️ **Application & Library Templates** - Complete scaffolding with examples, tests, and documentation
- 🔍 **System Requirements Check** - Automatic detection of compilers and build tools on first run
- ⚙️ **Global Configuration** - User-wide settings and preferences in `~/.porters/config.toml`
- 🛤️ **PATH Management** - Built-in commands to add/remove Cargo bin from system PATH
- 📚 **GitHub Integration** - Seamlessly publish releases and manage versions
- 🔄 **Self-Updating** - Keep Porters up-to-date with a single command
- 📦 **Package Manager Integration** - Works with Conan, vcpkg, and XMake package managers

### Why Porters?

C and C++ projects have historically lacked a unified package management solution. Porters bridges this gap by providing:

- **Consistency**: Manage all your C/C++ projects the same way
- **Simplicity**: Intuitive commands that just work
- **Flexibility**: Support for multiple build systems and workflows
- **Isolation**: Virtual environments per project using the `ports/` folder
- **Reliability**: Lock files ensure your builds are reproducible

## Quick Start

```bash
# Install Porters
cargo install porters

# Porters automatically checks system requirements on first run
# Detects compilers (gcc, g++, clang, MSVC) and build tools (CMake, Make, etc.)

# Add Cargo bin to PATH (optional, for convenience)
porters add-to-path

# Create a new project (interactive wizard)
porters create my-project
# Choose: Application or Library
# Select: License (MIT, Apache-2.0, GPL-3.0, BSD, etc.)
# Porters generates: LICENSE file, README, source structure

# Or initialize an existing project
cd my-existing-project
porters init
# Also generates LICENSE file based on your choice

# Add dependencies
porters add fmt --git https://github.com/fmtlib/fmt

# Build your project
porters build

# Execute single C/C++ files instantly (no project needed!)
porters execute hello.cpp
porters execute game.c --external  # Opens in new terminal window

# Publish to GitHub
porters publish
```

## System Architecture

Porters uses a dual-layer dependency system:

- **Global Dependencies** (`~/.porters/`): Centralized cache for packages used across projects
- **Local Dependencies** (`ports/`): Project-specific isolated environments

This design provides:
- Faster dependency resolution (cached globally)
- Complete isolation between projects
- Reproducible builds via lock files

## Getting Help

- 📖 Browse the [User Guide](./getting-started.md) for detailed instructions
- 🔍 Check the [Command Reference](./commands.md) for all available commands
- 🐛 Visit the [Troubleshooting](./troubleshooting.md) section for common issues
- 💡 Read the [Contributing](./contributing.md) guide to get involved

## License

Porters is licensed under the Apache License 2.0. See the LICENSE file for details.
