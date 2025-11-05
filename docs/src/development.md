# Development Guide

Guide for developers working on Porters.

## Development Setup

### Prerequisites

- Rust 1.70+ (install via [rustup](https://rustup.rs/))
- Git
- At least one build system (CMake, XMake, Meson, or Make)
- C/C++ compiler (for testing)

### Clone and Build

```bash
git clone https://github.com/muhammad-fiaz/Porters.git
cd Porters

# Build
cargo build

# Run tests
cargo test

# Run development version
cargo run -- --version
```

## Project Structure

```
Porters/
├── src/
│   ├── main.rs           # CLI entry point
│   ├── config.rs         # Configuration handling
│   ├── scan.rs           # Project scanning
│   ├── util.rs           # Utilities
│   ├── global.rs         # Global config management
│   ├── lockfile.rs       # Lock file management
│   ├── build/            # Build system modules
│   │   ├── mod.rs
│   │   ├── cmake.rs
│   │   ├── xmake.rs
│   │   └── ...
│   └── deps/             # Dependency management
│       ├── mod.rs
│       └── resolve.rs
├── tests/                # Integration tests
├── docs/                 # Documentation (mdBook)
├── Cargo.toml            # Rust manifest
└── README.md
```

## Development Workflow

### 1. Create Feature Branch

```bash
git checkout -b feature/my-feature
```

### 2. Make Changes

Follow Rust best practices:

```bash
# Format code
cargo fmt

# Check for errors
cargo check

# Run clippy
cargo clippy
```

### 3. Add Tests

```rust
#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_my_feature() {
        // Test code
    }
}
```

### 4. Run Tests

```bash
# All tests
cargo test

# Specific test
cargo test test_my_feature

# With output
cargo test -- --nocapture
```

### 5. Commit and Push

```bash
git add .
git commit -m "feat: add my feature"
git push origin feature/my-feature
```

### 6. Open Pull Request

Create PR on GitHub with:
- Clear description
- Test results
- Documentation updates

## Testing

### Unit Tests

Located in each module:

```rust
// src/config.rs
#[cfg(test)]
mod tests {
    #[test]
    fn test_parse_config() {
        // ...
    }
}
```

Run:
```bash
cargo test --lib
```

### Integration Tests

Located in `tests/` directory:

```rust
// tests/integration_test.rs
#[tokio::test]
async fn test_project_creation() {
    // ...
}
```

Run:
```bash
cargo test --test integration_test
```

### Test Coverage

```bash
# Install tarpaulin
cargo install cargo-tarpaulin

# Generate coverage
cargo tarpaulin --out Html
```

## Debugging

### Enable Logging

```bash
export RUST_LOG=debug
cargo run -- build
```

### VS Code Debugging

`.vscode/launch.json`:
```json
{
  "version": "0.2.0",
  "configurations": [
    {
      "type": "lldb",
      "request": "launch",
      "name": "Debug Porters",
      "cargo": {
        "args": ["build", "--bin=porters"]
      },
      "args": ["build"],
      "cwd": "${workspaceFolder}"
    }
  ]
}
```

## Documentation

### Code Documentation

```rust
/// Adds a dependency to the project
///
/// # Arguments
///
/// * `name` - Dependency name
/// * `git` - Git repository URL
///
/// # Examples
///
/// ```
/// add_dependency("fmt", Some("https://github.com/fmtlib/fmt")).await?;
/// ```
pub async fn add_dependency(name: &str, git: Option<String>) -> Result<()> {
    // ...
}
```

Generate docs:
```bash
cargo doc --open
```

### User Documentation

Uses mdBook:

```bash
cd docs
mdbook serve --open
```

## Release Process

### 1. Update Version

`Cargo.toml`:
```toml
[package]
version = "0.2.0"
```

### 2. Update CHANGELOG

```markdown
## [0.2.0] - 2024-01-15

### Added
- Feature X
- Feature Y

### Fixed
- Bug Z
```

### 3. Create Git Tag

```bash
git tag -a v0.2.0 -m "Release v0.2.0"
git push origin v0.2.0
```

### 4. Publish to crates.io

```bash
cargo publish
```

### 5. GitHub Release

Create release on GitHub with:
- Tag: v0.2.0
- Title: "Porters v0.2.0"
- Description: Copy from CHANGELOG
- Attach binaries

## Performance Profiling

### CPU Profiling

```bash
# Install flamegraph
cargo install flamegraph

# Profile
cargo flamegraph --bin porters -- build
```

### Memory Profiling

```bash
# Install valgrind (Linux)
sudo apt install valgrind

# Profile
valgrind --leak-check=full cargo run -- build
```

## Benchmarking

```rust
#[bench]
fn bench_dependency_resolution(b: &mut Bencher) {
    b.iter(|| {
        // Code to benchmark
    });
}
```

Run:
```bash
cargo bench
```

## Continuous Integration

GitHub Actions workflow (`.github/workflows/ci.yml`):

```yaml
name: CI

on: [push, pull_request]

jobs:
  test:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v3
      - uses: actions-rs/toolchain@v1
        with:
          toolchain: stable
      - run: cargo test
      - run: cargo clippy
      - run: cargo fmt --check
```

## Next Steps

- [Contributing](./contributing.md)
- [Architecture](./architecture.md)
