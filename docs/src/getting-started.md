# Getting Started

This guide will walk you through creating and managing your first C/C++ project with Porters.

## First Run: System Requirements Check

When you first install Porters and run any command, it will automatically check your system for:

- **C/C++ Compilers**: gcc, g++, clang, clang++, MSVC (Windows), MinGW (Windows)
- **Build Systems**: CMake, Make, XMake, Meson, Ninja

**Example First Run:**
```bash
$ porters --version

╭──────────────────────────────────────────────────╮
│  System Requirements Check                       │
╰──────────────────────────────────────────────────╯

Compilers
─────────
✅ g++ (version 11.4.0)
✅ gcc (version 11.4.0)

Build Systems
─────────────
✅ cmake (version 3.22.1)
✅ make (version 4.3)

Status: ✅ System ready!

Porters version 0.1.0
```

If any required tools are missing, Porters will display installation instructions for your platform.

**Manual System Check:**

You can run the system check anytime:
```bash
porters --check-system
```

## Creating a New Project

The easiest way to start is creating a new project:

```bash
porters create my-awesome-project
```

This launches an interactive wizard that asks:

1. **Project Type**: Application or Library
2. **Language**: C, C++, or Both
3. **Library Name**: (If creating a library)
4. **Author**: Your name (saved to global config for future use)
5. **Email**: Your email (saved to global config)
6. **Repository URL**: Git repository URL (optional)
7. **License**: Choose from MIT, Apache-2.0, GPL-3.0, BSD, MPL-2.0, LGPL-3.0, Unlicense
8. **Build System**: CMake, XMake, Meson, Make, or Custom

### What Gets Created

**Application Project:**
- `src/main.cpp` or `src/main.c` with "Hello, World!" starter code
- Build system configuration (CMakeLists.txt, xmake.lua, etc.)
- **LICENSE file** auto-generated with your name and current year
- README.md with build instructions
- `porters.toml` configuration
- Git repository initialization

**Library Project:**
- Complete library structure:
  - `include/<libname>/<libname>.hpp` - Public header with namespace/API
  - `src/<libname>.cpp` - Implementation
  - `examples/example.cpp` - Usage example
  - `tests/test_<libname>.cpp` - Test skeleton
- Build system configuration with library target
- **LICENSE file** auto-generated
- README.md with library usage examples
- `porters.toml` with `project-type = "library"`

### License File Generation

When you select a license, Porters automatically generates a complete LICENSE file containing:
- Full SPDX-compliant license text
- Your name as the copyright holder
- Current year in copyright notice

**Supported Licenses:**
- MIT - Simple and permissive
- Apache-2.0 - Permissive with patent grant
- GPL-3.0 / GPL-2.0 - Strong copyleft
- BSD-3-Clause / BSD-2-Clause - Permissive BSD variants
- MPL-2.0 - Weak copyleft (Mozilla Public License)
- LGPL-3.0 - Weak copyleft for libraries
- Unlicense - Public domain dedication

### Quick Create with Defaults

Skip the questions and use defaults:

```bash
porters create my-project --yes
```

This creates a C/C++ application with:
- CMake as the build system
- Apache-2.0 license
- Basic project structure

### Hybrid C/C++ Projects (Both Option)

When asked for language, you can choose **"🟣 Both (Hybrid C/C++ with extern \"C\")"** to create a project that seamlessly combines C and C++ code.

**Example Creation:**
```bash
porters create hybrid-project
# Select: 🟣 Both (Hybrid C/C++ with extern "C")
```

**Generated Project Structure:**
```
hybrid-project/
├── porters.toml
├── CMakeLists.txt
├── src/
│   ├── main.cpp        # C++ entry point
│   ├── c_module.c      # C implementation
│   ├── cpp_utils.cpp   # C++ utilities
├── include/
│   ├── c_module.h      # C header with extern "C"
│   └── cpp_utils.hpp   # C++ header
└── README.md
```

**Why Use Hybrid Projects?**

1. **Gradual Migration**: Migrate legacy C code to C++ incrementally
2. **Best of Both Worlds**: Use C for low-level operations, C++ for high-level features
3. **Library Integration**: Integrate existing C libraries in C++ applications
4. **Performance**: Keep performance-critical C code while using C++ for convenience

**Example Code Generated:**

C++ main calls C functions via `extern "C"`:
```cpp
// src/main.cpp
#include "c_module.h"     // C functions via extern "C"
#include "cpp_utils.hpp"  // C++ utilities

int main() {
    // Call C function
    const char* msg = get_c_message();
    
    // Use C++ class
    StringHelper helper;
    std::string upper = helper.to_upper(msg);
    
    return 0;
}
```

C module with `extern "C"` wrapper:
```c
// include/c_module.h
#ifdef __cplusplus
extern "C" {
#endif

const char* get_c_message(void);

#ifdef __cplusplus
}
#endif
```

**Build System Handling:**

CMake automatically handles mixed C/C++ compilation:
- C files compiled with `gcc`/`clang`
- C++ files compiled with `g++`/`clang++`
- Proper linking of both object files

**When to Choose "Both" vs Pure C/C++:**

Choose **"Both"** when:
- Migrating from C to C++ gradually
- Integrating existing C libraries
- Need C ABI for library exports
- Performance-critical C code with C++ convenience layer

Choose **Pure C** or **Pure C++** when:
- New project with no legacy code
- No need for cross-language interoperability
- Want to enforce single language standard

## Initializing Existing Project

Already have a C/C++ project? Initialize it with Porters:

```bash
cd your-existing-project
porters init
```

The init command will:
1. Detect your existing source files
2. Auto-detect your build system (if present)
3. Ask for project metadata interactively:
   - Project name (defaults to folder name)
   - Version (defaults to 0.1.0)
   - Author
   - Description
   - License
4. Create a `porters.toml` configuration file

### Non-Interactive Init

Use the defaults without questions:

```bash
porters init --yes
```

## Project Structure

A typical Porters project looks like:

```
my-project/
├── porters.toml       # Project configuration
├── porters.lock       # Dependency lock file (auto-generated)
├── ports/             # Local dependencies (isolated)
├── src/               # Source files
│   └── main.cpp
├── include/           # Header files
├── CMakeLists.txt     # Build system files (e.g., CMake)
└── README.md
```

### Key Files

#### `porters.toml`

The main configuration file for your project:

```toml
[project]
name = "my-project"
version = "0.1.0"
description = "My awesome C++ project"
license = "Apache-2.0"
authors = ["Your Name <you@example.com>"]

[dependencies]
fmt = { git = "https://github.com/fmtlib/fmt" }

[dev-dependencies]
# Test frameworks, etc.

[build]
system = "cmake"
```

#### `porters.lock`

Auto-generated lock file ensuring reproducible builds:

```toml
version = "1"
updated_at = "2024-01-15T10:30:00Z"

[dependencies.fmt]
name = "fmt"
version = "10.1.1"
source = { Git = { url = "https://github.com/fmtlib/fmt", rev = "abc123" } }
```

#### `ports/` Directory

Contains project-specific dependencies, isolated from other projects:

```
ports/
├── fmt/               # fmt library clone
└── spdlog/           # spdlog library clone
```

## Building Your Project

Once your project is set up:

```bash
# Build the project
porters build

# Build in release mode
porters build --release
```

Porters will:
1. Resolve dependencies from `porters.toml`
2. Clone missing dependencies to `ports/`
3. Update `porters.lock`
4. Run the configured build system

## Quick Syntax Validation

**Fast compilation checking** without creating executables:

```bash
# Check all source files in the project
porters check

# Check a specific file
porters check src/main.c

# Check with verbose compiler output
porters check --verbose
```

**Benefits:**
- ⚡ **Faster than full builds** - No linking or executable generation
- 🎯 **Focused error messages** - Only shows compilation issues
- 🔍 **File-level granularity** - Check individual files or entire projects
- 📊 **Clear summaries** - See pass/fail counts at a glance

**Example Output:**
```text
✅ Checking compilation (syntax-only)
🔍 Discovering source files in project...
📦 Found 3 source file(s)

🔨 Checking: src/main.c (C)
✅ PASSED: src/main.c

━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
📊 Compilation Check Summary
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
   Total files checked: 3
   ✅ Passed: 3
   ❌ Failed: 0
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
✅ All compilation checks passed! ✅
```

## Quick Single-File Execution (Zero Configuration!)

**No setup needed!** Execute any C/C++ file instantly - even without a project or `porters.toml`:

```bash
# Execute any C/C++ file - works immediately!
porters execute hello.c

# All C/C++ extensions supported
porters execute app.cpp     # C++
porters execute main.cxx    # C++
porters execute prog.cc     # C++
porters execute code.c      # C

# With arguments
porters execute main.cpp arg1 arg2
```

**100% Automatic - No Configuration Required:**
- ✅ **Works Anywhere** - No `porters.toml` needed
- ✅ **Auto Compiler Detection** - Finds gcc/clang/g++/clang++ in PATH
- ✅ **All Extensions** - `.c`, `.cpp`, `.cxx`, `.cc`, `.c++`, `.cp`, `.C`, `.CPP`
- ✅ **Dependency Resolution** - Reads `porters.toml` if present
- ✅ **Include/Lib Injection** - Automatic dependency paths
- ✅ **One Command** - Compiles and runs instantly

**Example - No Project Needed:**

```c
// hello.c (anywhere on your system)
#include <stdio.h>

int main(int argc, char** argv) {
    printf("Hello from Porters!\n");
    if (argc > 1) {
        printf("Arguments: ");
        for (int i = 1; i < argc; i++) {
            printf("%s ", argv[i]);
        }
        printf("\n");
    }
    return 0;
}
```

```bash
# Just execute - no setup!
$ porters execute hello.c
Compiling hello.c...
Hello from Porters!

$ porters execute hello.c foo bar baz
Compiling hello.c...
Hello from Porters!
Arguments: foo bar baz
```

**With Dependencies (Automatic):**

When `porters.toml` exists, dependencies are automatically resolved:

```toml
# porters.toml (optional)
[dependencies]
fmt = { git = "https://github.com/fmtlib/fmt" }
```

```cpp
// main.cpp
#include <fmt/core.h>

int main() {
    fmt::print("Formatted output: {}\n", 42);
    return 0;
}
```

```bash
$ porters sync      # Download dependencies once
$ porters execute main.cpp
# Automatically includes fmt - no configuration needed!
Formatted output: 42
```

**No [run] Section Needed:**

The `[run]` section in `porters.toml` is **completely optional**. Only add it if you need custom compiler flags or non-standard include paths. 99% of the time, it's not needed!

See [`porters execute` documentation](./commands.md#porters-execute) for more details.

## Using Package Managers

**NEW**: Porters integrates with popular C/C++ package managers, giving you access to thousands of libraries!

### Supported Package Managers

- **Conan** - C/C++ package manager with CMake integration
- **vcpkg** - Microsoft's C/C++ package manager
- **XMake** - Lua-based build system with package management

### Quick Start with Package Managers

**Install a package locally** (project-specific):

```bash
# Using Conan
porters conan add fmt
porters co add spdlog        # 'co' is a shortcut for 'conan'

# Using vcpkg
porters vcpkg add boost
porters vc add catch2        # 'vc' is a shortcut for 'vcpkg'

# Using XMake
porters xmake add imgui
porters xm add glfw          # 'xm' is a shortcut for 'xmake'
```

**Install a package globally** (shared across all projects):

```bash
# Global installation saves disk space
porters conan add --global fmt
porters vcpkg add -g boost   # -g is short for --global
```

### Local vs Global Packages

**Local Installation** (`ports/{manager}/`):
- ✅ Project-specific versions
- ✅ Reproducible builds
- ✅ No conflicts between projects
- ✅ Tracked in version control

**Global Installation** (`~/.porters/packages/{manager}/`):
- ✅ Shared across all projects
- ✅ Saves disk space
- ✅ Faster project setup
- ✅ Perfect for common libraries

**Example Workflow:**

```bash
# One-time: Install common libraries globally
porters co add --global fmt
porters co add --global spdlog
porters vc add --global catch2

# In each new project, they're available immediately!
cd my-new-project
porters list --global       # See what's available
```

### Removing Packages

**With confirmation** (safe):

```bash
porters conan remove fmt
# Prompts: ⚠️  Remove fmt from ports/conan? (y/N):
```

**Force remove** (no confirmation):

```bash
porters conan remove --force fmt
porters vc remove -f boost   # -f is short for --force
```

### Using Command Aliases

All package manager commands support shortcuts:

```bash
# Full commands
porters conan add fmt
porters vcpkg add boost
porters xmake add imgui

# Shortcuts (faster!)
porters co add fmt
porters vc add boost
porters xm add imgui
```

### Directory Structure with Package Managers

```
my-project/
├── porters.toml
├── .porters/              # Generated files (GITIGNORED)
│   └── cache/            # Build cache, temp files
├── ports/                # Local packages
│   ├── conan/           # Conan local packages
│   │   └── conanfile.txt
│   ├── vcpkg/           # vcpkg local packages
│   │   └── vcpkg.json
│   └── xmake/           # XMake local packages
│       └── xmake.lua
└── build/                # Final build output

~/.porters/               # Global directory
└── packages/            # Global package installations
    ├── conan/
    ├── vcpkg/
    └── xmake/
```

For complete details, see the [Package Managers Guide](./package-managers.md).

## What's Next?

- Learn about [Dependency Management](./dependencies.md)
- Explore [Package Managers](./package-managers.md) in detail
- Read about [Build Configuration](./building.md)
- Check the [Command Reference](./commands.md)
