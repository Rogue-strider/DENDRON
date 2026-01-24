# DENDRON 🌳

<div align="center">
<p align="center">
<pre>
██████╗ ███████╗███╗   ██╗██████╗ ██████╗  ██████╗ ███╗   ██╗
██╔══██╗██╔════╝████╗  ██║██╔══██╗██╔══██╗██╔═══██╗████╗  ██║
██║  ██║█████╗  ██╔██╗ ██║██║  ██║██████╔╝██║   ██║██╔██╗ ██║
██║  ██║██╔══╝  ██║╚██╗██║██║  ██║██╔══██╗██║   ██║██║╚██╗██║
██████╔╝███████╗██║ ╚████║██████╔╝██║  ██║╚██████╔╝██║ ╚████║
╚═════╝ ╚══════╝╚═╝  ╚═══╝╚═════╝ ╚═╝  ╚═╝ ╚═════╝ ╚═╝  ╚═══╝
</pre>
</p>

**A powerful, blazing-fast dependency graph visualizer and analyzer for Rust projects**

[![Rust](https://img.shields.io/badge/rust-1.70%2B-orange.svg)](https://www.rust-lang.org)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![Build Status](https://img.shields.io/badge/build-passing-brightgreen.svg)](https://github.com/Rogue-strider/DENDRON)

[Features](#-features) • [Installation](#-installation) • [Usage](#-usage) • [Examples](#-examples) • [Contributing](#-contributing)

</div>

---

## ✨ Features

### Core Features
- 🌳 **Beautiful Tree Visualization** - Color-coded dependency trees with Unicode box drawing
- 🔍 **Nested Dependencies** - Analyze transitive dependencies using `cargo metadata`
- 📊 **Multiple View Modes**
  - Full dependency tree
  - Direct dependencies only
  - Depth-limited view
  - Compact summary view
- 📤 **Export Formats**
  - JSON (pretty & compact)
  - DOT (GraphViz compatible)
- 🎨 **Colorful Output** - Syntax highlighting with depth-based coloring
- ⚡ **Fast & Lightweight** - Written in pure Rust for maximum performance

### Analysis Features
- 🔄 **Circular Dependency Detection** - Identify dependency cycles that could cause issues
- 🔍 **Duplicate Version Detection** - Find packages with multiple versions to reduce bloat
- 📈 **Comprehensive Statistics** - Detailed metrics about your dependency graph

### Developer Features
- 🛠️ **CLI Friendly** - Extensive command-line options for automation
- 🚫 **CI/CD Ready** - Disable colors for pipeline integration
- 📁 **Multi-Project Support** - Analyze any Rust project by path

### Planned Features
- 🔒 Security vulnerability scanning (RustSec integration)
- 📜 License compatibility checker
- 📦 Update checker for outdated dependencies
- 📊 Dependency size analysis
- 🌐 Multi-language support (npm, pip, Maven)

---

## 🚀 Installation

### From Source (Current)
```bash
git clone https://github.com/Rogue-strider/DENDRON
cd DENDRON
cargo install --path .
```

### From crates.io (Coming Soon)
```bash
cargo install dendron-viz
```

### Prerequisites

- Rust 1.70 or higher
- Cargo

---

## 📖 Usage

### Basic Usage
```bash
# Analyze current project
dendron

# Analyze specific project
dendron --path /path/to/project

# Show nested dependencies
dendron --nested

# Limit depth
dendron --nested --depth 2
```

### View Modes
```bash
# Direct dependencies only
dendron --direct-only

# Compact summary
dendron --summary --nested

# Full tree with depth limit
dendron --nested --depth 3
```

### Analysis Commands
```bash
# Check for duplicate versions
dendron --nested --check-duplicates

# Check for circular dependencies
dendron --nested --check-circular

# Combined analysis (recommended for CI/CD)
dendron --nested --check-duplicates
dendron --nested --check-circular
```

### Export Options
```bash
# Export to JSON (pretty)
dendron --nested --output json > dependencies.json

# Export to JSON (compact)
dendron --output json-compact > deps.json

# Export to DOT format (GraphViz)
dendron --nested --output dot > deps.dot

# Visualize with GraphViz
dendron --output dot > deps.dot && dot -Tpng deps.dot -o graph.png
```

### Advanced Options
```bash
# Disable colors (for CI/CD)
dendron --no-color

# Combine options
dendron --nested --depth 3 --summary --no-color
```

---

## 📋 Examples

### Example Output (Tree View)
```
🌳 Dependency Tree (Max Depth: 2)

📦 dendron (0.1.0)
├── anyhow (1.0.100)
├── cargo_metadata (0.18.1)
│   ├── camino (1.2.2)
│   ├── cargo-platform (0.1.9)
│   ├── semver (1.0.27)
│   └── serde (1.0.228)
├── clap (4.5.54)
│   ├── clap_builder (4.5.54)
│   └── clap_derive (4.5.49)
├── colored (2.2.0)
├── serde (1.0.228)
└── toml (0.8.23)

📊 Statistics:
  ├─ Direct Dependencies: 6
  ├─ Total Dependencies: 46
  └─ Max Depth: 6

✨ Analysis complete!
```

### Example Output (Summary View)
```
📋 Dependency Summary:

📦 dendron (0.1.0)

└─ Level 1: 6 packages (showing first 10)
   ├─ toml (0.8.23)
   ├─ serde (1.0.228)
   ├─ anyhow (1.0.100)
   ├─ clap (4.5.54)
   ├─ colored (2.2.0)
   └─ cargo_metadata (0.18.1)

└─ Level 2: 11 packages (showing first 10)
   ├─ semver (1.0.27)
   ├─ camino (1.2.2)
   ├─ cargo-platform (0.1.9)
   └─ 8 more packages...
```

### Example Output (Duplicate Detection)
```
⚠️ Found 3 packages with multiple versions:

📦 proc-macro2
  ├─ v1.0.92 (used 4 times)
  └─ v1.0.70 (used 2 times)

📦 syn
  ├─ v2.0.90 (used 5 times)
  └─ v1.0.109 (used 3 times)

📦 unicode-ident
  ├─ v1.0.22 (used 3 times)
  └─ v1.0.14 (used 1 time)

💡 Impact:
   • 3 duplicate versions increase binary size
   • Consider updating dependencies to use consistent versions
   • Run cargo update to update

📊 Duplicate Statistics:
  ├─ Packages with duplicates: 3
  ├─ Total versions: 6
  └─ Duplicate versions: 3
```

### Example Output (Circular Dependency Detection)
```
⚠️ Found 2 circular dependencies:

Cycle 1:
  ┌─→ package-a
  ├─→ package-b
  ├─→ package-c
  └─→ package-a (completes cycle)

Cycle 2:
  ┌─→ package-x
  ├─→ package-y
  └─→ package-x (completes cycle)

💡 Impact:
   • Circular dependencies can cause compilation issues
   • They may increase build time and binary size
   • Consider refactoring to break the cycles

📊 Circular Dependency Statistics:
  ├─ Total cycles: 2
  ├─ Packages involved: 5
  └─ Longest cycle: 3 packages
```

### Example JSON Export
```json
{
  "package": {
    "name": "dendron",
    "version": "0.1.0"
  },
  "dependencies": {
    "toml": {
      "version": "0.8.23",
      "depth": 1,
      "dependencies": ["serde", "indexmap", "toml_datetime"]
    },
    "serde": {
      "version": "1.0.228",
      "depth": 1,
      "dependencies": ["serde_derive"]
    }
  },
  "statistics": {
    "direct_dependencies": 6,
    "total_dependencies": 46,
    "max_depth": 6
  }
}
```

---

## 🎯 Command Reference
```bash
dendron [OPTIONS]

Options:
  -p, --path <PATH>           Path to project directory (default: current)
  -d, --depth <NUMBER>        Maximum depth to display
  -D, --direct-only           Show only direct dependencies
  -n, --nested                Show nested/transitive dependencies
  -s, --summary               Show compact summary view
  -o, --output <FORMAT>       Output format: json, json-compact, dot
      --check-duplicates      Check for duplicate dependency versions
      --check-circular        Check for circular dependencies
      --no-color              Disable colored output
  -h, --help                  Print help
  -V, --version               Print version
```

---

## 💡 Use Cases

### For Developers
- **Understand Dependencies**: Visualize your project's dependency tree
- **Optimize Build Size**: Find and eliminate duplicate versions
- **Detect Issues**: Identify circular dependencies before they cause problems
- **Documentation**: Export dependency graphs for documentation

### For CI/CD Pipelines
```bash
# In your CI/CD pipeline
dendron --nested --check-duplicates --no-color
dendron --nested --check-circular --no-color

# Export for artifacts
dendron --nested --output json > artifacts/dependencies.json
```

### For Team Collaboration
```bash
# Generate visual graph
dendron --output dot > deps.dot
dot -Tpng deps.dot -o dependency-graph.png

# Share with team for review
```

---

## 🛠️ Development

### Building from Source
```bash
# Clone the repository
git clone https://github.com/Rogue-strider/DENDRON
cd DENDRON

# Build the project
cargo build --release

# Run tests
cargo test

# Run locally
cargo run -- --nested --depth 2
```

### Project Structure
```
dendron/
├── src/
│   ├── main.rs           # CLI entry point
│   ├── parser/
│   │   ├── mod.rs
│   │   ├── cargo.rs      # Cargo.toml parser
│   │   └── metadata.rs   # cargo metadata parser
│   ├── graph/
│   │   ├── mod.rs
│   │   ├── node.rs       # Dependency node structure
│   │   ├── builder.rs    # Graph building logic
│   │   └── export.rs     # Export formats (JSON, DOT)
│   └── analyzer/
│       ├── mod.rs
│       ├── duplicates.rs # Duplicate version detection
│       └── circular.rs   # Circular dependency detection
├── Cargo.toml
└── README.md
```

### Tech Stack

- **Language**: Rust 2021 Edition
- **CLI Framework**: clap 4.x
- **Serialization**: serde, serde_json
- **Parsing**: toml, cargo_metadata
- **Colors**: colored

---

## 🤝 Contributing

Contributions are welcome! Here's how you can help:

1. Fork the repository
2. Create a feature branch (`git checkout -b feature/amazing-feature`)
3. Commit your changes (`git commit -m 'Add amazing feature'`)
4. Push to the branch (`git push origin feature/amazing-feature`)
5. Open a Pull Request

### Areas for Contribution

- 🔒 Security vulnerability scanning (RustSec integration)
- 📜 License compatibility checker
- 📦 Update checker for outdated dependencies
- 📊 Dependency size analysis
- 🌐 Multi-language support (npm, pip, etc.)
- 📚 Documentation improvements
- 🐛 Bug fixes
- ✨ New features

---

## 📝 License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.
![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)

---

## 🙏 Acknowledgments

- Inspired by tools like `cargo tree`, `npm ls`, and various dependency visualization tools
- Built with amazing Rust crates from the community
- Special thanks to all contributors

---

## 📞 Contact

**Author**: Rogue-strider  
**Repository**: [github.com/Rogue-strider/DENDRON](https://github.com/Rogue-strider/DENDRON)  
**Email**: satyamjha91064@gmail.com

---

<div align="center">

Made with ❤️ and 🦀 Rust

⭐ Star this repo if you find it useful!

</div>
