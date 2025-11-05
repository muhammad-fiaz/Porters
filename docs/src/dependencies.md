# Dependency Management

Porters provides flexible dependency management for C/C++ projects with support for multiple sources and isolation strategies.

## Adding Dependencies

### Basic Usage

Add a dependency from a Git repository:

```bash
porters add fmt --git https://github.com/fmtlib/fmt
```

This adds `fmt` to your `porters.toml`:

```toml
[dependencies]
fmt = { git = "https://github.com/fmtlib/fmt" }
```

### Adding Dev Dependencies

Development-only dependencies (tests, benchmarks):

```bash
porters add catch2 --git https://github.com/catchorg/Catch2 --dev
```

### Adding Optional Dependencies

Optional features:

```bash
porters add zlib --git https://github.com/madler/zlib --optional
```

## Global vs Local Dependencies

Porters supports two dependency scopes:

### Local Dependencies (Default)

Dependencies added with `porters add` are installed to the project's `ports/` folder:

```bash
porters add fmt --git https://github.com/fmtlib/fmt
```

**Location**: `./ports/fmt/`

**Characteristics**:
- Isolated per-project
- Version-locked via `porters.lock`
- Perfect for project-specific needs

### Global Dependencies

Install packages globally to `~/.porters/packages/`:

```bash
porters install fmt --git https://github.com/fmtlib/fmt
```

**Location (Linux/macOS)**: `~/.porters/packages/fmt/`  
**Location (Windows)**: `C:\Users\<username>\.porters\packages\fmt\`

**Characteristics**:
- Shared across all projects
- Faster setup for frequently-used libraries
- Centralized cache

## Git Dependencies

Porters supports both HTTPS and SSH Git URLs:

### HTTPS URLs

```bash
porters add fmt --git https://github.com/fmtlib/fmt
```

### SSH URLs

```bash
porters add fmt --git git@github.com:fmtlib/fmt.git
```

### Specific Branch

```bash
porters add fmt --git https://github.com/fmtlib/fmt --branch stable
```

### Specific Tag

```bash
porters add fmt --git https://github.com/fmtlib/fmt --tag 10.1.1
```

In `porters.toml`:

```toml
[dependencies]
fmt = { git = "https://github.com/fmtlib/fmt", tag = "10.1.1" }
```

## Syncing Dependencies

The `sync` command ensures all dependencies from `porters.toml` are installed:

```bash
porters sync
```

This will:
1. Read dependencies from `porters.toml`
2. Download missing packages to `ports/`
3. Resolve version constraints
4. Update `porters.lock`

### Include Dev Dependencies

```bash
porters sync --dev
```

Syncs both regular and dev dependencies.

### Include Optional Dependencies

```bash
porters sync --optional
```

Syncs both regular and optional dependencies.

### Include Everything

```bash
porters sync --dev --optional
```

## Lock File

The `porters.lock` file tracks resolved dependency versions.

### Generating/Updating Lock File

```bash
porters lock
```

This updates `porters.lock` with current installed dependencies.

### Lock File Format

```toml
version = "1"
updated_at = "2024-01-15T10:30:00Z"

[dependencies.fmt]
name = "fmt"
version = "10.1.1"
source = { Git = { url = "https://github.com/fmtlib/fmt", rev = "a1b2c3d" } }
checksum = "sha256:..."
dependencies = []
```

### Why Use Lock Files?

- **Reproducibility**: Same versions across all environments
- **Team Collaboration**: Everyone gets identical dependencies
- **CI/CD**: Reliable builds in pipelines

## Dependency Sources

Porters supports multiple dependency sources:

### Git Repository

```toml
[dependencies]
fmt = { git = "https://github.com/fmtlib/fmt" }
spdlog = { git = "git@github.com:gabime/spdlog.git", branch = "v1.x" }
```

### Local Path

```toml
[dependencies]
mylib = { path = "../mylib" }
```

### Registry (Future)

Support for package registries is planned:

```toml
[dependencies]
boost = { registry = "conan", version = "1.80" }
```

## Dependency Resolution

Porters resolves dependencies in this order:

1. Check `porters.lock` for existing resolution
2. Fetch from Git/path source
3. Verify constraints (version, branch, tag)
4. Clone to `ports/` directory
5. Update `porters.lock`

### Constraint Validation

Porters validates:
- Version requirements
- Branch/tag specifications
- Dependency conflicts

## Best Practices

### Version Pinning

Always specify versions or tags for production:

```toml
[dependencies]
fmt = { git = "https://github.com/fmtlib/fmt", tag = "10.1.1" }
```

### Lock File in Version Control

Commit `porters.lock` to ensure reproducible builds:

```bash
git add porters.lock
git commit -m "Update dependencies"
```

### Separate Dev Dependencies

Keep development tools separate:

```toml
[dependencies]
fmt = { git = "https://github.com/fmtlib/fmt" }

[dev-dependencies]
catch2 = { git = "https://github.com/catchorg/Catch2" }
benchmark = { git = "https://github.com/google/benchmark" }
```

### Use Global Install for Common Libraries

Install frequently-used libraries globally:

```bash
porters install boost --git https://github.com/boostorg/boost
porters install gtest --git https://github.com/google/googletest
```

Then reference them in projects without re-downloading.

## Troubleshooting

### Dependency Not Found

Ensure the Git URL is correct and accessible:

```bash
git clone https://github.com/fmtlib/fmt  # Test manually
```

### Version Conflicts

Check `porters.lock` and resolve conflicts:

```bash
porters lock  # Regenerate lock file
```

### Sync Failures

Clear the cache and retry:

```bash
rm -rf ports/
porters sync
```

## Next Steps

- Explore [Build Configuration](./building.md)
- Review [Command Reference](./commands.md)
- Check [Troubleshooting](./troubleshooting.md)
