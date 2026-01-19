use anyhow::{Context, Result};
use cargo_metadata::{MetadataCommand, Package, DependencyKind};
use std::collections::{HashMap, HashSet};
use std::path::Path;

#[derive(Debug, Clone)]
pub struct PackageInfo {
    pub name: String,
    pub version: String,
    pub dependencies: Vec<String>,
}

pub struct MetadataParser;

impl MetadataParser {
    pub fn parse<P: AsRef<Path>>(manifest_path: P) -> Result<HashMap<String, PackageInfo>> {
        let metadata = MetadataCommand::new()
            .manifest_path(manifest_path.as_ref())
            .exec()
            .context("Failed to execute cargo metadata")?;

        let mut packages = HashMap::new();

        // Get all packages from the dependency graph
        for package in &metadata.packages {
            let deps: Vec<String> = package
                .dependencies
                .iter()
                .filter(|dep| {
                    // Only include normal dependencies (not dev or build dependencies)
                    matches!(dep.kind, DependencyKind::Normal)
                })
                .map(|dep| dep.name.clone())
                .collect();

            let pkg_info = PackageInfo {
                name: package.name.clone(),
                version: package.version.to_string(),
                dependencies: deps,
            };

            packages.insert(package.name.clone(), pkg_info);
        }

        Ok(packages)
    }

    pub fn get_root_package_name<P: AsRef<Path>>(manifest_path: P) -> Result<String> {
        let metadata = MetadataCommand::new()
            .manifest_path(manifest_path.as_ref())
            .exec()
            .context("Failed to execute cargo metadata")?;

        let root_package = metadata
            .root_package()
            .context("No root package found")?;

        Ok(root_package.name.clone())
    }
}