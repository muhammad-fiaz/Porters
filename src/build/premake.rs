//! Premake build system support
//!
//! This module provides support for building projects using Premake.
//! Premake is a Lua-based build configuration tool that generates
//! project files for various IDEs and build systems.

use anyhow::{Context, Result};
use std::path::Path;
use std::process::Command;

use super::BuildSystem;
use crate::deps::ResolvedDependency;
use crate::scan::ProjectSources;
use crate::util::pretty::*;

/// Premake build system implementation
///
/// Handles projects with premake5.lua files, generating IDE projects
/// and executing builds through the target build system.
pub struct PremakeBuildSystem {
    root: String,
}

impl PremakeBuildSystem {
    pub fn new(root: &str) -> Self {
        Self {
            root: root.to_string(),
        }
    }
}

impl BuildSystem for PremakeBuildSystem {
    fn name(&self) -> &str {
        "Premake"
    }
    
    fn detect(root: &Path) -> bool {
        root.join("premake5.lua").exists() || root.join("premake4.lua").exists()
    }
    
    fn configure(&self, _sources: &ProjectSources, _deps: &[ResolvedDependency]) -> Result<()> {
        print_build("Generating project files with Premake...");
        
        // Detect platform and generate appropriate project files
        let action = if cfg!(windows) {
            "vs2022"  // Visual Studio 2022
        } else if cfg!(target_os = "macos") {
            "xcode4"
        } else {
            "gmake2"
        };
        
        let mut cmd = Command::new("premake5");
        cmd.current_dir(&self.root);
        cmd.arg(action);
        
        let output = cmd.output()
            .with_context(|| "Failed to run premake5. Is Premake installed?")?;
        
        if !output.status.success() {
            print_error(&format!("Premake generation failed:\n{}", String::from_utf8_lossy(&output.stderr)));
            return Err(anyhow::anyhow!("Premake generation failed"));
        }
        
        print_success(&format!("Generated {} project files", action));
        Ok(())
    }
    
    fn build(&self, sources: &ProjectSources, deps: &[ResolvedDependency], args: &[String]) -> Result<()> {
        // Generate project files first if not already done
        if cfg!(windows) && !Path::new(&self.root).join("*.sln").exists() {
            self.configure(sources, deps)?;
        } else if !cfg!(windows) && !Path::new(&self.root).join("Makefile").exists() {
            self.configure(sources, deps)?;
        }
        
        // Build using the generated project files
        let mut cmd = if cfg!(windows) {
            Command::new("msbuild")
        } else {
            Command::new("make")
        };
        
        cmd.current_dir(&self.root);
        cmd.args(args);
        
        let output = cmd.output()
            .with_context(|| "Failed to build project")?;
        
        if !output.status.success() {
            print_error(&format!("Build failed:\n{}", String::from_utf8_lossy(&output.stderr)));
            return Err(anyhow::anyhow!("Build failed"));
        }
        
        print_success("Build successful");
        Ok(())
    }
    
    fn run(&self, args: &[String]) -> Result<()> {
        print_warning("Premake generates project files. Run the executable from the build output.");
        print_info(&format!("Try: ./bin/Debug/<executable> {}", args.join(" ")));
        Ok(())
    }
    
    fn test(&self, _sources: &ProjectSources, _deps: &[ResolvedDependency]) -> Result<()> {
        print_warning("Premake doesn't define a standard test target");
        print_info("Please configure tests in your premake5.lua");
        Ok(())
    }
    
    fn clean(&self) -> Result<()> {
        if cfg!(windows) {
            let mut cmd = Command::new("msbuild");
            cmd.current_dir(&self.root);
            cmd.arg("/t:Clean");
            cmd.output().ok();
        } else {
            let mut cmd = Command::new("make");
            cmd.current_dir(&self.root);
            cmd.arg("clean");
            cmd.output().ok();
        }
        
        print_success("Clean successful");
        Ok(())
    }
}
