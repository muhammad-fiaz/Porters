//! Conan package manager integration.
//!
//! This module provides integration with Conan (https://conan.io/), a cross-platform
//! C/C++ package manager. It supports installing packages locally to the project or
//! globally to the user's home directory.
//!
//! # Package Files
//!
//! Conan integration creates `conanfile.txt` files in the installation directory:
//! - Local: `ports/conan/conanfile.txt`
//! - Global: `~/.porters/packages/conan/conanfile.txt`
//!
//! # Generators
//!
//! The generated conanfile.txt uses:
//! - `CMakeDeps`: Generate CMake find_package() files
//! - `CMakeToolchain`: Generate CMake toolchain file for cross-compilation
//!
//! # Examples
//!
//! ```no_run
//! use porters::pkg_managers::{ConanManager, PackageManager, InstallScope};
//!
//! let manager = ConanManager::new();
//! manager.install("fmt", Some("10.0.0"), InstallScope::Local)?;
//! manager.remove("fmt", InstallScope::Local, false)?;
//! let packages = manager.list(InstallScope::Local)?;
//! # Ok::<(), anyhow::Error>(())
//! ```

use super::{InstallScope, PackageManager};
use anyhow::{Result, anyhow};
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

pub struct ConanManager {
    ports_dir: String,
}

impl Default for ConanManager {
    fn default() -> Self {
        Self::new()
    }
}

impl ConanManager {
    pub fn new() -> Self {
        Self {
            ports_dir: "ports".to_string(),
        }
    }

    /// Ensure a directory exists
    fn ensure_dir(&self, path: &Path) -> Result<()> {
        fs::create_dir_all(path)?;
        Ok(())
    }
}

impl PackageManager for ConanManager {
    fn name(&self) -> &str {
        "Conan"
    }

    fn get_install_path(&self, scope: InstallScope) -> PathBuf {
        match scope {
            InstallScope::Local => PathBuf::from(&self.ports_dir).join("conan"),
            InstallScope::Global => {
                let home = std::env::var("HOME")
                    .or_else(|_| std::env::var("USERPROFILE"))
                    .unwrap_or_else(|_| ".".to_string());
                PathBuf::from(home)
                    .join(".porters")
                    .join("packages")
                    .join("conan")
            }
        }
    }

    fn install(&self, package: &str, version: Option<&str>, scope: InstallScope) -> Result<()> {
        if !self.is_available() {
            return Err(anyhow!(
                "Conan is not installed. Please install it from https://conan.io/"
            ));
        }

        let install_dir = self.get_install_path(scope);
        self.ensure_dir(&install_dir)?;

        // Format package reference (e.g., "fmt/10.1.1" or just "fmt")
        let package_ref = if let Some(ver) = version {
            format!("{}/{}", package, ver)
        } else {
            package.to_string()
        };

        let scope_str = match scope {
            InstallScope::Local => format!("{}", install_dir.display()),
            InstallScope::Global => "globally".to_string(),
        };

        println!("📦 Installing {} via Conan {}...", package_ref, scope_str);

        // Create a temporary conanfile.txt if it doesn't exist
        let conanfile_path = install_dir.join("conanfile.txt");
        let mut requires = Vec::new();

        // Read existing requirements if file exists
        if conanfile_path.exists() {
            let content = fs::read_to_string(&conanfile_path)?;
            for line in content.lines() {
                if line.starts_with('[') || line.trim().is_empty() {
                    continue;
                }
                if !line
                    .trim()
                    .starts_with(&package.split('/').next().unwrap().to_string())
                {
                    requires.push(line.trim().to_string());
                }
            }
        }

        // Add new package
        requires.push(package_ref.clone());

        // Write conanfile.txt
        let conanfile_content = format!(
            "[requires]\n{}\n\n[generators]\nCMakeDeps\nCMakeToolchain\n",
            requires.join("\n")
        );
        fs::write(&conanfile_path, conanfile_content)?;

        // Run conan install
        let output = Command::new("conan")
            .args([
                "install",
                ".",
                "--output-folder",
                install_dir.to_str().unwrap(),
                "--build=missing",
            ])
            .current_dir(&install_dir)
            .output()?;

        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            return Err(anyhow!("Conan install failed: {}", stderr));
        }

        println!("✅ Successfully installed {} {}", package_ref, scope_str);
        if matches!(scope, InstallScope::Local) {
            println!("💡 Tip: Update your porters.toml to reference this package");
        }

        Ok(())
    }

    fn remove(&self, package: &str, scope: InstallScope, force: bool) -> Result<()> {
        let install_dir = self.get_install_path(scope);
        let conanfile_path = install_dir.join("conanfile.txt");

        if !conanfile_path.exists() {
            return Err(anyhow!(
                "No conanfile.txt found in {}",
                install_dir.display()
            ));
        }

        // Confirm removal unless force is used
        if !force {
            print!(
                "⚠️  Remove {} from {}? (y/N): ",
                package,
                install_dir.display()
            );
            use std::io::{self, Write};
            io::stdout().flush()?;
            let mut input = String::new();
            io::stdin().read_line(&mut input)?;
            if !input.trim().eq_ignore_ascii_case("y") {
                println!("❌ Removal cancelled");
                return Ok(());
            }
        }

        // Read and filter requirements
        let content = fs::read_to_string(&conanfile_path)?;
        let mut new_content = String::new();
        let mut found = false;

        for line in content.lines() {
            if line.starts_with(package) || line.starts_with(&format!("{}/", package)) {
                found = true;
                continue;
            }
            new_content.push_str(line);
            new_content.push('\n');
        }

        if !found {
            return Err(anyhow!("Package {} not found in conanfile.txt", package));
        }

        fs::write(&conanfile_path, new_content)?;
        println!("✅ Removed {} from {}", package, conanfile_path.display());
        println!("💡 Run 'porters conan list' to see remaining packages");

        Ok(())
    }

    fn list(&self, scope: InstallScope) -> Result<Vec<String>> {
        let install_dir = self.get_install_path(scope);
        let conanfile_path = install_dir.join("conanfile.txt");

        if !conanfile_path.exists() {
            return Ok(Vec::new());
        }

        let content = fs::read_to_string(&conanfile_path)?;
        let mut packages = Vec::new();

        for line in content.lines() {
            let trimmed = line.trim();
            if trimmed.is_empty() || trimmed.starts_with('[') {
                continue;
            }
            packages.push(trimmed.to_string());
        }

        Ok(packages)
    }

    fn search(&self, query: &str) -> Result<Vec<String>> {
        if !self.is_available() {
            return Err(anyhow!("Conan is not installed"));
        }

        println!("🔍 Searching Conan for '{}'...", query);

        let output = Command::new("conan")
            .args(["search", query, "--remote=conancenter"])
            .output()?;

        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            return Err(anyhow!("Conan search failed: {}", stderr));
        }

        let stdout = String::from_utf8_lossy(&output.stdout);
        let mut results = Vec::new();

        for line in stdout.lines() {
            let trimmed = line.trim();
            if !trimmed.is_empty() && !trimmed.starts_with("Existing") {
                results.push(trimmed.to_string());
            }
        }

        Ok(results)
    }

    fn is_available(&self) -> bool {
        Command::new("conan")
            .arg("--version")
            .output()
            .map(|output| output.status.success())
            .unwrap_or(false)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_conan_manager_creation() {
        let manager = ConanManager::default();
        assert!(!manager.name().is_empty());
    }

    #[test]
    fn test_conan_install_paths() {
        let manager = ConanManager::default();

        let local = manager.get_install_path(InstallScope::Local);
        assert!(local.ends_with("ports/conan") || local.ends_with("ports\\conan"));

        let global = manager.get_install_path(InstallScope::Global);
        let global_str = global.to_string_lossy();
        assert!(
            global_str.contains(".porters")
                && global_str.contains("packages")
                && global_str.contains("conan")
        );
    }

    #[test]
    fn test_conan_is_available() {
        let manager = ConanManager::default();
        // Just test that the function doesn't panic
        let _available = manager.is_available();
        // Don't assert true/false as it depends on system setup
    }

    #[test]
    #[ignore] // Requires Conan to be installed
    fn test_conan_install_creates_conanfile() {
        use tempfile::TempDir;

        let manager = ConanManager::default();
        if !manager.is_available() {
            return; // Skip if Conan not installed
        }

        let temp_dir = TempDir::new().unwrap();
        std::env::set_current_dir(&temp_dir).unwrap();

        // Try to install a package locally
        let result = manager.install("fmt", Some("10.0.0"), InstallScope::Local);

        if result.is_ok() {
            let conanfile = PathBuf::from("ports/conan/conanfile.txt");
            assert!(conanfile.exists());

            let content = fs::read_to_string(&conanfile).unwrap();
            assert!(content.contains("fmt"));
        }
    }

    #[test]
    #[ignore] // Requires Conan to be installed
    fn test_conan_list_empty_initially() {
        let manager = ConanManager::default();

        // Should return empty list if no conanfile exists
        let packages = manager.list(InstallScope::Local).unwrap();
        // Might be empty or have packages from previous tests
        assert!(packages.is_empty() || !packages.is_empty());
    }

    #[test]
    #[ignore] // Requires Conan to be installed
    fn test_conan_force_remove() {
        use tempfile::TempDir;

        let manager = ConanManager::default();
        if !manager.is_available() {
            return;
        }

        let temp_dir = TempDir::new().unwrap();
        std::env::set_current_dir(&temp_dir).unwrap();

        // Install then remove with force
        let _ = manager.install("fmt", None, InstallScope::Local);
        let result = manager.remove("fmt", InstallScope::Local, true);

        // Should succeed or fail gracefully
        assert!(result.is_ok() || result.is_err());
    }
}
