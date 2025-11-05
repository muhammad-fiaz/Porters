# Configuration

Complete reference for Porters configuration files.

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
```

**When You Might Use [run]:**
- Adding project-specific include paths not in dependencies
- Setting custom compiler warnings or optimizations
- Linking additional system libraries
- Using a specific compiler version

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

## Next Steps

- [Command Reference](./commands.md)
- [Dependencies](./dependencies.md)
