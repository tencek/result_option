# Contributing to ResultOption

Thank you for your interest in contributing to ResultOption! This document outlines the development and release process.

## Development Workflow

1. **Make changes** to the code
2. **Add tests** for new functionality
3. **Update documentation** - ensure all public items have doc comments
4. **Add changelog entry** to the `[Unreleased]` section in `CHANGELOG.md`
5. **Run tests**: `cargo test`
6. **Check docs**: `cargo doc --open`
7. **Commit changes** with descriptive messages

## Release Workflow

Follow this complete workflow when releasing a new version:

### 1. Prepare the Release

```bash
# Ensure you're on main and up to date
git checkout main
git pull origin main

# Run final checks
cargo test
cargo doc
cargo clippy
```

### 2. Update Version and Changelog

1. **Update `Cargo.toml`**:

   ```toml
   [package]
   version = "0.1.2"  # Increment according to semver
   ```

2. **Update `CHANGELOG.md`**:
   - Move items from `[Unreleased]` to new version section
   - Add release date
   - Add new empty `[Unreleased]` section
   - Update comparison links at bottom

### 3. Commit Version Changes

```bash
git add Cargo.toml CHANGELOG.md
git commit -m "Release v0.1.2"
```

### 4. Create and Push Git Tag

```bash
# Create annotated tag
git tag -a v0.1.2 -m "Release v0.1.2 - Brief description of changes"

# Push commit and tag
git push origin main
git push origin --tags
```

### 5. Publish to Crates.io

```bash
# Dry run first (optional but recommended)
cargo publish --dry-run

# Publish to crates.io
cargo publish
```

### 6. Create GitHub Release (Optional but Recommended)

Go to [GitHub Releases](https://github.com/tencek/result_option/releases) and:

1. Click "Create a new release"
2. Select the tag you just pushed (v0.1.2)
3. Set release title: "Release v0.1.2"
4. Copy the changelog content for this version into the description
5. Click "Publish release"

**Example release description template:**

```markdown
## Added
- New feature descriptions from changelog

## Changed  
- Changes from changelog

## Fixed
- Bug fixes from changelog

---

**Links:**
- [Crates.io](https://crates.io/crates/result_option)
- [Documentation](https://docs.rs/result_option/0.1.2)
- [Changelog](https://github.com/tencek/result_option/blob/main/CHANGELOG.md)
```

## Versioning Guidelines

This project follows [Semantic Versioning](https://semver.org/):

- **MAJOR** (1.0.0): Breaking changes to public API
- **MINOR** (0.1.0): New features, backwards compatible
- **PATCH** (0.0.1): Bug fixes, backwards compatible

## Pre-release Checklist

Before releasing, ensure:

- [ ] All tests pass (`cargo test`)
- [ ] Documentation builds (`cargo doc`)
- [ ] No clippy warnings (`cargo clippy`)
- [ ] CHANGELOG.md is updated
- [ ] Version in Cargo.toml is updated
- [ ] All public items have documentation
- [ ] Breaking changes are clearly documented

## GitHub Actions (Future)

Consider setting up automated workflows for:

- Running tests on PRs
- Automatically creating GitHub releases when tags are pushed
- Publishing to crates.io on tag push
- Generating release notes from changelog

## Questions?

If you have questions about the release process or contributing, please open an issue!
