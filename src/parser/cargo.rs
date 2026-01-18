use anyhow::{Context, Result};
use serde::Deserialize;
use std::collections::HashMap;
use std::fs;
use std::path::Path;

// Cargo.toml ka structure
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

// Dependency do tarah ki ho sakti hai
#[derive(Debug, Deserialize, Clone)]
#[serde(untagged)]
pub enum Dependency {
    Simple(String),
    Detailed(DetailedDependency),
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
    pub fn parse<P: AsRef<Path>>(path: P) -> Result<CargoManifest> {
        let content = fs::read_to_string(path)
            .context("Failed to read Cargo.toml file")?;
        
        let manifest: CargoManifest = toml::from_str(&content)
            .context("Failed to parse Cargo.toml")?;
        
        Ok(manifest)
    }
    
    pub fn get_version(dep: &Dependency) -> Option<String> {
        match dep {
            Dependency::Simple(v) => Some(v.clone()),
            Dependency::Detailed(d) => d.version.clone(),
        }
    }
}