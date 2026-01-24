#[cfg(test)]
mod tests {
    use crate::parser::CargoParser;
    use crate::graph::DependencyGraph;
    use crate::analyzer::{DuplicateAnalyzer, CircularAnalyzer};
    use std::path::PathBuf;

    #[test]
    fn test_parse_current_project() {
        // Test parsing current project's Cargo.toml
        let result = CargoParser::parse("Cargo.toml");
        assert!(result.is_ok(), "Should parse Cargo.toml successfully");
        
        let manifest = result.unwrap();
        assert_eq!(manifest.package.name, "dendron");
    }

    #[test]
    fn test_build_graph_from_manifest() {
        let manifest = CargoParser::parse("Cargo.toml").unwrap();
        let graph = DependencyGraph::from_manifest(&manifest);
        
        assert_eq!(graph.root.name, "dendron");
        assert!(graph.root.dependencies.len() > 0, "Should have dependencies");
    }

    #[test]
    fn test_graph_statistics() {
        let manifest = CargoParser::parse("Cargo.toml").unwrap();
        let graph = DependencyGraph::from_manifest(&manifest);
        let stats = graph.stats();
        
        assert!(stats.direct_dependencies > 0);
        assert!(stats.total_dependencies >= stats.direct_dependencies);
        assert!(stats.max_depth > 0);
    }

    #[test]
    fn test_duplicate_analyzer_no_duplicates() {
        let manifest = CargoParser::parse("Cargo.toml").unwrap();
        let graph = DependencyGraph::from_manifest(&manifest);
        let duplicates = DuplicateAnalyzer::analyze(&graph);
        
        // Direct dependencies shouldn't have duplicates
        assert_eq!(duplicates.len(), 0);
    }

    #[test]
    fn test_circular_analyzer() {
        let manifest = CargoParser::parse("Cargo.toml").unwrap();
        let graph = DependencyGraph::from_manifest(&manifest);
        let cycles = CircularAnalyzer::analyze(&graph);
        
        // Direct dependencies shouldn't have cycles
        assert_eq!(cycles.len(), 0);
    }

    #[test]
    fn test_json_export() {
        let manifest = CargoParser::parse("Cargo.toml").unwrap();
        let graph = DependencyGraph::from_manifest(&manifest);
        
        let json = graph.to_json();
        assert!(json.is_ok(), "Should export to JSON successfully");
        
        let json_str = json.unwrap();
        assert!(json_str.contains("dendron"));
        assert!(json_str.contains("dependencies"));
    }

    #[test]
    fn test_json_compact_export() {
        let manifest = CargoParser::parse("Cargo.toml").unwrap();
        let graph = DependencyGraph::from_manifest(&manifest);
        
        let json = graph.to_json_compact();
        assert!(json.is_ok());
        
        let json_str = json.unwrap();
        assert!(!json_str.contains("\n"), "Compact JSON should not have newlines");
    }

    #[test]
    fn test_dot_export() {
        let manifest = CargoParser::parse("Cargo.toml").unwrap();
        let graph = DependencyGraph::from_manifest(&manifest);
        
        let dot = graph.to_dot();
        assert!(dot.contains("digraph dependencies"));
        assert!(dot.contains("dendron"));
    }

    #[test]
    fn test_invalid_path() {
        let result = CargoParser::parse("nonexistent/Cargo.toml");
        assert!(result.is_err(), "Should fail for non-existent file");
    }
}