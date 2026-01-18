mod parser;

use anyhow::Result;
use parser::CargoParser;

fn print_banner() {
    println!(r#"
██████╗ ███████╗███╗   ██╗██████╗ ██████╗  ██████╗ ███╗   ██╗
██╔══██╗██╔════╝████╗  ██║██╔══██╗██╔══██╗██╔═══██╗████╗  ██║
██║  ██║█████╗  ██╔██╗ ██║██║  ██║██████╔╝██║   ██║██╔██╗ ██║
██║  ██║██╔══╝  ██║╚██╗██║██║  ██║██╔══██╗██║   ██║██║╚██╗██║
██████╔╝███████╗██║ ╚████║██████╔╝██║  ██║╚██████╔╝██║ ╚████║
╚═════╝ ╚══════╝╚═╝  ╚═══╝╚═════╝ ╚═╝  ╚═╝ ╚═════╝ ╚═╝  ╚═══╝
"#);
    println!("    🌳 Dependency Graph Visualizer for Rust Projects");
    println!("    ================================================\n");
}

fn main() -> Result<()> {
    print_banner();
    
    // Current directory ki Cargo.toml parse karo
    let manifest = CargoParser::parse("Cargo.toml")?;
    
    // Basic info print karo
    println!("📦 Package: {}", manifest.package.name);
    println!("🏷️  Version: {}", manifest.package.version);
    println!("\n📚 Dependencies:");
    
    for (name, dep) in &manifest.dependencies {
        let version = CargoParser::get_version(dep)
            .unwrap_or_else(|| "unknown".to_string());
        println!("  - {} = {}", name, version);
    }
    
    Ok(())
}