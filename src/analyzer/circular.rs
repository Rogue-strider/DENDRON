use crate::graph::DependencyGraph;
use std::collections::{HashMap, HashSet};
use colored::*;

#[derive(Debug, Clone)]
pub struct CircularDependency {
    pub cycle: Vec<String>,
}

pub struct CircularAnalyzer;

impl CircularAnalyzer {
    pub fn analyze(graph: &DependencyGraph) -> Vec<CircularDependency> {
        let mut cycles = Vec::new();
        let mut visited = HashSet::new();
        let mut rec_stack = Vec::new();

        // Build adjacency list
        let mut adj_list: HashMap<String, Vec<String>> = HashMap::new();
        
        // Add root dependencies
        for dep in &graph.root.dependencies {
            adj_list
                .entry(graph.root.name.clone())
                .or_insert_with(Vec::new)
                .push(dep.clone());
        }

        // Add all node dependencies
        for (name, node) in &graph.nodes {
            if !node.dependencies.is_empty() {
                adj_list.insert(name.clone(), node.dependencies.clone());
            }
        }

        // DFS to find cycles
        for node in adj_list.keys() {
            if !visited.contains(node) {
                Self::dfs_find_cycles(
                    node,
                    &adj_list,
                    &mut visited,
                    &mut rec_stack,
                    &mut cycles,
                );
            }
        }

        // Remove duplicate cycles
        Self::deduplicate_cycles(&mut cycles);

        cycles
    }

    fn dfs_find_cycles(
        node: &str,
        adj_list: &HashMap<String, Vec<String>>,
        visited: &mut HashSet<String>,
        rec_stack: &mut Vec<String>,
        cycles: &mut Vec<CircularDependency>,
    ) {
        visited.insert(node.to_string());
        rec_stack.push(node.to_string());

        if let Some(neighbors) = adj_list.get(node) {
            for neighbor in neighbors {
                if !visited.contains(neighbor) {
                    Self::dfs_find_cycles(neighbor, adj_list, visited, rec_stack, cycles);
                } else if rec_stack.contains(neighbor) {
                    // Found a cycle
                    let cycle_start = rec_stack
                        .iter()
                        .position(|n| n == neighbor)
                        .unwrap();
                    
                    let mut cycle: Vec<String> = rec_stack[cycle_start..].to_vec();
                    cycle.push(neighbor.to_string()); // Complete the cycle

                    cycles.push(CircularDependency { cycle });
                }
            }
        }

        rec_stack.pop();
    }

    fn deduplicate_cycles(cycles: &mut Vec<CircularDependency>) {
        let mut seen = HashSet::new();
        cycles.retain(|cycle| {
            let mut sorted = cycle.cycle.clone();
            sorted.sort();
            let key = sorted.join("->");
            seen.insert(key)
        });
    }

    pub fn print_report(cycles: &[CircularDependency]) {
        if cycles.is_empty() {
            println!(
                "{}",
                "✅ No circular dependencies found!".bright_green().bold()
            );
            println!("   Your dependency graph is acyclic.");
            return;
        }

        println!(
            "{} {}",
            "⚠️".bright_red().bold(),
            format!("Found {} circular dependenc{}:", 
                cycles.len(),
                if cycles.len() == 1 { "y" } else { "ies" }
            )
            .bright_red()
            .bold()
        );
        println!();

        for (i, cycle) in cycles.iter().enumerate() {
            println!(
                "{} {}",
                format!("Cycle {}:", i + 1).bright_yellow().bold(),
                ""
            );

            for (j, package) in cycle.cycle.iter().enumerate() {
                let is_last = j == cycle.cycle.len() - 1;
                
                if is_last {
                    // Last item shows the cycle completion
                    println!(
                        "  {} {} {}",
                        "└─→".bright_black(),
                        package.bright_red().bold(),
                        "(completes cycle)".bright_black()
                    );
                } else {
                    let connector = if j == 0 { "┌─→" } else { "├─→" };
                    println!(
                        "  {} {}",
                        connector.bright_black(),
                        package.bright_cyan().bold()
                    );
                }
            }
            println!();
        }

        println!("{}", "💡 Impact:".bright_cyan().bold());
        println!(
            "   {} {}",
            "•".bright_black(),
            "Circular dependencies can cause compilation issues".white()
        );
        println!(
            "   {} {}",
            "•".bright_black(),
            "They may increase build time and binary size".white()
        );
        println!(
            "   {} {}",
            "•".bright_black(),
            "Consider refactoring to break the cycles".white()
        );
    }

    pub fn get_stats(cycles: &[CircularDependency]) -> CircularStats {
        let total_cycles = cycles.len();
        let mut total_packages_in_cycles = 0;
        let mut max_cycle_length = 0;

        for cycle in cycles {
            // -1 because last item is duplicate of first
            let cycle_len = cycle.cycle.len() - 1;
            total_packages_in_cycles += cycle_len;
            max_cycle_length = max_cycle_length.max(cycle_len);
        }

        CircularStats {
            total_cycles,
            total_packages_in_cycles,
            max_cycle_length,
        }
    }
}

#[derive(Debug)]
pub struct CircularStats {
    pub total_cycles: usize,
    pub total_packages_in_cycles: usize,
    pub max_cycle_length: usize,
}

impl CircularStats {
    pub fn print(&self) {
        println!();
        println!("{}", "📊 Circular Dependency Statistics:".bright_cyan().bold());
        println!(
            "  {} Total cycles: {}",
            "├─".bright_black(),
            self.total_cycles.to_string().bright_red().bold()
        );
        println!(
            "  {} Packages involved: {}",
            "├─".bright_black(),
            self.total_packages_in_cycles.to_string().bright_yellow().bold()
        );
        println!(
            "  {} Longest cycle: {} packages",
            "└─".bright_black(),
            self.max_cycle_length.to_string().bright_white().bold()
        );
    }
}