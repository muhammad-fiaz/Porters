//! Package manager integration module for Porters.
//!
//! This module provides unified interfaces for managing external C/C++ package managers
//! including Conan, vcpkg, and XMake. It supports both local (project-specific) and
//! global (system-wide) package installations.
//!
//! # Supported Package Managers
//!
//! - **Conan**: Cross-platform C/C++ package manager
//! - **vcpkg**: Microsoft's C/C++ package manager  
//! - **XMake**: Modern build system with built-in package manager
//!
//! # Installation Scopes
//!
//! Packages can be installed in two scopes:
//! - `InstallScope::Local`: Installed to `ports/{manager}/` in the project directory
//! - `InstallScope::Global`: Installed to `~/.porters/packages/{manager}/` for system-wide access
//!
//! # Examples
//!
//! ```no_run
//! use porters::pkg_managers::{ConanManager, PackageManager, InstallScope};
//!
//! let manager = ConanManager::new();
//! // Install locally to project
//! manager.install("fmt", Some("10.0.0"), InstallScope::Local)?;
//! // Install globally for system-wide use
//! manager.install("boost", Some("1.82.0"), InstallScope::Global)?;
//! # Ok::<(), anyhow::Error>(())
//! ```

pub mod conan;
pub mod vcpkg;
pub mod xmake;

pub use conan::ConanManager;
pub use vcpkg::VcpkgManager;
pub use xmake::XMakeManager;

use anyhow::Result;
use std::path::PathBuf;

/// Package installation scope
#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub enum InstallScope {
    /// Install to project-local ports/ directory
    #[default]
    Local,
    /// Install to global ~/.porters/packages directory
    Global,
}

impl std::fmt::Display for InstallScope {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Local => write!(f, "Local"),
            Self::Global => write!(f, "Global"),
        }
    }
}

/// Package manager operations trait
///
/// Provides a unified interface for managing packages from different sources
/// (Conan, vcpkg, XMake). Supports both local (project-specific) and global
/// (system-wide) package installation.
pub trait PackageManager {
    /// Get the name of the package manager
    #[allow(dead_code)]
    fn name(&self) -> &str;

    /// Install a package
    ///
    /// # Arguments
    /// * `package` - Package name (e.g., "fmt", "boost")
    /// * `version` - Optional version constraint (e.g., "10.1.1", ">=1.80.0")
    /// * `scope` - Installation scope (Local or Global)
    ///
    /// # Returns
    /// * `Ok(())` - Package installed successfully
    /// * `Err(...)` - Installation failed
    fn install(&self, package: &str, version: Option<&str>, scope: InstallScope) -> Result<()>;

    /// Remove a package
    ///
    /// # Arguments
    /// * `package` - Package name to remove
    /// * `scope` - Remove from Local or Global scope
    /// * `force` - Force removal without confirmation
    ///
    /// # Returns
    /// * `Ok(())` - Package removed successfully
    /// * `Err(...)` - Removal failed
    fn remove(&self, package: &str, scope: InstallScope, force: bool) -> Result<()>;

    /// List installed packages
    ///
    /// # Arguments
    /// * `scope` - List packages from Local or Global scope
    ///
    /// # Returns
    /// * `Ok(Vec<String>)` - List of installed packages
    /// * `Err(...)` - Error listing packages
    fn list(&self, scope: InstallScope) -> Result<Vec<String>>;

    /// Search for available packages in the package manager's registry
    ///
    /// # Arguments
    /// * `query` - Search query string
    ///
    /// # Returns
    /// * `Ok(Vec<String>)` - List of matching packages
    /// * `Err(...)` - Search failed
    fn search(&self, query: &str) -> Result<Vec<String>>;

    /// Check if the package manager tool is installed on the system
    ///
    /// # Returns
    /// * `true` - Package manager is available
    /// * `false` - Package manager not found
    fn is_available(&self) -> bool;

    /// Get the installation path for a package
    ///
    /// # Arguments
    /// * `scope` - Get path for Local or Global scope
    ///
    /// # Returns
    /// * Installation directory path
    fn get_install_path(&self, scope: InstallScope) -> PathBuf;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_install_scope_default() {
        let scope = InstallScope::default();
        assert!(matches!(scope, InstallScope::Local));
    }

    #[test]
    fn test_install_scope_display() {
        assert_eq!(format!("{}", InstallScope::Local), "Local");
        assert_eq!(format!("{}", InstallScope::Global), "Global");
    }

    #[test]
    fn test_install_scope_paths() {
        let conan = ConanManager::default();

        let local = conan.get_install_path(InstallScope::Local);
        assert!(local.to_string_lossy().contains("ports"));
        assert!(local.to_string_lossy().contains("conan"));

        let global = conan.get_install_path(InstallScope::Global);
        assert!(global.to_string_lossy().contains(".porters"));
        assert!(global.to_string_lossy().contains("packages"));
        assert!(global.to_string_lossy().contains("conan"));
    }

    #[test]
    fn test_all_managers_have_different_paths() {
        let conan = ConanManager::default();
        let vcpkg = VcpkgManager::default();
        let xmake = XMakeManager::default();

        let conan_local = conan.get_install_path(InstallScope::Local);
        let vcpkg_local = vcpkg.get_install_path(InstallScope::Local);
        let xmake_local = xmake.get_install_path(InstallScope::Local);

        // All should be in ports/ but different subdirectories
        assert!(conan_local.ends_with("ports/conan") || conan_local.ends_with("ports\\conan"));
        assert!(vcpkg_local.ends_with("ports/vcpkg") || vcpkg_local.ends_with("ports\\vcpkg"));
        assert!(xmake_local.ends_with("ports/xmake") || xmake_local.ends_with("ports\\xmake"));

        let conan_global = conan.get_install_path(InstallScope::Global);
        let vcpkg_global = vcpkg.get_install_path(InstallScope::Global);
        let xmake_global = xmake.get_install_path(InstallScope::Global);

        // All should be in ~/.porters/packages/ but different subdirectories
        let conan_str = conan_global.to_string_lossy();
        let vcpkg_str = vcpkg_global.to_string_lossy();
        let xmake_str = xmake_global.to_string_lossy();

        assert!(conan_str.contains("packages") && conan_str.contains("conan"));
        assert!(vcpkg_str.contains("packages") && vcpkg_str.contains("vcpkg"));
        assert!(xmake_str.contains("packages") && xmake_str.contains("xmake"));
    }

    #[test]
    fn test_manager_names() {
        let conan = ConanManager::default();
        let vcpkg = VcpkgManager::default();
        let xmake = XMakeManager::default();

        // Names should match their type
        assert!(!conan.name().is_empty());
        assert!(!vcpkg.name().is_empty());
        assert!(!xmake.name().is_empty());
    }
}
