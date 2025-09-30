# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added

- `From<Option<&T>>` implementation for converting `Option<&T>` to `ResultOption<&T, E>`

## [0.2.0] - 2025-09-29

### Added

- `From<Option<T>>` implementation for converting `Option<T>` to `ResultOption<T, E>`
- CHANGELOG.md
- CONTRIBUTING.md

### Removed

- Cargo.lock from repository

## [0.1.1] - 2025-09-20

### Added

- README.md

## [0.1.0] - 2025-09-20

### Added

- Initial project setup
- Basic project structure
- Initial implementation of `ResultOption<T, E>` enum
- Core methods: `is_ok()`, `is_none()`, `is_err()`, `is_ok_and()`, `is_err_and()`
- Conversion methods: `ok()`, `err()`, `as_ref()`, `as_mut()`
- Mapping methods: `map()`, `map_or()`, `map_or_else()`, `map_or_default()`, `map_err()`
- Utility methods: `inspect()`
- `From<Result<Option<T>, E>>` implementation
- Documentation enforcement with `#![deny(missing_docs)]`

#### Sections

(Changed|Deprecated|Removed|Fixed|Security)

[Unreleased]: https://github.com/tencek/result_option/compare/v0.2.0...HEAD
[0.2.0]: https://github.com/tencek/result_option/releases/tag/v0.2.0
[0.1.1]: https://github.com/tencek/result_option/releases/tag/v0.1.1
[0.1.0]: https://github.com/tencek/result_option/releases/tag/v0.1.0
