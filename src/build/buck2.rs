//! Buck2 build system support
//!
//! This module provides support for building projects using Buck2.
//! Buck2 is Meta's next-generation build system, designed for large monorepos
//! with fast, reproducible builds and advanced caching.

use anyhow::{Context, Result};
use std::path::Path;
use std::process::Command;

use super::BuildSystem;
use crate::deps::ResolvedDependency;
use crate::scan::ProjectSources;
use crate::util::pretty::*;

/// Buck2 build system implementation
///
/// Handles projects with BUCK files, providing incremental builds
/// and distributed execution capabilities.
pub struct Buck2BuildSystem {
    root: String,
}

impl Buck2BuildSystem {
    pub fn new(root: &str) -> Self {
        Self {
            root: root.to_string(),
        }
    }
}

impl BuildSystem for Buck2BuildSystem {
    fn name(&self) -> &str {
        "Buck2"
    }
    
    fn detect(root: &Path) -> bool {
        root.join(".buckconfig").exists() || root.join("BUCK").exists()
    }
    
    fn configure(&self, _sources: &ProjectSources, _deps: &[ResolvedDependency]) -> Result<()> {
        print_info("Buck2 configuration detected");
        Ok(())
    }
    
    fn build(&self, _sources: &ProjectSources, _deps: &[ResolvedDependency], args: &[String]) -> Result<()> {
        let mut cmd = Command::new("buck2");
        cmd.current_dir(&self.root);
        cmd.arg("build");
        
        if args.is_empty() {
            cmd.arg("//...");
        } else {
            cmd.args(args);
        }
        
        let output = cmd.output()
            .with_context(|| "Failed to run buck2. Is Buck2 installed?")?;
        
        if !output.status.success() {
            print_error(&format!("Buck2 build failed:\n{}", String::from_utf8_lossy(&output.stderr)));
            return Err(anyhow::anyhow!("Buck2 build failed"));
        }
        
        print_success("Buck2 build successful");
        Ok(())
    }
    
    fn run(&self, args: &[String]) -> Result<()> {
        let mut cmd = Command::new("buck2");
        cmd.current_dir(&self.root);
        cmd.arg("run");
        
        if args.is_empty() {
            print_warning("No target specified for buck2 run");
            print_info("Usage: porters run <buck2-target>");
            return Ok(());
        }
        
        cmd.args(args);
        
        let output = cmd.output()
            .with_context(|| "Failed to run buck2 run")?;
        
        if !output.status.success() {
            print_error("Buck2 run failed");
            return Err(anyhow::anyhow!("Buck2 run failed"));
        }
        
        Ok(())
    }
    
    fn test(&self, _sources: &ProjectSources, _deps: &[ResolvedDependency]) -> Result<()> {
        let mut cmd = Command::new("buck2");
        cmd.current_dir(&self.root);
        cmd.arg("test");
        cmd.arg("//...");
        
        let output = cmd.output()
            .with_context(|| "Failed to run buck2 test")?;
        
        if !output.status.success() {
            print_error("Tests failed");
            return Err(anyhow::anyhow!("Tests failed"));
        }
        
        Ok(())
    }
    
    fn clean(&self) -> Result<()> {
        let mut cmd = Command::new("buck2");
        cmd.current_dir(&self.root);
        cmd.arg("clean");
        
        cmd.output()
            .with_context(|| "Failed to run buck2 clean")?;
        
        print_success("Clean successful");
        Ok(())
    }
}
