//! XMake export functionality

use super::BuildSystemExporter;
use crate::config::{PortersConfig, ProjectType};
use crate::scan::ProjectSources;
use anyhow::Result;

pub struct XMakeExporter;

impl XMakeExporter {
    pub fn new() -> Self {
        Self
    }
}

impl BuildSystemExporter for XMakeExporter {
    fn name(&self) -> &str {
        "XMake"
    }

    fn config_file_name(&self) -> &str {
        "xmake.lua"
    }

    fn generate(&self, config: &PortersConfig, sources: &ProjectSources) -> Result<String> {
        let mut xmake = String::new();

        // XMake minimum version
        xmake.push_str("-- xmake.lua\n");
        xmake.push_str("-- Auto-generated from porters.toml by Porters\n\n");

        xmake.push_str("set_xmakever(\"2.7.0\")\n\n");

        // Project info
        xmake.push_str(&format!("set_project(\"{}\")\n", config.project.name));
        xmake.push_str(&format!("set_version(\"{}\")\n\n", config.project.version));

        // C/C++ standards
        xmake.push_str("set_languages(\"c11\", \"cxx17\")\n\n");

        // Build directory
        let build_dir = config.get_build_dir();
        xmake.push_str(&format!("set_targetdir(\"{}\")\n\n", build_dir.display()));

        // Target definition
        let target_name = config.get_executable_name();
        xmake.push_str(&format!("target(\"{}\")\n", target_name));

        // Target kind (binary or shared library)
        match config.project.project_type {
            ProjectType::Application => {
                xmake.push_str("    set_kind(\"binary\")\n");
            }
            ProjectType::Library => {
                xmake.push_str("    set_kind(\"shared\")\n");
            }
        }

        // Add source files
        if !sources.source_files.is_empty() {
            xmake.push_str("    add_files(\n");
            for src in &sources.source_files {
                xmake.push_str(&format!("        \"{}\",\n", src.display()));
            }
            xmake.push_str("    )\n");
        }

        // Include directories
        if !config.build.include.is_empty() {
            xmake.push_str("    add_includedirs(\n");
            for inc in &config.build.include {
                xmake.push_str(&format!("        \"{}\",\n", inc));
            }
            xmake.push_str("    )\n");
        }

        // Defines
        if !config.build.flags.defines.is_empty() {
            xmake.push_str("    add_defines(\n");
            for define in &config.build.flags.defines {
                xmake.push_str(&format!("        \"{}\",\n", define));
            }
            xmake.push_str("    )\n");
        }

        // C flags
        if !config.build.flags.cflags.is_empty() {
            xmake.push_str("    add_cflags(\n");
            for flag in &config.build.flags.cflags {
                xmake.push_str(&format!("        \"{}\",\n", flag));
            }
            xmake.push_str("    )\n");
        }

        // C++ flags
        if !config.build.flags.cxxflags.is_empty() {
            xmake.push_str("    add_cxxflags(\n");
            for flag in &config.build.flags.cxxflags {
                xmake.push_str(&format!("        \"{}\",\n", flag));
            }
            xmake.push_str("    )\n");
        }

        // Linker flags
        if !config.build.flags.ldflags.is_empty() {
            xmake.push_str("    add_ldflags(\n");
            for flag in &config.build.flags.ldflags {
                xmake.push_str(&format!("        \"{}\",\n", flag));
            }
            xmake.push_str("    )\n");
        }

        // Link libraries
        if !config.build.linking.libraries.is_empty() {
            xmake.push_str("    add_links(\n");
            for lib in &config.build.linking.libraries {
                xmake.push_str(&format!("        \"{}\",\n", lib));
            }
            xmake.push_str("    )\n");
        }

        // Link directories
        if !config.build.linking.library_paths.is_empty() {
            xmake.push_str("    add_linkdirs(\n");
            for lib_path in &config.build.linking.library_paths {
                xmake.push_str(&format!("        \"{}\",\n", lib_path));
            }
            xmake.push_str("    )\n");
        }

        // Frameworks (macOS)
        if !config.build.linking.frameworks.is_empty() {
            xmake.push_str("    add_frameworks(\n");
            for framework in &config.build.linking.frameworks {
                xmake.push_str(&format!("        \"{}\",\n", framework));
            }
            xmake.push_str("    )\n");
        }

        xmake.push_str("target_end()\n");

        Ok(xmake)
    }
}
