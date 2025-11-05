# Troubleshooting

Solutions to common issues when using Porters.

## porters execute - Single File Execution Issues

### "Execute: Compiler not found"

**Cause**: No C/C++ compiler installed or not in PATH

**Solution**:
```bash
# Install GCC (Linux)
sudo apt install build-essential

# Install Xcode Command Line Tools (macOS)
xcode-select --install

# Install MinGW (Windows)
# Download from mingw-w64.org
# Or install Visual Studio with C++ tools

# Verify installation
gcc --version        # For C
g++ --version        # For C++
clang --version      # Alternative compiler
clang++ --version    # Alternative C++ compiler
```

**Note**: `porters execute` automatically detects compilers in this order:
- **C files** (.c): gcc → clang → cc
- **C++ files** (.cpp, .cxx, .cc, .c++, .cp, .CPP): g++ → clang++ → c++

### "Execute: Compilation failed"

**Cause**: Syntax errors or missing dependencies

**Solution**:
```bash
# 1. Check compiler output for specific errors
porters execute myfile.c

# 2. Verify dependencies are installed (if using porters.toml)
porters list

# 3. Sync dependencies if missing
porters sync

# 4. Add custom flags ONLY if needed (rarely required)
[run]  # This section is OPTIONAL!
compiler-flags = ["-Wall", "-Wextra"]  # See detailed warnings

# 5. Test compilation directly
gcc -I./include myfile.c -o test
./test
```

### "Execute: Unsupported file extension"

**Cause**: File extension not recognized as C/C++

**Solution**:

**Supported Extensions:**
- **C files**: `.c`, `.C`
- **C++ files**: `.cpp`, `.cxx`, `.cc`, `.c++`, `.cp`, `.CPP`

**Note**: Header files (`.h`, `.hpp`, `.hxx`) cannot be executed directly - they must be included in a `.c`/`.cpp` file.

```bash
# Correct usage
porters execute main.c      # ✅ C file
porters execute app.cpp     # ✅ C++ file
porters execute code.cxx    # ✅ C++ file
porters execute prog.c++    # ✅ C++ file

# Incorrect - headers cannot be compiled
porters execute header.h    # ❌ Not supported
porters execute header.hpp  # ❌ Not supported
```

### "Execute: Dependency includes not found"

**Cause**: Dependencies not resolved or incorrect paths

**Solution**:
```bash
# 1. Ensure dependencies are in porters.toml
[dependencies]
fmt = { git = "https://github.com/fmtlib/fmt" }

# 2. Sync dependencies
porters sync

# 3. Verify dependency structure
ls ports/fmt/include  # Should contain header files

# 4. Execute - includes are AUTOMATIC
porters execute main.cpp  # Dependencies auto-included!

# 5. ONLY add manual paths if automatic detection fails (rare)
[run]  # OPTIONAL section
include-dirs = ["./custom/path"]
```

**Note**: `porters execute` automatically finds and includes all dependency paths from `porters.toml`. Manual configuration is rarely needed.

### "Execute: File not found"

**Cause**: Invalid file path or file doesn't exist

**Solution**:
```bash
# Use relative or absolute paths
porters execute ./src/main.c      # Relative
porters execute /full/path/file.c # Absolute

# Check file exists
ls ./src/main.c

# Ensure correct file extension
porters execute hello.c   # C file (uses gcc/clang)
porters execute hello.cpp # C++ file (uses g++/clang++)
```

### "Execute: Permission denied"

**Cause**: Compiled executable lacks execute permissions

**Solution**:
```bash
# Linux/macOS: Porters handles this automatically
# If issues persist, check cache directory permissions
chmod -R u+rwx ~/.porters/cache

# Windows: Run as administrator if needed
```

### "Execute: Custom compiler not used"

**Cause**: Compiler override in porters.toml not working

**Solution**:
```toml
# Ensure [run] section is correct (OPTIONAL - only if needed)
[run]
c-compiler = "clang"      # Must be in PATH
cpp-compiler = "clang++"

# Verify compiler is in PATH
which clang    # Linux/macOS
where clang    # Windows

# Test compiler directly
clang --version
```

**Note**: This is rarely needed! `porters execute` automatically finds compilers. Only use `[run]` section for custom compiler configurations.

## porters run - Project Execution Issues

### "Run: Build required before running"

**Cause**: Project not built or binary not found

**Solution**:
```bash
# Build project first
porters build

# Then run the compiled executable
porters run

# Or build and run in one go
porters build && porters run
```

### "Run: Executable not found"

**Cause**: Build didn't produce expected binary

**Solution**:
```bash
# 1. Check build output directory
ls build/      # CMake default
ls .build/     # XMake default
ls builddir/   # Meson default

# 2. Verify build succeeded
porters build

# 3. Check porters.toml for correct output path
[build]
output-dir = "build"  # Should match your build system

# 4. Manually locate binary
find . -name "myproject" -type f
```

### "Run: Arguments not passed to program"

**Cause**: Incorrect argument syntax

**Solution**:
```bash
# Correct usage
porters run -- arg1 arg2 arg3

# Everything after -- is passed to the program
porters run -- --flag=value input.txt

# Without -- for simple args (may work)
porters run arg1 arg2
```

**Note**: Use `--` to separate porters args from program args if your program uses flags that might conflict with porters options.

## Common Issues

### "porters: command not found"

**Cause**: Porters not in PATH

**Solution**:
```bash
# Check if installed
cargo install --list | grep porters

# Add cargo bin to PATH (Linux/macOS)
echo 'export PATH="$HOME/.cargo/bin:$PATH"' >> ~/.bashrc
source ~/.bashrc

# Windows: Add to PATH manually
# C:\Users\<username>\.cargo\bin
```

### "porters.toml not found"

**Cause**: Running porters command outside project directory OR executing single file without project

**Solution**:
```bash
# For project commands (build, run, sync, etc.)
porters init  # Initialize project first
# OR
cd /path/to/your/project

# For single file execution - NO porters.toml needed!
porters execute hello.c  # Works anywhere!
```

**Note**: `porters execute` works without `porters.toml`. Other commands (build, run, sync) require a project.
which clang    # Linux/macOS
where clang    # Windows

# Test compiler directly
clang --version
```

## Common Issues

### "porters: command not found"

**Cause**: Porters not in PATH

**Solution**:
```bash
# Check if installed
cargo install --list | grep porters

# Add cargo bin to PATH (Linux/macOS)
echo 'export PATH="$HOME/.cargo/bin:$PATH"' >> ~/.bashrc
source ~/.bashrc

# Windows: Add to PATH manually
# C:\Users\<username>\.cargo\bin
```

### "porters.toml not found"

**Cause**: Running porters command outside project directory

**Solution**:
```bash
# Initialize project first
porters init

# Or navigate to project directory
cd /path/to/your/project
```

## Build Errors

### "Build system not found"

**Cause**: Required build tool not installed

**Solution**:
```bash
# Install CMake
# macOS: brew install cmake
# Ubuntu: sudo apt install cmake
# Windows: Download from cmake.org

# Install XMake
# curl -fsSL https://xmake.io/shget.text | bash

# Install Meson
# pip install meson ninja
```

### "Compiler not found"

**Cause**: No C/C++ compiler installed

**Solution**:
```bash
# Install GCC (Linux)
sudo apt install build-essential

# Install Clang (macOS)
xcode-select --install

# Install MSVC (Windows)
# Download Visual Studio with C++ tools
```

### "Build failed with unknown error"

**Cause**: Various build issues

**Solution**:
```bash
# Clean build
rm -rf build/

# Rebuild
porters build

# Check build system files
# Ensure CMakeLists.txt, xmake.lua, etc. are valid
```

## Dependency Resolution

### "Failed to clone repository"

**Cause**: Git URL incorrect or authentication required

**Solution**:
```bash
# Test Git clone manually
git clone https://github.com/fmtlib/fmt

# For SSH URLs, ensure SSH keys are configured
ssh -T git@github.com

# Use HTTPS if SSH fails
porters add fmt --git https://github.com/fmtlib/fmt
```

### "Dependency version conflict"

**Cause**: Multiple incompatible versions required

**Solution**:
```bash
# Check porters.lock
cat porters.lock

# Regenerate lock file
porters lock

# Manually resolve in porters.toml
# Specify compatible versions
```

### "Dependency not found in ports/"

**Cause**: Dependencies not synchronized

**Solution**:
```bash
# Sync all dependencies
porters sync

# Force re-download
rm -rf ports/
porters sync
```

## Platform-Specific Issues

### Windows

**Issue**: "Permission denied" when running porters

**Solution**:
```powershell
# Run as Administrator
# Or add exception to antivirus
```

**Issue**: Long path errors

**Solution**:
```powershell
# Enable long paths
reg add HKLM\SYSTEM\CurrentControlSet\Control\FileSystem /v LongPathsEnabled /t REG_DWORD /d 1
```

### macOS

**Issue**: "xcrun: error: invalid active developer path"

**Solution**:
```bash
xcode-select --install
```

**Issue**: Library not found errors

**Solution**:
```bash
# Set library paths
export DYLD_LIBRARY_PATH=/usr/local/lib:$DYLD_LIBRARY_PATH
```

### Linux

**Issue**: Missing system libraries

**Solution**:
```bash
# Install development tools
sudo apt install build-essential cmake git

# For specific libraries
sudo apt install libssl-dev libcurl4-openssl-dev
```

## Performance Issues

### Slow dependency downloads

**Solution**:
```bash
# Use global install for common libraries
porters install fmt --git https://github.com/fmtlib/fmt
porters install spdlog --git https://github.com/gabime/spdlog

# Enable caching (should be default)
# Check ~/.porters/config.toml
```

### Slow builds

**Solution**:
```toml
# Increase parallel jobs in ~/.porters/config.toml
[settings]
parallel_jobs = 16  # Adjust based on CPU cores
```

## Getting Help

If you encounter issues not covered here:

1. **Check GitHub Issues**: [github.com/muhammad-fiaz/Porters/issues](https://github.com/muhammad-fiaz/Porters/issues)
2. **Open New Issue**: Provide:
   - Porters version (`porters --version`)
   - Operating system
   - Error message
   - Steps to reproduce
3. **Consult Documentation**: Review relevant sections

## Debug Mode

Enable verbose logging:

```bash
# Set environment variable
export RUST_LOG=debug
porters build
```

This provides detailed output for debugging.

## Next Steps

- [Command Reference](./commands.md)
- [Configuration](./configuration.md)
- [Contributing](./contributing.md)
