# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.1.0] - 2026-01-24

### Added
- 🌳 Beautiful tree visualization with color-coded output
- 🔍 Nested dependency analysis using cargo metadata
- 📊 Multiple view modes (tree, direct-only, summary, depth-limited)
- 📤 Export formats (JSON pretty, JSON compact, DOT/GraphViz)
- 🔄 Circular dependency detection
- 🔍 Duplicate version detection with statistics
- 📈 Comprehensive dependency statistics
- 🎨 Syntax highlighting with depth-based coloring
- 🛠️ Extensive CLI options for automation
- 🚫 No-color mode for CI/CD pipelines
- 📁 Multi-project support via --path flag

### Features
- Parse Cargo.toml and cargo metadata
- Build complete dependency graph
- Detect and report circular dependencies
- Identify duplicate package versions
- Generate visual dependency trees
- Export to JSON and DOT formats
- Professional command-line interface
- Comprehensive error handling

### Documentation
- Complete README with examples
- HTML documentation
- MIT License
- Contribution guidelines

## [Unreleased]

### Planned
- Security vulnerability scanning (RustSec integration)
- License compatibility checker
- Update checker for outdated dependencies
- Dependency size analysis
- Multi-language support (npm, pip, Maven)
- Interactive TUI mode
- GitHub Actions integration examples

---

[0.1.0]: https://github.com/Rogue-strider/DENDRON/releases/tag/v0.1.0