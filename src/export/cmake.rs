//! CMake export functionality

use super::BuildSystemExporter;
use crate::config::{PortersConfig, ProjectType};
use crate::scan::ProjectSources;
use anyhow::Result;

pub struct CMakeExporter;

impl CMakeExporter {
    pub fn new() -> Self {
        Self
    }
}

impl BuildSystemExporter for CMakeExporter {
    fn name(&self) -> &str {
        "CMake"
    }

    fn config_file_name(&self) -> &str {
        "CMakeLists.txt"
    }

    fn generate(&self, config: &PortersConfig, sources: &ProjectSources) -> Result<String> {
        let mut cmake = String::new();

        // CMake minimum version
        cmake.push_str("cmake_minimum_required(VERSION 3.15)\n\n");

        // Project declaration
        let project_name = config.get_output_name();
        cmake.push_str(&format!(
            "project({} VERSION {})\n\n",
            project_name, config.project.version
        ));

        // C/C++ standards
        cmake.push_str("set(CMAKE_C_STANDARD 11)\n");
        cmake.push_str("set(CMAKE_CXX_STANDARD 17)\n");
        cmake.push_str("set(CMAKE_C_STANDARD_REQUIRED ON)\n");
        cmake.push_str("set(CMAKE_CXX_STANDARD_REQUIRED ON)\n\n");

        // Set output directories to use build/ directory
        let build_dir = config.get_build_dir();
        cmake.push_str(&format!(
            "set(CMAKE_RUNTIME_OUTPUT_DIRECTORY ${{CMAKE_SOURCE_DIR}}/{})\n",
            build_dir.display()
        ));
        cmake.push_str(&format!(
            "set(CMAKE_LIBRARY_OUTPUT_DIRECTORY ${{CMAKE_SOURCE_DIR}}/{})\n",
            build_dir.display()
        ));
        cmake.push_str(&format!(
            "set(CMAKE_ARCHIVE_OUTPUT_DIRECTORY ${{CMAKE_SOURCE_DIR}}/{})\n\n",
            build_dir.display()
        ));

        // Include directories from config
        if !config.build.include.is_empty() {
            cmake.push_str("include_directories(\n");
            for inc in &config.build.include {
                cmake.push_str(&format!("    {}\n", inc));
            }
            cmake.push_str(")\n\n");
        }

        // Compile definitions
        if !config.build.flags.defines.is_empty() {
            cmake.push_str("add_definitions(\n");
            for define in &config.build.flags.defines {
                cmake.push_str(&format!("    -D{}\n", define));
            }
            cmake.push_str(")\n\n");
        }

        // Compiler flags
        if !config.build.flags.cflags.is_empty() {
            cmake.push_str(&format!(
                "set(CMAKE_C_FLAGS \"${{CMAKE_C_FLAGS}} {}\")\n",
                config.build.flags.cflags.join(" ")
            ));
        }
        if !config.build.flags.cxxflags.is_empty() {
            cmake.push_str(&format!(
                "set(CMAKE_CXX_FLAGS \"${{CMAKE_CXX_FLAGS}} {}\")\n",
                config.build.flags.cxxflags.join(" ")
            ));
        }
        if !config.build.flags.ldflags.is_empty() {
            cmake.push_str(&format!(
                "set(CMAKE_EXE_LINKER_FLAGS \"${{CMAKE_EXE_LINKER_FLAGS}} {}\")\n\n",
                config.build.flags.ldflags.join(" ")
            ));
        }

        // Collect source files
        let mut c_sources = Vec::new();
        let mut cpp_sources = Vec::new();

        for src in &sources.source_files {
            let src_str = src.display().to_string();
            if src_str.ends_with(".c") {
                c_sources.push(src_str);
            } else if src_str.ends_with(".cpp")
                || src_str.ends_with(".cc")
                || src_str.ends_with(".cxx")
            {
                cpp_sources.push(src_str);
            }
        }

        // Executable or library target
        let executable_name = config.get_executable_name();

        match config.project.project_type {
            ProjectType::Application => {
                cmake.push_str(&format!("add_executable({}\n", executable_name));
            }
            ProjectType::Library => {
                cmake.push_str(&format!("add_library({} SHARED\n", executable_name));
            }
        }

        // Add all source files
        for src in c_sources.iter().chain(cpp_sources.iter()) {
            cmake.push_str(&format!("    {}\n", src));
        }
        cmake.push_str(")\n\n");

        // Link libraries from config
        if !config.build.linking.libraries.is_empty() {
            cmake.push_str(&format!("target_link_libraries({}\n", executable_name));
            for lib in &config.build.linking.libraries {
                cmake.push_str(&format!("    {}\n", lib));
            }
            cmake.push_str(")\n\n");
        }

        // Link directories
        if !config.build.linking.library_paths.is_empty() {
            cmake.push_str("link_directories(\n");
            for lib_path in &config.build.linking.library_paths {
                cmake.push_str(&format!("    {}\n", lib_path));
            }
            cmake.push_str(")\n\n");
        }

        // Frameworks (macOS)
        if !config.build.linking.frameworks.is_empty() {
            for framework in &config.build.linking.frameworks {
                cmake.push_str(&format!(
                    "target_link_libraries({} \"-framework {}\")\n",
                    executable_name, framework
                ));
            }
            cmake.push('\n');
        }

        // Enable testing if needed
        cmake.push_str("enable_testing()\n");

        Ok(cmake)
    }
}
