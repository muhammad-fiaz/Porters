//! QMake build system support
//!
//! This module provides support for building projects using QMake.
//! QMake is Qt's build system generator, creating platform-specific
//! Makefiles for Qt and C++ projects.

use anyhow::{Context, Result};
use std::path::Path;
use std::process::Command;

use super::BuildSystem;
use crate::deps::ResolvedDependency;
use crate::scan::ProjectSources;
use crate::util::pretty::*;

/// QMake build system implementation
///
/// Handles projects with .pro files, generating and executing
/// Makefiles for Qt-based applications.
pub struct QMakeBuildSystem {
    root: String,
}

impl QMakeBuildSystem {
    pub fn new(root: &str) -> Self {
        Self {
            root: root.to_string(),
        }
    }
}

impl BuildSystem for QMakeBuildSystem {
    fn name(&self) -> &str {
        "QMake"
    }
    
    fn detect(root: &Path) -> bool {
        // Look for .pro files (Qt project files)
        if let Ok(entries) = std::fs::read_dir(root) {
            for entry in entries.flatten() {
                if let Some(ext) = entry.path().extension() {
                    if ext == "pro" {
                        return true;
                    }
                }
            }
        }
        false
    }
    
    fn configure(&self, _sources: &ProjectSources, _deps: &[ResolvedDependency]) -> Result<()> {
        print_build("Generating Makefile with QMake...");
        
        let mut cmd = Command::new("qmake");
        cmd.current_dir(&self.root);
        
        let output = cmd.output()
            .with_context(|| "Failed to run qmake. Is Qt/QMake installed?")?;
        
        if !output.status.success() {
            print_error(&format!("QMake failed:\n{}", String::from_utf8_lossy(&output.stderr)));
            return Err(anyhow::anyhow!("QMake failed"));
        }
        
        print_success("QMake successful");
        Ok(())
    }
    
    fn build(&self, sources: &ProjectSources, deps: &[ResolvedDependency], args: &[String]) -> Result<()> {
        // Generate Makefile if it doesn't exist
        if !Path::new(&self.root).join("Makefile").exists() {
            self.configure(sources, deps)?;
        }
        
        let mut cmd = if cfg!(windows) {
            Command::new("nmake")  // Use nmake on Windows
        } else {
            Command::new("make")
        };
        
        cmd.current_dir(&self.root);
        cmd.args(args);
        
        let output = cmd.output()
            .with_context(|| "Failed to build")?;
        
        if !output.status.success() {
            print_error(&format!("Build failed:\n{}", String::from_utf8_lossy(&output.stderr)));
            return Err(anyhow::anyhow!("Build failed"));
        }
        
        print_success("Build successful");
        Ok(())
    }
    
    fn run(&self, args: &[String]) -> Result<()> {
        print_warning("QMake doesn't define a standard run target");
        print_info(&format!("Run the executable from the build directory with: {}", args.join(" ")));
        Ok(())
    }
    
    fn test(&self, _sources: &ProjectSources, _deps: &[ResolvedDependency]) -> Result<()> {
        let cmd_name = if cfg!(windows) { "nmake" } else { "make" };
        
        let mut cmd = Command::new(cmd_name);
        cmd.current_dir(&self.root);
        cmd.arg("check");
        
        let output = cmd.output()
            .with_context(|| "Failed to run tests")?;
        
        if !output.status.success() {
            print_error("Tests failed");
            return Err(anyhow::anyhow!("Tests failed"));
        }
        
        Ok(())
    }
    
    fn clean(&self) -> Result<()> {
        let cmd_name = if cfg!(windows) { "nmake" } else { "make" };
        
        let mut cmd = Command::new(cmd_name);
        cmd.current_dir(&self.root);
        cmd.arg("clean");
        
        cmd.output()
            .with_context(|| "Failed to clean")?;
        
        print_success("Clean successful");
        Ok(())
    }
}
