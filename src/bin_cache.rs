//! Binary cache for compiled dependencies
//!
//! This module provides caching of compiled binary artifacts to avoid
//! recompiling dependencies when their source hasn't changed.

use anyhow::{Context, Result};
use std::fs;
use std::path::{Path, PathBuf};
use colored::Colorize;

/// Binary cache for compiled dependencies
/// 
/// Stores and retrieves pre-compiled binaries based on dependency hash
/// and build configuration hash.
pub struct BinaryCache {
    /// Directory where cached binaries are stored
    cache_dir: PathBuf,
    
    /// Whether caching is enabled
    enabled: bool,
}

impl BinaryCache {
    /// Create a new binary cache
    pub fn new(cache_dir: PathBuf, enabled: bool) -> Self {
        Self { cache_dir, enabled }
    }
    
    /// Check if binary caching is enabled
    pub fn is_enabled(&self) -> bool {
        self.enabled
    }

    /// Initialize the binary cache directory
    pub fn init(&self) -> Result<()> {
        if self.enabled && !self.cache_dir.exists() {
            fs::create_dir_all(&self.cache_dir)
                .context("Failed to create binary cache directory")?;
        }
        Ok(())
    }

    /// Get cache key from dependency hash and build configuration
    fn get_cache_key(
        &self,
        name: &str,
        version: &str,
        dep_hash: &str,
        build_hash: &str,
    ) -> String {
        // Cache key format: name-version-dephash-buildhash
        format!("{}-{}-{}-{}", name, version, &dep_hash[..8], &build_hash[..8])
    }

    /// Get the binary cache path
    pub fn get_cache_path(
        &self,
        name: &str,
        version: &str,
        dep_hash: &str,
        build_hash: &str,
    ) -> PathBuf {
        let key = self.get_cache_key(name, version, dep_hash, build_hash);
        self.cache_dir.join(key)
    }

    /// Check if compiled binary is cached
    pub fn is_cached(
        &self,
        name: &str,
        version: &str,
        dep_hash: &str,
        build_hash: &str,
    ) -> bool {
        if !self.enabled {
            return false;
        }

        let cache_path = self.get_cache_path(name, version, dep_hash, build_hash);
        cache_path.exists() && cache_path.join("lib").exists()
    }

    /// Store compiled binary in cache
    pub fn store(
        &self,
        name: &str,
        version: &str,
        dep_hash: &str,
        build_hash: &str,
        build_dir: &Path,
    ) -> Result<()> {
        if !self.enabled {
            return Ok(());
        }

        let cache_path = self.get_cache_path(name, version, dep_hash, build_hash);

        // Remove old cache if exists
        if cache_path.exists() {
            fs::remove_dir_all(&cache_path)?;
        }

        // Create cache directory
        fs::create_dir_all(&cache_path)?;

        // Copy build artifacts
        let lib_dir = cache_path.join("lib");
        let include_dir = cache_path.join("include");

        // Copy libraries
        if build_dir.join("lib").exists() {
            self.copy_dir_all(&build_dir.join("lib"), &lib_dir)?;
        } else if build_dir.exists() {
            // Try to find common library locations
            for pattern in &["*.a", "*.so", "*.dylib", "*.lib", "*.dll"] {
                self.copy_matching_files(build_dir, &lib_dir, pattern)?;
            }
        }

        // Copy headers if available
        if build_dir.join("include").exists() {
            self.copy_dir_all(&build_dir.join("include"), &include_dir)?;
        }

        println!("💾 Cached compiled binary for {} v{}", name.cyan(), version);
        Ok(())
    }

    /// Retrieve compiled binary from cache
    pub fn retrieve(
        &self,
        name: &str,
        version: &str,
        dep_hash: &str,
        build_hash: &str,
        dest_dir: &Path,
    ) -> Result<()> {
        if !self.enabled {
            anyhow::bail!("Binary cache is disabled");
        }

        let cache_path = self.get_cache_path(name, version, dep_hash, build_hash);
        if !cache_path.exists() {
            anyhow::bail!("Compiled binary not found in cache");
        }

        // Copy cached build artifacts to destination
        if cache_path.join("lib").exists() {
            let dest_lib = dest_dir.join("lib");
            fs::create_dir_all(&dest_lib)?;
            self.copy_dir_all(&cache_path.join("lib"), &dest_lib)?;
        }

        if cache_path.join("include").exists() {
            let dest_include = dest_dir.join("include");
            fs::create_dir_all(&dest_include)?;
            self.copy_dir_all(&cache_path.join("include"), &dest_include)?;
        }

        println!("⚡ Retrieved compiled binary for {} v{} from cache", name.cyan(), version);
        Ok(())
    }

    /// Clear binary cache
    pub fn clear(&self) -> Result<()> {
        if !self.cache_dir.exists() {
            println!("✨ Binary cache is already clean");
            return Ok(());
        }

        fs::remove_dir_all(&self.cache_dir)
            .context("Failed to clear binary cache")?;
        fs::create_dir_all(&self.cache_dir)
            .context("Failed to recreate binary cache directory")?;

        println!("🗑️  Binary cache cleared");
        Ok(())
    }

    /// Get binary cache statistics
    pub fn stats(&self) -> Result<BinaryCacheStats> {
        let mut stats = BinaryCacheStats::default();

        if !self.cache_dir.exists() {
            return Ok(stats);
        }

        let entries = fs::read_dir(&self.cache_dir)?;
        for entry in entries {
            let entry = entry?;
            let metadata = entry.metadata()?;
            if metadata.is_dir() {
                stats.count += 1;
                stats.size += self.dir_size(&entry.path())?;
            }
        }

        Ok(stats)
    }

    /// Calculate directory size
    fn dir_size(&self, path: &Path) -> Result<u64> {
        let mut size = 0;
        if path.is_dir() {
            for entry in fs::read_dir(path)? {
                let entry = entry?;
                let metadata = entry.metadata()?;
                if metadata.is_file() {
                    size += metadata.len();
                } else if metadata.is_dir() {
                    size += self.dir_size(&entry.path())?;
                }
            }
        }
        Ok(size)
    }

    /// Copy directory recursively
    fn copy_dir_all(&self, src: &Path, dst: &Path) -> Result<()> {
        fs::create_dir_all(dst)?;
        for entry in fs::read_dir(src)? {
            let entry = entry?;
            let ty = entry.file_type()?;
            let src_path = entry.path();
            let dst_path = dst.join(entry.file_name());

            if ty.is_dir() {
                self.copy_dir_all(&src_path, &dst_path)?;
            } else {
                fs::copy(&src_path, &dst_path)?;
            }
        }
        Ok(())
    }

    /// Copy files matching pattern
    fn copy_matching_files(&self, src: &Path, dst: &Path, pattern: &str) -> Result<()> {
        use walkdir::WalkDir;

        fs::create_dir_all(dst)?;

        let pattern = pattern.trim_start_matches('*');
        for entry in WalkDir::new(src)
            .max_depth(3)
            .into_iter()
            .filter_map(|e| e.ok())
        {
            if entry.file_type().is_file() {
                if let Some(name) = entry.file_name().to_str() {
                    if name.ends_with(pattern) {
                        let dest = dst.join(entry.file_name());
                        fs::copy(entry.path(), dest)?;
                    }
                }
            }
        }

        Ok(())
    }
}

#[derive(Debug, Default)]
pub struct BinaryCacheStats {
    pub count: usize,
    pub size: u64,
}

impl BinaryCacheStats {
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
