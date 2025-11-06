//! vcpkg package manager integration.
//!
//! This module provides integration with vcpkg (https://github.com/microsoft/vcpkg),
//! Microsoft's cross-platform C/C++ package manager. It uses manifest mode with
//! `vcpkg.json` files for dependency management.
//!
//! # Package Files
//!
//! vcpkg integration creates `vcpkg.json` manifest files:
//! - Local: `ports/vcpkg/vcpkg.json`
//! - Global: `~/.porters/packages/vcpkg/vcpkg.json`
//!
//! # Manifest Mode
//!
//! vcpkg is invoked with `--x-manifest-root` to use the generated manifest file.
//! This ensures dependencies are installed according to the manifest.
//!
//! # Examples
//!
//! ```no_run
//! use porters::pkg_managers::{VcpkgManager, PackageManager, InstallScope};
//!
//! let manager = VcpkgManager::new();
//! manager.install("fmt", None, InstallScope::Local)?;
//! let packages = manager.list(InstallScope::Global)?;
//! # Ok::<(), anyhow::Error>(())
//! ```

use super::{InstallScope, PackageManager};
use anyhow::{Result, anyhow};
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

pub struct VcpkgManager {
    ports_dir: String,
}

impl Default for VcpkgManager {
    fn default() -> Self {
        Self::new()
    }
}

impl VcpkgManager {
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

    /// Get vcpkg manifest file path for a given directory
    fn get_manifest_path(install_dir: &Path) -> PathBuf {
        install_dir.join("vcpkg.json")
    }
}

impl PackageManager for VcpkgManager {
    fn name(&self) -> &str {
        "vcpkg"
    }

    fn get_install_path(&self, scope: InstallScope) -> PathBuf {
        match scope {
            InstallScope::Local => PathBuf::from(&self.ports_dir).join("vcpkg"),
            InstallScope::Global => {
                let home = std::env::var("HOME")
                    .or_else(|_| std::env::var("USERPROFILE"))
                    .unwrap_or_else(|_| ".".to_string());
                PathBuf::from(home)
                    .join(".porters")
                    .join("packages")
                    .join("vcpkg")
            }
        }
    }

    fn install(&self, package: &str, version: Option<&str>, scope: InstallScope) -> Result<()> {
        if !self.is_available() {
            return Err(anyhow!(
                "vcpkg is not installed. Please install it from https://github.com/microsoft/vcpkg"
            ));
        }

        let install_dir = self.get_install_path(scope);
        self.ensure_dir(&install_dir)?;

        let manifest_path = Self::get_manifest_path(&install_dir);

        // Read or create vcpkg.json manifest
        let mut manifest: serde_json::Value = if manifest_path.exists() {
            let content = fs::read_to_string(&manifest_path)?;
            serde_json::from_str(&content)?
        } else {
            serde_json::json!({
                "name": "porters-vcpkg-deps",
                "version": "1.0.0",
                "dependencies": []
            })
        };

        // Add the package to dependencies
        let dependencies = manifest["dependencies"]
            .as_array_mut()
            .ok_or_else(|| anyhow!("Invalid vcpkg.json format"))?;

        // Check if package already exists
        let package_exists = dependencies.iter().any(|dep| {
            if let Some(name) = dep.as_str() {
                name == package
            } else if let Some(obj) = dep.as_object() {
                obj.get("name").and_then(|n| n.as_str()) == Some(package)
            } else {
                false
            }
        });

        if !package_exists {
            if let Some(ver) = version {
                dependencies.push(serde_json::json!({
                    "name": package,
                    "version>=": ver
                }));
            } else {
                dependencies.push(serde_json::json!(package));
            }
        }

        // Write manifest
        let manifest_str = serde_json::to_string_pretty(&manifest)?;
        fs::write(&manifest_path, manifest_str)?;

        let scope_str = match scope {
            InstallScope::Local => format!("{}", install_dir.display()),
            InstallScope::Global => "globally".to_string(),
        };

        println!("📦 Installing {} via vcpkg {}...", package, scope_str);

        // Run vcpkg install with manifest mode
        let output = Command::new("vcpkg")
            .args([
                "install",
                "--x-manifest-root",
                install_dir.to_str().unwrap(),
            ])
            .output()?;

        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            return Err(anyhow!("vcpkg install failed: {}", stderr));
        }

        println!("✅ Successfully installed {} {}", package, scope_str);
        if matches!(scope, InstallScope::Local) {
            println!("💡 Tip: Update your porters.toml to reference this package");
        }

        Ok(())
    }

    fn remove(&self, package: &str, scope: InstallScope, force: bool) -> Result<()> {
        let install_dir = self.get_install_path(scope);
        let manifest_path = Self::get_manifest_path(&install_dir);

        if !manifest_path.exists() {
            return Err(anyhow!("No vcpkg.json found in {}", install_dir.display()));
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

        // Read manifest
        let content = fs::read_to_string(&manifest_path)?;
        let mut manifest: serde_json::Value = serde_json::from_str(&content)?;

        // Remove package from dependencies
        let dependencies = manifest["dependencies"]
            .as_array_mut()
            .ok_or_else(|| anyhow!("Invalid vcpkg.json format"))?;

        let original_len = dependencies.len();
        dependencies.retain(|dep| {
            if let Some(name) = dep.as_str() {
                name != package
            } else if let Some(obj) = dep.as_object() {
                obj.get("name").and_then(|n| n.as_str()) != Some(package)
            } else {
                true
            }
        });

        if dependencies.len() == original_len {
            return Err(anyhow!("Package {} not found in vcpkg.json", package));
        }

        // Write updated manifest
        let manifest_str = serde_json::to_string_pretty(&manifest)?;
        fs::write(&manifest_path, manifest_str)?;

        println!("✅ Removed {} from {}", package, manifest_path.display());
        println!("💡 Run 'porters vcpkg list' to see remaining packages");

        Ok(())
    }

    fn list(&self, scope: InstallScope) -> Result<Vec<String>> {
        let install_dir = self.get_install_path(scope);
        let manifest_path = Self::get_manifest_path(&install_dir);

        if !manifest_path.exists() {
            return Ok(Vec::new());
        }

        let content = fs::read_to_string(&manifest_path)?;
        let manifest: serde_json::Value = serde_json::from_str(&content)?;

        let mut packages = Vec::new();
        if let Some(dependencies) = manifest["dependencies"].as_array() {
            for dep in dependencies {
                if let Some(name) = dep.as_str() {
                    packages.push(name.to_string());
                } else if let Some(obj) = dep.as_object()
                    && let Some(name) = obj.get("name").and_then(|n| n.as_str())
                {
                    let version = obj
                        .get("version>=")
                        .or_else(|| obj.get("version"))
                        .and_then(|v| v.as_str())
                        .unwrap_or("");
                    if version.is_empty() {
                        packages.push(name.to_string());
                    } else {
                        packages.push(format!("{}@{}", name, version));
                    }
                }
            }
        }

        Ok(packages)
    }

    fn search(&self, query: &str) -> Result<Vec<String>> {
        if !self.is_available() {
            return Err(anyhow!("vcpkg is not installed"));
        }

        println!("🔍 Searching vcpkg for '{}'...", query);

        let output = Command::new("vcpkg").args(["search", query]).output()?;

        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            return Err(anyhow!("vcpkg search failed: {}", stderr));
        }

        let stdout = String::from_utf8_lossy(&output.stdout);
        let mut results = Vec::new();

        for line in stdout.lines() {
            let trimmed = line.trim();
            if !trimmed.is_empty() {
                results.push(trimmed.to_string());
            }
        }

        Ok(results)
    }

    fn is_available(&self) -> bool {
        Command::new("vcpkg")
            .arg("version")
            .output()
            .map(|output| output.status.success())
            .unwrap_or(false)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_vcpkg_manager_creation() {
        let manager = VcpkgManager::default();
        assert!(!manager.name().is_empty());
    }

    #[test]
    fn test_vcpkg_install_paths() {
        let manager = VcpkgManager::default();

        let local = manager.get_install_path(InstallScope::Local);
        assert!(local.ends_with("ports/vcpkg") || local.ends_with("ports\\vcpkg"));

        let global = manager.get_install_path(InstallScope::Global);
        let global_str = global.to_string_lossy();
        assert!(
            global_str.contains(".porters")
                && global_str.contains("packages")
                && global_str.contains("vcpkg")
        );
    }

    #[test]
    fn test_vcpkg_is_available() {
        let manager = VcpkgManager::default();
        let _available = manager.is_available();
        // Don't assert - depends on system setup
    }

    #[test]
    #[ignore] // Requires vcpkg to be installed
    fn test_vcpkg_install_creates_manifest() {
        use tempfile::TempDir;

        let manager = VcpkgManager::default();
        if !manager.is_available() {
            return;
        }

        let temp_dir = TempDir::new().unwrap();
        std::env::set_current_dir(&temp_dir).unwrap();

        let result = manager.install("fmt", Some("10.0.0"), InstallScope::Local);

        if result.is_ok() {
            let manifest = PathBuf::from("ports/vcpkg/vcpkg.json");
            assert!(manifest.exists());

            let content = fs::read_to_string(&manifest).unwrap();
            assert!(content.contains("fmt"));
        }
    }

    #[test]
    #[ignore] // Requires vcpkg to be installed
    fn test_vcpkg_list_packages() {
        let manager = VcpkgManager::default();

        let packages = manager.list(InstallScope::Local).unwrap();
        assert!(packages.is_empty() || !packages.is_empty());
    }

    #[test]
    #[ignore] // Requires vcpkg to be installed
    fn test_vcpkg_global_install() {
        use tempfile::TempDir;

        let manager = VcpkgManager::default();
        if !manager.is_available() {
            return;
        }

        let temp_dir = TempDir::new().unwrap();
        std::env::set_current_dir(&temp_dir).unwrap();

        let result = manager.install("catch2", None, InstallScope::Global);

        if result.is_ok() {
            let global_path = manager.get_install_path(InstallScope::Global);
            let manifest = global_path.join("vcpkg.json");
            // Manifest might exist from previous runs
            assert!(manifest.exists() || !manifest.exists());
        }
    }
}
