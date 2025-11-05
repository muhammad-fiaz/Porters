//! Make build system support
//!
//! This module provides support for building projects using GNU Make or compatible make tools.
//! Detects projects with Makefile and executes make commands for building, testing, and cleaning.

use anyhow::{Context, Result};
use std::path::Path;
use std::process::Command;

use super::BuildSystem;
use crate::deps::ResolvedDependency;
use crate::scan::ProjectSources;
use crate::util::pretty::*;

/// Make build system implementation
///
/// Handles projects using Makefiles for build automation.
/// Executes make targets for build, test, and clean operations.
pub struct MakeBuildSystem {
    root: String,
}

impl MakeBuildSystem {
    pub fn new(root: &str) -> Self {
        Self {
            root: root.to_string(),
        }
    }
}

impl BuildSystem for MakeBuildSystem {
    fn name(&self) -> &str {
        "Make"
    }

    fn detect(root: &Path) -> bool {
        root.join("Makefile").exists() || root.join("makefile").exists()
    }

    fn configure(&self, _sources: &ProjectSources, _deps: &[ResolvedDependency]) -> Result<()> {
        // Make doesn't typically have a separate configure step
        Ok(())
    }

    fn build(
        &self,
        _sources: &ProjectSources,
        _deps: &[ResolvedDependency],
        args: &[String],
    ) -> Result<()> {
        print_build("Building with Make...");

        let mut cmd = Command::new("make");

        for arg in args {
            cmd.arg(arg);
        }

        let output = cmd
            .current_dir(&self.root)
            .output()
            .with_context(|| "Failed to run make")?;

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

    fn run(&self, _args: &[String]) -> Result<()> {
        // Try to run the 'run' target first
        let output = Command::new("make")
            .arg("run")
            .current_dir(&self.root)
            .output();

        if let Ok(output) = output
            && output.status.success()
        {
            return Ok(());
        }

        // If no 'run' target, try to find executable
        print_warning("No 'run' target found in Makefile");
        print_info("Please specify executable path or add 'run' target to Makefile");

        Ok(())
    }

    fn test(&self, _sources: &ProjectSources, _deps: &[ResolvedDependency]) -> Result<()> {
        print_build("Running tests with Make...");

        let output = Command::new("make")
            .arg("test")
            .current_dir(&self.root)
            .output()
            .with_context(|| "Failed to run make test")?;

        print!("{}", String::from_utf8_lossy(&output.stdout));

        if !output.status.success() {
            print_error("Tests failed");
            return Err(anyhow::anyhow!("Tests failed"));
        }

        Ok(())
    }

    fn clean(&self) -> Result<()> {
        print_build("Cleaning Make build...");

        let output = Command::new("make")
            .arg("clean")
            .current_dir(&self.root)
            .output()
            .with_context(|| "Failed to run make clean")?;

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
