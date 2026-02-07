src/
│
├── main.rs          → CLI entry point
│
├── analyzer/        → Analysis logic
│   ├── circular.rs  → Circular dependency detection
│   ├── duplicates.rs→ Duplicate version detection
│   └── mod.rs
│
├── graph/           → Dependency graph core
│   ├── builder.rs   → For making graph
│   ├── node.rs      → Single dependency node
│   ├── export.rs    → JSON / DOT output
│   └── mod.rs
│
├── parser/          → Cargo parsing
│   ├── cargo.rs     → Cargo.toml parsing
│   ├── metadata.rs  → cargo metadata parsing
│   └── mod.rs
