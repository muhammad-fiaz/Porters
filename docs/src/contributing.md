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
```text
feat: add support for Conan packages
fix: resolve dependency version conflicts
docs: update installation guide
test: add tests for lock file generation
```

### Pull Requests

When submitting a pull request:

1. **Use the PR Template** - GitHub automatically loads `.github/pull_request_template.md`
   - Complete all relevant sections (type of change, related issues, description)
   - Check off all applicable items in the checklists
   - Document breaking changes with migration guide if applicable
   
2. **Ensure all tests pass** - Run `cargo test` locally before pushing

3. **Update documentation** - Any new features or changes must update `docs/src/`

4. **Add tests for new features** - Maintain or improve code coverage

5. **Follow existing code style** - Run `cargo fmt` and `cargo clippy`

6. **Build system verification** - Test with all supported build systems (CMake, XMake, Meson, Make)

**PR Template Sections:**
- Type of change (bug fix, feature, breaking change, docs, refactor, etc.)
- Related issues
- Test configuration and steps
- Code quality checklist (clippy, fmt, self-review)
- Testing checklist (unit tests, integration tests, manual testing)
- Documentation checklist (commands.md, configuration.md, README, CHANGELOG)
- Breaking changes migration guide
- Performance impact assessment

See `.github/pull_request_template.md` for the complete checklist.

## Project Structure

```text
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
