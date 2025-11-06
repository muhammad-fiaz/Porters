//! Global package management and configuration
//!
//! This module handles globally installed packages, allowing system-wide
//! installation of C/C++ libraries accessible from any project. Includes
//! global settings for parallel jobs, cache paths, and package metadata.

use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::{Path, PathBuf};

/// Global Porters configuration and state
///
/// Stores globally installed packages and system-wide settings.
/// Persisted to ~/.porters/global.toml for cross-project access.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct GlobalConfig {
    /// Globally installed packages
    pub packages: HashMap<String, GlobalPackage>,

    /// Global settings
    #[serde(default)]
    pub settings: GlobalSettings,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GlobalPackage {
    pub name: String,
    pub version: String,
    pub source: String,
    pub install_path: PathBuf,
    pub installed_at: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GlobalSettings {
    #[serde(default = "default_parallel_jobs")]
    pub parallel_jobs: usize,

    #[serde(default)]
    pub cache_enabled: bool,
}

impl Default for GlobalSettings {
    fn default() -> Self {
        Self {
            parallel_jobs: default_parallel_jobs(),
            cache_enabled: true,
        }
    }
}

fn default_parallel_jobs() -> usize {
    num_cpus::get()
}

impl GlobalConfig {
    /// Get the global porters directory
    /// ~/.porters on Unix
    /// C:\Users\username\.porters on Windows
    pub fn global_dir() -> Result<PathBuf> {
        let home = dirs::home_dir().context("Failed to determine home directory")?;

        Ok(home.join(".porters"))
    }

    /// Get the global packages directory
    pub fn packages_dir() -> Result<PathBuf> {
        Ok(Self::global_dir()?.join("packages"))
    }

    /// Get the global cache directory
    pub fn cache_dir() -> Result<PathBuf> {
        Ok(Self::global_dir()?.join("cache"))
    }

    /// Get the global config file path
    pub fn config_path() -> Result<PathBuf> {
        Ok(Self::global_dir()?.join("config.toml"))
    }

    /// Load global configuration
    pub fn load() -> Result<Self> {
        let config_path = Self::config_path()?;

        if !config_path.exists() {
            // Create default config
            let config = Self::default();
            config.save()?;
            return Ok(config);
        }

        let content = std::fs::read_to_string(&config_path)
            .with_context(|| format!("Failed to read {}", config_path.display()))?;

        let config: GlobalConfig =
            toml::from_str(&content).with_context(|| "Failed to parse global config")?;

        Ok(config)
    }

    /// Save global configuration
    pub fn save(&self) -> Result<()> {
        let config_path = Self::config_path()?;

        // Ensure directory exists
        if let Some(parent) = config_path.parent() {
            std::fs::create_dir_all(parent)
                .with_context(|| format!("Failed to create directory {}", parent.display()))?;
        }

        let content = toml::to_string_pretty(self).context("Failed to serialize global config")?;

        std::fs::write(&config_path, content)
            .with_context(|| format!("Failed to write {}", config_path.display()))?;

        Ok(())
    }

    /// Initialize global porters directory structure
    pub fn initialize() -> Result<()> {
        let global_dir = Self::global_dir()?;
        let packages_dir = Self::packages_dir()?;
        let cache_dir = Self::cache_dir()?;

        std::fs::create_dir_all(&global_dir)
            .with_context(|| format!("Failed to create {}", global_dir.display()))?;

        std::fs::create_dir_all(&packages_dir)
            .with_context(|| format!("Failed to create {}", packages_dir.display()))?;

        std::fs::create_dir_all(&cache_dir)
            .with_context(|| format!("Failed to create {}", cache_dir.display()))?;

        // Load or create config
        let config = Self::load()?;
        config.save()?;

        Ok(())
    }

    /// Add a globally installed package
    pub fn add_package(
        &mut self,
        name: String,
        version: String,
        source: String,
        install_path: PathBuf,
    ) -> Result<()> {
        let package = GlobalPackage {
            name: name.clone(),
            version,
            source,
            install_path,
            installed_at: chrono::Utc::now().to_rfc3339(),
        };

        self.packages.insert(name, package);
        self.save()?;

        Ok(())
    }

    /// Remove a globally installed package
    #[allow(dead_code)]
    pub fn remove_package(&mut self, name: &str) -> Result<()> {
        if let Some(package) = self.packages.remove(name) {
            // Remove package files
            if package.install_path.exists() {
                std::fs::remove_dir_all(&package.install_path).with_context(|| {
                    format!("Failed to remove {}", package.install_path.display())
                })?;
            }
        }

        self.save()?;
        Ok(())
    }

    /// Get a globally installed package
    #[allow(dead_code)]
    pub fn get_package(&self, name: &str) -> Option<&GlobalPackage> {
        self.packages.get(name)
    }

    /// List all globally installed packages
    #[allow(dead_code)]
    pub fn list_packages(&self) -> Vec<&GlobalPackage> {
        self.packages.values().collect()
    }
}

/// Get the project-local dependencies directory (ports/)
pub fn project_deps_dir<P: AsRef<Path>>(project_path: P) -> PathBuf {
    project_path.as_ref().join("ports")
}

/// Get the project lock file path
pub fn project_lock_file<P: AsRef<Path>>(project_path: P) -> PathBuf {
    project_path.as_ref().join("porters.lock")
}
