//! vcpkg package manager integration
//!
//! This module provides integration with vcpkg, Microsoft's C/C++ package manager.
//! Handles dependency installation and build toolchain integration through vcpkg,
//! primarily used with CMake projects.

use anyhow::{Context, Result};
use std::path::Path;
use std::process::Command;

use super::BuildSystem;
use crate::deps::ResolvedDependency;
use crate::scan::ProjectSources;
use crate::util::pretty::*;

/// vcpkg build system implementation
///
/// Manages C/C++ dependencies through vcpkg and integrates with
/// CMake toolchains for cross-platform builds.
pub struct VcpkgBuildSystem {
    root: String,
}

impl VcpkgBuildSystem {
    pub fn new(root: &str) -> Self {
        Self {
            root: root.to_string(),
        }
    }
}

impl BuildSystem for VcpkgBuildSystem {
    fn name(&self) -> &str {
        "vcpkg"
    }
    
    fn detect(root: &Path) -> bool {
        root.join("vcpkg.json").exists() || root.join("vcpkg-configuration.json").exists()
    }
    
    fn configure(&self, _sources: &ProjectSources, _deps: &[ResolvedDependency]) -> Result<()> {
        print_build("Installing dependencies with vcpkg...");
        
        let mut cmd = Command::new("vcpkg");
        cmd.current_dir(&self.root);
        cmd.arg("install");
        
        let output = cmd.output()
            .with_context(|| "Failed to run vcpkg. Is vcpkg installed and in PATH?")?;
        
        if !output.status.success() {
            print_error(&format!("vcpkg install failed:\n{}", String::from_utf8_lossy(&output.stderr)));
            return Err(anyhow::anyhow!("vcpkg install failed"));
        }
        
        print_success("vcpkg dependencies installed");
        Ok(())
    }
    
    fn build(&self, sources: &ProjectSources, deps: &[ResolvedDependency], args: &[String]) -> Result<()> {
        // Install dependencies first
        self.configure(sources, deps)?;
        
        // vcpkg typically works with CMake
        if Path::new(&self.root).join("CMakeLists.txt").exists() {
            print_info("Found CMakeLists.txt, using CMake with vcpkg toolchain...");
            
            // Get vcpkg root
            let vcpkg_root = std::env::var("VCPKG_ROOT")
                .unwrap_or_else(|_| {
                    print_warning("VCPKG_ROOT not set, using default toolchain path");
                    "vcpkg".to_string()
                });
            
            let toolchain = Path::new(&vcpkg_root)
                .join("scripts")
                .join("buildsystems")
                .join("vcpkg.cmake");
            
            let mut cmd = Command::new("cmake");
            cmd.current_dir(&self.root);
            cmd.arg("-B").arg("build");
            cmd.arg("-S").arg(".");
            
            if toolchain.exists() {
                cmd.arg(format!("-DCMAKE_TOOLCHAIN_FILE={}", toolchain.display()));
            }
            
            cmd.arg("-DCMAKE_BUILD_TYPE=Release");
            
            let output = cmd.output()
                .with_context(|| "Failed to configure with CMake")?;
            
            if !output.status.success() {
                print_error(&format!("CMake configure failed:\n{}", String::from_utf8_lossy(&output.stderr)));
                return Err(anyhow::anyhow!("CMake configure failed"));
            }
            
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
            print_warning("No CMakeLists.txt found. vcpkg installed dependencies but no build system detected.");
            print_info("You may need to manually build with your configured build system.");
        }
        
        print_success("Build successful");
        Ok(())
    }
    
    fn run(&self, args: &[String]) -> Result<()> {
        print_warning("vcpkg is a package manager. Run the executable from the build directory.");
        print_info(&format!("Try: ./build/<executable> {}", args.join(" ")));
        Ok(())
    }
    
    fn test(&self, _sources: &ProjectSources, _deps: &[ResolvedDependency]) -> Result<()> {
        // vcpkg doesn't have built-in test support, defer to CMake if available
        if Path::new(&self.root).join("CMakeLists.txt").exists() {
            let mut cmd = Command::new("ctest");
            cmd.current_dir(Path::new(&self.root).join("build"));
            
            let output = cmd.output()
                .with_context(|| "Failed to run tests")?;
            
            if !output.status.success() {
                print_error("Tests failed");
                return Err(anyhow::anyhow!("Tests failed"));
            }
        } else {
            print_warning("No test configuration found");
        }
        
        Ok(())
    }
    
    fn clean(&self) -> Result<()> {
        let build_dir = Path::new(&self.root).join("build");
        if build_dir.exists() {
            std::fs::remove_dir_all(&build_dir).ok();
        }
        
        print_success("Clean successful");
        Ok(())
    }
}
