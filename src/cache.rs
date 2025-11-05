//! Dependency caching system
//!
//! This module provides caching of downloaded dependencies to avoid
//! redundant network operations and speed up dependency resolution.

use crate::hash::calculate_directory_hash;
use anyhow::{Context, Result};
use colored::Colorize;
use std::fs;
use std::path::{Path, PathBuf};

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
        self.copy_dir_all(source_path, &cache_path)?;

        // Calculate hash
        let hash = calculate_directory_hash(&cache_path)?;

        println!("💾 Cached {} v{}", name.cyan(), version);
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
        self.copy_dir_all(&cache_path, dest_path)?;

        println!("📦 Retrieved {} v{} from cache", name.cyan(), version);
        Ok(())
    }

    /// Clear all cache
    pub fn clear(&self, force: bool) -> Result<()> {
        if !self.cache_dir.exists() {
            println!("✨ Cache is already clean");
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
                stats.size += Self::dir_size(&entry.path())?;
            }
        }

        Ok(stats)
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
                    size += Self::dir_size(&entry.path())?;
                }
            }
        }
        Ok(size)
    }

    /// Copy directory recursively
    fn copy_dir_all(&self, src: &Path, dst: &Path) -> Result<()> {
        fn copy_recursive(src: &Path, dst: &Path) -> Result<()> {
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
                    copy_recursive(&src_path, &dst_path)?;
                } else {
                    fs::copy(&src_path, &dst_path)?;
                }
            }
            Ok(())
        }
        copy_recursive(src, dst)
    }
}

#[derive(Debug, Default)]
pub struct CacheStats {
    pub count: usize,
    pub size: u64,
}

impl CacheStats {
    pub fn human_size(&self) -> String {
        let size = self.size as f64;
        if size < 1024.0 {
            format!("{} B", size)
        } else if size < 1024.0 * 1024.0 {
            format!("{:.2} KB", size / 1024.0)
        } else if size < 1024.0 * 1024.0 * 1024.0 {
            format!("{:.2} MB", size / (1024.0 * 1024.0))
        } else {
            format!("{:.2} GB", size / (1024.0 * 1024.0 * 1024.0))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;

    #[test]
    fn test_cache_operations() {
        let temp_dir = TempDir::new().unwrap();
        let cache_dir = temp_dir.path().join("cache");
        let cache = DependencyCache::new(cache_dir.clone(), true);

        cache.init().unwrap();
        assert!(cache_dir.exists());
    }
}
