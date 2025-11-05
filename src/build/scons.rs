//! SCons build system support
//!
//! This module provides support for building projects using SCons.
//! SCons is a Python-based build tool that serves as an improved
//! alternative to Make with automatic dependency analysis.

use anyhow::{Context, Result};
use std::path::Path;
use std::process::Command;

use super::BuildSystem;
use crate::deps::ResolvedDependency;
use crate::scan::ProjectSources;
use crate::util::pretty::*;

/// SCons build system implementation
///
/// Handles projects with SConstruct files, providing Python-based
/// build configuration and execution.
pub struct SConsBuildSystem {
    root: String,
}

impl SConsBuildSystem {
    pub fn new(root: &str) -> Self {
        Self {
            root: root.to_string(),
        }
    }
}

impl BuildSystem for SConsBuildSystem {
    fn name(&self) -> &str {
        "SCons"
    }

    fn detect(root: &Path) -> bool {
        root.join("SConstruct").exists() || root.join("SConscript").exists()
    }

    fn configure(&self, _sources: &ProjectSources, _deps: &[ResolvedDependency]) -> Result<()> {
        // SCons doesn't have a separate configure step
        print_info("SCons build file detected (SConstruct)");
        Ok(())
    }

    fn build(
        &self,
        _sources: &ProjectSources,
        _deps: &[ResolvedDependency],
        args: &[String],
    ) -> Result<()> {
        let mut cmd = Command::new("scons");
        cmd.current_dir(&self.root);
        cmd.args(args);

        let output = cmd
            .output()
            .with_context(|| "Failed to run scons. Is SCons installed?")?;

        if !output.status.success() {
            print_error(&format!(
                "SCons build failed:\n{}",
                String::from_utf8_lossy(&output.stderr)
            ));
            return Err(anyhow::anyhow!("SCons build failed"));
        }

        print_success("SCons build successful");
        Ok(())
    }

    fn run(&self, args: &[String]) -> Result<()> {
        let mut cmd = Command::new("scons");
        cmd.current_dir(&self.root);
        cmd.arg("run");
        cmd.args(args);

        let output = cmd.output().with_context(|| "Failed to run scons run")?;

        if !output.status.success() {
            print_warning("No 'run' target defined in SConstruct");
            print_info("Try running the executable manually from the build directory");
        }

        Ok(())
    }

    fn test(&self, _sources: &ProjectSources, _deps: &[ResolvedDependency]) -> Result<()> {
        let mut cmd = Command::new("scons");
        cmd.current_dir(&self.root);
        cmd.arg("test");

        let output = cmd.output().with_context(|| "Failed to run scons test")?;

        if !output.status.success() {
            print_error("Tests failed");
            return Err(anyhow::anyhow!("Tests failed"));
        }

        Ok(())
    }

    fn clean(&self) -> Result<()> {
        let mut cmd = Command::new("scons");
        cmd.current_dir(&self.root);
        cmd.arg("-c");

        cmd.output().with_context(|| "Failed to run scons clean")?;

        print_success("Clean successful");
        Ok(())
    }
}
