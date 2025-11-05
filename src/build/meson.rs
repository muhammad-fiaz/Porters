//! Meson build system support
//!
//! This module provides support for building projects using Meson.
//! Meson is a modern build system designed for speed and usability,
//! generating Ninja build files for fast compilation.

use anyhow::{Context, Result};
use std::path::Path;
use std::process::Command;

use super::BuildSystem;
use crate::deps::ResolvedDependency;
use crate::scan::ProjectSources;
use crate::util::pretty::*;

/// Meson build system implementation
///
/// Handles projects with meson.build files, configuring and building
/// using Meson and Ninja.
pub struct MesonBuildSystem {
    root: String,
}

impl MesonBuildSystem {
    pub fn new(root: &str) -> Self {
        Self {
            root: root.to_string(),
        }
    }
}

impl BuildSystem for MesonBuildSystem {
    fn name(&self) -> &str {
        "Meson"
    }

    fn detect(root: &Path) -> bool {
        root.join("meson.build").exists()
    }

    fn configure(&self, _sources: &ProjectSources, _deps: &[ResolvedDependency]) -> Result<()> {
        print_build("Configuring Meson...");

        let output = Command::new("meson")
            .arg("setup")
            .arg("build")
            .current_dir(&self.root)
            .output()
            .with_context(|| "Failed to run meson setup")?;

        if !output.status.success() {
            print_error(&format!(
                "Meson configuration failed:\n{}",
                String::from_utf8_lossy(&output.stderr)
            ));
            return Err(anyhow::anyhow!("Meson configuration failed"));
        }

        Ok(())
    }

    fn build(
        &self,
        sources: &ProjectSources,
        deps: &[ResolvedDependency],
        args: &[String],
    ) -> Result<()> {
        if !Path::new("build").exists() {
            self.configure(sources, deps)?;
        }

        print_build("Building with Meson...");

        let mut cmd = Command::new("meson");
        cmd.arg("compile");
        cmd.arg("-C").arg("build");

        for arg in args {
            cmd.arg(arg);
        }

        let output = cmd
            .current_dir(&self.root)
            .output()
            .with_context(|| "Failed to run meson compile")?;

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
        let mut cmd = Command::new("meson");
        cmd.arg("devenv");
        cmd.arg("-C").arg("build");

        for arg in args {
            cmd.arg(arg);
        }

        let status = cmd
            .current_dir(&self.root)
            .status()
            .with_context(|| "Failed to run meson devenv")?;

        if !status.success() {
            return Err(anyhow::anyhow!("Execution failed with status: {}", status));
        }

        Ok(())
    }

    fn test(&self, sources: &ProjectSources, deps: &[ResolvedDependency]) -> Result<()> {
        self.build(sources, deps, &[])?;

        print_build("Running tests with Meson...");

        let output = Command::new("meson")
            .arg("test")
            .arg("-C")
            .arg("build")
            .current_dir(&self.root)
            .output()
            .with_context(|| "Failed to run meson test")?;

        print!("{}", String::from_utf8_lossy(&output.stdout));

        if !output.status.success() {
            print_error("Tests failed");
            return Err(anyhow::anyhow!("Tests failed"));
        }

        Ok(())
    }

    fn clean(&self) -> Result<()> {
        print_build("Cleaning Meson build...");

        if Path::new("build").exists() {
            std::fs::remove_dir_all("build").with_context(|| "Failed to remove build directory")?;
        }

        Ok(())
    }
}
