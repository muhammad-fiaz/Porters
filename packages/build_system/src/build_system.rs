#[derive(Debug)]
pub struct BuildSystem {
    pub requires: Vec<String>,
    pub build_backend: String,
}

impl BuildSystem {
    pub fn new(requires: Vec<&str>, build_backend: &str) -> Self {
        Self {
            requires: requires.into_iter().map(|s| s.to_string()).collect(),
            build_backend: build_backend.to_string(),
        }
    }
}