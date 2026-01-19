use crate::parser::{CargoManifest, CargoParser};
use super::node::DependencyNode;
use std::collections::HashMap;
use colored::*;  
pub struct DependencyGraph {
    pub root: DependencyNode,
    pub nodes: HashMap<String, DependencyNode>,
}

impl DependencyGraph {
    pub fn from_manifest(manifest: &CargoManifest) -> Self {
        let root_name = manifest.package.name.clone();
        let root_version = manifest.package.version.clone();
        
        let mut root = DependencyNode::new(root_name.clone(), root_version);
        let mut nodes = HashMap::new();

        for (dep_name, dep) in &manifest.dependencies {
            root.add_dependency(dep_name.clone());
            
            let version = CargoParser::get_version(dep)
                .unwrap_or_else(|| "unknown".to_string());
            
            let dep_node = DependencyNode::new(dep_name.clone(), version)
                .with_depth(1);
            
            nodes.insert(dep_name.clone(), dep_node);
        }

        Self { root, nodes }
    }

    pub fn print_tree(&self) {
        println!("{}", "🌳 Dependency Tree:".bright_cyan().bold());
        println!();
        self.print_node(&self.root, "", true);
    }

    fn print_node(&self, node: &DependencyNode, prefix: &str, is_last: bool) {
        let connector = if is_last { "└── " } else { "├── " };
        let extension = if is_last { "    " } else { "│   " };

        if node.depth == 0 {
            // Root node - Bright green with bold
            println!(
                "{} {} {}",
                "📦".bright_yellow(),
                node.name.bright_green().bold(),
                format!("({})", node.version).bright_black()
            );
        } else {
            // Child dependencies - Different colors based on depth
            let name_colored = match node.depth {
                1 => node.name.bright_blue(),
                2 => node.name.bright_magenta(),
                _ => node.name.bright_cyan(),
            };
            
            println!(
                "{}{}{} {}",
                prefix.bright_black(),
                connector.bright_black(),
                name_colored.bold(),
                format!("({})", node.version).bright_black()
            );
        }

        let child_prefix = format!("{}{}", prefix, extension);
        let dep_count = node.dependencies.len();

        for (i, dep_name) in node.dependencies.iter().enumerate() {
            let is_last_child = i == dep_count - 1;
            
            if let Some(child_node) = self.nodes.get(dep_name) {
                self.print_node(child_node, &child_prefix, is_last_child);
            } else {
                let child_connector = if is_last_child { "└── " } else { "├── " };
                println!(
                    "{}{}{} {}",
                    child_prefix.bright_black(),
                    child_connector.bright_black(),
                    dep_name.red(),
                    "(?)".bright_black()
                );
            }
        }
    }

    pub fn stats(&self) -> GraphStats {
        GraphStats {
            total_dependencies: self.nodes.len(),
            direct_dependencies: self.root.dependencies.len(),
            max_depth: self.calculate_max_depth(),
        }
    }

    fn calculate_max_depth(&self) -> usize {
        self.nodes.values()
            .map(|node| node.depth)
            .max()
            .unwrap_or(0)
    }
}

#[derive(Debug)]
pub struct GraphStats {
    pub total_dependencies: usize,
    pub direct_dependencies: usize,
    pub max_depth: usize,
}

impl GraphStats {
    pub fn print(&self) {
        println!();
        println!("{}", "📊 Statistics:".bright_cyan().bold());
        println!(
            "  {} Direct Dependencies: {}",
            "├─".bright_black(),
            self.direct_dependencies.to_string().bright_green().bold()
        );
        println!(
            "  {} Total Dependencies: {}",
            "├─".bright_black(),
            self.total_dependencies.to_string().bright_yellow().bold()
        );
        println!(
            "  {} Max Depth: {}",
            "└─".bright_black(),
            self.max_depth.to_string().bright_blue().bold()
        );
    }
}
