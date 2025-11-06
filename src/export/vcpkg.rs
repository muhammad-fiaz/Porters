//! Vcpkg export functionality

use super::BuildSystemExporter;
use crate::config::PortersConfig;
use crate::scan::ProjectSources;
use anyhow::Result;

pub struct VcpkgExporter;

impl VcpkgExporter {
    pub fn new() -> Self {
        Self
    }
}

impl BuildSystemExporter for VcpkgExporter {
    fn name(&self) -> &str {
        "vcpkg"
    }

    fn config_file_name(&self) -> &str {
        "vcpkg.json"
    }

    fn generate(&self, config: &PortersConfig, _sources: &ProjectSources) -> Result<String> {
        use serde_json::json;

        // Create vcpkg manifest
        let mut dependencies = Vec::new();

        // Convert porters dependencies to vcpkg dependencies
        for name in config.dependencies.keys() {
            // Basic dependency entry
            dependencies.push(name.clone());
        }

        let manifest = json!({
            "name": config.project.name.to_lowercase().replace(" ", "-"),
            "version": config.project.version,
            "description": config.project.description.as_deref().unwrap_or(""),
            "homepage": config.project.homepage.as_deref().unwrap_or(""),
            "dependencies": dependencies,
            "builtin-baseline": "master"
        });

        let json_str = serde_json::to_string_pretty(&manifest)?;
        Ok(json_str)
    }
}
