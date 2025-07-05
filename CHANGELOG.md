# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.1.1] - 2024-12-XX

### Added
- Comprehensive documentation and code comments for all modules
- Better error handling using `anyhow` crate for more informative error messages
- XDG Base Directory specification support for config directory location
- Separate `models.rs` module for better code organization
- SQL query constants for better maintainability
- Detailed function documentation with examples
- Development dependencies for testing (`tempfile`)
- Optimized release build configuration

### Changed
- **BREAKING**: Improved error handling - functions now return `Result<T>` instead of calling `exit(1)`
- Refactored database operations into separate `models.rs` module
- Better separation of concerns between database operations and SQL queries
- Improved config directory handling with XDG compliance
- Enhanced command execution flow with proper error propagation
- Updated task listing to show boolean values as "true"/"false" instead of "1"/"0"
- More robust date parsing with proper error handling

### Fixed
- Fixed inconsistent error handling throughout the application
- Resolved potential panic situations with proper error propagation
- Better handling of corrupted database timestamps
- Improved user feedback for non-existent tasks in remove/done operations

### Dependencies
- Added `anyhow` 1.0.89 for better error handling
- Updated `chrono` to include serde features for future extensibility
- Added `tempfile` 3.8 as dev dependency for testing

### Documentation
- Added comprehensive rustdoc documentation for all public functions
- Included usage examples in function documentation
- Added module-level documentation explaining purpose and functionality
- Updated README with more detailed usage instructions

## [0.1.0] - 2024-XX-XX

### Added
- Initial release of todo-rs CLI application
- Basic task management functionality:
  - Add new tasks with descriptions
  - List all tasks with their status and creation time
  - Mark tasks as complete
  - Remove tasks by ID
- SQLite database storage in user's config directory
- Command-line interface using clap
- User confirmation prompts for database creation
- Basic error handling and user feedback

### Dependencies
- `chrono` 0.4.41 for timestamp handling
- `clap` 4.3 with derive features for CLI parsing
- `rusqlite` 0.36.0 for SQLite database operations

[Unreleased]: https://github.com/0xSickb0y/todo-rs/compare/v0.1.1...HEAD
[0.1.1]: https://github.com/0xSickb0y/todo-rs/compare/v0.1.0...v0.1.1
[0.1.0]: https://github.com/0xSickb0y/todo-rs/releases/tag/v0.1.0