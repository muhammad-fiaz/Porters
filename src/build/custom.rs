//! Custom build system support
//!
//! This module provides support for projects with custom build commands.
//! Allows users to define their own build, test, run, and clean commands
//! in porters.toml for projects that don't use standard build systems.

use anyhow::{Context, Result};
use std::path::Path;
use std::process::Command;

use super::BuildSystem;
use crate::config::CustomBuild;
use crate::deps::ResolvedDependency;
use crate::scan::ProjectSources;
use crate::util::pretty::*;

/// Custom build system implementation
///
/// Executes user-defined commands from porters.toml for full control
/// over the build process.
pub struct CustomBuildSystem {
    root: String,
    config: CustomBuild,
}

impl CustomBuildSystem {
    pub fn new(root: &str, config: CustomBuild) -> Self {
        Self {
            root: root.to_string(),
            config,
        }
    }

    fn execute_command(&self, cmd_str: &str) -> Result<()> {
        let cmd_str = cmd_str.replace("$INSTALL_PREFIX", &self.get_install_prefix());

        print_build(&format!("Executing: {}", cmd_str));

        let output = if cfg!(windows) {
            Command::new("powershell")
                .arg("-Command")
                .arg(&cmd_str)
                .current_dir(&self.root)
                .output()
        } else {
            Command::new("sh")
                .arg("-c")
                .arg(&cmd_str)
                .current_dir(&self.root)
                .output()
        };

        let output = output.with_context(|| format!("Failed to execute: {}", cmd_str))?;

        if !output.status.success() {
            print_error(&format!(
                "Command failed:\n{}",
                String::from_utf8_lossy(&output.stderr)
            ));
            return Err(anyhow::anyhow!("Command failed"));
        }

        print!("{}", String::from_utf8_lossy(&output.stdout));

        Ok(())
    }

    fn get_install_prefix(&self) -> String {
        dirs::home_dir()
            .expect("Could not find home directory")
            .join(".porters")
            .join("install")
            .display()
            .to_string()
    }
}

impl BuildSystem for CustomBuildSystem {
    fn name(&self) -> &str {
        "Custom"
    }

    fn detect(_root: &Path) -> bool {
        false // Custom build systems are never auto-detected
    }

    fn configure(&self, _sources: &ProjectSources, _deps: &[ResolvedDependency]) -> Result<()> {
        if let Some(ref configure_cmd) = self.config.configure {
            print_build("Running custom configure command...");
            self.execute_command(configure_cmd)?;
        }
        Ok(())
    }

    fn build(
        &self,
        sources: &ProjectSources,
        deps: &[ResolvedDependency],
        _args: &[String],
    ) -> Result<()> {
        // Run configure first if not done
        self.configure(sources, deps)?;

        if let Some(ref build_cmd) = self.config.build {
            print_build("Running custom build command...");
            self.execute_command(build_cmd)?;
        } else {
            print_warning("No custom build command specified");
        }

        Ok(())
    }

    fn run(&self, _args: &[String]) -> Result<()> {
        print_warning("Run command not configured for custom build system");
        print_info("Please add a run target to your custom build configuration");
        Ok(())
    }

    fn test(&self, _sources: &ProjectSources, _deps: &[ResolvedDependency]) -> Result<()> {
        if let Some(ref test_cmd) = self.config.test {
            print_build("Running custom test command...");
            self.execute_command(test_cmd)?;
        } else {
            print_warning("No custom test command specified");
        }
        Ok(())
    }

    fn clean(&self) -> Result<()> {
        if let Some(ref clean_cmd) = self.config.clean {
            print_build("Running custom clean command...");
            self.execute_command(clean_cmd)?;
        } else {
            print_warning("No custom clean command specified");
        }
        Ok(())
    }
}
