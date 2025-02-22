#[derive(Debug)]
pub struct BuildConfig {
    pub toolchain: String,
    pub flags: Vec<String>,
}

impl BuildConfig {
    pub fn new(toolchain: &str, flags: Vec<&str>) -> Self {
        Self {
            toolchain: toolchain.to_string(),
            flags: flags.into_iter().map(|s| s.to_string()).collect(),
        }
    }
}