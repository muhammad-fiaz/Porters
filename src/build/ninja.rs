//! Ninja build system support
//!
//! This module provides support for building projects using Ninja.
//! Ninja is a small build system focused on speed, typically used as a backend
//! for higher-level build systems like CMake or Meson.

use anyhow::{Context, Result};
use std::path::Path;
use std::process::Command;

use super::BuildSystem;
use crate::deps::ResolvedDependency;
use crate::scan::ProjectSources;
use crate::util::pretty::*;

/// Ninja build system implementation
///
/// Handles projects with build.ninja files, executing fast incremental builds.
pub struct NinjaBuildSystem {
    root: String,
}

impl NinjaBuildSystem {
    pub fn new(root: &str) -> Self {
        Self {
            root: root.to_string(),
        }
    }
}

impl BuildSystem for NinjaBuildSystem {
    fn name(&self) -> &str {
        "Ninja"
    }

    fn detect(root: &Path) -> bool {
        root.join("build.ninja").exists()
    }

    fn configure(&self, _sources: &ProjectSources, _deps: &[ResolvedDependency]) -> Result<()> {
        // Ninja doesn't have a separate configure step
        print_info("Ninja build file detected (build.ninja)");
        Ok(())
    }

    fn build(
        &self,
        _sources: &ProjectSources,
        _deps: &[ResolvedDependency],
        args: &[String],
    ) -> Result<()> {
        let mut cmd = Command::new("ninja");
        cmd.current_dir(&self.root);
        cmd.args(args);

        let output = cmd
            .output()
            .with_context(|| "Failed to run ninja. Is Ninja installed?")?;

        if !output.status.success() {
            print_error(&format!(
                "Ninja build failed:\n{}",
                String::from_utf8_lossy(&output.stderr)
            ));
            return Err(anyhow::anyhow!("Ninja build failed"));
        }

        print_success("Ninja build successful");
        Ok(())
    }

    fn run(&self, args: &[String]) -> Result<()> {
        print_warning(
            "Ninja doesn't define a standard run target. Please run the executable manually.",
        );
        print_info(&format!("Try: ./build/<executable> {}", args.join(" ")));
        Ok(())
    }

    fn test(&self, _sources: &ProjectSources, _deps: &[ResolvedDependency]) -> Result<()> {
        let mut cmd = Command::new("ninja");
        cmd.current_dir(&self.root);
        cmd.arg("test");

        let output = cmd.output().with_context(|| "Failed to run ninja test")?;

        if !output.status.success() {
            print_error("Tests failed");
            return Err(anyhow::anyhow!("Tests failed"));
        }

        Ok(())
    }

    fn clean(&self) -> Result<()> {
        let mut cmd = Command::new("ninja");
        cmd.current_dir(&self.root);
        cmd.arg("clean");

        cmd.output().with_context(|| "Failed to run ninja clean")?;

        print_success("Clean successful");
        Ok(())
    }
}
