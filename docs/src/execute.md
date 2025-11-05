# Porters Execute Guide

**100% Automatic Single-File C/C++ Execution - Zero Configuration Required**

## Overview

`porters execute` lets you compile and run any C/C++ file instantly - no project setup, no configuration files, no build systems. Just point it at your code and go!

### Key Features

- ✅ **Works Anywhere** - No `porters.toml` required
- ✅ **Auto Compiler Detection** - Finds gcc, clang, g++, clang++ automatically
- ✅ **All File Extensions** - Supports `.c`, `.cpp`, `.cxx`, `.cc`, `.c++`, `.cp`, `.C`, `.CPP`
- ✅ **Dependency Resolution** - Reads `porters.toml` if present, optional otherwise
- ✅ **Automatic Include/Lib Paths** - Zero configuration dependency integration
- ✅ **One Command Execution** - Compiles and runs in a single step

## Basic Usage

### Simple Execution

```bash
# Execute any C file
porters execute hello.c

# Execute any C++ file
porters execute main.cpp
porters execute app.cxx
porters execute code.cc
porters execute prog.c++

# With command-line arguments
porters execute myprogram.c arg1 arg2 arg3
```

### Supported File Extensions

| Extension | Language | Compiler Used |
|-----------|----------|---------------|
| `.c`, `.C` | C | gcc → clang → cc |
| `.cpp`, `.CPP` | C++ | g++ → clang++ → c++ |
| `.cxx` | C++ | g++ → clang++ → c++ |
| `.cc` | C++ | g++ → clang++ → c++ |
| `.c++` | C++ | g++ → clang++ → c++ |
| `.cp` | C++ | g++ → clang++ → c++ |

**Note**: Header files (`.h`, `.hpp`, `.hxx`) cannot be executed directly - they must be included in a `.c` or `.cpp` file.

## How It Works

### Automatic Compiler Detection

Porters automatically finds your C/C++ compiler:

**For C files** (`.c`, `.C`):
1. Tries `gcc` first (most common)
2. Falls back to `clang`
3. Falls back to generic `cc`

**For C++ files** (`.cpp`, `.cxx`, `.cc`, `.c++`, `.cp`, `.CPP`):
1. Tries `g++` first (most common)
2. Falls back to `clang++`
3. Falls back to generic `c++`

**No configuration needed** - just have a compiler in your PATH!

### Dependency Resolution (Automatic)

When you have a `porters.toml` in your directory, `porters execute` automatically:
- Reads all dependencies
- Finds their include directories
- Finds their library directories
- Adds all paths to the compiler command
- Links all required libraries

**You don't need to configure anything!**

## Examples

### Example 1: Hello World (No Project)

Create a file anywhere on your system:

```c
// hello.c
#include <stdio.h>

int main() {
    printf("Hello from Porters!\n");
    return 0;
}
```

Execute it:

```bash
$ porters execute hello.c
Compiling hello.c...
Hello from Porters!
```

**No `porters.toml` needed!** Works instantly.

### Example 2: With Arguments

```cpp
// args.cpp
#include <iostream>

int main(int argc, char** argv) {
    std::cout << "Arguments received: " << argc - 1 << "\n";
    for (int i = 1; i < argc; i++) {
        std::cout << "  [" << i << "] " << argv[i] << "\n";
    }
    return 0;
}
```

Execute with arguments:

```bash
$ porters execute args.cpp foo bar baz
Compiling args.cpp...
Arguments received: 3
  [1] foo
  [2] bar
  [3] baz
```

### Example 3: With Dependencies (Automatic)

Create a project with dependencies:

```toml
# porters.toml
[package]
name = "my-app"
version = "0.1.0"

[dependencies]
fmt = { git = "https://github.com/fmtlib/fmt" }
```

Sync dependencies once:

```bash
$ porters sync
Syncing dependencies...
✓ fmt downloaded
```

Create your code:

```cpp
// format_test.cpp
#include <fmt/core.h>
#include <fmt/color.h>

int main() {
    fmt::print("Plain text\n");
    fmt::print(fg(fmt::color::green), "Green text\n");
    fmt::print(fg(fmt::color::red) | fmt::emphasis::bold, "Bold red text\n");
    return 0;
}
```

Execute - dependencies are **automatically included**:

```bash
$ porters execute format_test.cpp
Compiling format_test.cpp...
Plain text
Green text
Bold red text
```

**No configuration needed!** Porters found fmt automatically.

### Example 4: Math and System Libraries

```c
// math_test.c
#include <stdio.h>
#include <math.h>

int main() {
    double x = 16.0;
    double result = sqrt(x);
    printf("sqrt(%.0f) = %.2f\n", x, result);
    printf("sin(PI/2) = %.2f\n", sin(M_PI / 2.0));
    return 0;
}
```

Execute (math library automatically linked on Linux):

```bash
$ porters execute math_test.c
Compiling math_test.c...
sqrt(16) = 4.00
sin(PI/2) = 1.00
```

### Example 5: Multiple Source Files Pattern

When you need multiple files, use includes:

```c
// utils.h
#ifndef UTILS_H
#define UTILS_H

int add(int a, int b);
int multiply(int a, int b);

#endif
```

```c
// utils.c
#include "utils.h"

int add(int a, int b) {
    return a + b;
}

int multiply(int a, int b) {
    return a * b;
}
```

```c
// main.c
#include <stdio.h>
#include "utils.h"

int main() {
    printf("5 + 3 = %d\n", add(5, 3));
    printf("5 * 3 = %d\n", multiply(5, 3));
    return 0;
}
```

For multiple `.c` files, compile them together:

```bash
# Compile utils.c into object file
gcc -c utils.c -o utils.o

# Execute main.c and link utils.o
gcc main.c utils.o -o main && ./main
```

**Or** use `porters build` for multi-file projects:

```bash
$ porters init     # Create project
$ porters build    # Build all sources
$ porters run      # Execute compiled binary
```

**Note**: `porters execute` is for **single-file** quick execution. For multi-file projects, use `porters build` + `porters run`.

## Optional Configuration

### When is Configuration Needed?

**99% of the time: NEVER!**

`porters execute` works automatically. You only need the `[run]` section if:
- Using a non-standard compiler not in PATH
- Need custom compiler/linker flags
- Have include paths outside of dependency directories
- Want to exclude specific patterns from dependency resolution

### Optional [run] Section

```toml
# porters.toml - ALL FIELDS ARE OPTIONAL!

[run]
# Custom C compiler (default: auto-detect gcc/clang/cc)
c-compiler = "clang"

# Custom C++ compiler (default: auto-detect g++/clang++/c++)
cpp-compiler = "clang++"

# Additional include directories (default: auto-detect from deps)
include-dirs = ["./custom/include", "/usr/local/custom/include"]

# Additional library directories (default: auto-detect from deps)
library-dirs = ["./custom/lib"]

# Patterns to exclude from automatic include/lib detection (default: empty)
exclude-patterns = ["**/tests/**", "**/examples/**"]

# Custom compiler flags (default: none)
compiler-flags = ["-Wall", "-Wextra", "-O2", "-std=c++20"]

# Custom linker flags (default: none)
linker-flags = ["-lpthread", "-ldl"]
```

**Example - Custom Compiler:**

```toml
[run]
cpp-compiler = "clang++"
compiler-flags = ["-std=c++20", "-Wall"]
```

```bash
$ porters execute modern.cpp
# Uses clang++ with C++20 and warnings enabled
```

**Example - Custom Include Path:**

```toml
[run]
include-dirs = ["/opt/mylib/include"]
```

```bash
$ porters execute app.c
# Automatically adds -I/opt/mylib/include
```

## Differences from `porters run`

| Feature | `porters execute` | `porters run` |
|---------|-------------------|---------------|
| **Purpose** | Single-file quick execution | Run compiled project binary |
| **Requires Project** | ❌ No | ✅ Yes (porters.toml) |
| **Compilation** | ✅ Compiles on-the-fly | ❌ Uses pre-built binary |
| **Build System** | ❌ None (direct compiler) | ✅ CMake/XMake/Meson/Make/etc. |
| **Multi-file Support** | ⚠️ Single file only | ✅ Full project |
| **Speed** | ⚠️ Compiles each time | ✅ Fast (already built) |
| **Best For** | Prototyping, testing, scripts | Production applications |

**When to use `porters execute`:**
- Quick testing/prototyping
- Single-file programs
- Code demonstrations
- Scripting with C/C++
- Learning/teaching

**When to use `porters build` + `porters run`:**
- Multi-file projects
- Production applications
- Complex build configurations
- Performance-critical builds
- Cross-compilation

## Command Reference

### Syntax

```bash
porters execute <file> [args...]
```

### Parameters

- `<file>` - Path to C/C++ source file (required)
  - Supported extensions: `.c`, `.cpp`, `.cxx`, `.cc`, `.c++`, `.cp`, `.C`, `.CPP`
  - Can be relative or absolute path
- `[args...]` - Arguments to pass to the compiled program (optional)

### Examples

```bash
# Basic execution
porters execute hello.c

# With relative path
porters execute ./src/main.cpp

# With absolute path
porters execute /home/user/code/test.c

# With arguments
porters execute myprogram.c input.txt --verbose --count=5

# Different C++ extensions
porters execute app.cpp
porters execute app.cxx
porters execute app.cc
porters execute app.c++
```

## Troubleshooting

### "Compiler not found"

**Solution**: Install gcc/g++ or clang/clang++

```bash
# Linux
sudo apt install build-essential

# macOS
xcode-select --install

# Windows
# Install MinGW or Visual Studio with C++ tools
```

### "Unsupported file extension"

**Solution**: Use a supported extension

```bash
# Supported
porters execute main.c      # ✅
porters execute main.cpp    # ✅
porters execute main.cxx    # ✅

# Not supported
porters execute main.h      # ❌ Header files
porters execute main.txt    # ❌ Not C/C++
```

### "Compilation failed"

**Solution**: Check compiler error messages

```bash
# Add warnings for better diagnostics
[run]
compiler-flags = ["-Wall", "-Wextra"]
```

### "Missing includes"

**Solution**: Ensure dependencies are synced

```bash
# If using porters.toml with dependencies
porters sync

# Then execute
porters execute main.cpp
```

## Performance Tips

### Cache Compilation Results

Porters caches compiled binaries in `~/.porters/cache/` (Linux/macOS) or `%USERPROFILE%\.porters\cache\` (Windows).

The cache is automatically managed - binaries are recompiled when:
- Source file changes
- Dependencies change
- Compiler flags change

### For Repeated Execution

If you're running the same file many times:

**Option 1: Keep using execute** (cached after first run)
```bash
$ porters execute test.c  # Compiles
$ porters execute test.c  # Uses cache (fast!)
```

**Option 2: Use a project** (fastest for repeated use)
```bash
$ porters init
$ porters build
$ porters run    # Very fast!
$ porters run    # Very fast!
```

## Advanced Usage

### Custom Compiler Flags

```toml
[run]
compiler-flags = [
    "-std=c++20",      # Use C++20 standard
    "-O3",             # Maximum optimization
    "-march=native",   # CPU-specific optimizations
    "-Wall",           # All warnings
    "-Wextra",         # Extra warnings
    "-Werror",         # Treat warnings as errors
]
```

### Cross-Compilation

For cross-compilation, use `porters build` instead:

```bash
# Build for different platforms
porters build --linux
porters build --windows
porters build --macos
porters build --all-platforms
```

`porters execute` uses your local compiler for the current platform only.

### Integration with Scripts

Use `porters execute` in shell scripts:

```bash
#!/bin/bash
# compile_and_test.sh

# Execute C program
if porters execute test.c --self-test; then
    echo "✓ Tests passed"
else
    echo "✗ Tests failed"
    exit 1
fi
```

## Summary

**`porters execute` is the fastest way to run C/C++ code:**

1. ✅ **No configuration** - Works immediately
2. ✅ **No project setup** - Execute files anywhere
3. ✅ **Automatic dependencies** - Reads `porters.toml` if present
4. ✅ **All C/C++ extensions** - `.c`, `.cpp`, `.cxx`, `.cc`, `.c++`, `.cp`
5. ✅ **Smart compiler detection** - Finds gcc, g++, clang, clang++
6. ✅ **One command** - Compiles and runs instantly

**Perfect for:**
- Quick prototyping
- Code testing
- Single-file programs
- Learning C/C++
- Code demonstrations

**For larger projects, use:**
- `porters build` - Compile entire project
- `porters run` - Execute compiled binary
