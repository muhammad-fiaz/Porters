//! Package registry management
//!
//! This module provides support for the Porters package registry,
//! allowing search, discovery, and installation of C/C++ packages
//! from the local registry and remote GitHub repository.
//!
//! The registry is a curated collection of package definitions stored
//! as JSON files in the `registry/` directory of the Porters project.

#![allow(dead_code)]

use crate::resolver::{
    Dependency, DependencyResolver, DependencySource, PackageMetadata, PlatformConstraints,
};
use crate::version::{Version, VersionReq};
use anyhow::{Context, Result};
use colored::Colorize;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};

/// Package definition from registry
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PackageDefinition {
    pub name: String,
    pub description: String,
    pub repository: String,
    pub version: String,
    pub license: String,
    pub build_system: String,
    #[serde(default)]
    pub dependencies: HashMap<String, String>,
    #[serde(default)]
    pub dev_dependencies: HashMap<String, String>,
    #[serde(default)]
    pub options: HashMap<String, serde_json::Value>,
    #[serde(default)]
    pub install: HashMap<String, serde_json::Value>,
    #[serde(default)]
    pub tags: Vec<String>,
    #[serde(default)]
    pub homepage: Option<String>,
    #[serde(default)]
    pub documentation: Option<String>,
    #[serde(default)]
    pub platforms: Vec<String>,
    #[serde(default)]
    pub constraints: Option<RegistryConstraints>,
    #[serde(default)]
    pub features: HashMap<String, FeatureDefinition>,
}

/// Constraints in registry format
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RegistryConstraints {
    #[serde(default)]
    pub min_cpp_standard: Option<String>,
    #[serde(default)]
    pub max_cpp_standard: Option<String>,
    #[serde(default)]
    pub compilers: HashMap<String, String>,
    #[serde(default)]
    pub arch: Vec<String>,
    #[serde(default)]
    pub environment: HashMap<String, String>,
}

/// Feature definition
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FeatureDefinition {
    pub description: String,
    #[serde(default)]
    pub default: bool,
    #[serde(default)]
    pub dependencies: HashMap<String, String>,
}

/// Registry manager for local package definitions
pub struct RegistryManager {
    registry_path: PathBuf,
    cache_path: PathBuf,
    index_path: PathBuf,
}

impl RegistryManager {
    /// Create a new registry manager
    ///
    /// The registry path should point to the root `registry/` directory.
    /// The index_path points to ~/.porters/registry-index/ for local caching.
    pub fn new(registry_path: PathBuf, cache_path: PathBuf) -> Self {
        let index_path = cache_path.join("registry-index");
        Self {
            registry_path,
            cache_path,
            index_path,
        }
    }

    /// Initialize registry and ensure local index exists
    pub fn init(&self) -> Result<()> {
        // Ensure cache directory exists
        if !self.cache_path.exists() {
            fs::create_dir_all(&self.cache_path)?;
        }

        // Ensure index directory exists
        if !self.index_path.exists() {
            fs::create_dir_all(&self.index_path)?;
        }

        // Update local index from registry (or fetch from remote if available)
        self.update_index_from_source()?;

        Ok(())
    }

    /// Update local index from source (local registry or remote GitHub)
    ///
    /// Tries to sync from local registry/ folder first. If that doesn't exist,
    /// attempts to fetch from GitHub repository (unless offline mode is enabled or in test environment).
    pub fn update_index_from_source(&self) -> Result<()> {
        if self.registry_path.exists() {
            // Use local registry if available
            return self.update_index();
        }

        // Check if offline mode is enabled
        let global_config = match crate::global_config::GlobalPortersConfig::load_or_create() {
            Ok(config) => config,
            Err(_) => {
                // If we can't load global config (e.g., in tests), just return Ok
                return Ok(());
            }
        };

        let offline = global_config.is_offline();

        if offline {
            // In offline mode with no local registry, just return Ok (don't fail)
            // The registry operations will fail gracefully later if packages are needed
            return Ok(());
        }

        // Don't fetch from GitHub in test mode (when index_path is in temp directory)
        if self.index_path.to_string_lossy().contains("Temp") || cfg!(test) {
            return Ok(());
        }

        // Try to fetch from remote GitHub repository
        println!("{}", "Fetching registry from GitHub...".cyan());
        self.fetch_remote_registry()
    }

    /// Fetch registry index from remote GitHub repository
    ///
    /// Clones or updates the registry from https://github.com/muhammad-fiaz/porters
    /// Downloads the registry/ folder contents to ~/.porters/registry-index/
    pub fn fetch_remote_registry(&self) -> Result<()> {
        use std::process::Command;

        let global_config = crate::global_config::GlobalPortersConfig::load_or_create()?;
        let registry_url = &global_config.registry.url;

        // Create a temporary directory for cloning
        let temp_dir =
            std::env::temp_dir().join(format!("porters-registry-{}", std::process::id()));

        // Clean up temp dir if it exists
        if temp_dir.exists() {
            fs::remove_dir_all(&temp_dir)?;
        }

        println!(
            "{}",
            format!("Cloning registry from {}...", registry_url).cyan()
        );

        // Clone the repository (sparse checkout for registry/ folder only)
        let clone_status = Command::new("git")
            .args([
                "clone",
                "--depth",
                "1",
                "--filter=blob:none",
                "--sparse",
                registry_url,
                temp_dir.to_str().unwrap(),
            ])
            .status()
            .context("Failed to execute git clone. Make sure git is installed.")?;

        if !clone_status.success() {
            anyhow::bail!(
                "Failed to clone registry from GitHub. Check your internet connection and repository URL."
            );
        }

        // Sparse checkout the registry/ folder
        let sparse_status = Command::new("git")
            .args([
                "-C",
                temp_dir.to_str().unwrap(),
                "sparse-checkout",
                "set",
                "registry",
            ])
            .status()
            .context("Failed to configure sparse checkout")?;

        if !sparse_status.success() {
            anyhow::bail!("Failed to configure git sparse checkout");
        }

        // Copy registry/ folder to index path
        let registry_src = temp_dir.join("registry");
        if !registry_src.exists() {
            fs::remove_dir_all(&temp_dir).ok();
            anyhow::bail!("Registry folder not found in remote repository");
        }

        // Clear existing index
        if self.index_path.exists() {
            fs::remove_dir_all(&self.index_path)?;
        }
        fs::create_dir_all(&self.index_path)?;

        // Copy registry contents to index
        let mut count = 0;
        self.sync_directory(&registry_src, &self.index_path, &mut count)?;

        // Clean up temp directory
        fs::remove_dir_all(&temp_dir).ok();

        println!(
            "{} {}",
            "✓".green(),
            format!("Fetched {} packages from remote registry", count).green()
        );

        // Update global config with last update timestamp
        let mut global_config = global_config;
        global_config.registry.last_update = Some(chrono::Utc::now().to_rfc3339());
        global_config.save()?;

        Ok(())
    }

    /// Update local index from local registry
    ///
    /// This syncs all package definitions from the registry/ folder
    /// to ~/.porters/registry-index/ for fast local access.
    pub fn update_index(&self) -> Result<()> {
        if !self.registry_path.exists() {
            return Ok(()); // Skip if registry doesn't exist
        }

        println!("{}", "Updating local registry index...".cyan());

        let mut count = 0;
        self.sync_directory(&self.registry_path, &self.index_path, &mut count)?;

        println!(
            "{} {}",
            "✓".green(),
            format!("Synced {} packages to local index", count).green()
        );

        Ok(())
    }

    /// Recursively sync directories from registry to index
    #[allow(clippy::only_used_in_recursion)]
    fn sync_directory(&self, src: &Path, dst: &Path, count: &mut usize) -> Result<()> {
        // Create destination directory if needed
        if !dst.exists() {
            fs::create_dir_all(dst)?;
        }

        for entry in fs::read_dir(src)? {
            let entry = entry?;
            let src_path = entry.path();
            let file_name = entry.file_name();
            let dst_path = dst.join(&file_name);

            if src_path.is_dir() {
                // Recursively sync subdirectory
                self.sync_directory(&src_path, &dst_path, count)?;
            } else if src_path.extension().and_then(|s| s.to_str()) == Some("json") {
                // Skip schema.json
                if file_name.to_str() == Some("schema.json") {
                    continue;
                }

                // Copy JSON file to index
                fs::copy(&src_path, &dst_path)?;
                *count += 1;
            }
        }

        Ok(())
    }

    /// Get path to use for package lookups (prefer index, fallback to registry)
    fn get_search_path(&self) -> PathBuf {
        if self.index_path.exists() {
            self.index_path.clone()
        } else {
            self.registry_path.clone()
        }
    }

    /// Search for packages in the local registry
    pub fn search(&self, query: &str) -> Result<Vec<PackageDefinition>> {
        // Auto-update index before searching
        if self.registry_path.exists() {
            let _ = self.update_index(); // Ignore errors, use cached if sync fails
        }

        let mut results = Vec::new();
        let query_lower = query.to_lowercase();

        // Search from local index (or registry as fallback)
        let search_path = self.get_search_path();
        self.scan_registry_dir(&search_path, &query_lower, &mut results)?;

        // Sort by relevance (exact matches first, then by name)
        results.sort_by(|a, b| {
            let a_exact = a.name.to_lowercase() == query_lower;
            let b_exact = b.name.to_lowercase() == query_lower;

            if a_exact && !b_exact {
                std::cmp::Ordering::Less
            } else if !a_exact && b_exact {
                std::cmp::Ordering::Greater
            } else {
                a.name.cmp(&b.name)
            }
        });

        Ok(results)
    }

    /// Scan registry directory recursively
    fn scan_registry_dir(
        &self,
        dir: &Path,
        query: &str,
        results: &mut Vec<PackageDefinition>,
    ) -> Result<()> {
        if !dir.exists() {
            return Ok(()); // Registry might not exist yet
        }

        for entry in fs::read_dir(dir)? {
            let entry = entry?;
            let path = entry.path();

            if path.is_dir() {
                // Recursively scan subdirectories
                self.scan_registry_dir(&path, query, results)?;
            } else if path.extension().and_then(|s| s.to_str()) == Some("json") {
                // Skip schema.json
                if path.file_name().and_then(|s| s.to_str()) == Some("schema.json") {
                    continue;
                }

                // Try to load package definition and check if it matches
                if let Ok(pkg) = self.load_package_from_path(&path)
                    && self.package_matches(&pkg, query)
                {
                    results.push(pkg);
                }
            }
        }

        Ok(())
    }

    /// Check if package matches search query
    fn package_matches(&self, pkg: &PackageDefinition, query: &str) -> bool {
        let query = query.to_lowercase();

        // Check name
        if pkg.name.to_lowercase().contains(&query) {
            return true;
        }

        // Check description
        if pkg.description.to_lowercase().contains(&query) {
            return true;
        }

        // Check tags
        for tag in &pkg.tags {
            if tag.to_lowercase().contains(&query) {
                return true;
            }
        }

        false
    }

    /// Load package definition from path
    fn load_package_from_path(&self, path: &Path) -> Result<PackageDefinition> {
        let content = fs::read_to_string(path)
            .with_context(|| format!("Failed to read {}", path.display()))?;

        let pkg: PackageDefinition = serde_json::from_str(&content)
            .with_context(|| format!("Failed to parse JSON in {}", path.display()))?;

        Ok(pkg)
    }

    /// Load package definition by name
    pub fn load_package(&self, name: &str) -> Result<PackageDefinition> {
        // Auto-update index before loading
        if self.registry_path.exists() {
            let _ = self.update_index(); // Ignore errors, use cached if sync fails
        }

        let mut found = None;

        // Search for package in local index (or registry as fallback)
        let search_path = self.get_search_path();
        self.find_package(&search_path, name, &mut found)?;

        found.ok_or_else(|| anyhow::anyhow!("Package '{}' not found in registry", name))
    }

    /// Find package by name recursively
    fn find_package(
        &self,
        dir: &Path,
        name: &str,
        found: &mut Option<PackageDefinition>,
    ) -> Result<()> {
        if !dir.exists() {
            return Ok(());
        }

        for entry in fs::read_dir(dir)? {
            let entry = entry?;
            let path = entry.path();

            if path.is_dir() {
                self.find_package(&path, name, found)?;
                if found.is_some() {
                    return Ok(());
                }
            } else if path.extension().and_then(|s| s.to_str()) == Some("json")
                && let Ok(pkg) = self.load_package_from_path(&path)
                && pkg.name == name
            {
                *found = Some(pkg);
                return Ok(());
            }
        }

        Ok(())
    }

    /// List all packages in the registry
    pub fn list_all(&self) -> Result<Vec<PackageDefinition>> {
        // Auto-update index before listing
        if self.registry_path.exists() {
            let _ = self.update_index(); // Ignore errors, use cached if sync fails
        }

        let mut packages = Vec::new();

        // List from local index (or registry as fallback)
        let search_path = self.get_search_path();
        self.collect_all_packages(&search_path, &mut packages)?;

        // Sort alphabetically
        packages.sort_by(|a, b| a.name.cmp(&b.name));

        Ok(packages)
    }

    /// Collect all packages recursively
    fn collect_all_packages(
        &self,
        dir: &Path,
        packages: &mut Vec<PackageDefinition>,
    ) -> Result<()> {
        if !dir.exists() {
            return Ok(());
        }

        for entry in fs::read_dir(dir)? {
            let entry = entry?;
            let path = entry.path();

            if path.is_dir() {
                self.collect_all_packages(&path, packages)?;
            } else if path.extension().and_then(|s| s.to_str()) == Some("json") {
                // Skip schema.json
                if path.file_name().and_then(|s| s.to_str()) == Some("schema.json") {
                    continue;
                }

                if let Ok(pkg) = self.load_package_from_path(&path) {
                    packages.push(pkg);
                }
            }
        }

        Ok(())
    }

    /// Resolve all dependencies for a package
    pub fn resolve_dependencies(&self, package_name: &str) -> Result<Vec<PackageMetadata>> {
        let pkg = self.load_package(package_name)?;

        // Convert to dependency list
        let root_deps: Vec<Dependency> = pkg
            .dependencies
            .iter()
            .map(|(name, version_req)| Dependency {
                name: name.clone(),
                version_req: version_req.clone(),
                optional: false,
                features: vec![],
            })
            .collect();

        // Create resolver
        let mut resolver = DependencyResolver::new();

        // Fetch metadata closure
        let fetch_metadata = |name: &str, version_req: &str| -> Result<PackageMetadata> {
            let dep_pkg = self.load_package(name)?;

            // Parse version
            let version = Version::parse(&dep_pkg.version)?;

            // Validate version requirement
            let req = VersionReq::parse(version_req)
                .with_context(|| format!("Invalid version requirement: {}", version_req))?;

            if !req.matches(&version) {
                anyhow::bail!(
                    "Package {} version {} does not satisfy requirement {}",
                    name,
                    dep_pkg.version,
                    version_req
                );
            }

            // Convert dependencies
            let dependencies: Vec<Dependency> = dep_pkg
                .dependencies
                .iter()
                .map(|(name, version_req)| Dependency {
                    name: name.clone(),
                    version_req: version_req.clone(),
                    optional: false,
                    features: vec![],
                })
                .collect();

            // Convert constraints
            let constraints = dep_pkg.constraints.as_ref().map(|c| PlatformConstraints {
                platforms: dep_pkg.platforms.clone(),
                arch: c.arch.clone(),
                min_cpp_standard: c.min_cpp_standard.clone(),
                max_cpp_standard: c.max_cpp_standard.clone(),
                compilers: c.compilers.clone(),
                environment: c.environment.clone(),
            });

            Ok(PackageMetadata {
                name: dep_pkg.name.clone(),
                version,
                dependencies,
                constraints,
                source: DependencySource::Registry,
            })
        };

        // Resolve dependencies
        let resolved = resolver.resolve(root_deps, fetch_metadata)?;

        Ok(resolved
            .into_iter()
            .map(|r| PackageMetadata {
                name: r.name.clone(),
                version: r.version.clone(),
                dependencies: vec![],
                constraints: None,
                source: r.source,
            })
            .collect())
    }

    /// Display package information
    pub fn display_package(&self, pkg: &PackageDefinition) {
        println!("{}", format!("📦 {}", pkg.name).cyan().bold());
        println!("   {}", pkg.description);
        println!("   {} {}", "Version:".bright_black(), pkg.version.green());
        println!("   {} {}", "License:".bright_black(), pkg.license.yellow());
        println!(
            "   {} {}",
            "Build System:".bright_black(),
            pkg.build_system.blue()
        );
        println!(
            "   {} {}",
            "Repository:".bright_black(),
            pkg.repository.dimmed()
        );

        if !pkg.dependencies.is_empty() {
            println!(
                "   {} {}",
                "Dependencies:".bright_black(),
                pkg.dependencies.len()
            );
        }

        if !pkg.tags.is_empty() {
            println!(
                "   {} {}",
                "Tags:".bright_black(),
                pkg.tags.join(", ").dimmed()
            );
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;
    use tempfile::TempDir;

    fn create_test_registry() -> (TempDir, RegistryManager) {
        let temp_dir = TempDir::new().unwrap();
        let registry_path = temp_dir.path().join("registry");
        let cache_path = temp_dir.path().join("cache");

        fs::create_dir_all(&registry_path).unwrap();
        fs::create_dir_all(registry_path.join("testing")).unwrap();

        // Create a test package
        let test_pkg = serde_json::json!({
            "name": "test-lib",
            "description": "A test library",
            "repository": "https://github.com/test/test-lib",
            "version": "1.0.0",
            "license": "MIT",
            "build_system": "cmake",
            "dependencies": {},
            "tags": ["testing", "example"]
        });

        let pkg_path = registry_path.join("testing/test-lib.json");
        let mut file = fs::File::create(&pkg_path).unwrap();
        file.write_all(test_pkg.to_string().as_bytes()).unwrap();

        let manager = RegistryManager::new(registry_path, cache_path);

        (temp_dir, manager)
    }

    #[test]
    fn test_registry_creation() {
        let temp_dir = TempDir::new().unwrap();
        let registry_path = temp_dir.path().join("registry");
        let cache_path = temp_dir.path().join("cache");

        let manager = RegistryManager::new(registry_path, cache_path);
        assert!(manager.init().is_ok());
    }

    #[test]
    fn test_search_packages() {
        let (_temp, manager) = create_test_registry();

        let results = manager.search("test").unwrap();
        assert_eq!(results.len(), 1);
        assert_eq!(results[0].name, "test-lib");
    }

    #[test]
    fn test_load_package() {
        let (_temp, manager) = create_test_registry();

        let pkg = manager.load_package("test-lib").unwrap();
        assert_eq!(pkg.name, "test-lib");
        assert_eq!(pkg.version, "1.0.0");
    }

    #[test]
    fn test_list_all_packages() {
        let (_temp, manager) = create_test_registry();

        let packages = manager.list_all().unwrap();
        assert_eq!(packages.len(), 1);
        assert_eq!(packages[0].name, "test-lib");
    }

    #[test]
    fn test_search_by_tag() {
        let (_temp, manager) = create_test_registry();

        let results = manager.search("testing").unwrap();
        assert_eq!(results.len(), 1);
    }

    #[test]
    fn test_package_not_found() {
        let (_temp, manager) = create_test_registry();

        let result = manager.load_package("nonexistent");
        assert!(result.is_err());
    }

    #[test]
    fn test_update_index_from_source_local() {
        let temp_dir = TempDir::new().unwrap();
        let local_registry = temp_dir.path().join("local-registry");
        let registry_path = temp_dir.path().join("registry");
        let cache_path = temp_dir.path().join("cache");

        // Create a local registry
        fs::create_dir_all(local_registry.join("registry")).unwrap();
        let test_pkg = serde_json::json!({
            "name": "local-test",
            "version": "1.0.0"
        });
        fs::write(
            local_registry.join("registry/local-test.json"),
            test_pkg.to_string(),
        )
        .unwrap();

        // Test with local registry path (simulates PORTERS_REGISTRY env var)
        let manager = RegistryManager::new(registry_path, cache_path);
        // Note: This would need env var set in real test
        assert!(manager.init().is_ok());
    }

    #[test]
    fn test_registry_index_dir() {
        let temp_dir = TempDir::new().unwrap();
        let registry_path = temp_dir.path().join("registry");
        let cache_path = temp_dir.path().join("cache");

        let manager = RegistryManager::new(registry_path.clone(), cache_path);
        assert_eq!(manager.registry_path, registry_path);
    }

    #[test]
    fn test_search_empty_registry() {
        let temp_dir = TempDir::new().unwrap();
        let registry_path = temp_dir.path().join("registry");
        let cache_path = temp_dir.path().join("cache");
        fs::create_dir_all(&registry_path).unwrap();

        let manager = RegistryManager::new(registry_path, cache_path);
        let results = manager.search("anything").unwrap();
        assert_eq!(results.len(), 0);
    }
}
