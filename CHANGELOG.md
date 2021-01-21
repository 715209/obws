<!-- markdownlint-disable MD024 -->

# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased] - ReleaseDate

## [0.3.0] - 2021-01-11

### Added

- Github actions added to run automated tests and lints on each commit.
- Unit tests for all custom (de)serializers ensuring the right in-/output.
- More integration tests, covering almost all requests now.
- Added `.editorconfig` file to allow consistent indenting and other common editor settings.
- Added a `Justfile` (similar to a `Makefile`) that allows to run common tasks and especially code
  coverage conveniently.
- A new `connect_with_config` function that allows to customize the client behavior.
- Support for TLS connections.

### Changed

- Update readme to show the current code coverage.
- Upgraded to `tokio` **1.0** and all related dependencies.

### Fixed

- Corrected a few links in the API docs.
- A required tokio feature was missing as the dev dependencies added it.

## [0.2.0]

### Added

- Most request types implement `Default` for easier request creation.
- Many integration tests.
- Basic usage details in the docs and readme.

### Changed

- Parse into more concrete types where possible. For example durations and timestamps are
  represented as types from the `chrono` crate instead of strings and integers now.
- Errors are specific now, using `thiserror` instead of `anyhow` allowing to match against the error
  and find out what exactly went wrong on a type level.

### Fixed

- Various small fixes in request and response types that were found while creating the integration
  tests.
- Some links in the API docs were broken, pointing to private items.

## [0.1.0]

### Added

- Initial release.

[Unreleased]: https://github.com/dnaka91/obws/compare/v0.3.0...HEAD
[0.3.0]: https://github.com/dnaka91/obws/compare/v0.2.0...v0.3.0
[0.2.0]: https://github.com/dnaka91/obws/compare/v0.1.0...v0.2.0
[0.1.0]: https://github.com/dnaka91/obws/releases/tag/v0.1.0