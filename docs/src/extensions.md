# Extension System

Porters features a powerful extension system that allows you to create custom functionality and share it with others.

## What are Extensions?

Extensions are plugins that can:
- Hook into build lifecycle events (pre-build, post-build, pre-install, post-install)
- Add custom commands to Porters
- Extend functionality for specific build systems
- Automate project-specific tasks

## Installing Extensions

**Important**: Extensions are **user-installed** and managed locally. Porters does not automatically download extensions from crates.io or any package registry. You install extensions manually and Porters loads them from your local installation.

### Manual Installation (Recommended)

Install extensions to `~/.porters/extensions/<extension-name>/`:

```bash
# 1. Create extensions directory if it doesn't exist
mkdir -p ~/.porters/extensions

# 2. Clone extension repository
cd ~/.porters/extensions
git clone https://github.com/user/porters-ext-myext my-extension

# 3. Porters automatically loads all extensions from this directory
```

### From Git (via Porters)

Porters can help you clone extensions:

```bash
porters extension install my-extension --git https://github.com/user/porters-ext-myext
```

This clones the repository to `~/.porters/extensions/my-extension/`.

### From Local Path

Link a local extension directory:

```bash
porters extension install my-extension --path ./path/to/extension
```

This creates a copy or symlink in `~/.porters/extensions/`.

### Auto-Load from Configuration

List extensions in `porters.toml` to ensure they're loaded when Porters runs:

```toml
# Extensions to auto-load (must be installed in ~/.porters/extensions/)
extensions = [
    "my-extension",
    "another-extension",
    "formatter"
]
```

**Note**: This does NOT install extensions. It only tells Porters which installed extensions to load. You must install extensions manually first.

## Extension Installation Workflow

**Typical workflow for using an extension:**

1. **Find Extension**: Browse GitHub, documentation, or community recommendations
2. **Install Manually**: Clone to `~/.porters/extensions/<name>/`
3. **Configure (Optional)**: Add to `extensions` array in `porters.toml`
4. **Use**: Extension hooks and commands are now available

**Example:**

```bash
# 1. Install manually
cd ~/.porters/extensions
git clone https://github.com/user/porters-formatter formatter

# 2. Add to porters.toml (optional)
echo 'extensions = ["formatter"]' >> porters.toml

# 3. Use extension commands
porters format
```

## Creating Extensions

### Generate Extension Template

```bash
porters extension create my-awesome-extension
```

This creates a new extension directory with the following structure:

```
my-awesome-extension/
├── extension.toml      # Extension manifest
├── README.md          # Documentation
└── hooks/             # Hook scripts
    └── example.sh
```

### Extension Manifest (`extension.toml`)

```toml
[package]
name = "my-awesome-extension"
version = "0.1.0"
description = "My awesome Porters extension"
authors = ["Your Name <you@example.com>"]
license = "MIT"
repository = "https://github.com/user/porters-ext-myext"
homepage = "https://example.com"

[hooks]
pre-build = "hooks/pre_build.sh"
post-build = "hooks/post_build.sh"
pre-install = "hooks/pre_install.sh"
post-install = "hooks/post_install.sh"

[[commands]]
name = "custom-command"
description = "My custom command"
script = "scripts/custom.sh"
```

## Extension Hooks

Extensions can hook into various lifecycle events:

### Build Hooks

```toml
[hooks]
pre-build = "hooks/pre_build.sh"   # Run before building
post-build = "hooks/post_build.sh" # Run after building
```

Example pre-build hook (`hooks/pre_build.sh`):

```bash
#!/bin/sh
echo "🔧 Running pre-build checks..."

# Check for required tools
command -v cmake >/dev/null 2>&1 || {
    echo "❌ CMake is required but not installed"
    exit 1
}

# Generate build files
cmake -B build -S .

echo "✅ Pre-build complete"
```

### Install Hooks

```toml
[hooks]
pre-install = "hooks/pre_install.sh"   # Run before installing dependencies
post-install = "hooks/post_install.sh" # Run after installing dependencies
```

Example post-install hook:

```bash
#!/bin/sh
echo "📦 Post-install: Setting up environment..."

# Copy configuration files
cp config.template.toml config.toml

# Set permissions
chmod +x scripts/*.sh

echo "✅ Post-install complete"
```

## Custom Commands

Extensions can add new commands to Porters:

```toml
[[commands]]
name = "lint"
description = "Run code linting"
script = "scripts/lint.sh"

[[commands]]
name = "format"
description = "Format code"
script = "scripts/format.sh"
```

Usage:

```bash
porters lint     # Runs the extension's lint command
porters format   # Runs the extension's format command
```

## Extension Examples

### Example 1: Code Formatter Extension

```toml
# extension.toml
name = "porters-format"
version = "0.1.0"
description = "Code formatting extension for C/C++"

[[commands]]
name = "format"
description = "Format C/C++ code with clang-format"
script = "scripts/format.sh"
```

```bash
# scripts/format.sh
#!/bin/sh
find src -name "*.c" -o -name "*.cpp" -o -name "*.h" | xargs clang-format -i
echo "✅ Code formatted!"
```

### Example 2: Documentation Generator

```toml
name = "porters-docs"
version = "0.1.0"
description = "Generate documentation with Doxygen"

[hooks]
post-build = "hooks/generate_docs.sh"

[[commands]]
name = "docs"
description = "Generate documentation"
script = "scripts/docs.sh"
```

### Example 3: Testing Extension

```toml
name = "porters-test"
version = "0.1.0"
description = "Enhanced testing utilities"

[[commands]]
name = "test-all"
description = "Run all tests with coverage"
script = "scripts/test_coverage.sh"

[[commands]]
name = "test-watch"
description = "Watch for changes and re-run tests"
script = "scripts/test_watch.sh"
```

## Publishing Extensions

### 1. Prepare for Publishing

Ensure your extension has:
- A complete `extension.toml` manifest
- A `README.md` with usage instructions
- Proper license file
- Working hooks/scripts

### 2. Create a Rust Crate

Since extensions can be published to crates.io, create a minimal Rust wrapper:

```bash
cd my-extension
cargo init --lib
```

Edit `Cargo.toml`:

```toml
[package]
name = "porters-ext-myext"
version = "0.1.0"
description = "My Porters extension"
authors = ["Your Name <you@example.com>"]
license = "MIT"
repository = "https://github.com/user/porters-ext-myext"

[package.metadata.porters]
extension = true
```

Add extension files to the package:

```toml
[package]
include = [
    "extension.toml",
    "hooks/**/*",
    "scripts/**/*",
    "README.md",
]
```

### 3. Publish to crates.io

```bash
cargo publish
```

### 4. Users Can Now Install

```bash
porters extension install porters-ext-myext
```

## Managing Extensions

### List Installed Extensions

```bash
porters extension list
```

Output:
```
📦 Installed Extensions:

  porters-format v1.0.0
    Code formatting extension for C/C++
    Repository: https://github.com/user/porters-format

  porters-docs v0.5.0
    Generate documentation with Doxygen
    Repository: https://github.com/user/porters-docs
```

### Uninstall Extension

```bash
porters extension uninstall my-extension
```

## Extension Best Practices

### 1. Error Handling

Always provide clear error messages:

```bash
#!/bin/sh
if ! command -v tool >/dev/null 2>&1; then
    echo "❌ ERROR: 'tool' is required but not installed"
    echo "   Install with: apt install tool"
    exit 1
fi
```

### 2. Cross-Platform Support

Use POSIX-compliant shell scripts or provide platform-specific scripts:

```toml
[hooks.linux]
pre-build = "hooks/linux/pre_build.sh"

[hooks.windows]
pre-build = "hooks/windows/pre_build.bat"

[hooks.macos]
pre-build = "hooks/macos/pre_build.sh"
```

### 3. Environment Variables

Extensions have access to project context:

```bash
#!/bin/sh
# Available environment variables:
# PORTERS_PROJECT_DIR - Project root directory
# PORTERS_BUILD_DIR - Build directory
# PORTERS_PROJECT_NAME - Project name from porters.toml

echo "Building ${PORTERS_PROJECT_NAME}..."
```

### 4. Dependency Management

Document required tools in your README:

```markdown
## Requirements

- clang-format >= 10.0
- cmake >= 3.15
- python3

## Installation

Ubuntu/Debian:
apt install clang-format cmake python3

macOS:
brew install clang-format cmake python3
```

### 5. Versioning

Follow semantic versioning:
- MAJOR: Breaking changes
- MINOR: New features (backward compatible)
- PATCH: Bug fixes

## Advanced Features

### Extension Configuration

Extensions can have their own configuration in `porters.toml`:

```toml
[extensions.my-extension]
enabled = true
config-file = ".my-extension.yml"
options = { verbose = true, strict = false }
```

Access in scripts:

```bash
#!/bin/sh
# Check if extension is enabled
if [ "${MY_EXT_ENABLED}" = "true" ]; then
    # Run extension logic
fi
```

### Chaining Extensions

Multiple extensions can hook into the same lifecycle event. They execute in installation order.

### Extension Dependencies

Extensions can depend on other extensions:

```toml
[dependencies]
porters-base-utils = "1.0"
porters-cmake-helpers = "0.5"
```

## Troubleshooting

### Extension Not Found

```bash
❌ Extension 'my-ext' not found
```

**Solution**: Check the extension name and ensure it's published to crates.io or use full git URL.

### Hook Script Not Executable

```bash
❌ Permission denied: hooks/pre_build.sh
```

**Solution**: Make scripts executable:

```bash
chmod +x hooks/*.sh
```

### Extension Conflicts

```bash
⚠️  Warning: Multiple extensions define command 'build'
```

**Solution**: Uninstall conflicting extensions or rename commands.

## Community Extensions

Popular Porters extensions:

- **porters-format**: Code formatting with clang-format
- **porters-lint**: Static analysis with clang-tidy
- **porters-docs**: Documentation generation
- **porters-test**: Enhanced testing utilities
- **porters-cmake**: CMake project helpers
- **porters-conan**: Conan integration
- **porters-vcpkg**: vcpkg integration

## Contributing

Create useful extensions and share them with the community! Submit your extension to [awesome-porters](https://github.com/muhammad-fiaz/awesome-porters) to be featured.

## Resources

- [Extension API Reference](./api-reference.md)
- [Extension Examples](https://github.com/muhammad-fiaz/porters-extensions)
- [Publishing Guide](./publishing.md)
- [Community Forum](https://github.com/muhammad-fiaz/Porters/discussions)
