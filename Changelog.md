# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added

- Support for unit enum variants
- Support for variadic tuple enum variants
- Support for orphan struct enum variants
- Added the `TaggedBox::dangling` and `TaggedBox::dangling_unchecked` methods
- Added the `TaggedBox::new_unchecked` method
- Added support for more reserved pointer widths (58bits-63bits)

### Changed

- Split up `tagged_box!` into smaller component macros
- Modified tests to cover variadic tuple, unit and orphan enum variants
- Changed the default reserved width from 48bits to 60bits

## [0.1.1] - 2020-03-07

### Fixed

- Links and badges in README and src/lib.rs

## 0.1.0 - 2020-03-07

### Added

- `TaggedBox`
- `TaggedPointer`
- `tagged_box!` macro with support for single-element tuple enums

[Unreleased]: https://github.com/Kixiron/tagged-box/compare/v0.1.1...HEAD
[0.1.1]: https://github.com/Kixiron/tagged-box/compare/v0.1.1
