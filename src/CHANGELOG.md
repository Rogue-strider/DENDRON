# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.1.1] - 2026-01-25

### Fixed
- 📝 Updated README with correct installation command
- 🔧 Fixed package name references from `dendron` to `dendron-viz`
- 📦 Updated crates.io installation instructions

### Changed
- Package name changed from `dendron` to `dendron-viz` (name conflict on crates.io)

### Documentation
- Fixed installation command: `cargo install dendron-viz`
- Updated all documentation to reflect correct package name

## [0.1.0] - 2026-01-24

### Added
- 🌳 Beautiful tree visualization with color-coded output
- 🔍 Nested dependency analysis using cargo metadata
- 📊 Multiple view modes (tree, direct-only, summary, depth-limited)
- 📤 Export formats (JSON pretty, JSON compact, DOT/GraphViz)
- 🔄 Circular dependency detection with cycle reporting
- 🔍 Duplicate version detection with comprehensive statistics
- 📈 Comprehensive dependency statistics and metrics
- 🎨 Syntax highlighting with depth-based coloring
- 🛠️ Extensive CLI options for automation
- 🚫 No-color mode for CI/CD pipeline integration
- 📁 Multi-project support via --path flag
- ⚡ Fast and lightweight implementation in pure Rust

### Features
- Parse Cargo.toml and cargo metadata
- Build complete dependency graph with transitive dependencies
- Detect and report circular dependencies with detailed cycle paths
- Identify duplicate package versions across the dependency tree
- Generate visual dependency trees with Unicode box drawing
- Export dependency data to JSON (pretty and compact) formats
- Export to DOT format for GraphViz visualization
- Professional command-line interface with clap
- Comprehensive error handling with helpful messages

### Documentation
- Complete README with usage examples and screenshots
- HTML documentation for web viewing
- MIT License
- Contribution guidelines
- Command reference guide

### Technical Details
- Built with Rust 2021 Edition
- Uses clap 4.x for CLI
- Integrates cargo_metadata for dependency resolution
- Employs serde for serialization
- Implements colored output with the colored crate
- Total: ~1,087 lines of code
- Supports Rust 1.70+

## [Unreleased]

### Planned
- 🔒 Security vulnerability scanning (RustSec integration)
- 📜 License compatibility checker for legal compliance
- 📦 Update checker for outdated dependencies
- 📊 Dependency size analysis to optimize binary size
- 🌐 Multi-language support (npm, pip, Maven, Gradle)
- 🖥️ Interactive TUI mode for better user experience
- 🎯 GitHub Actions integration examples
- 📸 Better error messages and user guidance
- 🧪 Expanded test coverage
- 🚀 Performance optimizations for large projects

---

[0.1.1]: https://github.com/Rogue-strider/DENDRON/compare/v0.1.0...v0.1.1
[0.1.0]: https://github.com/Rogue-strider/DENDRON/releases/tag/v0.1.0