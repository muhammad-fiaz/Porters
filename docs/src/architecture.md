# Architecture

Technical overview of Porters' architecture and design.

## System Overview

Porters follows a modular architecture with clear separation of concerns:

```text
┌─────────────────────────────────────────┐
│          CLI Interface (main.rs)        │
│         Command Parsing (clap)          │
└─────────────┬───────────────────────────┘
              │
              ▼
┌─────────────────────────────────────────┐
│        Core Modules                     │
├─────────────────────────────────────────┤
│  • Config (config.rs)                   │
│  • Scanner (scan.rs)                    │
│  • Dependencies (deps/)                 │
│  • Build Systems (build/)               │
│  • Package Managers (pkg_managers/)     │
│  • Global Config (global.rs)            │
│  • Lock File (lockfile.rs)              │
│  • Binary Cache (bin_cache.rs)          │
│  • Registry (registry.rs)               │
└─────────────┬───────────────────────────┘
              │
              ▼
┌─────────────────────────────────────────┐
│       External Tools                    │
├─────────────────────────────────────────┤
│  • Git (git2)                           │
│  • Build Tools (cmake, xmake, etc.)     │
│  • Package Managers (conan, vcpkg)      │
│  • GitHub API (octocrab)                │
└─────────────────────────────────────────┘
```

## Module Breakdown

### CLI Layer (`main.rs`)

Handles command-line interface using `clap`:

- Command parsing with aliases (e.g., `rm` for `remove`, `ls` for `list`)
- Argument validation
- User interaction (via `dialoguer`)
- Output formatting with emojis and colors

### Configuration (`config.rs`)

Manages project configuration:

- TOML parsing/writing (`serde`, `toml`)
- Configuration validation
- Schema enforcement
- Default configuration generation
- Config overrides from command line

### Package Managers (`pkg_managers/`)

**NEW**: Integration with C/C++ package ecosystems:

#### Supported Managers:
- **Conan** (`conan.rs`) - Creates `conanfile.txt`, CMakeDeps/CMakeToolchain
- **vcpkg** (`vcpkg.rs`) - Manifest mode with `vcpkg.json`
- **XMake** (`xmake.rs`) - `xmake.lua` with xrepo integration

#### Features:
- **Install Scopes**: Local (`ports/`) vs Global (`~/.porters/packages/`)
- **Force Flags**: Skip confirmation prompts with `--force`
- **Version Pinning**: Specify exact package versions
- **Unified Interface**: `PackageManager` trait for consistency

### Scanner (`scan.rs`)

Project structure detection:

- C/C++ file discovery
- Build system detection
- Directory traversal
- Excluded directories (`.porters`, `build`, etc.)

### Dependency Management (`deps/`)

Core dependency resolution:

- Git repository cloning (`git2`)
- Version constraint validation
- Dependency graph resolution
- Lock file management
- Transitive dependency tracking

### Build Systems (`build/`)

Integration with build tools:

- **CMake** support
- **XMake** support
- **Meson** support
- **Make** support
- **Ninja** support
- Custom commands

### Global Configuration (`global.rs`)

Manages global state:

- `~/.porters/` directory structure
- Global package registry
- Settings persistence
- Parallel job configuration
- Global package installations

### Lock File (`lockfile.rs`)

Ensures reproducible builds:

- Dependency version pinning
- Checksum generation (SHA-256)
- Transitive dependency tracking
- Timestamp management
- Git commit resolution

### Binary Cache (`bin_cache.rs`)

Performance optimization:

- Compiled binary caching
- Download caching
- Build artifact reuse
- Cache invalidation strategies

### Registry (`registry.rs`)

Package registry integration:

- Package discovery
- Version resolution
- Metadata management
- Custom registry support (planned)

## Data Flow

### Adding a Dependency

```text
User: porters add fmt --git https://...
  │
  ├─> Parse arguments (main.rs)
  │
  ├─> Validate Git URL (deps/mod.rs)
  │
  ├─> Clone repository (git2)
  │      └─> Download to .porters/cache/sources/
  │
  ├─> Update porters.toml (config.rs)
  │
  └─> Update porters.lock (lockfile.rs)
```

### Adding a Package Manager Dependency

```text
User: porters conan add fmt --global
  │
  ├─> Parse arguments (main.rs)
  │
  ├─> Check if Conan is installed
  │
  ├─> Determine scope (Global vs Local)
  │      ├─> Global: ~/.porters/packages/conan/
  │      └─> Local:  ports/conan/
  │
  ├─> Update conanfile.txt
  │
  └─> Run conan install (optional)
```

### Building a Project

```text
User: porters build
  │
  ├─> Read porters.toml (config.rs)
  │
  ├─> Resolve dependencies (deps/)
  │      ├─> Check porters.lock
  │      ├─> Sync missing deps to .porters/cache/
  │      └─> Validate constraints
  │
  ├─> Check binary cache (bin_cache.rs)
  │      └─> Return if cached build is valid
  │
  ├─> Detect build system (scan.rs, build/)
  │
  ├─> Run build in .porters/build/
  │
  └─> Copy output to build/
         └─> cmake/xmake/meson/make
```

## Directory Structure

### Global Directory (`~/.porters/`)

```text
.porters/
├── config.toml           # Global settings
├── packages/             # Globally installed packages
│   ├── conan/           # Conan global packages
│   │   └── conanfile.txt
│   ├── vcpkg/           # vcpkg global packages
│   │   └── vcpkg.json
│   └── xmake/           # XMake global packages
│       └── xmake.lua
└── cache/                # Download and build cache
    ├── sources/         # Downloaded source archives
    ├── build/           # Build cache
    └── registry/        # Registry metadata cache
```

### Project Directory

```text
my-project/
├── porters.toml          # Project config
├── porters.lock          # Lock file (reproducible builds)
├── .porters/             # Project-specific cache (GITIGNORED)
│   ├── cache/           # Build artifacts, temp files
│   │   ├── sources/     # Cached downloads
│   │   └── build/       # Intermediate build files
│   └── temp/            # Temporary working directory
├── ports/                # Local dependencies
│   ├── fmt/             # Git dependency
│   ├── spdlog/          # Git dependency
│   ├── conan/           # Conan packages (local scope)
│   │   └── conanfile.txt
│   ├── vcpkg/           # vcpkg packages (local scope)
│   │   └── vcpkg.json
│   └── xmake/           # XMake packages (local scope)
│       └── xmake.lua
├── src/                  # Source files
├── include/              # Headers
└── build/                # Final build output (executables, libraries)
```

### Cache Organization

**Key Principle**: `.porters/` contains all generated/temporary files, `build/` contains final artifacts

- **`.porters/cache/`**: Intermediate build files, downloads, temporary data
- **`build/`**: Final compiled binaries and libraries
- **`ports/`**: Local dependency source code
- **`~/.porters/packages/`**: Global package manager installations

## Key Design Decisions

### 1. Dual-Layer Dependencies

**Global** (`~/.porters/packages/`):
- Shared across projects
- Faster setup for common libraries
- Centralized cache
- Reduces disk space usage

**Local** (`ports/`):
- Project-specific isolation
- Version pinning per project
- Reproducible builds
- No conflicts between projects

**Rationale**: Balance between performance and isolation.

### 2. Separated Cache and Build Directories

**`.porters/` (cache)**:
- All temporary and generated files
- Safe to gitignore
- Can be cleaned without losing source code

**`build/` (output)**:
- Final build artifacts only
- Clear separation of concerns
- Matches developer expectations

**Rationale**: Clean separation between intermediate and final artifacts.

### 3. Lock File Format

Uses TOML for human readability and compatibility with `porters.toml`.

**Alternative considered**: Binary format (rejected for poor diff-ability in Git)

### 4. Build System Agnostic

Supports multiple build systems instead of enforcing one.

**Rationale**: Respects existing project structures and developer preferences.

### 5. Git-First Dependency Model

Prioritizes Git repositories over package registries.

**Rationale**:
- C/C++ ecosystem heavily Git-based
- Simpler initial implementation
- Future registry support planned

### 6. Package Manager Integration (NEW)

Integrates with existing C/C++ package managers rather than replacing them.

**Rationale**:
- Leverages existing ecosystems
- No need to repackage libraries
- Familiar workflow for C/C++ developers
- Unified interface across managers

### 7. InstallScope Enum

Type-safe distinction between local and global installations.

**Rationale**:
- Prevents accidental scope mixing
- Clear intent in code
- Compile-time safety

## Technology Stack

### Core Dependencies

- **clap** (4.5): CLI framework with derive macros
- **serde** (1.0): Serialization/deserialization
- **toml** (0.9): TOML parsing and writing
- **tokio** (1.48): Async runtime for concurrent operations
- **git2** (0.20): Git operations (cloning, fetching)
- **dialoguer** (0.12): User prompts and confirmations
- **reqwest** (0.12): HTTP client for downloads
- **self_update** (0.42): Self-update mechanism
- **anyhow** (1.0): Error handling
- **thiserror** (2.0): Error derive macros
- **colored** (3.0): Terminal colors
- **walkdir** (2.5): Recursive directory traversal
- **regex** (1.12): Pattern matching
- **petgraph** (0.8): Dependency graph
- **sha2** (0.10): SHA-256 hashing
- **chrono** (0.4): Timestamps
- **semver** (1.0): Semantic versioning
- **which** (8.0): Executable detection

### Performance Characteristics

- **Async I/O**: Concurrent downloads with tokio
- **Parallel builds**: Configurable via `parallel_jobs`
- **Caching**: Minimizes redundant downloads and builds
- **Binary cache**: Reuses compiled artifacts
- **Incremental builds**: Only rebuilds changed files

## Extensibility

### Adding a New Build System

1. Create module in `src/build/`
2. Implement build detection logic
3. Add build command execution
4. Update `detect_existing_build_system()`
5. Add tests in module
6. Update documentation

### Adding a New Package Manager

1. Create module in `src/pkg_managers/`
2. Implement `PackageManager` trait
3. Add scope-aware install/remove methods
4. Support local and global paths
5. Add confirmation prompts
6. Create comprehensive tests
7. Update package-managers.md

Example skeleton:

```rust
pub struct NewManager {
    ports_dir: String,
}

impl PackageManager for NewManager {
    fn install(&self, package: &str, version: Option<&str>, scope: InstallScope) -> Result<()> {
        // Implementation
    }
    
    fn remove(&self, package: &str, scope: InstallScope, force: bool) -> Result<()> {
        // With confirmation if !force
    }
    
    fn get_install_path(&self, scope: InstallScope) -> PathBuf {
        match scope {
            InstallScope::Local => PathBuf::from(&self.ports_dir).join("newmanager"),
            InstallScope::Global => {
                let home = std::env::var("HOME")
                    .or_else(|_| std::env::var("USERPROFILE"))
                    .unwrap_or_else(|_| ".".to_string());
                PathBuf::from(home).join(".porters").join("packages").join("newmanager")
            }
        }
    }
    
    // ... other methods
}
```

### Adding a New Dependency Source

1. Extend `DependencySource` enum (deps/mod.rs)
2. Implement cloning/fetching logic
3. Add constraint validation
4. Update lock file format
5. Add tests
6. Update documentation

## Configuration Override System

**Automatic Behavior** (default):
- Detects build system automatically
- Uses sensible defaults
- Configures caching automatically

**Manual Overrides** (`porters.toml`):
- `[build]` section overrides build system
- `[cache]` section overrides cache settings
- `[dependencies]` overrides auto-detection

Example:

```toml
[project]
name = "myproject"

[build]
system = "cmake"  # Overrides auto-detection
generator = "Ninja"  # Overrides default

[cache]
enabled = true
max_size = "5GB"  # Overrides default

[dependencies]
# Manual dependency specification overrides auto-scan
```

## Cross-Platform Support

### Path Handling
- Uses `PathBuf` for all paths
- Handles both `/` and `\` separators
- Respects `HOME` (Unix) and `USERPROFILE` (Windows)

### Build System Detection
- Checks for executables with `which` crate
- Platform-specific defaults (MSVC on Windows, GCC on Unix)

### Testing
- All tests pass on Windows, Linux, macOS
- Path assertions handle both separators
- No hardcoded platform assumptions

## Next Steps

- [Development Guide](./development.md)
- [Contributing](./contributing.md)
- [Package Managers Guide](./package-managers.md)
