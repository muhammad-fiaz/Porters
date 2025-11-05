# Building Projects

Learn how to build C/C++ projects with Porters.

## Build Systems

Porters natively supports 14 build systems with full build execution and automatically detects which one your project uses.

### Automatic Detection

Porters automatically detects build systems by looking for specific files:

| Build System | Detection Files | Status |
|--------------|----------------|--------|
| **CMake** | `CMakeLists.txt` | ✅ Full Support |
| **XMake** | `xmake.lua` | ✅ Full Support |
| **Meson** | `meson.build` | ✅ Full Support |
| **Make** | `Makefile`, `makefile`, `GNUmakefile` | ✅ Full Support |
| **Ninja** | `build.ninja` | ✅ Full Support |
| **Autotools** | `configure`, `configure.ac` | ✅ Full Support |
| **SCons** | `SConstruct`, `SConscript` | ✅ Full Support |
| **Conan** | `conanfile.txt`, `conanfile.py` | ✅ Full Support |
| **vcpkg** | `vcpkg.json` | ✅ Full Support |
| **Bazel** | `BUILD`, `BUILD.bazel`, `WORKSPACE` | ✅ Full Support |
| **Buck2** | `BUCK`, `.buckconfig` | ✅ Full Support |
| **Premake** | `premake5.lua`, `premake4.lua` | ✅ Full Support |
| **QMake** | `*.pro` | ✅ Full Support |
| **Custom** | `porters.toml` with `[build.custom]` | ✅ Full Support |

### Manual Configuration

You can explicitly specify the build system in `porters.toml`:

```toml
[build]
system = "cmake"  # or "xmake", "meson", "make", "ninja", etc.
```

### Supported Build Systems

#### Traditional Build Systems

##### Make

```toml
[build]
system = "make"
```

Runs: `make` (respects existing Makefile)

##### Ninja

```toml
[build]
system = "ninja"
```

Runs: `ninja` (fast parallel builds)

##### Autotools

```toml
[build]
system = "autotools"
```

Runs:
```bash
./configure
make
make install
```

#### Modern Build Systems

##### CMake (Recommended)

```toml
[build]
system = "cmake"
```

Porters runs:
```bash
cmake -B build
cmake --build build
```

##### XMake

```toml
[build]
system = "xmake"
```

Runs:
```bash
xmake config
xmake build
```

##### Meson

```toml
[build]
system = "meson"
```

Runs:
```bash
meson setup build
meson compile -C build
```

##### SCons

```toml
[build]
system = "scons"
```

Runs: `scons`

##### Bazel

```toml
[build]
system = "bazel"
```

Runs: `bazel build //...`

##### Buck2

```toml
[build]
system = "buck2"
```

Runs: `buck2 build //...`

#### Package Managers

##### Conan

```toml
[build]
system = "conan"
```

Integrates with Conan package manager:
```bash
conan install .
cmake --preset conan-default
cmake --build build
```

##### vcpkg

```toml
[build]
system = "vcpkg"
```

Integrates with vcpkg:
```bash
vcpkg install
cmake -DCMAKE_TOOLCHAIN_FILE=...
cmake --build build
```

#### Other Build Systems

##### Premake

```toml
[build]
system = "premake"
```

##### QMake

```toml
[build]
system = "qmake"
```

##### Gradle (C++)

```toml
[build]
system = "gradle-cpp"
```

### Custom Build

For projects with unique build requirements:

```toml
[build]
system = "custom"

[build.custom]
configure = "./configure.sh"
build = "./build.sh"
install = "./install.sh"
test = "./test.sh"
clean = "./clean.sh"
```

## Compiler Detection

Porters automatically detects available compilers:

- **GCC** (`gcc`, `g++`)
- **Clang** (`clang`, `clang++`)
- **MSVC** (`cl`)
- **LLVM** (`llvm`)
- **MinGW** (Windows)
- **Emscripten** (`emcc`, `em++`)
- **Intel C++ Compiler** (`icc`, `icpc`)

Set preferred compiler:

```toml
[build.env]
CC = "clang"
CXX = "clang++"
```

## Build Configuration

### Enhanced Build Settings

Configure build flags, include paths, and linking options:

```toml
[build]
system = "cmake"

[build.flags]
cflags = ["-Wall", "-Wextra", "-O2"]
cxxflags = ["-std=c++17", "-Wall", "-Wextra"]
ldflags = ["-pthread"]
defines = ["DEBUG", "USE_FEATURE_X"]

[build.include]
include = [
    "include/",
    "src/",
    "/usr/local/include",
]

[build.linking]
libraries = ["pthread", "m", "dl"]
library_paths = ["/usr/local/lib"]
frameworks = ["CoreFoundation"]  # macOS only
```

### Build Scripts

Run custom scripts before/after building:

```toml
[build.scripts]
pre-build = "scripts/pre_build.sh"
post-build = "scripts/post_build.sh"
pre-install = "scripts/pre_install.sh"
post-install = "scripts/post_install.sh"
```

Example pre-build script:

```bash
#!/bin/sh
echo "🔧 Running pre-build tasks..."

# Generate version header
echo "#define VERSION \"$(git describe --tags)\"" > src/version.h

# Check dependencies
command -v cmake >/dev/null || {
    echo "❌ CMake not found"
    exit 1
}

echo "✅ Pre-build complete"
```

### Debug vs Release

```bash
# Debug build (default)
porters build

# Release build (optimized)
porters build --release
```

Override in porters.toml:

```toml
[build.flags]
cflags = ["-g", "-O0"]          # Debug
# cflags = ["-O3", "-DNDEBUG"]  # Release
```

## Environment Variables

Porters resolves environment variables in build configuration:

```toml
[build.env]
CC = "clang"
CXX = "clang++"
CMAKE_PREFIX_PATH = "/usr/local"
PKG_CONFIG_PATH = "/usr/local/lib/pkgconfig"
CFLAGS = "-march=native"
CXXFLAGS = "-std=c++20"
```

Platform-specific:

```toml
[build.env.linux]
CC = "gcc-11"
CXX = "g++-11"

[build.env.macos]
CC = "clang"
CXX = "clang++"

[build.env.windows]
CC = "cl"
CXX = "cl"
```

## Auto Source Discovery

Porters automatically discovers source files:

```toml
[build]
auto_discover = true  # Default
```

Detected file types:
- `.c`, `.cpp`, `.cc`, `.cxx` - Source files
- `.h`, `.hpp`, `.hh`, `.hxx` - Header files

Exclude patterns:

```toml
[build]
exclude = [
    "tests/",
    "examples/",
    "vendor/",
    "**/*.test.cpp",
]
```

## Build Targets

Specify what to build:

```toml
[build]
targets = ["all"]  # Default

# Or specific targets
targets = ["main", "library"]
```

For CMake:

```toml
[build]
system = "cmake"
targets = ["MyApp", "MyLib"]
```

## Parallel Builds

Control build parallelism:

```toml
[build]
jobs = 4  # Number of parallel jobs

# Or auto-detect CPU cores
jobs = "auto"
```

Command line:

```bash
porters build --jobs 8
porters build -j 8
```

## Cross-Compilation

Configure for cross-compilation:

```toml
[build]
system = "cmake"
toolchain = "cmake/arm-toolchain.cmake"

[build.env]
CMAKE_TOOLCHAIN_FILE = "cmake/arm-toolchain.cmake"
TARGET_ARCH = "arm64"
```

## Build Artifacts

Specify output locations:

```toml
[build]
output_dir = "dist/"
binary_name = "my-app"
```

Install artifacts:

```bash
porters build --install
```

With custom prefix:

```bash
porters build --install --prefix /usr/local
```

## Next Steps

- [Publishing](./publishing.md)
- [Configuration](./configuration.md)
