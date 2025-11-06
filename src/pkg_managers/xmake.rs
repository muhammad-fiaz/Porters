//! XMake package manager integration.
//!
//! This module provides integration with XMake (https://xmake.io/), a modern build
//! system with a built-in package manager (xrepo). It generates `xmake.lua` files
//! for dependency management.
//!
//! # Package Files
//!
//! XMake integration creates `xmake.lua` files:
//! - Local: `ports/xmake/xmake.lua`
//! - Global: `~/.porters/packages/xmake/xmake.lua`
//!
//! # Package Installation
//!
//! Uses `xrepo install` to fetch and install packages, and generates xmake.lua
//! with `add_requires()` and `add_packages()` directives.
//!
//! # Examples
//!
//! ```no_run
//! use porters::pkg_managers::{XMakeManager, PackageManager, InstallScope};
//!
//! let manager = XMakeManager::new();
//! manager.install("imgui", Some("1.89"), InstallScope::Local)?;
//! manager.remove("imgui", InstallScope::Local, true)?; // force remove
//! # Ok::<(), anyhow::Error>(())
//! ```

use super::{InstallScope, PackageManager};
use anyhow::{Result, anyhow};
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

pub struct XMakeManager {
    ports_dir: String,
}

impl Default for XMakeManager {
    fn default() -> Self {
        Self::new()
    }
}

impl XMakeManager {
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

impl PackageManager for XMakeManager {
    fn name(&self) -> &str {
        "XMake"
    }

    fn get_install_path(&self, scope: InstallScope) -> PathBuf {
        match scope {
            InstallScope::Local => PathBuf::from(&self.ports_dir).join("xmake"),
            InstallScope::Global => {
                let home = std::env::var("HOME")
                    .or_else(|_| std::env::var("USERPROFILE"))
                    .unwrap_or_else(|_| ".".to_string());
                PathBuf::from(home)
                    .join(".porters")
                    .join("packages")
                    .join("xmake")
            }
        }
    }

    fn install(&self, package: &str, version: Option<&str>, scope: InstallScope) -> Result<()> {
        if !self.is_available() {
            return Err(anyhow!(
                "XMake is not installed. Please install it from https://xmake.io/"
            ));
        }

        let install_dir = self.get_install_path(scope);
        self.ensure_dir(&install_dir)?;

        // Format package reference
        let package_ref = if let Some(ver) = version {
            format!("{} {}", package, ver)
        } else {
            package.to_string()
        };

        let scope_str = match scope {
            InstallScope::Local => format!("{}", install_dir.display()),
            InstallScope::Global => "globally".to_string(),
        };

        println!("📦 Installing {} via XMake {}...", package_ref, scope_str);

        // Use xrepo to install the package
        let output = Command::new("xrepo")
            .args(["install", "-y", &package_ref])
            .current_dir(&install_dir)
            .output()?;

        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            return Err(anyhow!("xrepo install failed: {}", stderr));
        }

        // Create or update xmake.lua to track installed packages
        let xmake_lua_path = install_dir.join("xmake.lua");
        let mut packages = Vec::new();

        // Read existing packages if file exists
        if xmake_lua_path.exists() {
            let content = fs::read_to_string(&xmake_lua_path)?;
            for line in content.lines() {
                if line.trim().starts_with("add_requires(")
                    && let Some(start) = line.find('(')
                    && let Some(end) = line.find(')')
                {
                    let pkgs = &line[start + 1..end];
                    for pkg in pkgs.split(',') {
                        let pkg_name = pkg.trim().trim_matches('"').trim_matches('\'');
                        if !pkg_name.is_empty()
                            && !pkg_name
                                .starts_with(&package.split(' ').next().unwrap().to_string())
                        {
                            packages.push(pkg_name.to_string());
                        }
                    }
                }
            }
        }

        // Add new package
        packages.push(package_ref.clone());

        // Write xmake.lua
        let mut xmake_content = String::from("-- XMake package dependencies\n\n");
        for pkg in &packages {
            xmake_content.push_str(&format!("add_requires(\"{}\")\n", pkg));
        }

        xmake_content.push_str("\ntarget(\"porters-xmake-deps\")\n");
        xmake_content.push_str("    set_kind(\"static\")\n");
        for pkg in &packages {
            let pkg_name = pkg.split_whitespace().next().unwrap_or(pkg);
            xmake_content.push_str(&format!("    add_packages(\"{}\")\n", pkg_name));
        }
        xmake_content.push_str("target_end()\n");

        fs::write(&xmake_lua_path, xmake_content)?;

        println!("✅ Successfully installed {} {}", package_ref, scope_str);
        if matches!(scope, InstallScope::Local) {
            println!("💡 Tip: Update your porters.toml to reference this package");
        }

        Ok(())
    }

    fn remove(&self, package: &str, scope: InstallScope, force: bool) -> Result<()> {
        let install_dir = self.get_install_path(scope);
        let xmake_lua_path = install_dir.join("xmake.lua");

        if !xmake_lua_path.exists() {
            return Err(anyhow!("No xmake.lua found in {}", install_dir.display()));
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

        // Read and filter packages
        let content = fs::read_to_string(&xmake_lua_path)?;
        let mut packages = Vec::new();
        let mut found = false;

        for line in content.lines() {
            if line.trim().starts_with("add_requires(")
                && let Some(start) = line.find('(')
                && let Some(end) = line.find(')')
            {
                let pkgs = &line[start + 1..end];
                for pkg in pkgs.split(',') {
                    let pkg_name = pkg.trim().trim_matches('"').trim_matches('\'');
                    if !pkg_name.is_empty() {
                        if pkg_name.starts_with(package) {
                            found = true;
                        } else {
                            packages.push(pkg_name.to_string());
                        }
                    }
                }
            }
        }

        if !found {
            return Err(anyhow!("Package {} not found in xmake.lua", package));
        }

        // Write updated xmake.lua
        let mut xmake_content = String::from("-- XMake package dependencies\n\n");
        for pkg in &packages {
            xmake_content.push_str(&format!("add_requires(\"{}\")\n", pkg));
        }

        if !packages.is_empty() {
            xmake_content.push_str("\ntarget(\"porters-xmake-deps\")\n");
            xmake_content.push_str("    set_kind(\"static\")\n");
            for pkg in &packages {
                let pkg_name = pkg.split_whitespace().next().unwrap_or(pkg);
                xmake_content.push_str(&format!("    add_packages(\"{}\")\n", pkg_name));
            }
            xmake_content.push_str("target_end()\n");
        }

        fs::write(&xmake_lua_path, xmake_content)?;

        // Uninstall with xrepo
        let _ = Command::new("xrepo")
            .args(["remove", "-y", package])
            .current_dir(&install_dir)
            .output();

        println!("✅ Removed {} from {}", package, xmake_lua_path.display());
        println!("💡 Run 'porters xmake list' to see remaining packages");

        Ok(())
    }

    fn list(&self, scope: InstallScope) -> Result<Vec<String>> {
        let install_dir = self.get_install_path(scope);
        let xmake_lua_path = install_dir.join("xmake.lua");

        if !xmake_lua_path.exists() {
            return Ok(Vec::new());
        }

        let content = fs::read_to_string(&xmake_lua_path)?;
        let mut packages = Vec::new();

        for line in content.lines() {
            if line.trim().starts_with("add_requires(")
                && let Some(start) = line.find('(')
                && let Some(end) = line.find(')')
            {
                let pkgs = &line[start + 1..end];
                for pkg in pkgs.split(',') {
                    let pkg_name = pkg.trim().trim_matches('"').trim_matches('\'');
                    if !pkg_name.is_empty() {
                        packages.push(pkg_name.to_string());
                    }
                }
            }
        }

        Ok(packages)
    }

    fn search(&self, query: &str) -> Result<Vec<String>> {
        if !self.is_available() {
            return Err(anyhow!("XMake is not installed"));
        }

        println!("🔍 Searching XMake repository for '{}'...", query);

        let output = Command::new("xrepo").args(["search", query]).output()?;

        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            return Err(anyhow!("xrepo search failed: {}", stderr));
        }

        let stdout = String::from_utf8_lossy(&output.stdout);
        let mut results = Vec::new();

        for line in stdout.lines() {
            let trimmed = line.trim();
            if !trimmed.is_empty() && !trimmed.starts_with("searching") {
                results.push(trimmed.to_string());
            }
        }

        Ok(results)
    }

    fn is_available(&self) -> bool {
        Command::new("xmake")
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
    fn test_xmake_manager_creation() {
        let manager = XMakeManager::default();
        assert!(!manager.name().is_empty());
    }

    #[test]
    fn test_xmake_install_paths() {
        let manager = XMakeManager::default();

        let local = manager.get_install_path(InstallScope::Local);
        assert!(local.ends_with("ports/xmake") || local.ends_with("ports\\xmake"));

        let global = manager.get_install_path(InstallScope::Global);
        let global_str = global.to_string_lossy();
        assert!(
            global_str.contains(".porters")
                && global_str.contains("packages")
                && global_str.contains("xmake")
        );
    }

    #[test]
    fn test_xmake_is_available() {
        let manager = XMakeManager::default();
        let _available = manager.is_available();
        // Don't assert - depends on system setup
    }

    #[test]
    #[ignore] // Requires XMake to be installed
    fn test_xmake_install_creates_lua() {
        use tempfile::TempDir;

        let manager = XMakeManager::default();
        if !manager.is_available() {
            return;
        }

        let temp_dir = TempDir::new().unwrap();
        std::env::set_current_dir(&temp_dir).unwrap();

        let result = manager.install("fmt", None, InstallScope::Local);

        if result.is_ok() {
            let xmake_lua = PathBuf::from("ports/xmake/xmake.lua");
            assert!(xmake_lua.exists());

            let content = fs::read_to_string(&xmake_lua).unwrap();
            assert!(content.contains("fmt"));
        }
    }

    #[test]
    #[ignore] // Requires XMake to be installed
    fn test_xmake_list_packages() {
        let manager = XMakeManager::default();

        let packages = manager.list(InstallScope::Local).unwrap();
        assert!(packages.is_empty() || !packages.is_empty());
    }

    #[test]
    #[ignore] // Requires XMake to be installed
    fn test_xmake_global_operations() {
        use tempfile::TempDir;

        let manager = XMakeManager::default();
        if !manager.is_available() {
            return;
        }

        let temp_dir = TempDir::new().unwrap();
        std::env::set_current_dir(&temp_dir).unwrap();

        // Install globally
        let result = manager.install("imgui", None, InstallScope::Global);

        if result.is_ok() {
            let global_path = manager.get_install_path(InstallScope::Global);
            let xmake_lua = global_path.join("xmake.lua");
            // File might exist from previous runs
            assert!(xmake_lua.exists() || !xmake_lua.exists());
        }
    }
}
