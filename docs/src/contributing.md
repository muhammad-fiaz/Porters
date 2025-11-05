# Contributing

Thank you for your interest in contributing to Porters!

## Ways to Contribute

- 🐛 Report bugs
- 💡 Suggest features
- 📝 Improve documentation
- 🔧 Submit code fixes
- ✨ Add new features

## Getting Started

### 1. Fork and Clone

```bash
git clone https://github.com/YOUR_USERNAME/Porters.git
cd Porters
```

### 2. Create a Branch

```bash
git checkout -b feature/my-awesome-feature
```

### 3. Build and Test

```bash
cargo build
cargo test
cargo run -- --version
```

## Development Guidelines

### Code Style

Follow Rust conventions:
```bash
cargo fmt
cargo clippy
```

### Commit Messages

Use conventional commits:
```
feat: add support for Conan packages
fix: resolve dependency version conflicts
docs: update installation guide
test: add tests for lock file generation
```

### Pull Requests

1. Ensure all tests pass
2. Update documentation if needed
3. Add tests for new features
4. Follow existing code style

## Project Structure

```
Porters/
├── src/
│   ├── main.rs          # CLI entry point
│   ├── config.rs        # Configuration handling
│   ├── scan.rs          # Project scanning
│   ├── build/           # Build system integration
│   ├── deps/            # Dependency management
│   ├── global.rs        # Global configuration
│   ├── lockfile.rs      # Lock file management
│   └── util.rs          # Utilities
├── docs/                # Documentation (mdBook)
├── Cargo.toml           # Rust manifest
└── README.md
```

## Testing

```bash
# Run all tests
cargo test

# Run specific test
cargo test test_name

# Integration tests
cargo test --test integration_tests
```

## Documentation

Documentation uses mdBook:

```bash
# Install mdBook
cargo install mdbook

# Serve locally
cd docs
mdbook serve

# Build
mdbook build
```

## Need Help?

- Open an issue for questions
- Join discussions on GitHub
- Check existing issues before reporting

## License

By contributing, you agree that your contributions will be licensed under the Apache License 2.0.
