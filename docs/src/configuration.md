# Configuration

Complete reference for Porters configuration files.

## Global Configuration

Porters maintains a global configuration file at `~/.porters/config.toml` for user-wide settings and preferences.

**Location:**
- **Windows**: `C:\Users\<YourName>\.porters\config.toml`
- **Linux/macOS**: `~/.porters/config.toml`

**Automatic Creation:**

The global config is automatically created on first run. Porters will:
1. Create the `~/.porters/` directory
2. Generate `config.toml` with default settings
3. Run system requirements check
4. Save detected compilers and build tools

### Global Config Structure

```toml
[porters]
version = "0.1.0"
config_version = "1"

[user]
# Default author information for new projects
name = "Your Name"
email = "you@example.com"
default_license = "MIT"  # Default license for new projects

[cache]
# Global cache directory for compiled executables
dir = "~/.porters/cache"
max_size_mb = 1024
auto_clean = true

[system]
# Last system requirements check
last_check = "2024-01-15T10:30:00Z"

[system.compilers]
# Detected C/C++ compilers (auto-populated)
gcc = "/usr/bin/gcc"
gpp = "/usr/bin/g++"
clang = "/usr/bin/clang"
clangpp = "/usr/bin/clang++"

[system.build_tools]
# Detected build systems (auto-populated)
cmake = "/usr/bin/cmake"
make = "/usr/bin/make"
xmake = "/usr/local/bin/xmake"
meson = "/usr/bin/meson"
ninja = "/usr/bin/ninja"

[preferences]
# User preferences
default_build_system = "cmake"
default_language = "cpp"
auto_add_to_path = true
check_updates = true
```

### Configuration Fields

#### `[porters]` Section
- `version` - Porters version that created this config
- `config_version` - Configuration file format version

#### `[user]` Section
- `name` - Default author name for new projects
- `email` - Default email for new projects
- `default_license` - Default license (MIT, Apache-2.0, GPL-3.0, etc.)

#### `[cache]` Section
- `dir` - Directory for temporary build files and executables
- `max_size_mb` - Maximum cache size in megabytes
- `auto_clean` - Automatically clean old cache files
- `cache_dir` - Custom global cache directory path (optional)

**Example**:
```toml
[cache]
enabled = true
max_size_mb = 2048
auto_clean = true
cache_dir = "~/.porters/cache"  # Optional custom path
```

#### `[registry]` Section
- `url` - Registry repository URL (default: https://github.com/muhammad-fiaz/porters)
- `auto_update` - Automatically update registry index
- `index_path` - Local registry index directory
- `last_update` - Timestamp of last registry update

**Example**:
```toml
[registry]
url = "https://github.com/muhammad-fiaz/porters"
auto_update = true
index_path = "~/.porters/registry-index"
last_update = "2024-01-15T10:30:00Z"
```

#### Global Offline Mode

Enable offline mode globally to prevent all network operations:

```toml
# In ~/.porters/config.toml
offline = true
```

When offline mode is enabled:
- ✅ Uses only cached dependencies
- ✅ Uses local registry index
- ❌ No network requests
- ❌ Cannot download new packages

**Use case**: Working in environments without internet access or wanting to ensure reproducible builds.

#### `[system]` Section
- `last_check` - Timestamp of last system requirements check
- `compilers` - Detected compiler paths (auto-populated)
- `build_tools` - Detected build system paths (auto-populated)

#### `[preferences]` Section
- `default_build_system` - Preferred build system (cmake, xmake, meson, make)
- `default_language` - Preferred language (c, cpp, both)
- `auto_add_to_path` - Automatically add Cargo bin to PATH on first run
- `check_updates` - Check for Porters updates automatically

### System Requirements Check

On first run (or when running `porters --check-system`), Porters will:

1. **Check for C/C++ Compilers:**
   - gcc, g++
   - clang, clang++
   - MSVC (Windows)
   - MinGW (Windows)

2. **Check for Build Systems:**
   - CMake
   - Make
   - XMake
   - Meson
   - Ninja

3. **Display Results:**
   - ✅ Found tools with version numbers
   - ❌ Missing tools with installation instructions

4. **Save to Config:**
   - Detected tool paths saved to `~/.porters/config.toml`
   - Used for faster detection in future runs

**Example Output:**
```text
╭──────────────────────────────────────────────────╮
│  System Requirements Check                       │
╰──────────────────────────────────────────────────╯

Compilers
─────────
✅ g++ (version 11.4.0)
✅ gcc (version 11.4.0)
❌ clang++ (not found)

Build Systems
─────────────
✅ cmake (version 3.22.1)
✅ make (version 4.3)

Status: ✅ System ready!
```

**Installation Instructions:**

If tools are missing, Porters displays platform-specific installation commands:

**Windows:**
```text
Install C/C++ compiler:
  - MSVC: Install Visual Studio Build Tools
  - MinGW: choco install mingw
  - Clang: choco install llvm

Install build tools:
  - CMake: choco install cmake
```

**Linux (Debian/Ubuntu):**
```text
sudo apt-get update
sudo apt-get install gcc g++ clang cmake make
```

**macOS:**
```text
xcode-select --install
brew install cmake
```

### Manual Configuration

You can manually edit `~/.porters/config.toml` to customize settings:

```toml
[user]
name = "Jane Developer"
email = "jane@example.com"
default_license = "Apache-2.0"

[preferences]
default_build_system = "xmake"
default_language = "cpp"
check_updates = true

[cache]
max_size_mb = 2048
auto_clean = true
```

**After editing**, run any Porters command to apply changes.

---

## porters.toml

Main project configuration file.

### Complete Example

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
offline = false  # Enable offline mode for this project

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
```

### Project Section

Required fields:
- `name` - Project name
- `version` - Semantic version (e.g., "1.0.0")

Optional fields:
- `description` - Project description
- `license` - SPDX license identifier
- `authors` - List of authors
- `repository` - Git repository URL
- `project-type` - "application" or "library"
- `entry_point` - Main entry point
- `platforms` - Target platforms
- `keywords` - Search keywords
- `readme` - README file path

### Dependencies Section

Dependency sources:

**Git (HTTPS):**
```toml
fmt = { git = "https://github.com/fmtlib/fmt" }
```

**Git (SSH):**
```toml
spdlog = { git = "git@github.com:gabime/spdlog.git" }
```

**With version constraints:**
```toml
fmt = { git = "https://github.com/fmtlib/fmt", tag = "10.1.1" }
boost = { git = "https://github.com/boostorg/boost", branch = "boost-1.80.0" }
```

**Local path:**
```toml
mylib = { path = "../mylib" }
```

### Build Section

```toml
[build]
system = "cmake"  # cmake, xmake, meson, make, custom
options = ["-DCMAKE_BUILD_TYPE=Release"]

[build.env]
CMAKE_PREFIX_PATH = "/usr/local"
```

### Run Section - Direct File Execution (COMPLETELY OPTIONAL)

**⚠️ YOU DON'T NEED THIS SECTION!**

The `porters execute` command works **100% automatically** without any configuration. This section only exists for advanced manual overrides.

**What Works Automatically (No [run] Section Needed):**
- ✅ **Compiler Detection** - Finds gcc/clang/g++/clang++ in your PATH
- ✅ **Dependency Resolution** - Reads dependencies from `[dependencies]` 
- ✅ **Include Paths** - Automatically adds `-I` for all dependency includes
- ✅ **Library Paths** - Automatically adds `-L` for all dependency libraries
- ✅ **File Type Detection** - `.c` → C compiler, `.cpp` → C++ compiler
- ✅ **Supported Extensions** - `.c`, `.cpp`, `.cxx`, `.cc`, `.c++`, `.cp`, `.C`, `.CPP`

**Zero Configuration Example:**
```bash
# No porters.toml needed at all!
porters execute hello.c

# With dependencies - automatically resolved
porters execute my_app.cpp
```

**Optional Manual Overrides (Only If You Need Custom Behavior):**

```toml
[run]
# Extra include directories (beyond automatic dependency includes)
include-dirs = ["./include", "./extra/include"]

# Exclude patterns (rarely needed)
exclude-patterns = ["test_*", "*_backup.c"]

# Compiler flags (only if you want warnings, optimizations, etc.)
compiler-flags = ["-Wall", "-O2", "-std=c17"]

# Linker flags (only if you need extra libraries)
linker-flags = ["-lm", "-lpthread"]

# Override compiler (only if you need a specific one)
c-compiler = "clang"    # Default: auto-detect
cpp-compiler = "clang++"  # Default: auto-detect

# Execution mode settings (default: false for both)
use-external-terminal = false  # Open programs in new external terminal window
no-console = false             # Run without console window (Windows GUI apps)
```

**Execution Mode Settings:**

Configure how `porters execute` runs your programs:

- **`use-external-terminal`** (default: `false`)
  - When `true`: Opens program in new external terminal window
  - Useful for: GUI applications, interactive programs needing separate window
  - CLI override: `--external` flag
  - Example use cases:
    - Interactive TUI applications that manage terminal state
    - Games that need full terminal control
    - Programs that should run independently of IDE/editor

- **`no-console`** (default: `false`)
  - When `true`: Runs program without console window (Windows only)
  - Useful for: Pure GUI applications with no console output
  - CLI override: `--no-console` flag
  - Example use cases:
    - Windows GUI applications using Win32 API, SDL, SFML
    - Applications where console window would be distracting
    - Release builds of GUI apps

**Example Configurations:**

```toml
# For GUI game development
[run]
use-external-terminal = true  # Run in separate window
no-console = true            # No console clutter
linker-flags = ["-lSDL2"]    # Link SDL library

# For interactive terminal application
[run]
use-external-terminal = true  # Needs dedicated terminal
compiler-flags = ["-Wall", "-Wextra"]

# For standard CLI tool (defaults are fine, no [run] needed)
# Just use: porters execute main.c
```

**Note:** CLI flags (`--external`, `--no-console`) always override config settings.

**When You Might Use [run]:**
- Adding project-specific include paths not in dependencies
- Setting custom compiler warnings or optimizations
- Linking additional system libraries
- Using a specific compiler version
- Configuring default execution mode for GUI apps
- Setting up consistent behavior for team development

### Tool Version Requirements

Specify minimum versions for build tools and compilers. Porters will validate these before building.

**Supported version operators:**
- `*` - Any version
- `>=1.2.3` - Greater than or equal
- `<=1.2.3` - Less than or equal
- `>1.2.3` - Greater than
- `<1.2.3` - Less than
- `^1.2.3` - Compatible (allows patch and minor updates, ~= caret in Cargo)
- `~1.2.3` - Tilde (allows patch updates only)
- `1.2.3` or `==1.2.3` - Exact version

**Example:**
```toml
[requires]
c = ">=11"              # C11 or later
cpp = ">=17"            # C++17 or later
cmake = ">=3.20"        # CMake 3.20+
gcc = ">=9.0"           # GCC 9.0+
clang = "^12.0"         # Clang 12.x (allows 12.1, 12.2, but not 13.0)
ninja = "~1.10.0"       # Ninja 1.10.x (allows 1.10.1, but not 1.11)
make = "*"              # Any Make version
meson = ">=0.60"        # Meson 0.60+
bazel = ">=5.0"         # Bazel 5.0+
conan = ">=1.50"        # Conan 1.50+
vcpkg = "*"             # Any vcpkg version
xmake = ">=2.7"         # XMake 2.7+
msvc = ">=19.30"        # MSVC 19.30+ (Visual Studio 2022)
```

**Version check output:**
```bash
$ porters build
✓ Checking tool version requirements...
  ✓ C++ Compiler: requires >=17, found 20
  ✓ CMake: requires >=3.20, found 3.25.1
  ✓ GCC: requires >=9.0, found 11.3.0
✓ All tool requirements satisfied ✓
```

**If requirements not met:**
```bash
$ porters build
✓ Checking tool version requirements...

🤔 Oops! Unsatisfied version requirements:
  ❌ C++: requires >=17, found 14
  ❌ CMake: requires >=3.20, found 3.15.0
  ❌ GCC: requires >=9.0, but tool not found in PATH

Please install or upgrade the required tools to continue.
See: https://github.com/muhammad-fiaz/porters#requirements
```

### Extension Auto-Install

Automatically install extensions from crates.io when running `porters sync`.

**Example:**
```toml
extensions = [
    "porters-format",      # Code formatter
    "porters-lint",        # Linter
    "porters-doc",         # Documentation generator
    "porters-test-runner"  # Test runner
]
```

Extensions are always installed in the `dev` category and can be published to crates.io by anyone.

**During sync:**
```bash
$ porters sync
✓ Syncing dependencies from porters.toml
ℹ Auto-installing 3 extensions from config...
📦 Installing extension 'porters-format'...
✓ Extension 'porters-format' installed successfully! 🔌
📦 Extension 'porters-lint' already installed, skipping
📦 Installing extension 'porters-doc'...
✓ Extension 'porters-doc' installed successfully! 🔌
```

### Custom Commands

Define custom CLI commands in your porters.toml for project-specific tasks.

**Example:**
```toml
[[commands]]
name = "format"
description = "Format all C++ source files"
script = "clang-format -i src/**/*.cpp include/**/*.hpp"

[[commands]]
name = "lint"
description = "Run clang-tidy linter"
script = "clang-tidy src/*.cpp -- -Iinclude"

[[commands]]
name = "docs"
description = "Generate Doxygen documentation"
script = "doxygen Doxyfile"

[commands.env]
DOXYGEN_OUTPUT = "docs/html"
OUTPUT_DIR = "build/docs"
```

**Usage:**
```bash
$ porters format       # Runs clang-format
$ porters lint         # Runs clang-tidy
$ porters docs         # Generates documentation
```

### Named Scripts

Quick script shortcuts for common tasks.

**Example:**
```toml
[scripts]
test-all = "cargo build && cargo test --all"
deploy-prod = "./scripts/deploy.sh production"
clean-all = "rm -rf build ports .porters-cache"
bench = "cargo build --release && ./build/benchmark"
```

**Usage:**
```bash
$ porters run-script test-all
$ porters run-script deploy-prod
$ porters run-script bench
```

### Dependency Checksums

Verify dependency integrity with SHA-256 checksums (automatically managed by lockfile).

**Example:**
```toml
[dependencies]
fmt = { 
    git = "https://github.com/fmtlib/fmt", 
    tag = "10.1.1",
    checksum = "sha256:a1b2c3d4e5f6..."  # Optional, auto-calculated if omitted
}
```

Porters automatically:
1. Calculates checksums on first download
2. Stores checksums in `porters.lock`
3. Verifies checksums on subsequent builds

**Checksum mismatch:**
```bash
⚠️ Dependency hash mismatch for 'fmt':
   Expected: abc123...
   Found:    def456...

This may indicate tampering or corruption. Please verify the source.
```

## Global Configuration

Location:
- **Linux/macOS**: `~/.porters/config.toml`
- **Windows**: `C:\Users\<username>\.porters\config.toml`

### Example

```toml
[settings]
parallel_jobs = 8
cache_enabled = true

[packages.fmt]
name = "fmt"
version = "10.1.1"
source = "https://github.com/fmtlib/fmt"
install_path = "/home/user/.porters/packages/fmt"
installed_at = "2024-01-15T10:30:00Z"
```

## Lock File (porters.lock)

Auto-generated file ensuring reproducible builds.

### Format

```toml
version = "1"
updated_at = "2024-01-15T10:30:00Z"

[dependencies.fmt]
name = "fmt"
version = "10.1.1"
source = { Git = { url = "https://github.com/fmtlib/fmt", rev = "a1b2c3d" } }
checksum = "sha256:..."
dependencies = []

[dependencies.spdlog]
name = "spdlog"
version = "1.12.0"
source = { Git = { url = "https://github.com/gabime/spdlog", rev = "e4f5678" } }
dependencies = ["fmt"]
```

**Do not edit manually.** Use `porters lock` to regenerate.

---

## Transitive Dependencies

Porters automatically resolves dependencies-of-dependencies recursively.

**How it works:**
1. When you add a dependency, Porters checks if it has its own `porters.toml`
2. If found, Porters reads the dependency's dependencies
3. This process repeats recursively to build a full dependency graph
4. Circular dependencies are detected and reported as errors
5. Version conflicts are detected and must be manually resolved

**Example:**

Your project's `porters.toml`:
```toml
[dependencies]
mylib = { path = "../mylib" }
```

`mylib`'s `porters.toml` (transitive dependencies):
```toml
[dependencies]
fmt = { git = "https://github.com/fmtlib/fmt", tag = "10.1.1" }
spdlog = { git = "https://github.com/gabime/spdlog" }
```

When you run `porters sync`, Porters will:
- Install `mylib` (your direct dependency)
- Detect `mylib` has dependencies on `fmt` and `spdlog`
- Automatically install `fmt` and `spdlog` (transitive dependencies)
- Build them in the correct order: `fmt` → `spdlog` → `mylib` → `your-project`

**Dependency graph visualization:**
```bash
porters graph
# Output:
📦 Resolved dependency build order: fmt → spdlog → mylib
```

**Circular dependency detection:**
```bash
❌ Circular dependency detected involving: mylib

This means there's a circular dependency chain. Check your dependency tree.
```

**Version conflict detection:**
```bash
❌ Dependency conflicts detected:
  fmt requires versions: 10.1.1, 9.0.0

Two dependencies require different versions of 'fmt'. You must manually resolve this conflict.
```

---

## Offline Mode

Porters supports working entirely offline using only cached dependencies and local registry index. This is useful for:

- 🔒 Secure environments without internet access
- ✈️ Air-gapped networks
- 🏗️ Reproducible builds
- 📦 CI/CD pipelines with pre-populated cache

### Enabling Offline Mode

**Global offline mode** (affects all projects):

```toml
# ~/.porters/config.toml
offline = true
```

**Project offline mode** (project-specific):

```toml
# porters.toml
[project]
offline = true
```

**Temporary offline mode** (command-line):

```bash
porters build --offline
porters add <dep> --offline
porters sync --offline
```

### How Offline Mode Works

When offline mode is enabled:

1. **Network Operations Blocked**:
   - ❌ No Git clones/fetches
   - ❌ No registry updates from GitHub
   - ❌ No package downloads

2. **Cache-First Resolution**:
   - ✅ Uses global cache (`~/.porters/cache/`)
   - ✅ Uses local cache (`.porters/cache/`)
   - ✅ Uses local registry index (`~/.porters/registry-index/`)

3. **Error Handling**:
   - If dependency not in cache → clear error message
   - Suggests running without `--offline` to download

### Preparing for Offline Mode

Before going offline, ensure all dependencies are cached:

```bash
# 1. Sync all dependencies (downloads if needed)
porters sync

# 2. Verify cache contents
porters cache list

# 3. Enable offline mode
porters config set offline true

# 4. Test that build works offline
porters build --offline
```

### Offline Mode Example Workflow

**Setup (with internet)**:
```bash
# Initialize project
porters init

# Add dependencies
porters add https://github.com/fmtlib/fmt
porters add https://github.com/gabime/spdlog

# Sync (downloads and caches)
porters sync

# Verify cache
porters cache stats
# Output: Packages: 2, Total Size: 15.2 MB
```

**Work offline (no internet)**:
```bash
# Enable offline mode
export PORTERS_OFFLINE=1
# or add offline=true to porters.toml

# Build works entirely from cache
porters build --offline
# ✅ Using globally cached fmt
# ✅ Using globally cached spdlog
# 🔨 Building project...
```

### Offline Mode with Registry

The registry can be used offline with a local index:

```bash
# 1. Update registry index (with internet)
porters registry update

# 2. Work offline
porters search fmt --offline
# Searches local registry index

porters info spdlog --offline
# Shows info from local index
```

### Troubleshooting Offline Mode

**"Package not found in cache"**:
```bash
# Temporarily disable offline to download
porters sync
# Then re-enable offline
```

**"Registry index not found"**:
```bash
# Update registry index first
porters registry update
# Then work offline
```

**Check what's cached**:
```bash
# List all cached packages
porters cache list

# Show cache statistics
porters cache stats
```

### Best Practices

1. **Pre-populate cache before going offline**:
   ```bash
   porters sync
   porters cache verify
   ```

2. **Use lock file for reproducibility**:
   ```bash
   porters lock
   # Commit porters.lock to version control
   ```

3. **Periodic cache updates**:
   ```bash
   # Weekly: update registry and dependencies
   porters registry update
   porters update
   ```

4. **CI/CD offline builds**:
   ```yaml
   # .github/workflows/build.yml
   - name: Restore cache
     uses: actions/cache@v3
     with:
       path: ~/.porters/cache
       key: ${{ runner.os }}-porters-${{ hashFiles('**/porters.lock') }}
   
   - name: Build offline
     run: porters build --offline
   ```

---

## Next Steps

- [Caching](./caching.md)
- [Command Reference](./commands.md)
- [Dependencies](./dependencies.md)
