use anyhow::{Context, Result};
use serde::Deserialize;
use std::collections::HashMap;
use std::fs;
use std::path::Path;

// Cargo.toml ka structure define kar rahe hain
#[derive(Debug, Deserialize)]
pub struct CargoManifest {
    pub package: Package,
    #[serde(default)]
    pub dependencies: HashMap<String, Dependency>,
}

#[derive(Debug, Deserialize)]
pub struct Package {
    pub name: String,
    pub version: String,
}

// Dependency do tarah ki ho sakti hai:
// 1. Simple: serde = "1.0"
// 2. Detailed: serde = { version = "1.0", features = ["derive"] }
#[derive(Debug, Deserialize, Clone)]
#[serde(untagged)]
pub enum Dependency {
    Simple(String),                    // "1.0"
    Detailed(DetailedDependency),      // { version = "1.0", ... }
}

#[derive(Debug, Deserialize, Clone)]
pub struct DetailedDependency {
    pub version: Option<String>,
    #[serde(default)]
    pub features: Vec<String>,
    pub optional: Option<bool>,
}

pub struct CargoParser;

impl CargoParser {
    // Cargo.toml file parse karta hai
    pub fn parse<P: AsRef<Path>>(path: P) -> Result<CargoManifest> {
        // File read karo
        let content = fs::read_to_string(path)
            .context("Failed to read Cargo.toml file")?;
        
        // TOML parse karo
        let manifest: CargoManifest = toml::from_str(&content)
            .context("Failed to parse Cargo.toml")?;
        
        Ok(manifest)
    }
    
    // Dependency ka version extract karta hai
    pub fn get_version(dep: &Dependency) -> Option<String> {
        match dep {
            Dependency::Simple(v) => Some(v.clone()),
            Dependency::Detailed(d) => d.version.clone(),
        }
    }
}