//! Bazel build system support
//!
//! This module provides support for building projects using Bazel.
//! Bazel is Google's build system designed for large-scale, multi-language
//! projects with hermetic builds and distributed caching.

use anyhow::{Context, Result};
use std::path::Path;
use std::process::Command;

use super::BuildSystem;
use crate::deps::ResolvedDependency;
use crate::scan::ProjectSources;
use crate::util::pretty::*;

/// Bazel build system implementation
///
/// Handles projects with WORKSPACE and BUILD files, providing reproducible
/// and scalable builds for large codebases.
pub struct BazelBuildSystem {
    root: String,
}

impl BazelBuildSystem {
    pub fn new(root: &str) -> Self {
        Self {
            root: root.to_string(),
        }
    }
}

impl BuildSystem for BazelBuildSystem {
    fn name(&self) -> &str {
        "Bazel"
    }
    
    fn detect(root: &Path) -> bool {
        root.join("BUILD").exists() || 
        root.join("BUILD.bazel").exists() ||
        root.join("WORKSPACE").exists() ||
        root.join("WORKSPACE.bazel").exists()
    }
    
    fn configure(&self, _sources: &ProjectSources, _deps: &[ResolvedDependency]) -> Result<()> {
        // Bazel doesn't have a separate configure step
        print_info("Bazel workspace detected");
        Ok(())
    }
    
    fn build(&self, _sources: &ProjectSources, _deps: &[ResolvedDependency], args: &[String]) -> Result<()> {
        let mut cmd = Command::new("bazel");
        cmd.current_dir(&self.root);
        cmd.arg("build");
        
        // Default target if none specified
        if args.is_empty() {
            cmd.arg("//...");
        } else {
            cmd.args(args);
        }
        
        let output = cmd.output()
            .with_context(|| "Failed to run bazel. Is Bazel installed?")?;
        
        if !output.status.success() {
            print_error(&format!("Bazel build failed:\n{}", String::from_utf8_lossy(&output.stderr)));
            return Err(anyhow::anyhow!("Bazel build failed"));
        }
        
        print_success("Bazel build successful");
        Ok(())
    }
    
    fn run(&self, args: &[String]) -> Result<()> {
        let mut cmd = Command::new("bazel");
        cmd.current_dir(&self.root);
        cmd.arg("run");
        
        if args.is_empty() {
            print_warning("No target specified for bazel run");
            print_info("Usage: porters run <bazel-target>");
            return Ok(());
        }
        
        cmd.args(args);
        
        let output = cmd.output()
            .with_context(|| "Failed to run bazel run")?;
        
        if !output.status.success() {
            print_error("Bazel run failed");
            return Err(anyhow::anyhow!("Bazel run failed"));
        }
        
        Ok(())
    }
    
    fn test(&self, _sources: &ProjectSources, _deps: &[ResolvedDependency]) -> Result<()> {
        let mut cmd = Command::new("bazel");
        cmd.current_dir(&self.root);
        cmd.arg("test");
        cmd.arg("//...");
        
        let output = cmd.output()
            .with_context(|| "Failed to run bazel test")?;
        
        if !output.status.success() {
            print_error("Tests failed");
            return Err(anyhow::anyhow!("Tests failed"));
        }
        
        Ok(())
    }
    
    fn clean(&self) -> Result<()> {
        let mut cmd = Command::new("bazel");
        cmd.current_dir(&self.root);
        cmd.arg("clean");
        
        cmd.output()
            .with_context(|| "Failed to run bazel clean")?;
        
        print_success("Clean successful");
        Ok(())
    }
}
