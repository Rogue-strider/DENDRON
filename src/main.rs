mod parser;
mod graph;
mod analyzer;

use anyhow::Result;
use parser::CargoParser;
use graph::DependencyGraph;
use analyzer::DuplicateAnalyzer; 
use colored::*;
use clap::Parser;
use std::path::PathBuf;

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
    summary: bool,

    /// Output format: json, json-compact, dot
    #[arg(short, long, value_name = "FORMAT")]
    output: Option<String>,

    /// Check for duplicate dependency versions
    #[arg(long)]
    check_duplicates: bool,  // <-- NEW
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

    // If output format is specified, skip banner for clean output
    if args.output.is_none() && !args.check_duplicates {
        print_banner();
    }
    
    let cargo_path = if let Some(path) = args.path {
        if path.is_dir() {
            path.join("Cargo.toml")
        } else {
            path
        }
    } else {
        PathBuf::from("Cargo.toml")
    };

    if args.output.is_none() && !args.check_duplicates {
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
    }
    
    // Build graph based on mode
    let graph = if args.nested {
        DependencyGraph::from_metadata(&cargo_path)?
    } else {
        let manifest = CargoParser::parse(&cargo_path)?;
        DependencyGraph::from_manifest(&manifest)
    };
    
    // Handle duplicate check (NEW LOGIC)
    if args.check_duplicates {
        if !args.nested {
            println!("{}", "⚠️  Warning: Duplicate detection works best with --nested flag".bright_yellow());
            println!("   Run: dendron --nested --check-duplicates");
            println!();
        }

        let duplicates = DuplicateAnalyzer::analyze(&graph);
        DuplicateAnalyzer::print_report(&duplicates);
        
        let dup_stats = DuplicateAnalyzer::get_stats(&duplicates);
        dup_stats.print();
        
        return Ok(());
    }
    
    // Handle output format
    if let Some(format) = args.output {
        match format.to_lowercase().as_str() {
            "json" => {
                let json = graph.to_json()?;
                println!("{}", json);
            }
            "json-compact" => {
                let json = graph.to_json_compact()?;
                println!("{}", json);
            }
            "dot" => {
                let dot = graph.to_dot();
                println!("{}", dot);
            }
            _ => {
                eprintln!("❌ Unknown output format: {}", format);
                eprintln!("Available formats: json, json-compact, dot");
                std::process::exit(1);
            }
        }
        return Ok(());
    }
    
    // Normal output - Print tree based on options
    if args.summary {
        graph.print_summary();
    } else if args.direct_only {
        graph.print_tree_direct_only();
    } else if let Some(max_depth) = args.depth {
        graph.print_tree_with_depth(max_depth);
    } else {
        graph.print_tree();
    }
    
    let stats = graph.stats();
    stats.print();
    
    println!();
    println!("{} {}", "✨".bright_yellow(), "Analysis complete!".bright_green().bold());
    
    Ok(())
}