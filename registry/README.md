# Porters Package Registry

This directory contains the official Porters package registry - a collection of curated C/C++ packages that can be installed via Porters.

## 📦 What is the Registry?

The Porters registry is a community-driven collection of package definitions for popular C/C++ libraries. Each package includes:
- Repository URL
- Short description
- Version information
- Dependencies
- Build system configuration
- Installation instructions

## 🚀 Using the Registry

### Search for Packages
```bash
porters registry search <name>
porters search <name>  # alias
```

### Add a Package from Registry
```bash
porters registry add <package-name>
porters add <package-name>  # Will check registry if not a package manager package
```

### List All Registry Packages
```bash
porters registry list
```

## 🤝 Contributing Packages

Want to add your library or a popular library to the registry? Follow these steps:

### 1. Create a Package Definition

Create a JSON file in the appropriate category folder (e.g., `networking/`, `graphics/`, `utilities/`):

```json
{
  "name": "awesome-lib",
  "description": "An awesome C++ library for doing amazing things",
  "repository": "https://github.com/username/awesome-lib",
  "version": "1.2.3",
  "license": "MIT",
  "build_system": "cmake",
  "dependencies": {
    "fmt": "^9.0.0",
    "spdlog": "^1.10.0"
  },
  "options": {
    "shared": false,
    "tests": true
  },
  "install": {
    "cmake": {
      "find_package": "AwesomeLib",
      "targets": ["AwesomeLib::AwesomeLib"]
    }
  },
  "tags": ["networking", "async", "modern-cpp"]
}
```

### 2. Required Fields

- **name**: Package name (lowercase, hyphenated)
- **description**: Short description (max 120 characters)
- **repository**: Git repository URL
- **version**: Latest stable version
- **license**: License identifier (SPDX format)
- **build_system**: One of: `cmake`, `meson`, `xmake`, `autotools`, `custom`

### 3. Optional Fields

- **dependencies**: Map of dependency names to version requirements
- **options**: Build options (shared libs, tests, examples, etc.)
- **install**: Installation metadata for different build systems
- **tags**: Searchable tags for categorization
- **homepage**: Project homepage URL
- **documentation**: Documentation URL
- **platforms**: Supported platforms (default: all)

### 4. Submit Your Package

1. Fork the Porters repository
2. Add your package JSON file to the appropriate category in `registry/`
3. Create categories if needed (e.g., `registry/databases/`, `registry/gui/`)
4. Submit a Pull Request with:
   - Package JSON file
   - Brief description of the library
   - Any special installation notes

## 📂 Directory Structure

```
registry/
├── README.md              # This file
├── schema.json            # JSON schema for package definitions
├── audio/                 # Audio processing libraries
├── compression/           # Compression/decompression libraries
├── crypto/                # Cryptography libraries
├── databases/             # Database clients and libraries
├── graphics/              # Graphics and rendering libraries
├── gui/                   # GUI frameworks
├── networking/            # Networking libraries
├── serialization/         # Serialization formats (JSON, XML, etc.)
├── testing/               # Testing frameworks
└── utilities/             # General utilities and tools
```

## 🔍 Package Discovery

Porters searches the registry when you:
- Run `porters add <name>` and the package isn't in your current package managers
- Run `porters registry search <query>`
- Run `porters search <query>`

## 🎯 Best Practices

1. **Keep descriptions concise** - Max 120 characters
2. **Use semantic versioning** - Follow SemVer for version requirements
3. **Test your package** - Ensure it builds on major platforms
4. **Add relevant tags** - Help users discover your package
5. **Specify dependencies** - Include all required dependencies
6. **Document build options** - Explain available options
7. **Update regularly** - Keep version information current

## 📋 Examples

See existing packages in the registry for examples:
- `registry/networking/asio.json` - Header-only library
- `registry/testing/catch2.json` - CMake-based testing framework
- `registry/compression/zlib.json` - Classic C library
- `registry/graphics/sdl2.json` - Cross-platform multimedia library

## ⚖️ License

All package definitions in this registry are under CC0 1.0 (Public Domain).
The packages themselves are under their respective licenses as specified in each definition.

## 🆘 Support

- **Issues**: https://github.com/muhammad-fiaz/Porters/issues
- **Discussions**: https://github.com/muhammad-fiaz/Porters/discussions
- **Documentation**: https://muhammad-fiaz.github.io/Porters/
- **Repository**: https://github.com/muhammad-fiaz/Porters
