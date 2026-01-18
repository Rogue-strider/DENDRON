mod parser;
mod graph;

use anyhow::Result;
use parser::CargoParser;
use graph::DependencyGraph;
use colored::*;

fn print_banner() {
    let banner = r#"
██████╗ ███████╗███╗   ██╗██████╗ ██████╗  ██████╗ ███╗   ██╗
██╔══██╗██╔════╝████╗  ██║██╔══██╗██╔══██╗██╔═══██╗████╗  ██║
██║  ██║█████╗  ██╔██╗ ██║██║  ██║██████╔╝██║   ██║██╔██╗ ██║
██║  ██║██╔══╝  ██║╚██╗██║██║  ██║██╔══██╗██║   ██║██║╚██╗██║
██████╔╝███████╗██║ ╚████║██████╔╝██║  ██║╚██████╔╝██║ ╚████║
╚═════╝ ╚══════╝╚═╝  ╚═══╝╚═════╝ ╚═╝  ╚═╝ ╚═════╝ ╚═╝  ╚═══╝
"#;
    println!("{}", banner.bright_cyan().bold());
    println!("    {} Dependency Graph Visualizer for Rust Projects", "🌳".bright_green());
    println!("    {}", "================================================".bright_black());
    println!();
}

fn main() -> Result<()> {
    print_banner();
    
    // Cargo.toml parse karo
    println!("{} Parsing Cargo.toml...", "🔍".bright_yellow());
    println!();
    
    let manifest = CargoParser::parse("Cargo.toml")?;
    
    // Graph banao
    let graph = DependencyGraph::from_manifest(&manifest);
    
    // Tree print karo
    graph.print_tree();
    
    // Statistics show karo
    let stats = graph.stats();
    stats.print();
    
    println!();
    println!("{} {}", "✨".bright_yellow(), "Analysis complete!".bright_green().bold());
    
    Ok(())
}