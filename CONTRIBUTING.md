# Contributing to Porters

Thank you for your interest in contributing to Porters! This document provides guidelines and instructions for contributing.

## Table of Contents

- [Code of Conduct](#code-of-conduct)
- [Getting Started](#getting-started)
- [Development Setup](#development-setup)
- [Project Structure](#project-structure)
- [Making Changes](#making-changes)
- [Testing](#testing)
- [Submitting Changes](#submitting-changes)
- [Coding Standards](#coding-standards)
- [Release Process](#release-process)

## Code of Conduct

- Be respectful and inclusive
- Focus on constructive feedback
- Help others learn and grow
- Follow the project's technical standards

## Getting Started

### Prerequisites

- **Rust**: 1.83+ with edition 2024 support
- **Git**: For version control
- **C/C++ Toolchain**: For testing build systems
  - Windows: MSVC or MinGW
  - Linux: GCC or Clang
  - macOS: Xcode Command Line Tools

### Fork and Clone

1. Fork the repository on GitHub
2. Clone your fork:
   ```bash
   git clone https://github.com/YOUR-USERNAME/porters.git
   cd porters
   ```
3. Add upstream remote:
   ```bash
   git remote add upstream https://github.com/muhammad-fiaz/porters.git
   ```

## Development Setup

### Build from Source

```bash
# Debug build (faster compilation, slower runtime)
cargo build

# Release build (slower compilation, optimized runtime)
cargo build --release
```

### Run Tests

```bash
# Run all tests
cargo test

# Run specific test
cargo test test_name

# Run with output
cargo test -- --nocapture
```

### Run Clippy (Linter)

```bash
cargo clippy -- -D warnings
```

### Run Rustfmt (Formatter)

```bash
cargo fmt --check  # Check formatting
cargo fmt          # Auto-format code
```

### Run the Binary

```bash
# Debug build
cargo run -- --help

# Release build
./target/release/porters --help
```

## Project Structure

```
porters/
├── src/
│   ├── main.rs           # CLI entry point, commands
│   ├── config.rs         # porters.toml parsing
│   ├── scanner.rs        # Project scanning
│   ├── build/
│   │   ├── mod.rs        # Build system trait
│   │   ├── cmake.rs      # CMake implementation
│   │   ├── xmake.rs      # XMake implementation
│   │   ├── meson.rs      # Meson implementation
│   │   ├── make.rs       # Make implementation
│   │   └── custom.rs     # Custom build scripts
│   ├── deps/
│   │   ├── mod.rs        # Dependency management
│   │   └── resolver.rs   # Dependency resolution
│   ├── publish.rs        # GitHub publishing
│   ├── update.rs         # Self-update mechanism
│   └── util/
│       └── mod.rs        # Utility functions
├── Cargo.toml            # Rust project manifest
├── README.md             # Project documentation
├── CHANGELOG.md          # Version history
├── CONTRIBUTING.md       # This file
├── TESTING.md            # Testing guide
└── .github/
    └── workflows/
        ├── ci.yml        # CI pipeline
        └── release.yml   # Release automation
```

## Making Changes

### Branch Naming

Use descriptive branch names:
- `feature/add-bazel-support` - New features
- `fix/cmake-generation-bug` - Bug fixes
- `docs/update-readme` - Documentation
- `refactor/deps-module` - Code refactoring
- `test/add-integration-tests` - Tests

### Commit Messages

Follow conventional commits:

```
type(scope): subject

body (optional)

footer (optional)
```

**Types**:
- `feat`: New feature
- `fix`: Bug fix
- `docs`: Documentation only
- `style`: Formatting, missing semicolons, etc.
- `refactor`: Code restructuring
- `test`: Adding tests
- `chore`: Maintenance tasks

**Examples**:
```
feat(build): add Bazel build system support

Implements BuildSystem trait for Bazel, including:
- Project detection via WORKSPACE file
- Build command execution
- Test command support

Closes #42
```

```
fix(deps): resolve version conflict detection

Fixed issue where conflicting versions weren't detected
when using different platforms.

Fixes #56
```

### Keep Changes Focused

- One feature/fix per pull request
- Break large changes into smaller PRs
- Update tests and documentation
- Ensure CI passes before requesting review

## Testing

### Writing Tests

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_porters_config() {
        let toml = r#"
            [project]
            name = "test"
            version = "1.0.0"
        "#;
        
        let config = parse_porters_config(toml).unwrap();
        assert_eq!(config.project.name, "test");
        assert_eq!(config.project.version, "1.0.0");
    }
}
```

### Test Coverage

Aim for:
- **Unit tests**: Individual functions
- **Integration tests**: Component interactions
- **End-to-end tests**: Full workflows

### Manual Testing

See `TESTING.md` for comprehensive manual test checklist.

## Submitting Changes

### Before Submitting

1. ✅ Update documentation if needed
2. ✅ Add/update tests
3. ✅ Run `cargo test`
4. ✅ Run `cargo clippy`
5. ✅ Run `cargo fmt`
6. ✅ Update `CHANGELOG.md` (under `[Unreleased]`)
7. ✅ Ensure CI passes

### Pull Request Process

1. **Update your fork**:
   ```bash
   git fetch upstream
   git rebase upstream/main
   ```

2. **Push changes**:
   ```bash
   git push origin your-branch-name
   ```

3. **Create Pull Request** on GitHub with:
   - Clear title and description
   - Link to related issues
   - Screenshots/examples if applicable
   - Checklist of changes

4. **Respond to review feedback**:
   - Address comments promptly
   - Push additional commits if needed
   - Use `git push --force-with-lease` if rebasing

5. **Squash commits** (if requested):
   ```bash
   git rebase -i HEAD~N  # N = number of commits
   git push --force-with-lease
   ```

## Coding Standards

### Rust Style

- Follow the [Rust API Guidelines](https://rust-lang.github.io/api-guidelines/)
- Use `rustfmt` for formatting
- Use `clippy` for linting
- Prefer explicit error types over `.unwrap()`
- Document public APIs with `///` comments

### Error Handling

```rust
use anyhow::{Context, Result};

fn load_config(path: &Path) -> Result<Config> {
    let content = fs::read_to_string(path)
        .context("Failed to read config file")?;
    
    toml::from_str(&content)
        .context("Failed to parse TOML")
}
```

### Code Comments

```rust
/// Parses a porters.toml configuration file.
///
/// # Arguments
///
/// * `content` - The TOML file content as a string
///
/// # Returns
///
/// Returns a `Result` containing the parsed `PortersConfig` or an error.
///
/// # Examples
///
/// ```
/// let config = parse_porters_config(toml_content)?;
/// println!("Project: {}", config.project.name);
/// ```
pub fn parse_porters_config(content: &str) -> Result<PortersConfig> {
    // Implementation
}
```

### Performance

- Use `&str` instead of `String` when possible
- Avoid unnecessary cloning
- Use iterators over loops where appropriate
- Profile before optimizing

### Dependencies

- Minimize external dependencies
- Prefer well-maintained crates
- Check license compatibility (MIT/Apache-2.0)
- Update `Cargo.toml` and document why dependency is needed

## Release Process

Maintainers only:

### 1. Prepare Release

```bash
# Update version in Cargo.toml
# Update CHANGELOG.md with release date
# Commit changes
git commit -am "chore: prepare v1.0.0 release"
```

### 2. Create Tag

```bash
git tag -a v1.0.0 -m "Release version 1.0.0"
git push origin main
git push origin v1.0.0
```

### 3. CI/CD Automation

GitHub Actions will automatically:
1. Build binaries for all platforms
2. Create GitHub release
3. Upload binaries
4. Publish to crates.io

### 4. Post-Release

1. Verify release on GitHub
2. Test download links
3. Test `porters upgrade`
4. Announce on social media
5. Update documentation site

## Feature Requests

1. Check existing issues first
2. Create detailed issue with:
   - Use case description
   - Expected behavior
   - Proposed solution (optional)
3. Wait for maintainer feedback
4. Discuss implementation approach
5. Submit PR when approved

## Bug Reports

Include:
- Porters version (`porters --version`)
- Operating system and version
- Rust version (`rustc --version`)
- Steps to reproduce
- Expected vs actual behavior
- Error messages/logs
- `porters.toml` (if applicable)

## Questions?

- 📧 Email: contact@muhammadfiaz.com
- 💬 GitHub Discussions: [Start a discussion](https://github.com/muhammad-fiaz/porters/discussions)
- 🐛 Issues: [Report a bug](https://github.com/muhammad-fiaz/porters/issues)

## License

By contributing, you agree that your contributions will be licensed under the MIT License.

---

Thank you for contributing to Porters! 🚀
