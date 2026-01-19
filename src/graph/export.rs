use super::{DependencyGraph, GraphStats};
use serde::{Serialize, Deserialize};
use std::collections::HashMap;
use anyhow::Result;

#[derive(Debug, Serialize, Deserialize)]
pub struct JsonOutput {
    pub package: PackageJson,
    pub dependencies: HashMap<String, DependencyJson>,
    pub statistics: StatsJson,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PackageJson {
    pub name: String,
    pub version: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DependencyJson {
    pub version: String,
    pub depth: usize,
    pub dependencies: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StatsJson {
    pub direct_dependencies: usize,
    pub total_dependencies: usize,
    pub max_depth: usize,
}

impl DependencyGraph {
    pub fn to_json(&self) -> Result<String> {
        let mut dependencies = HashMap::new();

        for (name, node) in &self.nodes {
            dependencies.insert(
                name.clone(),
                DependencyJson {
                    version: node.version.clone(),
                    depth: node.depth,
                    dependencies: node.dependencies.clone(),
                },
            );
        }

        let stats = self.stats();

        let output = JsonOutput {
            package: PackageJson {
                name: self.root.name.clone(),
                version: self.root.version.clone(),
            },
            dependencies,
            statistics: StatsJson {
                direct_dependencies: stats.direct_dependencies,
                total_dependencies: stats.total_dependencies,
                max_depth: stats.max_depth,
            },
        };

        let json = serde_json::to_string_pretty(&output)?;
        Ok(json)
    }

    pub fn to_json_compact(&self) -> Result<String> {
        let mut dependencies = HashMap::new();

        for (name, node) in &self.nodes {
            dependencies.insert(
                name.clone(),
                DependencyJson {
                    version: node.version.clone(),
                    depth: node.depth,
                    dependencies: node.dependencies.clone(),
                },
            );
        }

        let stats = self.stats();

        let output = JsonOutput {
            package: PackageJson {
                name: self.root.name.clone(),
                version: self.root.version.clone(),
            },
            dependencies,
            statistics: StatsJson {
                direct_dependencies: stats.direct_dependencies,
                total_dependencies: stats.total_dependencies,
                max_depth: stats.max_depth,
            },
        };

        let json = serde_json::to_string(&output)?;
        Ok(json)
    }

    pub fn to_dot(&self) -> String {
        let mut dot = String::from("digraph dependencies {\n");
        dot.push_str("  rankdir=LR;\n");
        dot.push_str("  node [shape=box, style=rounded];\n\n");

        // Root node
        dot.push_str(&format!(
            "  \"{}\" [label=\"{}\\n{}\", style=\"rounded,filled\", fillcolor=lightgreen];\n",
            self.root.name, self.root.name, self.root.version
        ));

        // All dependencies
        for (name, node) in &self.nodes {
            let color = match node.depth {
                1 => "lightblue",
                2 => "lightyellow",
                _ => "white",
            };

            dot.push_str(&format!(
                "  \"{}\" [label=\"{}\\n{}\", fillcolor={}, style=filled];\n",
                name, name, node.version, color
            ));

            // Edges
            for dep in &node.dependencies {
                dot.push_str(&format!("  \"{}\" -> \"{}\";\n", name, dep));
            }
        }

        // Root edges
        for dep in &self.root.dependencies {
            dot.push_str(&format!("  \"{}\" -> \"{}\";\n", self.root.name, dep));
        }

        dot.push_str("}\n");
        dot
    }
}