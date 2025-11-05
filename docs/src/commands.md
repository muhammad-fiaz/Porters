# Command Reference

Complete reference for all Porters commands.

## `porters init`

Initialize a Porters project in the current directory.

**Usage:**
```bash
porters init [OPTIONS]
```

**Options:**
- `--yes, -y` - Use default values without prompting

**Interactive Prompts:**
- Project name (defaults to folder name)
- Version (default: 0.1.0)
- Author name
- Description
- License (Apache-2.0, MIT, GPL, etc.)

**Behavior:**
- Detects existing C/C++ source files
- Auto-detects build system (CMake, XMake, Meson, Make)
- Creates `porters.toml` configuration
- Initializes cache directories

**Example:**
```bash
cd my-existing-project
porters init
```

---

## `porters create`

Create a new Porters project.

**Usage:**
```bash
porters create <NAME> [OPTIONS]
```

**Arguments:**
- `<NAME>` - Project name

**Options:**
- `--yes, -y` - Use default values without prompting

**Interactive Prompts:**
- Project type (Application or Library)
- Language (C, C++, or Both)
- Library name (for libraries)
- Author name
- Email
- Repository URL
- License
- Build system

**Behavior:**
- Creates project directory
- Generates source structure (`src/`, `include/`)
- Creates build system files
- Initializes Git repository
- Creates `porters.toml` and README

**Example:**
```bash
porters create my-app
porters create my-lib --yes  # Use defaults
```

---

## `porters add`

Add a dependency to the current project.

**Usage:**
```bash
porters add <PACKAGE> [OPTIONS]
```

**Arguments:**
- `<PACKAGE>` - Package name

**Options:**
- `--git <URL>` - Git repository URL (HTTPS or SSH)
- `--branch <BRANCH>` - Specific Git branch
- `--tag <TAG>` - Specific Git tag
- `--dev` - Add as development dependency
- `--optional` - Add as optional dependency

**Behavior:**
- Adds dependency to `porters.toml`
- Clones package to `ports/<package>/`
- Updates `porters.lock`

**Examples:**
```bash
# Add from Git (HTTPS)
porters add fmt --git https://github.com/fmtlib/fmt

# Add from Git (SSH)
porters add spdlog --git git@github.com:gabime/spdlog.git

# Add specific tag
porters add fmt --git https://github.com/fmtlib/fmt --tag 10.1.1

# Add as dev dependency
porters add catch2 --git https://github.com/catchorg/Catch2 --dev

# Add optional dependency
porters add zlib --git https://github.com/madler/zlib --optional
```

---

## `porters install`

Install a package globally to `~/.porters/packages/`.

**Usage:**
```bash
porters install <PACKAGE> [OPTIONS]
```

**Arguments:**
- `<PACKAGE>` - Package name

**Options:**
- `--git <URL>` - Git repository URL
- `--branch <BRANCH>` - Specific Git branch
- `--tag <TAG>` - Specific Git tag

**Behavior:**
- Creates `~/.porters/` directory structure
- Clones package to `~/.porters/packages/<package>/`
- Updates global config (`~/.porters/config.toml`)

**Examples:**
```bash
porters install fmt --git https://github.com/fmtlib/fmt
porters install boost --git https://github.com/boostorg/boost --tag boost-1.80.0
```

---

## `porters sync`

Synchronize dependencies from `porters.toml`.

**Usage:**
```bash
porters sync [OPTIONS]
```

**Options:**
- `--dev` - Include development dependencies
- `--optional` - Include optional dependencies

**Behavior:**
- Reads `porters.toml`
- Downloads missing dependencies to `ports/`
- Resolves version constraints
- Updates `porters.lock`

**Examples:**
```bash
# Sync regular dependencies
porters sync

# Sync with dev dependencies
porters sync --dev

# Sync everything
porters sync --dev --optional
```

---

## `porters lock`

Update the lock file with installed dependencies.

**Usage:**
```bash
porters lock
```

**Behavior:**
- Scans `ports/` directory
- Resolves current dependency versions
- Updates `porters.lock` with checksums and metadata
- Records timestamp

**Example:**
```bash
porters lock
```

---

## `porters run`

Run the compiled project executable.

**Usage:**
```bash
porters run [ARGS...]
```

**Arguments:**
- `[ARGS...]` - Arguments to pass to the executable

**Behavior:**
- Locates the compiled executable from build directory
- Executes the program with provided arguments
- Displays program output

**Examples:**
```bash
# Run without arguments
porters run

# Run with arguments
porters run --verbose input.txt

# After building
porters build
porters run
```

---

## `porters execute`

Execute a single C/C++ source file directly with **zero configuration required**.

**Usage:**
```bash
porters execute <FILE> [ARGS...]
```

**Arguments:**
- `<FILE>` - C/C++ source file to compile and run
- `[ARGS...]` - Arguments to pass to the compiled program

**Supported File Extensions:**
- **C**: `.c`
- **C++**: `.cpp`, `.cxx`, `.cc`, `.c++`, `.cp`, `.C`, `.CPP`

**Note**: Header files (`.h`, `.hpp`, `.hxx`) cannot be compiled directly.

**100% Automatic - No Configuration Needed:**
- ✅ **Compiler Auto-Detection** - Finds gcc/clang (C) or g++/clang++ (C++)
- ✅ **Dependency Resolution** - Reads `porters.toml` and adds include/lib paths automatically
- ✅ **File Type Detection** - Determines C vs C++ from file extension
- ✅ **Smart Compilation** - Compiles to temporary executable and runs immediately
- ✅ **Works Without porters.toml** - Can execute any C/C++ file even outside a project

**[run] Section - Optional Manual Overrides:**

**You don't need this section!** Everything works automatically. Only add if you need custom configuration.

```toml
[run]
# Additional include directories (dependencies auto-included)
include-dirs = ["./include", "./extra/include"]

# Exclude patterns for automatic includes
exclude-patterns = ["test_*", "*_backup.c"]

# Compiler flags (optional - compiles without these)
compiler-flags = ["-Wall", "-O2", "-std=c17"]

# Linker flags (optional)
linker-flags = ["-lm", "-lpthread"]

# Override default compilers (auto-detected by default)
c-compiler = "gcc"      # or "clang", "cc"
cpp-compiler = "g++"    # or "clang++", "c++"
```

**Examples:**
```bash
# Execute C file - works immediately, no setup!
porters execute hello.c

# Execute C++ file with arguments
porters execute main.cpp arg1 arg2

# With dependencies (automatic)
# Porters reads porters.toml and adds all dependency includes/libs
porters execute my_program.c

# Debug build (with compiler flags in porters.toml)
porters execute --release my_app.cpp
```

**How It Works:**

1. **Detect File Type**: `.c` → C compiler, `.cpp/.cxx/.cc` → C++ compiler
2. **Find Compiler**: Searches for gcc/clang/g++/clang++ in PATH
3. **Resolve Dependencies**: Reads `porters.toml` [dependencies] section
4. **Add Include Paths**: Automatically adds `-I` flags for all dependencies
5. **Add Library Paths**: Automatically adds `-L` flags for dependency libs
6. **Compile**: Compiles to temporary executable in `.porters/cache/`
7. **Execute**: Runs compiled program with provided arguments

**Error Handling:**

If compilation fails, Porters displays the compiler output with errors highlighted. Common issues:

- Missing compiler: Install gcc/g++ or clang/clang++
- Dependency not found: Run `porters sync` to download dependencies
- Compilation errors: Check source code and compiler output

---

## `porters build`

Build the current project.

**Usage:**
```bash
porters build [OPTIONS]
```

**Options:**
- `--release, -r` - Build in release mode (optimized)

**Behavior:**
- Detects build system from `porters.toml` or auto-detects
- Resolves dependencies
- Runs build commands (e.g., `cmake`, `xmake`, `meson`)
- Compiles source files

**Examples:**
```bash
# Debug build
porters build

# Release build
porters build --release
```

---

## `porters publish`

Publish project to GitHub releases.

**Usage:**
```bash
porters publish [OPTIONS]
```

**Options:**
- `--version, -v <VERSION>` - Version to publish (e.g., 1.0.0)
- `--token <TOKEN>` - GitHub personal access token

**Behavior:**
- Reads GitHub repository from `porters.toml`
- Creates Git tag
- Builds release binaries
- Creates GitHub release
- Uploads artifacts

**Example:**
```bash
porters publish --version 1.0.0
```

**Prerequisites:**
- GitHub repository configured in `porters.toml`
- GitHub token (via `--token` or `GITHUB_TOKEN` env variable)

---

## `porters self-update`

Update Porters to the latest version.

**Usage:**
```bash
porters self-update
```

**Behavior:**
- Fetches latest release from GitHub
- Downloads platform-specific binary
- Replaces current executable

**Example:**
```bash
porters self-update
```

---

## `porters run-script`

Run a named script defined in `porters.toml`.

**Usage:**
```bash
porters run-script <NAME>
```

**Arguments:**
- `<NAME>` - Script name defined in `[scripts]` section

**Behavior:**
- Looks up script in `porters.toml` `[scripts]` section
- Executes the script command in a shell
- Shows available scripts if name not found
- Cross-platform: Uses `cmd.exe` on Windows, `sh` on Unix

**Configuration:**
Define scripts in `porters.toml`:
```toml
[scripts]
format = "clang-format -i src/**/*.{c,cpp,h,hpp}"
lint = "cppcheck --enable=all src/"
generate = "python scripts/codegen.py"
```

**Examples:**
```bash
# Run the 'format' script
porters run-script format

# Run the 'lint' script
porters run-script lint

# If script doesn't exist, shows available scripts
porters run-script unknown
```

---

## Custom Commands

Execute custom commands defined in `porters.toml`.

**Usage:**
```bash
porters <command> [args...]
```

**Behavior:**
- Searches for custom commands in `[[commands]]` array
- Executes the matching command's script
- Sets environment variables from command config
- Falls back to showing available commands if not found

**Configuration:**
Define custom commands in `porters.toml`:
```toml
[[commands]]
name = "docs"
script = "doxygen Doxyfile"

[commands.env]
DOXYGEN_OUTPUT = "docs/html"

[[commands]]
name = "benchmark"
script = "cmake --build build --target benchmark && ./build/benchmark"

[commands.env]
BENCHMARK_ITERATIONS = "1000"
```

**Examples:**
```bash
# Run custom 'docs' command
porters docs

# Run custom 'benchmark' command  
porters benchmark

# Custom commands are part of the main CLI
porters --help  # Shows all commands including custom ones
```

---

## `porters extension`

Manage Porters extensions.

### `porters extension install`

Install an extension.

**Usage:**
```bash
porters extension install <NAME> [OPTIONS]
```

**Arguments:**
- `<NAME>` - Extension name

**Options:**
- `--git <URL>` - Install from git repository
- `--path <PATH>` - Install from local path

**Examples:**
```bash
# Install from crates.io
porters extension install porters-format

# Install from GitHub
porters extension install my-ext --git https://github.com/user/porters-ext-myext

# Install from local directory
porters extension install my-ext --path ./my-extension
```

### `porters extension uninstall`

Uninstall an extension.

**Usage:**
```bash
porters extension uninstall <NAME>
```

**Example:**
```bash
porters extension uninstall porters-format
```

### `porters extension list`

List installed extensions.

**Usage:**
```bash
porters extension list
```

**Output:**
```text
📦 Installed Extensions:

  porters-format v1.0.0
    Code formatting extension for C/C++
    Repository: https://github.com/user/porters-format

  porters-docs v0.5.0
    Generate documentation with Doxygen
```

### `porters extension create`

Create a new extension template.

**Usage:**
```bash
porters extension create <NAME>
```

**Example:**
```bash
porters extension create my-awesome-extension
```

Creates:
```text
my-awesome-extension/
├── extension.toml
├── README.md
└── hooks/
    └── example.sh
```

---

## Global Options

Available for all commands:

- `--help, -h` - Show help message
- `--version, -V` - Show version information

**Examples:**
```bash
porters --version
porters build --help
```

---

## Environment Variables

Porters respects these environment variables:

- `GITHUB_TOKEN` - GitHub personal access token for publishing
- `PORTERS_CONFIG` - Path to global config (default: `~/.porters/config.toml`)
- `PORTERS_CACHE` - Path to global cache (default: `~/.porters/cache/`)

**Example:**
```bash
export GITHUB_TOKEN=ghp_xxxxxxxxxxxxx
porters publish --version 1.0.0
```

---

## Exit Codes

- `0` - Success
- `1` - Error (build failed, dependency not found, etc.)
- `2` - Invalid arguments

---

## Next Steps

- Review [Configuration](./configuration.md)
- Read [Troubleshooting](./troubleshooting.md)
- Explore [Getting Started](./getting-started.md)
