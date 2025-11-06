//! Conan export functionality

use super::BuildSystemExporter;
use crate::config::PortersConfig;
use crate::scan::ProjectSources;
use anyhow::Result;

pub struct ConanExporter;

impl ConanExporter {
    pub fn new() -> Self {
        Self
    }
}

impl BuildSystemExporter for ConanExporter {
    fn name(&self) -> &str {
        "Conan"
    }

    fn config_file_name(&self) -> &str {
        "conanfile.py"
    }

    fn generate(&self, config: &PortersConfig, _sources: &ProjectSources) -> Result<String> {
        let mut conanfile = String::new();

        // Header
        conanfile.push_str("from conan import ConanFile\n");
        conanfile.push_str("from conan.tools.cmake import CMakeToolchain, CMake, cmake_layout\n\n");

        // ConanFile class
        let class_name = config.project.name.replace("-", "_").replace(" ", "_");
        conanfile.push_str(&format!("class {}Conan(ConanFile):\n", class_name));
        conanfile.push_str(&format!("    name = \"{}\"\n", config.project.name));
        conanfile.push_str(&format!("    version = \"{}\"\n", config.project.version));

        if let Some(license) = &config.project.license {
            conanfile.push_str(&format!("    license = \"{}\"\n", license));
        }

        if let Some(homepage) = &config.project.homepage {
            conanfile.push_str(&format!("    url = \"{}\"\n", homepage));
        }

        if let Some(description) = &config.project.description {
            conanfile.push_str(&format!("    description = \"{}\"\n", description));
        }

        conanfile.push_str("    settings = \"os\", \"compiler\", \"build_type\", \"arch\"\n");
        conanfile
            .push_str("    exports_sources = \"CMakeLists.txt\", \"src/*\", \"include/*\"\n\n");

        // Dependencies
        if !config.dependencies.is_empty() {
            conanfile.push_str("    def requirements(self):\n");
            for name in config.dependencies.keys() {
                // Simple dependency format for now
                conanfile.push_str(&format!("        self.requires(\"{}\")\n", name));
            }
            conanfile.push('\n');
        }

        // Layout
        conanfile.push_str("    def layout(self):\n");
        conanfile.push_str("        cmake_layout(self)\n\n");

        // Generate method
        conanfile.push_str("    def generate(self):\n");
        conanfile.push_str("        tc = CMakeToolchain(self)\n");
        conanfile.push_str("        tc.generate()\n\n");

        // Build method
        conanfile.push_str("    def build(self):\n");
        conanfile.push_str("        cmake = CMake(self)\n");
        conanfile.push_str("        cmake.configure()\n");
        conanfile.push_str("        cmake.build()\n\n");

        // Package method
        conanfile.push_str("    def package(self):\n");
        conanfile.push_str("        cmake = CMake(self)\n");
        conanfile.push_str("        cmake.install()\n");

        Ok(conanfile)
    }
}
