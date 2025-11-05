//! XMake build system support
//!
//! This module provides support for building projects using XMake.
//! XMake is a modern Lua-based build system for C/C++ projects with
//! cross-platform support and package management.

use anyhow::{Context, Result};
use std::path::Path;
use std::process::Command;

use super::BuildSystem;
use crate::deps::ResolvedDependency;
use crate::scan::ProjectSources;
use crate::util::pretty::*;

/// XMake build system implementation
///
/// Handles projects with xmake.lua files, providing modern build automation
/// with integrated dependency management.
pub struct XMakeBuildSystem {
    root: String,
}

impl XMakeBuildSystem {
    pub fn new(root: &str) -> Self {
        Self {
            root: root.to_string(),
        }
    }
}

impl BuildSystem for XMakeBuildSystem {
    fn name(&self) -> &str {
        "XMake"
    }

    fn detect(root: &Path) -> bool {
        root.join("xmake.lua").exists()
    }

    fn configure(&self, _sources: &ProjectSources, _deps: &[ResolvedDependency]) -> Result<()> {
        print_build("Configuring XMake...");

        let output = Command::new("xmake")
            .arg("config")
            .current_dir(&self.root)
            .output()
            .with_context(|| "Failed to run xmake config")?;

        if !output.status.success() {
            print_error(&format!(
                "XMake configuration failed:\n{}",
                String::from_utf8_lossy(&output.stderr)
            ));
            return Err(anyhow::anyhow!("XMake configuration failed"));
        }

        Ok(())
    }

    fn build(
        &self,
        _sources: &ProjectSources,
        _deps: &[ResolvedDependency],
        args: &[String],
    ) -> Result<()> {
        print_build("Building with XMake...");

        let mut cmd = Command::new("xmake");
        for arg in args {
            cmd.arg(arg);
        }

        let output = cmd
            .current_dir(&self.root)
            .output()
            .with_context(|| "Failed to run xmake")?;

        if !output.status.success() {
            print_error(&format!(
                "Build failed:\n{}",
                String::from_utf8_lossy(&output.stderr)
            ));
            return Err(anyhow::anyhow!("Build failed"));
        }

        print!("{}", String::from_utf8_lossy(&output.stdout));

        Ok(())
    }

    fn run(&self, args: &[String]) -> Result<()> {
        let mut cmd = Command::new("xmake");
        cmd.arg("run");

        for arg in args {
            cmd.arg(arg);
        }

        let status = cmd
            .current_dir(&self.root)
            .status()
            .with_context(|| "Failed to run xmake run")?;

        if !status.success() {
            return Err(anyhow::anyhow!("Execution failed with status: {}", status));
        }

        Ok(())
    }

    fn test(&self, sources: &ProjectSources, deps: &[ResolvedDependency]) -> Result<()> {
        self.build(sources, deps, &[])?;

        print_build("Running tests with XMake...");

        let output = Command::new("xmake")
            .arg("test")
            .current_dir(&self.root)
            .output()
            .with_context(|| "Failed to run xmake test")?;

        print!("{}", String::from_utf8_lossy(&output.stdout));

        if !output.status.success() {
            print_error("Tests failed");
            return Err(anyhow::anyhow!("Tests failed"));
        }

        Ok(())
    }

    fn clean(&self) -> Result<()> {
        print_build("Cleaning XMake build...");

        let output = Command::new("xmake")
            .arg("clean")
            .current_dir(&self.root)
            .output()
            .with_context(|| "Failed to run xmake clean")?;

        if !output.status.success() {
            print_error(&format!(
                "Clean failed:\n{}",
                String::from_utf8_lossy(&output.stderr)
            ));
            return Err(anyhow::anyhow!("Clean failed"));
        }

        Ok(())
    }
}
