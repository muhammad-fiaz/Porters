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
- License (MIT, Apache-2.0, GPL-3.0, GPL-2.0, BSD-3-Clause, BSD-2-Clause, MPL-2.0, LGPL-3.0, Unlicense)

**Behavior:**
- Detects existing C/C++ source files
- Auto-detects build system (CMake, XMake, Meson, Make)
- Creates `porters.toml` configuration
- **Automatically generates LICENSE file** based on your selection
- Initializes cache directories

**License File Generation:**

When you select a license, Porters automatically creates a `LICENSE` file with:
- Full license text (SPDX-compliant)
- Your name as the copyright holder
- Current year in copyright notice

**Supported Licenses:**
- **MIT** - Permissive, simple license
- **Apache-2.0** - Permissive with patent grant
- **GPL-3.0** - Strong copyleft
- **GPL-2.0** - Classic copyleft
- **BSD-3-Clause** - Permissive, 3-clause variant
- **BSD-2-Clause** - Permissive, simplified
- **MPL-2.0** - Weak copyleft (Mozilla)
- **LGPL-3.0** - Weak copyleft for libraries
- **Unlicense** - Public domain dedication

**Example:**
```bash
cd my-existing-project
porters init
```

**Example Flow:**
```text
$ porters init
? Project name: my-project
? Version: 0.1.0
? Author: John Doe
? Description: My awesome C++ library
? License:
  > MIT
    Apache-2.0
    GPL-3.0
    BSD-3-Clause

✅ Created porters.toml
✅ Generated LICENSE file (MIT License)
✅ Project initialized!
```

---

## `porters create`

Create a new Porters project with automatic scaffolding.

**Usage:**
```bash
porters create <NAME> [OPTIONS]
```

**Arguments:**
- `<NAME>` - Project name

**Options:**
- `--yes, -y` - Use default values without prompting

**Interactive Prompts:**
- **Project type**: 🚀 Application (executable) or 📦 Library (static/shared)
- **Language**: 🔵 C (Pure C), 🔴 C++ (Pure C++), or 🟣 Both (Hybrid C/C++)
- 📚 Library name (for libraries, optional)
- 👤 Author name (optional)
- 📧 Email (optional)
- 🔗 Repository URL (optional)
- 📝 **License** with descriptions:
  - ⚖️ Apache-2.0 (Permissive, with patent protection)
  - 📄 MIT (Very permissive, simple)
  - 🔓 GPL-3.0 (Copyleft, strong protection)
  - 🔓 GPL-2.0 (Copyleft, older version)
  - 📋 BSD-3-Clause (Permissive, with attribution)
  - 📋 BSD-2-Clause (Permissive, simpler)
  - 🔧 MPL-2.0 (Weak copyleft, file-level)
  - 📚 LGPL-3.0 (For libraries, weak copyleft)
  - 🆓 Unlicense (Public domain)
  - ✏️ Custom (Create your own)
  - ❌ None
- ⚙️ **Build system**:
  - 🔨 CMake (Industry standard, most popular)
  - ⚡ XMake (Modern, fast, Lua-based)
  - 🏗️ Meson (Fast, Python-based)
  - 🔧 Make (Traditional, simple)
  - ✨ Custom (Manual configuration)

**Project Types:**

### Application Projects

Creates an executable application with:
- `src/main.c` or `src/main.cpp` (based on language choice)
- Basic "Hello, World!" starter code
- Build system configuration (CMakeLists.txt, xmake.lua, etc.)
- `porters.toml` with `project-type = "application"`
- **LICENSE file** (auto-generated from your selection)
- README.md with build instructions
- Git repository initialization

**Directory Structure (Application):**
```text
my-app/
├── porters.toml
├── LICENSE             # Auto-generated
├── README.md
├── .gitignore
├── CMakeLists.txt      # or xmake.lua, meson.build, etc.
└── src/
    └── main.cpp        # or main.c
```

**main.cpp Template:**
```cpp
#include <iostream>

int main() {
    std::cout << "Hello from my-app!" << std::endl;
    return 0;
}
```

### Library Projects

Creates a reusable library with complete structure:

**Directory Structure (Library):**
```text
my-lib/
├── porters.toml
├── LICENSE             # Auto-generated
├── README.md
├── .gitignore
├── CMakeLists.txt      # or xmake.lua, meson.build, etc.
├── include/
│   └── my_lib/
│       └── my_lib.hpp  # or my_lib.h
├── src/
│   └── my_lib.cpp      # or my_lib.c
├── examples/
│   └── example.cpp     # or example.c
└── tests/
    └── test_my_lib.cpp # or test_my_lib.c
```

**Library Components:**

**1. Header File (`include/my_lib/my_lib.hpp`):**
```cpp
#ifndef MY_LIB_HPP
#define MY_LIB_HPP

namespace my_lib {
    void greet();
}

#endif // MY_LIB_HPP
```

**2. Implementation (`src/my_lib.cpp`):**
```cpp
#include "my_lib/my_lib.hpp"
#include <iostream>

namespace my_lib {
    void greet() {
        std::cout << "Hello from my_lib library!" << std::endl;
    }
}
```

**3. Example (`examples/example.cpp`):**
```cpp
#include "my_lib/my_lib.hpp"

int main() {
    my_lib::greet();
    return 0;
}
```

**4. Test (`tests/test_my_lib.cpp`):**
```cpp
#include "my_lib/my_lib.hpp"

int main() {
    // Add your tests here
    my_lib::greet();
    return 0;
}
```

**For C Libraries:**

When creating a C library, Porters generates C-style code:

**Header (`include/my_lib/my_lib.h`):**
```c
#ifndef MY_LIB_H
#define MY_LIB_H

#ifdef __cplusplus
extern "C" {
#endif

void my_lib_greet(void);

#ifdef __cplusplus
}
#endif

#endif // MY_LIB_H
```

**Implementation (`src/my_lib.c`):**
```c
#include "my_lib/my_lib.h"
#include <stdio.h>

void my_lib_greet(void) {
    printf("Hello from my_lib library!\n");
}
```

### Hybrid C/C++ Projects (Both Option)

When you select **"🟣 Both (Hybrid C/C++)"**, Porters creates a project that seamlessly integrates C and C++ code with proper `extern "C"` usage.

**Why Use Hybrid Projects?**
- Gradually migrate from C to C++ (or vice versa)
- Use C libraries from C++ code
- Leverage both C's low-level control and C++'s high-level features
- Integrate legacy C code with modern C++

**Directory Structure (Hybrid Application):**
```text
my-hybrid-app/
├── porters.toml
├── LICENSE
├── README.md
├── .gitignore
├── CMakeLists.txt
├── include/
│   ├── c_module.h       # C header with extern "C"
│   └── cpp_utils.hpp    # C++ header
└── src/
    ├── main.cpp         # C++ entry point
    ├── c_module.c       # C implementation
    └── cpp_utils.cpp    # C++ implementation
```

**Generated Files:**

**1. Main Entry Point (`src/main.cpp`):**
```cpp
#include <iostream>
#include "c_module.h"

int main(int argc, char *argv[]) {
    std::cout << "🚀 Hello from C++ (Porters Hybrid Project)!" << std::endl;
    
    // Call C function from C++ code
    const char* c_message = get_c_message();
    std::cout << "📦 Message from C module: " << c_message << std::endl;
    
    return 0;
}
```

**2. C Module Header (`include/c_module.h`):**
```c
#ifndef C_MODULE_H
#define C_MODULE_H

#ifdef __cplusplus
extern "C" {
#endif

/**
 * Get a message from the C module
 * This function can be called from both C and C++ code
 */
const char* get_c_message(void);

/**
 * Process a number using C code
 */
int c_process_number(int value);

#ifdef __cplusplus
}
#endif

#endif /* C_MODULE_H */
```

**3. C Module Implementation (`src/c_module.c`):**
```c
#include "c_module.h"
#include <stdio.h>

const char* get_c_message(void) {
    return "This is a C function callable from C++!";
}

int c_process_number(int value) {
    printf("Processing %d in C code\n", value);
    return value * 2;
}
```

**4. C++ Utilities Header (`include/cpp_utils.hpp`):**
```cpp
#ifndef CPP_UTILS_HPP
#define CPP_UTILS_HPP

#include <string>
#include <vector>

namespace utils {

class StringHelper {
public:
    static std::string to_upper(const std::string& str);
    static std::vector<std::string> split(const std::string& str, char delimiter);
};

} // namespace utils

#endif /* CPP_UTILS_HPP */
```

**5. C++ Utilities Implementation (`src/cpp_utils.cpp`):**
```cpp
#include "cpp_utils.hpp"
#include <algorithm>
#include <sstream>

namespace utils {

std::string StringHelper::to_upper(const std::string& str) {
    std::string result = str;
    std::transform(result.begin(), result.end(), result.begin(), ::toupper);
    return result;
}

std::vector<std::string> StringHelper::split(const std::string& str, char delimiter) {
    std::vector<std::string> tokens;
    std::stringstream ss(str);
    std::string token;
    
    while (std::getline(ss, token, delimiter)) {
        tokens.push_back(token);
    }
    
    return tokens;
}

} // namespace utils
```

**Key Features of Hybrid Projects:**

1. **Automatic `extern "C"` Setup**: C headers properly wrapped for C++ compatibility
2. **Mixed Compilation**: Build system automatically handles both .c and .cpp files
3. **Namespace Separation**: C++ code uses namespaces, C code uses prefixes
4. **Documentation Comments**: Clear examples of cross-language usage
5. **Best Practices**: Follows industry standards for C/C++ interoperability

**Build System Support:**

All build systems (CMake, XMake, Meson) are configured to handle mixed C/C++ compilation:

**CMake Configuration:**
```cmake
# Automatically generated - handles both C and C++ files
project(my-hybrid-app C CXX)

# C files compiled with C compiler
# C++ files compiled with C++ compiler
# Linking works automatically
add_executable(my-hybrid-app
    src/main.cpp
    src/c_module.c
    src/cpp_utils.cpp
)
```

**When to Use Hybrid Projects:**

✅ **Use Hybrid When:**
- Migrating existing C code to C++
- Integrating with C libraries
- Need C's performance + C++'s features
- Team has expertise in both languages
- Gradual modernization of legacy code

❌ **Use Pure C or C++ When:**
- Starting fresh project
- Team specializes in one language
- Project doesn't need cross-language features


**Behavior:**
- Creates project directory
- Generates appropriate source structure based on project type
- Creates build system files (CMake, XMake, Meson, or Make)
- **Automatically generates LICENSE file** with:
  - Full license text
  - Your name as copyright holder
  - Current year
- Creates comprehensive README.md with:
  - Project description
  - Build instructions
  - Usage examples (for libraries)
  - License information
- Initializes Git repository
- Creates `porters.toml` and `.gitignore`

**Examples:**
```bash
# Interactive creation (recommended)
porters create my-app

# Quick create with defaults (application)
porters create my-project --yes

# The wizard will ask:
# 1. Project type → Application or Library
# 2. Language → C, C++, or Both
# 3. License → MIT, Apache-2.0, GPL-3.0, etc.
```

**Example Interactive Flow:**
```text
$ porters create awesome-lib

? Project type:
  Application
  > Library

? Language:
  C
  > C++
  Both

? Library name: awesome_lib
? Author: Jane Developer
? Email: jane@example.com
? Repository URL: https://github.com/jane/awesome-lib
? License:
  > MIT
    Apache-2.0
    GPL-3.0
    BSD-3-Clause

? Build system:
  > CMake
    XMake
    Meson
    Make

✅ Created project: awesome-lib
✅ Generated LICENSE file (MIT License)
✅ Created library structure:
   - include/awesome_lib/awesome_lib.hpp
   - src/awesome_lib.cpp
   - examples/example.cpp
   - tests/test_awesome_lib.cpp
✅ Initialized Git repository
✅ Project ready!

Next steps:
  cd awesome-lib
  porters build
```

**Generated README (Library):**

The README includes:
- Project title and description
- Build instructions
- **Usage examples** showing how to use the library
- Installation guide
- License badge and information
- Contributing guidelines

**Generated README (Application):**

For applications, the README includes:
- Project title and description
- Build and run instructions
- Command-line usage examples
- Configuration information
- License information

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

Synchronize dependencies from `porters.toml` with global cache support.

**Usage:**
```bash
porters sync [OPTIONS]
```

**Options:**
- `--dev` - Include development dependencies
- `--optional` - Include optional dependencies
- `--no-cache` - Disable cache, force re-download all dependencies

**Behavior:**
- Reads `porters.toml`
- **Checks global cache first** (`~/.porters/cache/`)
- Downloads missing dependencies to `ports/`
- **Stores new dependencies in global cache** for future reuse
- Resolves version constraints
- Updates `porters.lock`
- **Supports offline mode** (uses only cached dependencies)

**Examples:**
```bash
# Sync regular dependencies (cache-first)
porters sync

# Sync with dev dependencies
porters sync --dev

# Sync everything
porters sync --dev --optional

# Force re-download (bypass cache)
porters sync --no-cache
```

**Cache Behavior:**
- First checks `~/.porters/cache/<package>/<version>/`
- If found, copies from cache to `ports/`
- If not found, downloads from Git and caches globally
- In offline mode, only uses cached dependencies

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

## `porters registry`

Manage Porters package registry operations.

**Usage:**
```bash
porters registry <SUBCOMMAND>
```

**Subcommands:**

### `porters registry search`

Search for packages in the registry.

**Usage:**
```bash
porters registry search <QUERY> [OPTIONS]
```

**Options:**
- `--tag <TAG>` - Filter by tag (e.g., graphics, networking)
- `--limit <N>` - Limit results (default: 20)

**Examples:**
```bash
# Search for packages
porters registry search json

# Search by tag
porters registry search --tag graphics

# Limit results
porters registry search fmt --limit 5
```

### `porters registry list`

List all available packages in the registry.

**Usage:**
```bash
porters registry list
```

**Example:**
```bash
porters registry list
```

### `porters registry update`

Update the local registry index from remote source.

**Usage:**
```bash
porters registry update
```

**Behavior:**
- Fetches latest package metadata from GitHub
- Updates `~/.porters/registry-index/`
- Uses Git sparse checkout for efficiency
- Respects offline mode setting

**Example:**
```bash
porters registry update
```

**Note:** Registry auto-updates are configurable in `~/.porters/config.toml`:
```toml
[registry]
auto_update = true  # Auto-update when checking for packages
```

---

## `porters clean`

Clean build artifacts and temporary files.

**Usage:**
```bash
porters clean
```

**Behavior:**
- Removes `build/` directory
- Removes `.porters/cache/` directory
- Preserves `ports/` dependencies
- Preserves `porters.lock`

**Example:**
```bash
porters clean
```

---

## `porters clean-cache`

Clean dependency cache (local and/or global).

**Usage:**
```bash
porters clean-cache [OPTIONS]
```

**Options:**
- `--force, -f` - Clean global cache as well (default: local only)

**Behavior:**
- **Without `--force`**: Cleans only `.porters/cache/` (project-local)
- **With `--force`**: Also cleans `~/.porters/cache/` (global cache)

**Examples:**
```bash
# Clean local cache only
porters clean-cache

# Clean both local and global cache
porters clean-cache --force
```

**Warning:** Cleaning global cache will require re-downloading dependencies for all projects.

---

## `porters update`

Update dependencies to latest compatible versions.

**Usage:**
```bash
porters update
```

**Behavior:**
- Checks for newer versions of dependencies
- Respects version constraints in `porters.toml`
- Updates `porters.lock` with new versions
- Downloads updated dependencies

**Example:**
```bash
porters update
```

---

## `porters update-deps`

Update all dependencies to latest versions (ignoring constraints).

**Usage:**
```bash
porters update-deps [OPTIONS]
```

**Options:**
- `--latest` - Update to absolute latest versions (ignore semver constraints)

**Behavior:**
- Updates all dependencies to latest compatible versions
- With `--latest`: Ignores semver constraints in `porters.toml`
- Updates `porters.lock`

**Examples:**
```bash
# Update to latest compatible
porters update-deps

# Update to absolute latest (may break compatibility)
porters update-deps --latest
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
porters execute <FILE> [ARGS...] [OPTIONS]
```

**Arguments:**
- `<FILE>` - C/C++ source file to compile and run
- `[ARGS...]` - Arguments to pass to the compiled program

**Options:**
- `--external` - Open the program in a new external terminal window (instead of current terminal)
- `--no-console` - Run without a console window (useful for GUI applications on Windows)

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

# Execution mode settings
use-external-terminal = false  # Open programs in external terminal (GUI apps)
no-console = false             # Run without console window (Windows GUI apps)
```

**Execution Mode Settings:**

Configure default execution behavior in `porters.toml`:

- **`use-external-terminal`** - Opens program in new terminal window
  - Useful for: GUI applications, interactive programs that need separate window
  - Default: `false` (runs in current terminal)
  - CLI override: `--external` flag

- **`no-console`** - Runs program without console window (Windows only)
  - Useful for: Pure GUI applications (no console output expected)
  - Default: `false` (shows console window)
  - CLI override: `--no-console` flag

**Example Configuration for GUI App:**
```toml
[run]
use-external-terminal = true  # Always open in new window
no-console = true            # No console window needed
```

**Note:** CLI flags (`--external`, `--no-console`) always override config settings.

**Examples:**
```bash
# Execute C file - works immediately, no setup!
porters execute hello.c

# Execute C++ file with arguments
porters execute main.cpp arg1 arg2

# Open in external terminal window (new console window)
porters execute game.cpp --external

# Run GUI app without console window (Windows)
porters execute gui_app.cpp --no-console

# Both flags together for standalone GUI app
porters execute standalone_gui.cpp --external --no-console

# Pass arguments to the program
porters execute calculator.cpp 10 + 20

# Use config defaults (porters.toml [run] section)
# If use-external-terminal = true in config:
porters execute interactive.cpp  # Opens in external terminal automatically

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

## `porters check`

Check compilation of source files without creating executables (syntax-only check).

**Usage:**
```bash
porters check [FILE] [OPTIONS]
```

**Arguments:**
- `[FILE]` - Optional path to a specific source file to check (e.g., `src/main.c`)
  - If omitted, checks all source files in the project

**Options:**
- `--verbose, -v` - Display detailed compiler output including full error traces and warnings

**Behavior:**
- **Fast Compilation Check**: Validates code syntax without generating executables
- **Smart Compiler Detection**: Automatically selects appropriate compiler (GCC, Clang, MSVC)
- **Dependency Aware**: Includes dependency paths from `porters.toml` configuration
- **Multi-Language Support**: Handles both C and C++ files with correct standards
- **Detailed Error Reporting**: Shows compilation errors with color-coded emoji indicators
- **Project or File Mode**: Check entire project or single files

**Compiler Flags Used:**
- **GCC/Clang**: `-fsyntax-only` (skips code generation and linking)
- **MSVC**: `/Zs` (syntax check only)

**Examples:**
```bash
# Check all source files in the project
porters check

# Check a specific file
porters check src/main.c
porters check src/utils.cpp

# Check with verbose compiler output
porters check --verbose
porters check src/main.c --verbose

# Quick syntax validation before committing
porters check && git commit -m "Fixed compilation errors"
```

**Output Example (Success):**
```text
✅ Checking compilation (syntax-only)
🔍 Discovering source files in project...
📦 Found 3 source file(s)

🔨 Checking: src/main.c (C)
✅ PASSED: src/main.c

🔨 Checking: src/utils.c (C)
✅ PASSED: src/utils.c

🔨 Checking: src/math.cpp (C++)
✅ PASSED: src/math.cpp

━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
📊 Compilation Check Summary
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
   Total files checked: 3
   ✅ Passed: 3
   ❌ Failed: 0
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
✅ All compilation checks passed! ✅
```

**Output Example (Error):**
```text
✅ Checking compilation (syntax-only)
🎯 Checking single file: src/main.c

🔨 Checking: src/main.c (C)
❌ FAILED: src/main.c

❌ Compilation errors:
  src/main.c:15:5: error: expected ';' before 'return'
  src/main.c:23:12: error: 'undeclared_var' undeclared
  ... (5 more lines, use --verbose for full output)

━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
📊 Compilation Check Summary
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
   Total files checked: 1
   ✅ Passed: 0
   ❌ Failed: 1
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
❌ Compilation check failed: 1 error(s) found
```

**Use Cases:**
- **Rapid Feedback**: Fast syntax validation during development
- **CI/CD Integration**: Pre-build validation in pipelines
- **Code Review**: Verify changes compile before creating pull requests
- **Debugging**: Identify compilation errors without waiting for full builds
- **Learning**: Students can quickly validate code syntax

**Benefits:**
- ⚡ **Faster than full builds** - No linking or executable generation
- 🎯 **Focused error messages** - Only shows compilation issues
- 🔍 **File-level granularity** - Check individual files or entire projects
- 📊 **Clear summaries** - See pass/fail counts at a glance
- 🛠️ **Tool-agnostic** - Works with any project structure

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

## `porters add-to-path`

Add the Cargo bin directory (`~/.cargo/bin`) to your system PATH environment variable.

**Usage:**
```bash
porters add-to-path
```

**Behavior:**
- **Windows**: Adds `%USERPROFILE%\.cargo\bin` to User PATH via registry
- **Linux/macOS**: Adds `export PATH="$HOME/.cargo/bin:$PATH"` to shell profile (~/.bashrc, ~/.zshrc, etc.)
- Automatically detects your shell and configuration file
- Creates backup before modifying shell profile
- Requires administrator/elevated privileges on Windows

**Platform-Specific Behavior:**

**Windows:**
- Modifies User environment variables in registry
- Requires PowerShell with administrator privileges
- Takes effect in new terminals after restart

**Linux/macOS:**
- Appends to shell configuration file
- Detects: bash, zsh, fish, or sh
- Takes effect after: `source ~/.bashrc` (or restart terminal)

**Example:**
```bash
# Run with appropriate permissions
porters add-to-path
```

**Output:**
```text
✅ Successfully added C:\Users\YourName\.cargo\bin to PATH
ℹ️  Please restart your terminal for changes to take effect
```

---

## `porters remove-from-path`

Remove the Cargo bin directory from your system PATH environment variable.

**Usage:**
```bash
porters remove-from-path [OPTIONS]
```

**Options:**
- `--overwrite` - Completely overwrite PATH with the new value (removes all Cargo bin references)

**Behavior:**
- **Without `--overwrite`**: Removes first occurrence of Cargo bin directory
- **With `--overwrite`**: Removes all occurrences and overwrites the entire PATH variable
- **Windows**: Modifies User PATH via registry
- **Linux/macOS**: Removes export statement from shell profile
- Creates backup before modifications

**Examples:**
```bash
# Remove first occurrence of cargo bin from PATH
porters remove-from-path

# Remove all occurrences and overwrite PATH
porters remove-from-path --overwrite
```

**Output:**
```text
✅ Successfully removed C:\Users\YourName\.cargo\bin from PATH
ℹ️  Please restart your terminal for changes to take effect
```

---

## `porters --check-system`

Check system requirements and display installation status of C/C++ compilers and build tools.

**Usage:**
```bash
porters --check-system
```

**Behavior:**
- Automatically runs on first launch after installation
- Checks for C/C++ compilers: gcc, g++, clang, clang++, MSVC, MinGW
- Checks for build systems: CMake, Make, XMake, Meson, Ninja
- Displays installation instructions if requirements are missing
- Saves check results to global config (`~/.porters/config.toml`)

**Example:**
```bash
porters --check-system
```

**Example Output (Linux):**
```text
╭──────────────────────────────────────────────────╮
│  System Requirements Check                       │
╰──────────────────────────────────────────────────╯

Compilers
─────────
✅ g++ (version 11.4.0)
✅ gcc (version 11.4.0)
❌ clang++ (not found)
❌ clang (not found)

Build Systems
─────────────
✅ cmake (version 3.22.1)
✅ make (version 4.3)
❌ xmake (not found)

Status: ⚠️  Some tools are missing

Installation Instructions:
──────────────────────────

To install missing compilers and tools on Linux:

  sudo apt-get update
  sudo apt-get install clang build-essential cmake

For other distributions, use your package manager:
  - Fedora/RHEL: sudo dnf install clang cmake
  - Arch: sudo pacman -S clang cmake
```

**First Run Behavior:**

When you install Porters and run it for the first time (any command), it will:
1. Automatically run the system check
2. Display found compilers and build tools
3. Show installation instructions if anything is missing
4. Block execution if no C/C++ compiler is found

This ensures you have the necessary tools before using Porters.

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
- `PORTERS_OFFLINE` - Enable offline mode (values: `1`, `true`, `yes`)
- `HOME` (Unix) / `USERPROFILE` (Windows) - User home directory

**Example:**
```bash
export GITHUB_TOKEN=ghp_xxxxxxxxxxxxx
porters publish --version 1.0.0

# Enable offline mode
export PORTERS_OFFLINE=1
porters sync  # Will use only cached dependencies

# Custom cache directory
export PORTERS_CACHE=/mnt/fast-ssd/porters-cache
porters sync
```

**Offline Mode:**

Enable offline mode to prevent all network access:

1. **Via Environment Variable:**
   ```bash
   export PORTERS_OFFLINE=1
   ```

2. **Via Global Config** (`~/.porters/config.toml`):
   ```toml
   offline = true
   ```

3. **Via Project Config** (`porters.toml`):
   ```toml
   offline = true
   ```

When offline mode is enabled:
- All dependencies must be cached
- Registry searches use local cache only
- No Git clones or fetches
- Clear error messages for missing resources

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
