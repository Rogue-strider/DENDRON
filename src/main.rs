mod parser;
mod graph;

use anyhow::Result;
use parser::CargoParser;
use graph::DependencyGraph;
use colored::*;
use clap::Parser;
use std::path::PathBuf;

/// DENDRON - Dependency Graph Visualizer for Rust Projects
#[derive(Parser, Debug)]
#[command(name = "dendron")]
#[command(author = "Your Name")]
#[command(version = "0.1.0")]
#[command(about = "🌳 A powerful dependency graph visualizer", long_about = None)]
struct Args {
    /// Path to the project directory (default: current directory)
    #[arg(short, long, value_name = "PATH")]
    path: Option<PathBuf>,

    /// Maximum depth to display (default: unlimited)
    #[arg(short, long, value_name = "NUMBER")]
    depth: Option<usize>,

    /// Show only direct dependencies
    #[arg(short = 'D', long)]
    direct_only: bool,

    /// Disable colors
    #[arg(long)]
    no_color: bool,

    /// Show nested/transitive dependencies (uses cargo metadata)
    #[arg(short, long)]
    nested: bool,

    /// Show summary instead of full tree
    #[arg(short, long)]
    summary: bool,  // <-- NEW ARGUMENT
}

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
    let args = Args::parse();

    if args.no_color {
        colored::control::set_override(false);
    }

    print_banner();
    
    let cargo_path = if let Some(path) = args.path {
        if path.is_dir() {
            path.join("Cargo.toml")
        } else {
            path
        }
    } else {
        PathBuf::from("Cargo.toml")
    };

    println!(
        "{} Analyzing: {}",
        "🔍".bright_yellow(),
        cargo_path.display().to_string().bright_white()
    );

    if args.nested {
        println!("{} Mode: Nested dependencies (transitive)", "📊".bright_cyan());
    } else {
        println!("{} Mode: Direct dependencies only", "📊".bright_cyan());
    }
    println!();
    
    // Build graph based on mode
    let graph = if args.nested {
        DependencyGraph::from_metadata(&cargo_path)?
    } else {
        let manifest = CargoParser::parse(&cargo_path)?;
        DependencyGraph::from_manifest(&manifest)
    };
    
    // Print tree based on options (UPDATED LOGIC)
    if args.summary {
        // Summary view
        graph.print_summary();
    } else if args.direct_only {
        // Direct dependencies only
        graph.print_tree_direct_only();
    } else if let Some(max_depth) = args.depth {
        // Tree with depth limit
        graph.print_tree_with_depth(max_depth);
    } else {
        // Full tree
        graph.print_tree();
    }
    
    // Print statistics
    let stats = graph.stats();
    stats.print();
    
    println!();
    println!("{} {}", "✨".bright_yellow(), "Analysis complete!".bright_green().bold());
    
    Ok(())
}