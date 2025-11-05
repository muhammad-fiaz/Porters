# Getting Started

This guide will walk you through creating and managing your first C/C++ project with Porters.

## Creating a New Project

The easiest way to start is creating a new project:

```bash
porters create my-awesome-project
```

This launches an interactive wizard that asks:

1. **Project Type**: Application or Library
2. **Language**: C, C++, or Both
3. **Library Name**: (If creating a library)
4. **Author**: Your name (optional)
5. **Email**: Your email (optional)
6. **Repository URL**: Git repository URL (optional)
7. **License**: Choose from Apache-2.0, MIT, GPL, etc.
8. **Build System**: CMake, XMake, Meson, Make, or Custom

### Quick Create with Defaults

Skip the questions and use defaults:

```bash
porters create my-project --yes
```

This creates a C/C++ application with:
- CMake as the build system
- Apache-2.0 license
- Basic project structure

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

## What's Next?

- Learn about [Dependency Management](./dependencies.md)
- Explore [Build Configuration](./building.md)
- Read the [Command Reference](./commands.md)
