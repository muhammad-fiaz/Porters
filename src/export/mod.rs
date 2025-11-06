//! Export functionality for converting porters.toml to build system configs
//!
//! This module provides the ability to export a Porters project to various
//! build system formats (CMake, XMake, Meson, etc.) for compatibility with
//! standard build tools.

pub mod cmake;
pub mod conan;
pub mod vcpkg;
pub mod xmake;
// pub mod meson;
// pub mod make;
// pub mod ninja;
// pub mod bazel;

use crate::config::PortersConfig;
use crate::scan::ProjectSources;
use anyhow::Result;

/// Trait for build system exporters
pub trait BuildSystemExporter {
    /// Name of the build system (e.g., "CMake", "XMake")
    fn name(&self) -> &str;

    /// File extension for the config file (e.g., "txt" for CMakeLists.txt)
    fn config_file_name(&self) -> &str;

    /// Generate the build system configuration from porters.toml
    fn generate(&self, config: &PortersConfig, sources: &ProjectSources) -> Result<String>;

    /// Export the configuration to a file in the project root
    fn export(&self, config: &PortersConfig, sources: &ProjectSources) -> Result<()> {
        let content = self.generate(config, sources)?;
        let filename = self.config_file_name();

        // Backup existing file if it exists
        if std::path::Path::new(filename).exists() {
            let backup = format!("{}.backup", filename);
            std::fs::copy(filename, &backup)?;
            eprintln!("⚠️  Backed up existing {} to {}", filename, backup);
        }

        std::fs::write(filename, content)?;
        println!("✅ Exported {} to project root", filename);
        println!("💡 You can now use this project with {}", self.name());

        Ok(())
    }
}
