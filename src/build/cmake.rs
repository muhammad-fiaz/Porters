//! CMake build system support
//!
//! This module provides support for building projects using CMake.
//! Detects projects with CMakeLists.txt and handles configuration, generation,
//! and compilation using CMake and the underlying build tool (Make, Ninja, etc.).

use anyhow::{Context, Result};
use std::path::Path;
use std::process::Command;

use super::BuildSystem;
use crate::deps::ResolvedDependency;
use crate::scan::ProjectSources;
use crate::util::pretty::*;

/// CMake build system implementation
///
/// Handles projects using CMake for cross-platform build configuration.
/// Generates native build files and invokes the appropriate build tool.
pub struct CMakeBuildSystem {
    root: String,
}

impl CMakeBuildSystem {
    pub fn new(root: &str) -> Self {
        Self {
            root: root.to_string(),
        }
    }
}

impl BuildSystem for CMakeBuildSystem {
    fn name(&self) -> &str {
        "CMake"
    }

    fn detect(root: &Path) -> bool {
        root.join("CMakeLists.txt").exists()
    }

    fn configure(&self, _sources: &ProjectSources, deps: &[ResolvedDependency]) -> Result<()> {
        print_build("Configuring CMake...");

        let mut cmd = Command::new("cmake");
        cmd.arg("-B").arg("build");
        cmd.arg("-S").arg(&self.root);

        // Add dependency include paths
        for dep in deps {
            for inc_path in &dep.include_paths {
                cmd.arg(format!("-DCMAKE_PREFIX_PATH={}", inc_path.display()));
            }
        }

        let output = cmd.output().with_context(|| "Failed to run cmake")?;

        if !output.status.success() {
            print_error(&format!(
                "CMake configuration failed:\n{}",
                String::from_utf8_lossy(&output.stderr)
            ));
            return Err(anyhow::anyhow!("CMake configuration failed"));
        }

        Ok(())
    }

    fn build(
        &self,
        sources: &ProjectSources,
        deps: &[ResolvedDependency],
        args: &[String],
    ) -> Result<()> {
        // Configure first if needed
        if !Path::new("build").exists() {
            self.configure(sources, deps)?;
        }

        print_build("Building with CMake...");

        let mut cmd = Command::new("cmake");
        cmd.arg("--build").arg("build");

        for arg in args {
            cmd.arg(arg);
        }

        let output = cmd
            .output()
            .with_context(|| "Failed to run cmake --build")?;

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
        // Find executable in build directory
        let build_dir = Path::new("build");

        // Look for executable (this is a simple heuristic)
        let exe = if cfg!(windows) {
            // Look for .exe files
            std::fs::read_dir(build_dir)?
                .filter_map(|e| e.ok())
                .find(|e| {
                    e.path()
                        .extension()
                        .map(|ext| ext == "exe")
                        .unwrap_or(false)
                })
        } else {
            // Look for executable files
            std::fs::read_dir(build_dir)?
                .filter_map(|e| e.ok())
                .find(|e| {
                    e.metadata()
                        .ok()
                        .map(|m| m.permissions().mode() & 0o111 != 0)
                        .unwrap_or(false)
                })
        };

        if let Some(exe) = exe {
            let mut cmd = Command::new(exe.path());
            for arg in args {
                cmd.arg(arg);
            }

            let status = cmd.status().with_context(|| "Failed to run executable")?;

            if !status.success() {
                return Err(anyhow::anyhow!("Execution failed with status: {}", status));
            }
        } else {
            print_warning("No executable found in build directory");
        }

        Ok(())
    }

    fn test(&self, sources: &ProjectSources, deps: &[ResolvedDependency]) -> Result<()> {
        // Build first
        self.build(sources, deps, &[])?;

        print_build("Running tests with CTest...");

        let output = Command::new("ctest")
            .arg("--test-dir")
            .arg("build")
            .arg("--output-on-failure")
            .output()
            .with_context(|| "Failed to run ctest")?;

        print!("{}", String::from_utf8_lossy(&output.stdout));

        if !output.status.success() {
            print_error("Tests failed");
            return Err(anyhow::anyhow!("Tests failed"));
        }

        Ok(())
    }

    fn clean(&self) -> Result<()> {
        print_build("Cleaning CMake build...");

        if Path::new("build").exists() {
            std::fs::remove_dir_all("build").with_context(|| "Failed to remove build directory")?;
        }

        Ok(())
    }
}

#[cfg(unix)]
use std::os::unix::fs::PermissionsExt;

#[cfg(not(unix))]
trait PermissionsExt {
    fn mode(&self) -> u32 {
        0
    }
}

#[cfg(not(unix))]
impl PermissionsExt for std::fs::Permissions {}
