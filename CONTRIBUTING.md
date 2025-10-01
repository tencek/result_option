# Contributing to ResultOption

Thank you for your interest in contributing to ResultOption! This document outlines
the development and release process.

## Development Workflow

1. **Make changes** to the code
2. **Add tests** for new functionality
3. **Update documentation** - ensure all public items have doc comments
4. **Add changelog entry** to the `[Unreleased]` section in `CHANGELOG.md`
5. **Run tests**: `cargo test`
6. **Check docs**: `cargo doc --open`
7. **Commit changes** with descriptive messages

## Testing the Features

It's crucial to test all possible feature
combinations to ensure the code compiles and works correctly in all scenarios.
This crate currently has the following features:

- `unwrap_infallible` (enabled by default)

### Feature Testing Strategy

#### 1. **Test All Feature Combinations**

For `n` features, there are `2^n` possible combinations. Currently with 1 feature,
we have:

```bash
# Test with all default features (unwrap_infallible enabled)
cargo test

# Test with no features
cargo test --no-default-features

# Test with specific features enabled
cargo test --no-default-features --features unwrap_infallible
```

#### 2. **Compilation Tests**

Ensure the code compiles in all configurations:

```bash
# Check compilation with default features
cargo check

# Check compilation without any features  
cargo check --no-default-features

# Check compilation with specific features
cargo check --no-default-features --features unwrap_infallible
```

#### 3. **Documentation Tests**

Test that documentation examples work across feature configurations:

```bash
# Test docs with default features
cargo test --doc

# Test docs without features (should exclude feature-gated examples)
cargo test --doc --no-default-features
```

#### 4. **Automated Feature Testing**

Consider using tools like:

- **[`cargo-hack`](https://github.com/taiki-e/cargo-hack)**: Tests all feature combinations automatically

  ```bash
  # Install cargo-hack
  cargo install cargo-hack
  
  # Test all feature combinations
  cargo hack test --feature-powerset
  
  # Check all feature combinations compile
  cargo hack check --feature-powerset
  ```

#### 5. **Feature Testing Checklist**

Before releasing, verify:

- [ ] Code compiles with all default features: `cargo check`
- [ ] Code compiles with no features: `cargo check --no-default-features`  
- [ ] All tests pass with default features: `cargo test`
- [ ] All tests pass without optional features: `cargo test --no-default-features`
- [ ] Documentation builds correctly: `cargo doc --no-default-features` and `cargo doc`
- [ ] Feature-gated code is properly conditional with `#[cfg(feature = "...")]`
- [ ] Dependencies are properly marked as optional when they're only needed for specific features

#### 6. **Feature Documentation**

When adding new optional features:

1. **Document in `Cargo.toml`**: Add clear feature descriptions
2. **Update README**: Explain what each feature does and how to enable/disable it
3. **Use conditional compilation**: Properly gate code with `#[cfg(feature = "...")]`
4. **Test examples**: Ensure documentation examples work with and without the feature

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
