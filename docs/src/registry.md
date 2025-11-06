# Package Registry

Porters includes its own package registry system for discovering and installing C/C++ libraries. The registry is a curated collection of package definitions stored as JSON files.

## 📦 What is the Registry?

The Porters registry is a community-driven collection of package definitions for popular C/C++ libraries. Each package includes:

- **Repository URL** - Where to find the source code
- **Version Information** - Latest stable version
- **Dependencies** - Required packages with version constraints
- **Build System** - How to build the package (CMake, Meson, XMake, etc.)
- **Platform Support** - Which operating systems are supported
- **Constraints** - Compiler requirements, C++ standard, architecture, etc.

## 🌐 Registry Synchronization

The Porters registry is hosted on GitHub at [muhammad-fiaz/porters](https://github.com/muhammad-fiaz/porters) and is automatically synchronized to your local machine.

### How Registry Sync Works

1. **First Use**: On first registry operation, Porters clones the registry from GitHub
2. **Local Index**: Registry is stored in `~/.porters/registry-index/`
3. **Fast Searches**: All searches use the local index (no network needed)
4. **Auto-Update**: Registry can auto-update based on configuration

### Registry Locations

| Location | Purpose | Path |
|----------|---------|------|
| **Remote** | Official registry | https://github.com/muhammad-fiaz/porters |
| **Local Index** | Cached registry | `~/.porters/registry-index/` |

### Update Registry

Manually update the local registry index from GitHub:

```bash
porters registry update
```

This fetches the latest package definitions from the remote repository.

### Auto-Update Configuration

Configure automatic registry updates in `~/.porters/config.toml`:

```toml
[registry]
url = "https://github.com/muhammad-fiaz/porters"
auto_update = true  # Update registry automatically
index_path = "~/.porters/registry-index"
```

**Auto-update behavior**:
- Checks for updates on first use each day
- Updates in background (non-blocking)
- Falls back to cached index if update fails

### Offline Mode

The registry works offline using the local index:

```bash
# Search offline
porters search --offline fmt

# Get package info offline
porters info --offline spdlog
```

When offline:
- ✅ Uses local registry index
- ❌ No remote updates
- ⚠️ Package info may be outdated

**Tip**: Run `porters registry update` periodically to keep the local index fresh.

### Registry Structure

The local registry index is organized as:

```
~/.porters/registry-index/
└── packages/
    ├── fmt.json
    ├── spdlog.json
    ├── catch2.json
    ├── boost.json
    └── ...
```

Each JSON file contains the package definition with metadata, dependencies, and build instructions.

## 🚀 Using the Registry

### Search for Packages

Search the registry by name, description, or tags:

```bash
porters registry search <query>
porters search <query>  # Shorter alias
```

**Examples:**
```bash
# Search for networking libraries
porters search networking

# Search for a specific library
porters search catch2

# Search by tag
porters search testing
```

### Install from Registry

Add a package from the registry to your project:

```bash
porters registry add <package-name>
porters add <package-name>  # Will check registry if not in package managers
```

**Examples:**
```bash
# Install from registry
porters registry add catch2

# Install with automatic fallback
porters add asio  # Checks Conan, vcpkg, XMake, then Registry
```

### List All Packages

View all available packages in the registry:

```bash
porters registry list
```

### Package Information

View detailed information about a package:

```bash
porters info <package-name>
```

## 📖 Package Definition Format

Packages are defined in JSON files following this schema:

```json
{
  "name": "package-name",
  "description": "Short description (max 120 characters)",
  "repository": "https://github.com/owner/repo",
  "version": "1.2.3",
  "license": "MIT",
  "build_system": "cmake",
  
  "dependencies": {
    "fmt": "^9.0.0",
    "spdlog": "^1.10.0"
  },
  
  "dev_dependencies": {
    "catch2": "^3.0.0"
  },
  
  "options": {
    "shared": false,
    "tests": true,
    "examples": false
  },
  
  "install": {
    "cmake": {
      "find_package": "PackageName",
      "targets": ["PackageName::PackageName"],
      "components": ["Core", "Extras"]
    }
  },
  
  "tags": ["networking", "async", "modern-cpp"],
  "homepage": "https://example.com",
  "documentation": "https://docs.example.com",
  "platforms": ["linux", "windows", "macos"],
  
  "constraints": {
    "min_cpp_standard": "17",
    "max_cpp_standard": "23",
    "compilers": {
      "gcc": ">=9.0",
      "clang": ">=10.0",
      "msvc": ">=19.20"
    },
    "arch": ["x86_64", "arm64"],
    "environment": {
      "SOME_VAR": "required_value"
    }
  },
  
  "features": {
    "ssl": {
      "description": "Enable SSL support",
      "default": false,
      "dependencies": {
        "openssl": "^3.0.0"
      }
    }
  }
}
```

### Required Fields

- **name** - Package name (lowercase, hyphenated, e.g., `awesome-lib`)
- **description** - Short description (10-120 characters)
- **repository** - Git repository URL (must be `https://`)
- **version** - Latest stable version (SemVer format, e.g., `1.2.3`)
- **license** - License identifier (SPDX format, e.g., `MIT`, `Apache-2.0`)
- **build_system** - One of: `cmake`, `meson`, `xmake`, `autotools`, `bazel`, `custom`

### Optional Fields

- **dependencies** - Runtime dependencies (map of name → version requirement)
- **dev_dependencies** - Development-only dependencies
- **options** - Build options (shared libraries, tests, examples, etc.)
- **install** - Installation metadata for build systems
- **tags** - Searchable tags for categorization
- **homepage** - Project homepage URL
- **documentation** - Documentation URL
- **platforms** - Supported platforms (default: all)
- **constraints** - Build constraints and requirements
- **features** - Optional features with their own dependencies

## 🔄 Dependency Resolution

Porters automatically resolves nested dependencies with:

### Version Constraints

Use semantic versioning constraints:

```json
{
  "dependencies": {
    "fmt": "^9.0.0",      // Compatible with 9.x.x
    "spdlog": "~1.10.0",  // Compatible with 1.10.x
    "boost": ">=1.75.0",  // At least 1.75.0
    "zlib": "1.2.11"      // Exact version
  }
}
```

**Constraint Syntax:**
- `^1.2.3` - Compatible (1.2.3 ≤ version < 2.0.0)
- `~1.2.3` - Approximately (1.2.3 ≤ version < 1.3.0)
- `>=1.2.3` - Greater than or equal
- `>1.2.3` - Greater than
- `<=1.2.3` - Less than or equal
- `<1.2.3` - Less than
- `1.2.3` - Exact version

### Nested Dependencies

Porters resolves dependencies recursively:

```
your-project
├── awesome-lib ^1.0.0
│   ├── fmt ^9.0.0
│   └── spdlog ^1.10.0
│       └── fmt ^8.0.0  ← Conflict detected!
```

**Conflict Resolution:**
- Porters detects version conflicts
- Shows all conflicting requirements
- Fails with clear error messages
- Suggests resolution strategies

### Platform Constraints

Packages can specify platform requirements:

```json
{
  "platforms": ["linux", "windows", "macos"],
  "constraints": {
    "arch": ["x86_64", "arm64"]
  }
}
```

If a platform constraint fails, installation is prevented with a clear error message.

### Compiler Requirements

Packages can specify compiler versions:

```json
{
  "constraints": {
    "min_cpp_standard": "17",
    "compilers": {
      "gcc": ">=9.0",
      "clang": ">=10.0",
      "msvc": ">=19.20"
    }
  }
}
```

Porters validates constraints before installation and warns about incompatibilities.

### Environment Variables

Packages can require specific environment variables:

```json
{
  "constraints": {
    "environment": {
      "JAVA_HOME": "/usr/lib/jvm/java-11",
      "PYTHON_VERSION": "3.9"
    }
  }
}
```

Missing or mismatched environment variables cause installation to fail.

## 🤝 Contributing Packages

Want to add your library to the registry? Here's how:

### 1. Create Package Definition

Create a JSON file in the appropriate category:

```bash
registry/
├── networking/       # Network libraries
├── graphics/         # Graphics & rendering
├── testing/          # Testing frameworks
├── serialization/    # JSON, XML, etc.
├── compression/      # Compression libraries
├── crypto/           # Cryptography
├── databases/        # Database clients
├── gui/              # GUI frameworks
├── audio/            # Audio processing
└── utilities/        # General utilities
```

### 2. Validate Schema

Ensure your package follows the schema in `registry/schema.json`:

```bash
# JSON schema validator
jsonschema -i registry/your-category/your-package.json registry/schema.json
```

### 3. Test Package

Test that your package can be found and installed:

```bash
# Search for your package
porters search your-package

# Install and build
porters add your-package
porters build
```

### 4. Submit Pull Request

1. Fork the [Porters repository](https://github.com/muhammad-fiaz/Porters)
2. Add your package JSON file to `registry/category/`
3. Create categories if needed (e.g., `registry/ml/` for machine learning)
4. Submit a Pull Request with:
   - Package JSON file
   - Brief description of the library
   - Build and test instructions
   - Special installation notes (if any)

## 📂 Registry Structure

The registry is organized into categories:

```
registry/
├── README.md              # Registry documentation
├── schema.json            # JSON schema for validation
├── audio/                 # Audio processing
│   └── portaudio.json
├── compression/           # Compression
│   ├── zlib.json
│   └── bzip2.json
├── crypto/                # Cryptography
│   ├── openssl.json
│   └── libsodium.json
├── databases/             # Databases
│   ├── sqlite.json
│   └── postgresql.json
├── graphics/              # Graphics
│   ├── sdl2.json
│   └── glfw.json
├── gui/                   # GUI frameworks
│   ├── qt.json
│   └── wxwidgets.json
├── networking/            # Networking
│   ├── asio.json
│   └── curl.json
├── serialization/         # Serialization
│   ├── json.json
│   └── protobuf.json
├── testing/               # Testing
│   ├── catch2.json
│   └── gtest.json
└── utilities/             # Utilities
    ├── fmt.json
    └── spdlog.json
```

## 🎯 Best Practices

### Package Naming

- Use lowercase letters
- Separate words with hyphens: `awesome-lib`
- Match the upstream project name when possible
- Keep it short and descriptive

### Descriptions

- Maximum 120 characters
- Focus on what the library does
- Include key features or use cases
- Example: "Modern C++ networking library with async I/O and coroutines"

### Version Requirements

- Use semantic versioning
- Prefer compatible (`^`) or approximate (`~`) constraints
- Avoid exact versions unless absolutely necessary
- Test with multiple dependency versions

### Tags

- Use lowercase
- Separate words with hyphens
- Include relevant categories: `networking`, `async`, `header-only`
- Add use cases: `gamedev`, `web`, `embedded`
- Include C++ features: `modern-cpp`, `cpp17`, `coroutines`

### Dependencies

- Only list runtime dependencies in `dependencies`
- Put testing/benchmarking deps in `dev_dependencies`
- Always specify version constraints
- Test dependency resolution

### Build Options

- Document all available options
- Provide sensible defaults
- Explain what each option does
- Consider platform-specific defaults

## 🔍 How It Works

### Registry Discovery

When you run `porters add <package>`, Porters searches in order:

1. **Conan** - Check if available in Conan Center
2. **vcpkg** - Check if available in vcpkg registry
3. **XMake** - Check if available in XMake repo
4. **Porters Registry** - Search local registry JSON files

### Dependency Resolution Process

1. **Parse root dependencies** from your `porters.toml`
2. **Fetch package metadata** from registry
3. **Validate version constraints** (SemVer)
4. **Check platform requirements** (OS, architecture)
5. **Verify compiler constraints** (version, C++ standard)
6. **Resolve nested dependencies** recursively
7. **Detect circular dependencies** using graph analysis
8. **Detect version conflicts** across dependency tree
9. **Topological sort** for installation order
10. **Install packages** from source or cache

### Caching

All dependencies are cached globally:

```
~/.porters/
└── packages/
    ├── conan/
    │   └── fmt/
    │       └── 9.1.0/
    ├── vcpkg/
    │   └── spdlog/
    │       └── 1.11.0/
    ├── xmake/
    │   └── boost/
    │       └── 1.80.0/
    └── registry/
        └── catch2/
            └── 3.5.0/
```

Packages are shared across all projects, avoiding redundant downloads.

## 📊 Examples

### Example: Adding Catch2

```bash
# Search for Catch2
$ porters search catch2
📦 catch2
   Modern, C++-native, header-only test framework
   Version: 3.5.0
   License: BSL-1.0
   Build System: cmake
   Repository: https://github.com/catchorg/Catch2
   Tags: testing, unit-testing, tdd, bdd

# Add to project
$ porters add catch2
✅ Added catch2 ^3.5.0 to dependencies
📦 Resolving dependencies...
   catch2 ^3.5.0
   └── (no dependencies)
✅ All dependencies resolved (1 package)

# Build and test
$ porters build
$ porters test
```

### Example: Nested Dependencies

```bash
# Add awesome-lib which depends on fmt and spdlog
$ porters add awesome-lib
📦 Resolving dependencies...
   awesome-lib ^1.0.0
   ├── fmt ^9.0.0
   └── spdlog ^1.10.0
       └── fmt ^8.0.0  ← Different version required

❌ Dependency Conflicts Detected:
  Package: fmt
  Requested versions:
    - awesome-lib requires ^9.0.0
    - spdlog requires ^8.0.0

💡 Suggestion: Update spdlog to a version compatible with fmt ^9.0.0
```

### Example: Platform Constraints

```bash
# Try to install Windows-only library on Linux
$ porters add windows-only-lib
❌ Error: Package windows-only-lib is not available for platform 'linux'
   Supported platforms: windows

# Architecture warning
$ porters add x86-only-lib
⚠️  Warning: Package x86-only-lib may not support architecture 'x86_64'
   Supported architectures: x86
```

## 🆘 Troubleshooting

### Package Not Found

```bash
$ porters add nonexistent
❌ Error: Package 'nonexistent' not found in registry
💡 Try: porters search nonexistent
```

**Solution:** Check spelling, search the registry, or add the package to the registry.

### Version Conflicts

```bash
❌ Dependency Conflicts Detected:
  Package: fmt
  Requested versions:
    - lib-a requires ^9.0.0
    - lib-b requires ^8.0.0
```

**Solutions:**
1. Update one library to a compatible version
2. Use features to make dependencies optional
3. Fork and update dependency versions
4. Report conflict to library maintainers

### Platform Incompatibility

```bash
❌ Error: Package only-linux is not available for platform 'windows'
```

**Solution:** Check `platforms` field, use alternatives, or contribute Windows support.

### Compiler Requirements

```bash
⚠️  Warning: Package requires GCC >=11.0, you have 9.4.0
```

**Solutions:**
1. Upgrade your compiler
2. Use a different package version
3. Check if constraint can be relaxed

## 🔗 Related

- [Dependencies](dependencies.md) - Managing project dependencies
- [Building](building.md) - Building your project
- [Configuration](configuration.md) - Configuring Porters
- [Contributing](contributing.md) - Contributing to Porters

## 📚 Reference

- **Registry Path:** `registry/` in Porters repository
- **Schema:** `registry/schema.json`
- **GitHub:** https://github.com/muhammad-fiaz/Porters
- **Documentation:** https://muhammad-fiaz.github.io/Porters/
- **Issues:** https://github.com/muhammad-fiaz/Porters/issues
