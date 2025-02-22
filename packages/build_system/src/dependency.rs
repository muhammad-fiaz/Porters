use std::collections::HashMap;

#[derive(Debug)]
pub struct Dependency {
    pub name: String,
    pub version: Option<String>,
    pub git: Option<String>,
    pub rev: Option<String>,
}

impl Dependency {
    pub fn new(name: &str, version: Option<&str>, git: Option<&str>, rev: Option<&str>) -> Self {
        Self {
            name: name.to_string(),
            version: version.map(|v| v.to_string()),
            git: git.map(|g| g.to_string()),
            rev: rev.map(|r| r.to_string()),
        }
    }
}

#[derive(Debug)]
pub struct Dependencies {
    pub dependencies: HashMap<String, Dependency>,
    pub optional_dependencies: HashMap<String, Dependency>,
}

impl Dependencies {
    pub fn new() -> Self {
        Self {
            dependencies: HashMap::new(),
            optional_dependencies: HashMap::new(),
        }
    }

    pub fn add_dependency(&mut self, dep: Dependency) {
        self.dependencies.insert(dep.name.clone(), dep);
    }

    pub fn add_optional_dependency(&mut self, dep: Dependency) {
        self.optional_dependencies.insert(dep.name.clone(), dep);
    }
}