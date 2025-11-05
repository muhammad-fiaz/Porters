# Porters Documentation

Welcome to the official documentation for **Porters** - a modern, universal package manager for C and C++ projects.

## What is Porters?

Porters is a comprehensive project management tool designed specifically for C/C++ developers. It simplifies the entire development workflow from project creation to building, dependency management, and publishing.

### Key Features

- 🚀 **Universal Build System Support** - Works with CMake, XMake, Meson, Make, and custom build systems
- 📦 **Smart Dependency Management** - Supports Git, local paths, and registries
- 🌍 **Global and Local Dependencies** - Install packages globally or isolate them per-project
- 🔄 **Lock File Support** - Ensures reproducible builds across environments
- 🎯 **Auto-Detection** - Automatically detects existing build systems and project structure
- 🔧 **Interactive Project Creation** - Step-by-step project setup with customizable options
- 📚 **GitHub Integration** - Seamlessly publish releases and manage versions
- ⚡ **Self-Updating** - Keep Porters up-to-date with a single command

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

# Create a new project
porters create my-project

# Or initialize an existing project
cd my-existing-project
porters init

# Add dependencies
porters add fmt --git https://github.com/fmtlib/fmt

# Build your project
porters build

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
