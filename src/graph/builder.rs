use crate::parser::{CargoManifest, CargoParser, MetadataParser, PackageInfo};
use super::node::DependencyNode;
use std::collections::{HashMap, HashSet};
use std::path::Path;
use colored::*;

pub struct DependencyGraph {
    pub root: DependencyNode,
    pub nodes: HashMap<String, DependencyNode>,
}

impl DependencyGraph {
    pub fn print_summary(&self) {
        println!("{}", "📋 Dependency Summary:".bright_cyan().bold());
        println!();
        
        println!(
            "{} {} {}",
            "📦".bright_yellow(),
            self.root.name.bright_green().bold(),
            format!("({})", self.root.version).bright_black()
        );
        println!();
        
        // Group by depth
        let mut by_depth: std::collections::HashMap<usize, Vec<&DependencyNode>> = std::collections::HashMap::new();
        
        for node in self.nodes.values() {
            by_depth.entry(node.depth).or_insert_with(Vec::new).push(node);
        }
        
        for depth in 1..=3 {
            if let Some(nodes) = by_depth.get(&depth) {
                println!(
                    "{} {} {}",
                    "└─".bright_black(),
                    format!("Level {}: {} packages", depth, nodes.len()).bright_blue().bold(),
                    format!("(showing first 10)").bright_black()
                );
                
                for (i, node) in nodes.iter().take(10).enumerate() {
                    let connector = if i == 9 || i == nodes.len() - 1 { "   └─" } else { "   ├─" };
                    println!(
                        "   {} {} {}",
                        connector.bright_black(),
                        node.name.white(),
                        format!("({})", node.version).bright_black()
                    );
                }
                
                if nodes.len() > 10 {
                    println!("   {} {} more packages...", "   └─".bright_black(), (nodes.len() - 10).to_string().bright_yellow());
                }
                println!();
            }
        }
    }
    // Build graph from Cargo.toml (direct dependencies only)
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

    // Build graph from cargo metadata (includes nested dependencies)
    pub fn from_metadata<P: AsRef<Path>>(manifest_path: P) -> anyhow::Result<Self> {
        let packages = MetadataParser::parse(&manifest_path)?;
        let root_name = MetadataParser::get_root_package_name(&manifest_path)?;

        let root_package = packages
            .get(&root_name)
            .ok_or_else(|| anyhow::anyhow!("Root package not found"))?;

        let mut root = DependencyNode::new(
            root_package.name.clone(),
            root_package.version.clone(),
        );

        let mut nodes = HashMap::new();
        let mut visited = HashSet::new();

        // Build the tree recursively
        for dep_name in &root_package.dependencies {
            root.add_dependency(dep_name.clone());
            Self::build_tree_recursive(
                dep_name,
                1,
                &packages,
                &mut nodes,
                &mut visited,
            );
        }

        Ok(Self { root, nodes })
    }

    fn build_tree_recursive(
        package_name: &str,
        depth: usize,
        all_packages: &HashMap<String, PackageInfo>,
        nodes: &mut HashMap<String, DependencyNode>,
        visited: &mut HashSet<String>,
    ) {
        // Prevent infinite recursion in case of circular dependencies
        if visited.contains(package_name) {
            return;
        }

        visited.insert(package_name.to_string());

        if let Some(package) = all_packages.get(package_name) {
            let mut node = DependencyNode::new(
                package.name.clone(),
                package.version.clone(),
            )
            .with_depth(depth);

            // Add this package's dependencies
            for dep_name in &package.dependencies {
                node.add_dependency(dep_name.clone());
                
                // Recursively build children
                Self::build_tree_recursive(
                    dep_name,
                    depth + 1,
                    all_packages,
                    nodes,
                    visited,
                );
            }

            nodes.insert(package_name.to_string(), node);
        }
    }

    // Print full dependency tree
    pub fn print_tree(&self) {
        println!("{}", "🌳 Dependency Tree:".bright_cyan().bold());
        println!();
        self.print_node(&self.root, "", true);
    }

    fn print_node(&self, node: &DependencyNode, prefix: &str, is_last: bool) {
        let connector = if is_last { "└── " } else { "├── " };
        let extension = if is_last { "    " } else { "│   " };

        if node.depth == 0 {
            // Root node
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
                3 => node.name.bright_cyan(),
                _ => node.name.white(),
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

    // Print only direct dependencies
    pub fn print_tree_direct_only(&self) {
        println!("{}", "🌳 Dependency Tree (Direct Only):".bright_cyan().bold());
        println!();
        
        println!(
            "{} {} {}",
            "📦".bright_yellow(),
            self.root.name.bright_green().bold(),
            format!("({})", self.root.version).bright_black()
        );

        let dep_count = self.root.dependencies.len();
        for (i, dep_name) in self.root.dependencies.iter().enumerate() {
            let is_last = i == dep_count - 1;
            let connector = if is_last { "└── " } else { "├── " };
            
            if let Some(node) = self.nodes.get(dep_name) {
                println!(
                    "{}{} {}",
                    connector.bright_black(),
                    node.name.bright_blue().bold(),
                    format!("({})", node.version).bright_black()
                );
            }
        }
    }

    // Print tree with max depth limit
    pub fn print_tree_with_depth(&self, max_depth: usize) {
        println!(
            "{}",
            format!("🌳 Dependency Tree (Max Depth: {})", max_depth).bright_cyan().bold()
        );
        println!();
        self.print_node_with_depth(&self.root, "", true, max_depth);
    }

    fn print_node_with_depth(&self, node: &DependencyNode, prefix: &str, is_last: bool, max_depth: usize) {
        let connector = if is_last { "└── " } else { "├── " };
        let extension = if is_last { "    " } else { "│   " };

        if node.depth == 0 {
            println!(
                "{} {} {}",
                "📦".bright_yellow(),
                node.name.bright_green().bold(),
                format!("({})", node.version).bright_black()
            );
        } else {
            let name_colored = match node.depth {
                1 => node.name.bright_blue(),
                2 => node.name.bright_magenta(),
                3 => node.name.bright_cyan(),
                _ => node.name.white(),
            };
            
            println!(
                "{}{}{} {}",
                prefix.bright_black(),
                connector.bright_black(),
                name_colored.bold(),
                format!("({})", node.version).bright_black()
            );
        }

        // Don't print children if we've reached max depth
        if node.depth >= max_depth {
            return;
        }

        let child_prefix = format!("{}{}", prefix, extension);
        let dep_count = node.dependencies.len();

        for (i, dep_name) in node.dependencies.iter().enumerate() {
            let is_last_child = i == dep_count - 1;
            
            if let Some(child_node) = self.nodes.get(dep_name) {
                self.print_node_with_depth(child_node, &child_prefix, is_last_child, max_depth);
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

    // Get statistics
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