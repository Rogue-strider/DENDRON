#[derive(Debug, Clone)]
pub struct DependencyNode {
    pub name: String,
    pub version: String,
    pub dependencies: Vec<String>,
    pub depth: usize,
}

impl DependencyNode {
    pub fn new(name: String, version: String) -> Self {
        Self {
            name,
            version,
            dependencies: Vec::new(),
            depth: 0,
        }
    }

    pub fn with_depth(mut self, depth: usize) -> Self {
        self.depth = depth;
        self
    }

    pub fn add_dependency(&mut self, dep_name: String) {
        self.dependencies.push(dep_name);
    }
}
