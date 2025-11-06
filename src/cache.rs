//! Dependency caching system
//!
//! This module provides caching of downloaded dependencies to avoid
//! redundant network operations and speed up dependency resolution.
//! Supports both local project caches and global caches in ~/.porters/cache/

use crate::global_config::GlobalPortersConfig;
use crate::hash::calculate_directory_hash;
use anyhow::{Context, Result};
use colored::Colorize;
use std::fs;
use std::path::{Path, PathBuf};

/// Global dependency cache manager
///
/// Manages centralized cache in ~/.porters/cache/ for all projects.
/// This allows dependencies to be shared across multiple projects,
/// avoiding redundant downloads.
#[allow(dead_code)]
pub struct GlobalCache {
    cache_dir: PathBuf,
    enabled: bool,
}

#[allow(dead_code)]
impl GlobalCache {
    /// Create a new global cache instance
    pub fn new() -> Result<Self> {
        let config = GlobalPortersConfig::load_or_create()?;
        let cache_dir = config.cache_dir()?;
        let enabled = config.cache.enabled;

        Ok(Self { cache_dir, enabled })
    }

    /// Create with specific cache directory
    pub fn with_dir(cache_dir: PathBuf, enabled: bool) -> Self {
        Self { cache_dir, enabled }
    }

    /// Initialize global cache directory
    pub fn init(&self) -> Result<()> {
        if self.enabled && !self.cache_dir.exists() {
            fs::create_dir_all(&self.cache_dir)
                .context("Failed to create global cache directory")?;
        }
        Ok(())
    }

    /// Get cache path for a specific package
    pub fn get_package_cache_path(&self, name: &str, version: &str) -> PathBuf {
        self.cache_dir.join(name).join(version)
    }

    /// Check if a package is in global cache
    pub fn has_package(&self, name: &str, version: &str) -> bool {
        if !self.enabled {
            return false;
        }
        self.get_package_cache_path(name, version).exists()
    }

    /// Store package in global cache
    pub fn store_package(&self, name: &str, version: &str, source_path: &Path) -> Result<String> {
        if !self.enabled {
            return Ok(String::new());
        }

        let cache_path = self.get_package_cache_path(name, version);

        // Create parent directory
        if let Some(parent) = cache_path.parent() {
            fs::create_dir_all(parent).context("Failed to create package cache directory")?;
        }

        // Remove existing if present
        if cache_path.exists() {
            fs::remove_dir_all(&cache_path).context("Failed to remove old package cache")?;
        }

        // Copy to cache
        copy_dir_all(source_path, &cache_path)?;

        // Calculate hash
        let hash = calculate_directory_hash(&cache_path)?;

        println!("💾  Cached {} v{} globally", name.cyan(), version);
        Ok(hash)
    }

    /// Retrieve package from global cache
    pub fn retrieve_package(&self, name: &str, version: &str, dest_path: &Path) -> Result<()> {
        if !self.enabled {
            anyhow::bail!("Global cache is disabled");
        }

        let cache_path = self.get_package_cache_path(name, version);
        if !cache_path.exists() {
            anyhow::bail!("Package {} v{} not found in global cache", name, version);
        }

        // Remove destination if exists
        if dest_path.exists() {
            fs::remove_dir_all(dest_path).context("Failed to remove destination")?;
        }

        // Copy from cache
        copy_dir_all(&cache_path, dest_path)?;

        println!(
            "📦  Retrieved {} v{} from global cache",
            name.cyan(),
            version
        );
        Ok(())
    }

    /// List all cached packages
    pub fn list_packages(&self) -> Result<Vec<(String, Vec<String>)>> {
        let mut packages = Vec::new();

        if !self.cache_dir.exists() {
            return Ok(packages);
        }

        for entry in fs::read_dir(&self.cache_dir)? {
            let entry = entry?;
            if entry.file_type()?.is_dir() {
                let name = entry.file_name().to_string_lossy().to_string();
                let mut versions = Vec::new();

                for version_entry in fs::read_dir(entry.path())? {
                    let version_entry = version_entry?;
                    if version_entry.file_type()?.is_dir() {
                        versions.push(version_entry.file_name().to_string_lossy().to_string());
                    }
                }

                if !versions.is_empty() {
                    packages.push((name, versions));
                }
            }
        }

        Ok(packages)
    }

    /// Clear global cache
    pub fn clear(&self) -> Result<()> {
        if !self.cache_dir.exists() {
            println!("✨  Global cache is already clean");
            return Ok(());
        }

        fs::remove_dir_all(&self.cache_dir)?;
        fs::create_dir_all(&self.cache_dir)?;
        println!("🗑️  Global cache cleared");
        Ok(())
    }

    /// Get global cache statistics
    pub fn stats(&self) -> Result<GlobalCacheStats> {
        let mut stats = GlobalCacheStats::default();

        if !self.cache_dir.exists() {
            return Ok(stats);
        }

        for entry in fs::read_dir(&self.cache_dir)? {
            let entry = entry?;
            if entry.file_type()?.is_dir() {
                let package_name = entry.file_name().to_string_lossy().to_string();

                for version_entry in fs::read_dir(entry.path())? {
                    let version_entry = version_entry?;
                    if version_entry.file_type()?.is_dir() {
                        stats.package_count += 1;
                        stats.total_size += dir_size(&version_entry.path())?;
                        stats.packages.push((
                            package_name.clone(),
                            version_entry.file_name().to_string_lossy().to_string(),
                        ));
                    }
                }
            }
        }

        Ok(stats)
    }
}

#[allow(dead_code)]
#[derive(Debug, Default)]
pub struct GlobalCacheStats {
    pub package_count: usize,
    pub total_size: u64,
    pub packages: Vec<(String, String)>,
}

#[allow(dead_code)]
impl GlobalCacheStats {
    pub fn human_size(&self) -> String {
        human_readable_size(self.total_size)
    }
}

/// Helper function to get human-readable size
fn human_readable_size(size: u64) -> String {
    let size_f = size as f64;
    if size_f < 1024.0 {
        format!("{} B", size_f)
    } else if size_f < 1024.0 * 1024.0 {
        format!("{:.2} KB", size_f / 1024.0)
    } else if size_f < 1024.0 * 1024.0 * 1024.0 {
        format!("{:.2} MB", size_f / (1024.0 * 1024.0))
    } else {
        format!("{:.2} GB", size_f / (1024.0 * 1024.0 * 1024.0))
    }
}

/// Calculate directory size recursively
fn dir_size(path: &Path) -> Result<u64> {
    let mut size = 0;
    if path.is_dir() {
        for entry in fs::read_dir(path)? {
            let entry = entry?;
            let metadata = entry.metadata()?;
            if metadata.is_file() {
                size += metadata.len();
            } else if metadata.is_dir() {
                size += dir_size(&entry.path())?;
            }
        }
    }
    Ok(size)
}

/// Copy directory recursively, skipping .git directories
fn copy_dir_all(src: &Path, dst: &Path) -> Result<()> {
    fs::create_dir_all(dst)?;
    for entry in fs::read_dir(src)? {
        let entry = entry?;
        let ty = entry.file_type()?;
        let src_path = entry.path();
        let dst_path = dst.join(entry.file_name());

        if ty.is_dir() {
            // Skip .git directories
            if entry.file_name() == ".git" {
                continue;
            }
            copy_dir_all(&src_path, &dst_path)?;
        } else {
            fs::copy(&src_path, &dst_path)?;
        }
    }
    Ok(())
}

/// Dependency cache manager for downloaded packages
///
/// Manages a cache directory (typically `.porters/cache/`) where downloaded
/// dependencies are stored with hash verification.
pub struct DependencyCache {
    /// Directory where cached dependencies are stored
    cache_dir: PathBuf,

    /// Whether caching is enabled
    enabled: bool,
}

impl DependencyCache {
    /// Create a new dependency cache
    ///
    /// # Arguments
    /// * `cache_dir` - Path to the cache directory
    /// * `enabled` - Whether caching should be active
    ///
    /// # Returns
    /// A new `DependencyCache` instance
    pub fn new(cache_dir: PathBuf, enabled: bool) -> Self {
        Self { cache_dir, enabled }
    }

    /// Initialize the cache directory
    pub fn init(&self) -> Result<()> {
        if self.enabled && !self.cache_dir.exists() {
            fs::create_dir_all(&self.cache_dir).context("Failed to create cache directory")?;
        }
        Ok(())
    }

    /// Get the cache directory for a specific dependency
    pub fn get_cache_path(&self, name: &str, version: &str) -> PathBuf {
        self.cache_dir.join(format!("{}-{}", name, version))
    }

    /// Check if a dependency is cached
    pub fn is_cached(
        &self,
        name: &str,
        version: &str,
        expected_hash: Option<&str>,
    ) -> Result<bool> {
        if !self.enabled {
            return Ok(false);
        }

        let cache_path = self.get_cache_path(name, version);
        if !cache_path.exists() {
            return Ok(false);
        }

        // If hash is provided, verify it
        if let Some(expected) = expected_hash {
            let actual = calculate_directory_hash(&cache_path)?;
            if actual != expected {
                println!(
                    "⚠️  Cache hash mismatch for {}, re-downloading",
                    name.yellow()
                );
                return Ok(false);
            }
        }

        Ok(true)
    }

    /// Store a dependency in cache
    pub fn store(&self, name: &str, version: &str, source_path: &Path) -> Result<String> {
        if !self.enabled {
            return Ok(String::new());
        }

        let cache_path = self.get_cache_path(name, version);

        // Remove existing cache if present
        if cache_path.exists() {
            fs::remove_dir_all(&cache_path).context("Failed to remove old cache")?;
        }

        // Copy to cache
        copy_dir_all(source_path, &cache_path)?;

        // Calculate hash
        let hash = calculate_directory_hash(&cache_path)?;

        println!("💾  Cached {} v{}", name.cyan(), version);
        Ok(hash)
    }

    /// Retrieve a dependency from cache
    pub fn retrieve(&self, name: &str, version: &str, dest_path: &Path) -> Result<()> {
        if !self.enabled {
            anyhow::bail!("Cache is disabled");
        }

        let cache_path = self.get_cache_path(name, version);
        if !cache_path.exists() {
            anyhow::bail!("Dependency not found in cache");
        }

        // Remove destination if exists
        if dest_path.exists() {
            fs::remove_dir_all(dest_path).context("Failed to remove destination")?;
        }

        // Copy from cache
        copy_dir_all(&cache_path, dest_path)?;

        println!("📦  Retrieved {} v{} from cache", name.cyan(), version);
        Ok(())
    }

    /// Clear all cache
    pub fn clear(&self, force: bool) -> Result<()> {
        if !self.cache_dir.exists() {
            println!("✨  Cache is already clean");
            return Ok(());
        }

        if force {
            fs::remove_dir_all(&self.cache_dir).context("Failed to clear cache")?;
            fs::create_dir_all(&self.cache_dir).context("Failed to recreate cache directory")?;
            println!("🗑️  Cache cleared (forced)");
        } else {
            // Only remove old cache entries (could be enhanced with age check)
            let entries = fs::read_dir(&self.cache_dir)?;
            let mut count = 0;
            for entry in entries {
                let entry = entry?;
                fs::remove_dir_all(entry.path())?;
                count += 1;
            }
            println!("🗑️  Removed {} cached dependencies", count);
        }

        Ok(())
    }

    /// Get cache statistics
    pub fn stats(&self) -> Result<CacheStats> {
        let mut stats = CacheStats::default();

        if !self.cache_dir.exists() {
            return Ok(stats);
        }

        let entries = fs::read_dir(&self.cache_dir)?;
        for entry in entries {
            let entry = entry?;
            let metadata = entry.metadata()?;
            if metadata.is_dir() {
                stats.count += 1;
                stats.size += dir_size(&entry.path())?;
            }
        }

        Ok(stats)
    }
}

#[derive(Debug, Default)]
pub struct CacheStats {
    pub count: usize,
    pub size: u64,
}

impl CacheStats {
    pub fn human_size(&self) -> String {
        human_readable_size(self.size)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use tempfile::TempDir;

    #[test]
    fn test_dependency_cache_operations() {
        let temp_dir = TempDir::new().unwrap();
        let cache_dir = temp_dir.path().join("cache");
        let cache = DependencyCache::new(cache_dir.clone(), true);

        cache.init().unwrap();
        assert!(cache_dir.exists());
    }

    #[test]
    fn test_global_cache_init() {
        let temp_dir = TempDir::new().unwrap();
        let cache = GlobalCache::with_dir(temp_dir.path().to_path_buf(), true);

        cache.init().unwrap();
        assert!(temp_dir.path().exists());
    }

    #[test]
    fn test_global_cache_has_package() {
        let temp_dir = TempDir::new().unwrap();
        let cache = GlobalCache::with_dir(temp_dir.path().to_path_buf(), true);
        cache.init().unwrap();

        // Package doesn't exist initially
        assert!(!cache.has_package("test-pkg", "1.0.0"));

        // Create a dummy package
        let pkg_path = cache.get_package_cache_path("test-pkg", "1.0.0");
        fs::create_dir_all(&pkg_path).unwrap();
        fs::write(pkg_path.join("test.txt"), "test content").unwrap();

        // Now it should exist
        assert!(cache.has_package("test-pkg", "1.0.0"));
    }

    #[test]
    fn test_global_cache_store_and_retrieve() {
        let temp_dir = TempDir::new().unwrap();
        let cache = GlobalCache::with_dir(temp_dir.path().to_path_buf(), true);
        cache.init().unwrap();

        // Create a source directory
        let source_dir = temp_dir.path().join("source");
        fs::create_dir_all(&source_dir).unwrap();
        fs::write(source_dir.join("file1.txt"), "content1").unwrap();
        fs::write(source_dir.join("file2.txt"), "content2").unwrap();

        // Store package
        cache.store_package("my-pkg", "2.0.0", &source_dir).unwrap();
        assert!(cache.has_package("my-pkg", "2.0.0"));

        // Retrieve package
        let dest_dir = temp_dir.path().join("dest");
        cache
            .retrieve_package("my-pkg", "2.0.0", &dest_dir)
            .unwrap();

        assert!(dest_dir.join("file1.txt").exists());
        assert!(dest_dir.join("file2.txt").exists());
        assert_eq!(
            fs::read_to_string(dest_dir.join("file1.txt")).unwrap(),
            "content1"
        );
    }

    #[test]
    fn test_global_cache_list_packages() {
        let temp_dir = TempDir::new().unwrap();
        let cache = GlobalCache::with_dir(temp_dir.path().to_path_buf(), true);
        cache.init().unwrap();

        // Initially empty
        let packages = cache.list_packages().unwrap();
        assert_eq!(packages.len(), 0);

        // Add some packages
        let pkg1 = cache.get_package_cache_path("pkg1", "1.0.0");
        let pkg2 = cache.get_package_cache_path("pkg2", "2.0.0");
        fs::create_dir_all(&pkg1).unwrap();
        fs::create_dir_all(&pkg2).unwrap();

        let packages = cache.list_packages().unwrap();
        assert_eq!(packages.len(), 2);
    }

    #[test]
    fn test_global_cache_stats() {
        let temp_dir = TempDir::new().unwrap();
        let cache = GlobalCache::with_dir(temp_dir.path().to_path_buf(), true);
        cache.init().unwrap();

        // Initially empty
        let stats = cache.stats().unwrap();
        assert_eq!(stats.package_count, 0);
        assert_eq!(stats.total_size, 0);

        // Add a package
        let source_dir = temp_dir.path().join("source");
        fs::create_dir_all(&source_dir).unwrap();
        fs::write(source_dir.join("test.txt"), "test").unwrap();
        cache.store_package("test", "1.0.0", &source_dir).unwrap();

        let stats = cache.stats().unwrap();
        assert_eq!(stats.package_count, 1);
        assert!(stats.total_size > 0);
    }

    #[test]
    fn test_global_cache_clear() {
        let temp_dir = TempDir::new().unwrap();
        let cache = GlobalCache::with_dir(temp_dir.path().to_path_buf(), true);
        cache.init().unwrap();

        // Add packages
        let source_dir = temp_dir.path().join("source");
        fs::create_dir_all(&source_dir).unwrap();
        fs::write(source_dir.join("test.txt"), "test").unwrap();
        cache.store_package("pkg1", "1.0.0", &source_dir).unwrap();
        cache.store_package("pkg2", "2.0.0", &source_dir).unwrap();

        assert!(cache.has_package("pkg1", "1.0.0"));
        assert!(cache.has_package("pkg2", "2.0.0"));

        // Clear cache
        cache.clear().unwrap();

        assert!(!cache.has_package("pkg1", "1.0.0"));
        assert!(!cache.has_package("pkg2", "2.0.0"));
    }

    #[test]
    fn test_human_readable_size() {
        assert_eq!(human_readable_size(500), "500 B");
        assert_eq!(human_readable_size(1024), "1.00 KB");
        assert_eq!(human_readable_size(1024 * 1024), "1.00 MB");
        assert_eq!(human_readable_size(1024 * 1024 * 1024), "1.00 GB");
    }
}
