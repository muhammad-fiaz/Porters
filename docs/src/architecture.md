# Architecture

Technical overview of Porters' architecture and design.

## System Overview

Porters follows a modular architecture with clear separation of concerns:

```
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
│  • Global Config (global.rs)            │
│  • Lock File (lockfile.rs)              │
└─────────────┬───────────────────────────┘
              │
              ▼
┌─────────────────────────────────────────┐
│       External Tools                    │
├─────────────────────────────────────────┤
│  • Git (git2)                           │
│  • Build Tools (cmake, xmake, etc.)     │
│  • GitHub API (octocrab)                │
└─────────────────────────────────────────┘
```

## Module Breakdown

### CLI Layer (`main.rs`)

Handles command-line interface using `clap`:

- Command parsing
- Argument validation
- User interaction (via `dialoguer`)
- Output formatting

### Configuration (`config.rs`)

Manages project configuration:

- TOML parsing/writing (`serde`, `toml`)
- Configuration validation
- Schema enforcement

### Scanner (`scan.rs`)

Project structure detection:

- C/C++ file discovery
- Build system detection
- Directory traversal

### Dependency Management (`deps/`)

Core dependency resolution:

- Git repository cloning (`git2`)
- Version constraint validation
- Dependency graph resolution
- Lock file management

### Build Systems (`build/`)

Integration with build tools:

- CMake support
- XMake support
- Meson support
- Make support
- Custom commands

### Global Configuration (`global.rs`)

Manages global state:

- `~/.porters/` directory structure
- Global package registry
- Settings persistence
- Parallel job configuration

### Lock File (`lockfile.rs`)

Ensures reproducible builds:

- Dependency version pinning
- Checksum generation
- Transitive dependency tracking
- Timestamp management

## Data Flow

### Adding a Dependency

```
User: porters add fmt --git https://...
  │
  ├─> Parse arguments (main.rs)
  │
  ├─> Validate Git URL (deps/mod.rs)
  │
  ├─> Clone repository (git2)
  │      └─> Download to ports/fmt/
  │
  ├─> Update porters.toml (config.rs)
  │
  └─> Update porters.lock (lockfile.rs)
```

### Building a Project

```
User: porters build
  │
  ├─> Read porters.toml (config.rs)
  │
  ├─> Resolve dependencies (deps/)
  │      ├─> Check porters.lock
  │      ├─> Sync missing deps
  │      └─> Validate constraints
  │
  ├─> Detect build system (scan.rs, build/)
  │
  └─> Execute build commands
         └─> cmake/xmake/meson/make
```

## Directory Structure

### Global Directory (`~/.porters/`)

```
.porters/
├── config.toml           # Global settings
├── packages/             # Globally installed packages
│   ├── fmt/
│   ├── spdlog/
│   └── ...
└── cache/                # Download cache
```

### Project Directory

```
my-project/
├── porters.toml          # Project config
├── porters.lock          # Lock file
├── ports/                # Local dependencies
│   ├── fmt/
│   └── spdlog/
├── src/                  # Source files
├── include/              # Headers
└── build/                # Build output
```

## Key Design Decisions

### 1. Dual-Layer Dependencies

**Global** (`~/.porters/packages/`):
- Shared across projects
- Faster setup for common libraries
- Centralized cache

**Local** (`ports/`):
- Project-specific isolation
- Version pinning per project
- Reproducible builds

**Rationale**: Balance between performance and isolation.

### 2. Lock File Format

Uses TOML for human readability and compatibility with `porters.toml`.

**Alternative considered**: Binary format (rejected for poor diff-ability in Git)

### 3. Build System Agnostic

Supports multiple build systems instead of enforcing one.

**Rationale**: Respects existing project structures and developer preferences.

### 4. Git-First Dependency Model

Prioritizes Git repositories over package registries.

**Rationale**:
- C/C++ ecosystem heavily Git-based
- Simpler initial implementation
- Future registry support planned

## Technology Stack

### Core Dependencies

- **clap** (4.5): CLI framework
- **serde** (1.0): Serialization
- **toml** (0.8): TOML parsing
- **tokio** (1.41): Async runtime
- **git2** (0.19): Git operations
- **dialoguer** (0.11): User prompts
- **reqwest** (0.12): HTTP client
- **self_update** (0.41): Self-update mechanism

### Performance Characteristics

- Async I/O for concurrent downloads
- Parallel builds (configurable via `parallel_jobs`)
- Caching to minimize redundant downloads

## Extensibility

### Adding a New Build System

1. Create module in `src/build/`
2. Implement build detection logic
3. Add build command execution
4. Update `detect_existing_build_system()`
5. Add tests

### Adding a New Dependency Source

1. Extend `DependencySource` enum (deps/mod.rs)
2. Implement cloning logic
3. Add constraint validation
4. Update lock file format
5. Add tests

## Next Steps

- [Development Guide](./development.md)
- [Contributing](./contributing.md)
