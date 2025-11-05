//! Autotools build system support
//!
//! This module provides support for building projects using GNU Autotools (autoconf/automake).
//! Detects projects with configure or configure.ac files and executes the standard
//! ./configure && make workflow.

use anyhow::{Context, Result};
use std::path::Path;
use std::process::Command;

use super::BuildSystem;
use crate::deps::ResolvedDependency;
use crate::scan::ProjectSources;
use crate::util::pretty::*;

/// Autotools build system implementation
///
/// Handles projects that use GNU Autotools (autoconf, automake, libtool).
/// Executes the standard configuration and build process.
pub struct AutotoolsBuildSystem {
    root: String,
}

impl AutotoolsBuildSystem {
    pub fn new(root: &str) -> Self {
        Self {
            root: root.to_string(),
        }
    }
}

impl BuildSystem for AutotoolsBuildSystem {
    fn name(&self) -> &str {
        "Autotools"
    }

    fn detect(root: &Path) -> bool {
        root.join("configure").exists() || root.join("configure.ac").exists()
    }

    fn configure(&self, _sources: &ProjectSources, deps: &[ResolvedDependency]) -> Result<()> {
        print_build("Configuring Autotools...");

        // Run autoreconf if configure doesn't exist
        if !Path::new(&self.root).join("configure").exists() {
            print_info("Running autoreconf...");
            let output = Command::new("autoreconf")
                .arg("-i")
                .current_dir(&self.root)
                .output()
                .with_context(|| "Failed to run autoreconf")?;

            if !output.status.success() {
                print_warning("autoreconf failed, trying to continue...");
            }
        }

        let mut cmd = Command::new("./configure");
        cmd.current_dir(&self.root);

        // Add dependency paths
        for dep in deps {
            for inc_path in &dep.include_paths {
                cmd.env("CPPFLAGS", format!("-I{}", inc_path.display()));
            }
            for lib_path in &dep.lib_paths {
                cmd.env("LDFLAGS", format!("-L{}", lib_path.display()));
            }
        }

        let output = cmd.output().with_context(|| "Failed to run ./configure")?;

        if !output.status.success() {
            print_error(&format!(
                "Configure failed:\n{}",
                String::from_utf8_lossy(&output.stderr)
            ));
            return Err(anyhow::anyhow!("Configure failed"));
        }

        print_success("Configure successful");
        Ok(())
    }

    fn build(
        &self,
        sources: &ProjectSources,
        deps: &[ResolvedDependency],
        args: &[String],
    ) -> Result<()> {
        // Configure first if needed
        if !Path::new(&self.root).join("Makefile").exists() {
            self.configure(sources, deps)?;
        }

        let mut cmd = Command::new("make");
        cmd.current_dir(&self.root);
        cmd.args(args);

        let output = cmd.output().with_context(|| "Failed to run make")?;

        if !output.status.success() {
            print_error(&format!(
                "Make failed:\n{}",
                String::from_utf8_lossy(&output.stderr)
            ));
            return Err(anyhow::anyhow!("Make failed"));
        }

        print_success("Build successful");
        Ok(())
    }

    fn run(&self, args: &[String]) -> Result<()> {
        print_warning(
            "Autotools doesn't define a standard run target. Please run the installed executable.",
        );
        print_info(&format!(
            "Try: make install, then run the executable with: {}",
            args.join(" ")
        ));
        Ok(())
    }

    fn test(&self, _sources: &ProjectSources, _deps: &[ResolvedDependency]) -> Result<()> {
        let mut cmd = Command::new("make");
        cmd.current_dir(&self.root);
        cmd.arg("check");

        let output = cmd.output().with_context(|| "Failed to run make check")?;

        if !output.status.success() {
            print_error("Tests failed");
            return Err(anyhow::anyhow!("Tests failed"));
        }

        Ok(())
    }

    fn clean(&self) -> Result<()> {
        let mut cmd = Command::new("make");
        cmd.current_dir(&self.root);
        cmd.arg("clean");

        cmd.output().with_context(|| "Failed to run make clean")?;

        print_success("Clean successful");
        Ok(())
    }
}
