use crate::graph::DependencyGraph;
use colored::*;
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct DuplicatePackage {
    pub name: String,
    pub versions: Vec<VersionInfo>,
}

#[derive(Debug, Clone)]
pub struct VersionInfo {
    pub version: String,
    pub used_by_count: usize,
    pub depth: usize,
}

pub struct DuplicateAnalyzer;

impl DuplicateAnalyzer {
    pub fn analyze(graph: &DependencyGraph) -> Vec<DuplicatePackage> {
        let mut package_versions: HashMap<String, Vec<(String, usize)>> = HashMap::new();

        // Collect all package versions
        for (name, node) in &graph.nodes {
            package_versions
                .entry(name.clone())
                .or_insert_with(Vec::new)
                .push((node.version.clone(), node.depth));
        }

        // Find duplicates (packages with multiple versions)
        let mut duplicates = Vec::new();

        for (name, versions) in package_versions {
            // Group by version
            let mut version_map: HashMap<String, Vec<usize>> = HashMap::new();
            for (version, depth) in versions {
                version_map
                    .entry(version)
                    .or_insert_with(Vec::new)
                    .push(depth);
            }

            // If more than one version exists
            if version_map.len() > 1 {
                let mut version_infos = Vec::new();

                for (version, depths) in version_map {
                    version_infos.push(VersionInfo {
                        version,
                        used_by_count: depths.len(),
                        depth: *depths.iter().min().unwrap_or(&0),
                    });
                }

                // Sort by version (newer first typically)
                version_infos.sort_by(|a, b| b.version.cmp(&a.version));

                duplicates.push(DuplicatePackage {
                    name,
                    versions: version_infos,
                });
            }
        }

        // Sort by package name
        duplicates.sort_by(|a, b| a.name.cmp(&b.name));

        duplicates
    }

    pub fn print_report(duplicates: &[DuplicatePackage]) {
        if duplicates.is_empty() {
            println!(
                "{}",
                "✅ No duplicate versions found!".bright_green().bold()
            );
            println!("   All dependencies use consistent versions.");
            return;
        }

        println!(
            "{} {}",
            "⚠️".bright_yellow(),
            format!(
                "Found {} packages with multiple versions:",
                duplicates.len()
            )
            .bright_yellow()
            .bold()
        );
        println!();

        let mut total_duplicates = 0;

        for dup in duplicates {
            total_duplicates += dup.versions.len() - 1;

            println!("{} {}", "📦".bright_cyan(), dup.name.bright_white().bold());

            for (i, version_info) in dup.versions.iter().enumerate() {
                let is_last = i == dup.versions.len() - 1;
                let connector = if is_last { "└─" } else { "├─" };

                let version_color = if i == 0 {
                    version_info.version.bright_green()
                } else {
                    version_info.version.bright_yellow()
                };

                println!(
                    "  {} {} {} {}",
                    connector.bright_black(),
                    "v".bright_black(),
                    version_color.bold(),
                    format!(
                        "(used {} time{})",
                        version_info.used_by_count,
                        if version_info.used_by_count > 1 {
                            "s"
                        } else {
                            ""
                        }
                    )
                    .bright_black()
                );
            }
            println!();
        }

        println!("{}", "💡 Impact:".bright_cyan().bold());
        println!(
            "   {} {}",
            "•".bright_black(),
            format!(
                "{} duplicate versions increase binary size",
                total_duplicates
            )
            .white()
        );
        println!(
            "   {} {}",
            "•".bright_black(),
            "Consider updating dependencies to use consistent versions".white()
        );
        println!(
            "   {} {}",
            "•".bright_black(),
            format!("Run {} to update", "cargo update".bright_green()).white()
        );
    }

    pub fn get_stats(duplicates: &[DuplicatePackage]) -> DuplicateStats {
        let total_packages = duplicates.len();
        let mut total_versions = 0;
        let mut total_duplicates = 0;

        for dup in duplicates {
            total_versions += dup.versions.len();
            total_duplicates += dup.versions.len() - 1;
        }

        DuplicateStats {
            total_packages_with_duplicates: total_packages,
            total_versions,
            total_duplicate_versions: total_duplicates,
        }
    }
}

#[derive(Debug)]
pub struct DuplicateStats {
    pub total_packages_with_duplicates: usize,
    pub total_versions: usize,
    pub total_duplicate_versions: usize,
}

impl DuplicateStats {
    pub fn print(&self) {
        println!();
        println!("{}", "📊 Duplicate Statistics:".bright_cyan().bold());
        println!(
            "  {} Packages with duplicates: {}",
            "├─".bright_black(),
            self.total_packages_with_duplicates
                .to_string()
                .bright_yellow()
                .bold()
        );
        println!(
            "  {} Total versions: {}",
            "├─".bright_black(),
            self.total_versions.to_string().bright_white().bold()
        );
        println!(
            "  {} Duplicate versions: {}",
            "└─".bright_black(),
            self.total_duplicate_versions
                .to_string()
                .bright_red()
                .bold()
        );
    }
}
