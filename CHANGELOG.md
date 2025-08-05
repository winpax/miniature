# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Changed

- Add 32bit support

## [0.1.3]

### Fixed

- Free console when opening Windows (GUI) app
- Modify subsystem of shim if needed

### Changed

- Added release script
- Support aarch64 architecture

## [0.1.2]

### Fixed

- Exit handling

### Changed

- Refactored error handling

## [0.1.1]

### Changed

- Static link and removed references to CRT

## [0.1.0]

### Changed

- Got miniature shim working entirely and consistently
- Added numin program for creating new shims
- Removed static link sections in favor of using windows resource string tables

[Unreleased]: https://github.com/winpax/miniature/compare/v0.1.3...HEAD
[0.1.3]: https://github.com/winpax/miniature/releases/tag/v0.1.3
[0.1.2]: https://github.com/winpax/miniature/releases/tag/v0.1.2
[0.1.1]: https://github.com/winpax/miniature/releases/tag/v0.1.1
[0.1.0]: https://github.com/winpax/miniature/releases/tag/v0.1.0
