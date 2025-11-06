# Package Managers Integration

Porters provides seamless integration with popular C/C++ package managers, allowing you to manage external dependencies directly from your Porters project.

## Supported Package Managers

Porters currently supports three major package managers:

- **Conan** - Cross-platform package manager for C/C++
- **vcpkg** - Microsoft's C/C++ package manager
- **XMake** - Modern build system with built-in package manager

## Installation Scopes

All package managers support two installation scopes:

### Local Installation (Default)

Local packages are installed in the `ports/` directory of your project:

```bash
porters conan add fmt
# Installs to: ports/conan/
```

Local installations are project-specific and won't affect other projects.

### Global Installation

Global packages are installed in `~/.porters/packages/` and can be shared across multiple projects:

```bash
porters conan add --global fmt
# Installs to: ~/.porters/packages/conan/
```

Global installations are useful for:
- Common libraries used across multiple projects
- Reducing disk space by sharing package installations
- Faster setup for new projects

## Conan Integration

[Conan](https://conan.io/) is a widely-used C/C++ package manager.

### Prerequisites

Install Conan first:

```bash
pip install conan
```

### Basic Usage

**Add a package locally:**
```bash
porters conan add fmt
porters conan add boost --version 1.82.0
```

**Add a package globally:**
```bash
porters conan add --global fmt
porters conan add --global boost --version 1.82.0
```

**List installed packages:**
```bash
# List local packages
porters conan list

# List global packages
porters conan list --global
```

**Search for packages:**
```bash
porters conan search json
```

**Remove a package:**
```bash
# Remove local package (with confirmation)
porters conan remove fmt

# Remove global package (with confirmation)
porters conan remove --global fmt

# Force removal without confirmation
porters conan remove --force fmt
porters conan remove --global --force fmt
```

### How It Works

Porters creates a `conanfile.txt` in either:
- `ports/conan/conanfile.txt` (local)
- `~/.porters/packages/conan/conanfile.txt` (global)

The conanfile uses:
- **CMakeDeps** generator for CMake integration
- **CMakeToolchain** generator for cross-compilation support
- **--build=missing** to build packages from source if binaries unavailable

## vcpkg Integration

[vcpkg](https://github.com/microsoft/vcpkg) is Microsoft's cross-platform package manager.

### Prerequisites

Install vcpkg first:

```bash
# Windows
git clone https://github.com/microsoft/vcpkg
.\vcpkg\bootstrap-vcpkg.bat

# Linux/macOS
git clone https://github.com/microsoft/vcpkg
./vcpkg/bootstrap-vcpkg.sh

# Add to PATH
```

### Basic Usage

**Add a package locally:**
```bash
porters vcpkg add fmt
porters vcpkg add nlohmann-json --version 3.11.2
```

**Add a package globally:**
```bash
porters vcpkg add --global fmt
```

**List installed packages:**
```bash
# List local packages
porters vcpkg list

# List global packages
porters vcpkg list --global
```

**Search for packages:**
```bash
porters vcpkg search json
```

**Remove a package:**
```bash
# Remove local package (with confirmation)
porters vcpkg remove fmt

# Remove global package without confirmation
porters vcpkg remove --global --force fmt
```

### How It Works

Porters creates a `vcpkg.json` manifest in either:
- `ports/vcpkg/vcpkg.json` (local)
- `~/.porters/packages/vcpkg/vcpkg.json` (global)

The manifest is used with `vcpkg install --x-manifest-root` for dependency management.

## XMake Integration

[XMake](https://xmake.io/) is a modern build system with a built-in package manager (xrepo).

### Prerequisites

Install XMake first:

```bash
# Windows (via installer or Scoop)
scoop install xmake

# Linux/macOS
curl -fsSL https://xmake.io/shget.text | bash
```

### Basic Usage

**Add a package locally:**
```bash
porters xmake add fmt
porters xmake add boost 1.82.0
```

**Add a package globally:**
```bash
porters xmake add --global fmt
```

**List installed packages:**
```bash
# List local packages
porters xmake list

# List global packages
porters xmake list --global
```

**Search for packages:**
```bash
porters xmake search json
```

**Remove a package:**
```bash
# Remove local package (with confirmation)
porters xmake remove fmt

# Remove with force flag
porters xmake remove --force fmt
```

### How It Works

Porters creates an `xmake.lua` file in either:
- `ports/xmake/xmake.lua` (local)
- `~/.porters/packages/xmake/xmake.lua` (global)

The xmake.lua uses:
- `add_requires()` to declare package dependencies
- `xrepo install` to fetch and install packages
- `add_packages()` to link packages to targets

## Common Workflows

### Starting a New Project with External Dependencies

```bash
# Create a new project
porters new myproject --lang cpp

# Add dependencies locally
cd myproject
porters conan add fmt
porters conan add spdlog
porters vcpkg add catch2

# Build the project
porters build
```

### Using Global Packages

```bash
# Install common libraries globally once
porters conan add --global fmt
porters conan add --global spdlog
porters vcpkg add --global catch2

# In each new project, reference them in porters.toml
# (Future feature: automatic global package detection)
```

### Removing Dependencies

```bash
# Remove with confirmation prompt
porters conan remove old-package

# Remove multiple packages with force (no prompts)
porters conan remove --force package1
porters conan remove --force package2
porters conan remove --force package3
```

### Searching for Packages

```bash
# Search across different package managers
porters conan search json
porters vcpkg search json
porters xmake search json

# Compare results and choose the best option
```

## Integration with porters.toml

Package manager installations are complementary to `porters.toml` dependencies. 

Currently, you need to:
1. Add the package via package manager: `porters conan add fmt`
2. Reference it in your `porters.toml` if needed
3. Use it in your build system (CMake, XMake, etc.)

Future versions will provide tighter integration between package managers and `porters.toml`.

## Using with Non-Porters Projects

All package manager commands work even without a `porters.toml` file, making Porters useful as a general package management tool:

```bash
# In any C/C++ project (even without porters.toml)
cd my-cmake-project
porters conan add fmt
porters conan add boost

# The packages are installed to ports/conan/
# Update your CMakeLists.txt to use them
```

## Best Practices

### When to Use Local vs Global

**Use Local Installation when:**
- Different projects need different versions of the same library
- Project has specific configuration requirements
- You want complete isolation between projects
- Working in a team with shared build environments

**Use Global Installation when:**
- Using the same library version across many projects
- Want to save disk space and installation time
- Working on personal projects with consistent dependencies
- Library is stable and unlikely to have version conflicts

### Choosing a Package Manager

**Use Conan if:**
- You need the largest package ecosystem
- Working on cross-platform projects
- Need advanced features like custom profiles
- Building for embedded systems or cross-compilation

**Use vcpkg if:**
- Working primarily on Windows
- Prefer Microsoft ecosystem integration
- Need CMake integration
- Want simple, straightforward manifest-based management

**Use XMake if:**
- Using XMake as your build system
- Want unified build + package management
- Need fast package installation
- Prefer Lua-based configuration

### Package Manager Compatibility

You can use multiple package managers in the same project:

```bash
porters conan add fmt      # Use Conan for fmt
porters vcpkg add catch2   # Use vcpkg for Catch2
porters xmake add imgui    # Use XMake for imgui
```

Each package manager maintains its own directory under `ports/`.

## Troubleshooting

### Package Manager Not Found

If you get "not installed" errors:

1. Verify the tool is installed: `conan --version`, `vcpkg version`, `xmake --version`
2. Ensure the tool is in your PATH
3. Restart your terminal after installation

### Installation Fails

If package installation fails:

1. Check network connectivity
2. Try with `--verbose` flag (future feature)
3. Check package manager logs in `ports/{manager}/`
4. Verify the package name is correct using search: `porters conan search package-name`

### Global Packages Not Found

If global packages aren't accessible:

1. Verify installation: `porters conan list --global`
2. Check `~/.porters/packages/` directory exists
3. Ensure proper permissions on global directory

### Version Conflicts

If you encounter version conflicts:

1. Use local installations for conflicting packages
2. Create separate global environments (future feature)
3. Specify exact versions when adding packages

## Advanced Topics

### Custom Package Repositories (Future)

Future versions will support custom package repositories:

```bash
# Add custom Conan remote
porters conan remote add mycompany https://conan.mycompany.com

# Add private vcpkg registry
porters vcpkg registry add mycompany https://vcpkg.mycompany.com
```

### Environment-Specific Packages (Future)

Future versions will support environment-specific package installations:

```toml
# porters.toml
[dependencies]
fmt = { version = "10.0", package-manager = "conan" }

[dev-dependencies]
catch2 = { version = "3.0", package-manager = "vcpkg" }
```

### Package Locking (Future)

Future versions will include a lock file for reproducible builds:

```bash
# Generate lock file
porters lock generate

# Install from lock file
porters install --locked
```

## See Also

- [Dependencies](dependencies.md) - Managing Porters dependencies
- [Commands Reference](commands.md) - Complete command reference
- [Build Configuration](build-configuration.md) - Configuring builds
