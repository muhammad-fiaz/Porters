# Publishing

Publish your C/C++ projects to GitHub releases.

## Prerequisites

1. GitHub repository configured in `porters.toml`:
```toml
[project]
repository = "https://github.com/username/project"
```

2. GitHub Personal Access Token:
   - Create at [github.com/settings/tokens](https://github.com/settings/tokens)
   - Required scopes: `repo`

## Publishing a Release

```bash
porters publish --version 1.0.0
```

With token:
```bash
porters publish --version 1.0.0 --token ghp_xxxxx
```

Or set environment variable:
```bash
export GITHUB_TOKEN=ghp_xxxxx
porters publish --version 1.0.0
```

## What Gets Published

1. Creates Git tag (e.g., `v1.0.0`)
2. Builds release binaries
3. Creates GitHub release
4. Uploads artifacts:
   - Source tarball
   - Platform-specific binaries
   - `porters.toml`

## Version Management

### Semantic Versioning

Follow [SemVer](https://semver.org/):

- **Major** (1.0.0): Breaking changes
- **Minor** (0.1.0): New features, backward-compatible
- **Patch** (0.0.1): Bug fixes

### Updating Version

Update in `porters.toml`:
```toml
[project]
version = "1.2.3"
```

## Next Steps

- [Configuration](./configuration.md)
- [Command Reference](./commands.md)
