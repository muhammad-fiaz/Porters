//! Conan package manager integration
//!
//! This module provides integration with Conan, a C/C++ package manager.
//! Handles dependency installation and build configuration through Conan,
//! working alongside CMake or other build systems.

use anyhow::{Context, Result};
use std::path::Path;
use std::process::Command;

use super::BuildSystem;
use crate::deps::ResolvedDependency;
use crate::scan::ProjectSources;
use crate::util::pretty::*;

/// Conan build system implementation
///
/// Manages C/C++ dependencies through Conan and builds projects
/// using Conan-generated build files.
pub struct ConanBuildSystem {
    root: String,
}

impl ConanBuildSystem {
    pub fn new(root: &str) -> Self {
        Self {
            root: root.to_string(),
        }
    }
}

impl BuildSystem for ConanBuildSystem {
    fn name(&self) -> &str {
        "Conan"
    }
    
    fn detect(root: &Path) -> bool {
        root.join("conanfile.txt").exists() || root.join("conanfile.py").exists()
    }
    
    fn configure(&self, _sources: &ProjectSources, _deps: &[ResolvedDependency]) -> Result<()> {
        print_build("Installing dependencies with Conan...");
        
        let mut cmd = Command::new("conan");
        cmd.current_dir(&self.root);
        cmd.arg("install");
        cmd.arg(".");
        cmd.arg("--build=missing");
        
        let output = cmd.output()
            .with_context(|| "Failed to run conan. Is Conan installed?")?;
        
        if !output.status.success() {
            print_error(&format!("Conan install failed:\n{}", String::from_utf8_lossy(&output.stderr)));
            return Err(anyhow::anyhow!("Conan install failed"));
        }
        
        print_success("Conan dependencies installed");
        Ok(())
    }
    
    fn build(&self, sources: &ProjectSources, deps: &[ResolvedDependency], args: &[String]) -> Result<()> {
        // Install dependencies first
        self.configure(sources, deps)?;
        
        // Conan typically generates CMake files or other build system files
        // Look for the generated build system
        if Path::new(&self.root).join("CMakeLists.txt").exists() {
            print_info("Found CMakeLists.txt, using CMake for build...");
            
            let mut cmd = Command::new("cmake");
            cmd.current_dir(&self.root);
            cmd.arg("-B").arg("build");
            cmd.arg("-S").arg(".");
            cmd.arg("-DCMAKE_TOOLCHAIN_FILE=conan_toolchain.cmake");
            cmd.arg("-DCMAKE_BUILD_TYPE=Release");
            
            cmd.output()
                .with_context(|| "Failed to configure with CMake")?;
            
            let mut build_cmd = Command::new("cmake");
            build_cmd.current_dir(&self.root);
            build_cmd.arg("--build").arg("build");
            build_cmd.args(args);
            
            let output = build_cmd.output()
                .with_context(|| "Failed to build with CMake")?;
            
            if !output.status.success() {
                print_error(&format!("Build failed:\n{}", String::from_utf8_lossy(&output.stderr)));
                return Err(anyhow::anyhow!("Build failed"));
            }
        } else {
            print_warning("No CMakeLists.txt found. Conan installed dependencies but no build system detected.");
            print_info("You may need to manually build with your configured build system.");
        }
        
        print_success("Build successful");
        Ok(())
    }
    
    fn run(&self, args: &[String]) -> Result<()> {
        print_warning("Conan is a package manager. Run the executable from the build directory.");
        print_info(&format!("Try: ./build/bin/<executable> {}", args.join(" ")));
        Ok(())
    }
    
    fn test(&self, _sources: &ProjectSources, _deps: &[ResolvedDependency]) -> Result<()> {
        // Conan can create test packages
        let mut cmd = Command::new("conan");
        cmd.current_dir(&self.root);
        cmd.arg("test");
        
        let output = cmd.output()
            .with_context(|| "Failed to run conan test")?;
        
        if !output.status.success() {
            print_warning("Conan test not configured or failed");
        }
        
        Ok(())
    }
    
    fn clean(&self) -> Result<()> {
        // Remove Conan build artifacts
        let paths_to_clean = vec!["build", "conan_cache", "conanbuildinfo.txt"];
        
        for path in paths_to_clean {
            let full_path = Path::new(&self.root).join(path);
            if full_path.exists() {
                if full_path.is_dir() {
                    std::fs::remove_dir_all(&full_path).ok();
                } else {
                    std::fs::remove_file(&full_path).ok();
                }
            }
        }
        
        print_success("Clean successful");
        Ok(())
    }
}
