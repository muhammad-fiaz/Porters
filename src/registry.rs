//! Package registry management
//!
//! This module provides support for custom package registries,
//! allowing search, download, and publication of C/C++ packages.
//!
//! **Note**: This is a future feature for package registry support.

use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::fs;
use colored::Colorize;

/// Package registry configuration
/// 
/// Represents a single package registry with authentication and metadata.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[allow(dead_code)]
pub struct Registry {
    pub name: String,
    pub url: String,
    #[serde(default)]
    pub auth_token: Option<String>,
    #[serde(default)]
    pub enabled: bool,
}

/// Registry manager
#[allow(dead_code)]
pub struct RegistryManager {
    registries: Vec<Registry>,
    cache_dir: PathBuf,
}

#[allow(dead_code)]
impl RegistryManager {
    /// Create a new registry manager
    pub fn new(registries: Vec<Registry>, cache_dir: PathBuf) -> Self {
        Self {
            registries,
            cache_dir,
        }
    }

    /// Initialize registry cache
    pub fn init(&self) -> Result<()> {
        if !self.cache_dir.exists() {
            fs::create_dir_all(&self.cache_dir)
                .context("Failed to create registry cache directory")?;
        }
        Ok(())
    }

    /// Search for a package across all registries
    pub async fn search(&self, name: &str) -> Result<Vec<PackageInfo>> {
        let mut results = Vec::new();

        for registry in &self.registries {
            if !registry.enabled {
                continue;
            }

            match self.search_registry(registry, name).await {
                Ok(mut packages) => results.append(&mut packages),
                Err(e) => {
                    println!(
                        "⚠️  Failed to search registry {}: {}",
                        registry.name.yellow(),
                        e
                    );
                }
            }
        }

        Ok(results)
    }

    /// Search a specific registry
    async fn search_registry(&self, registry: &Registry, name: &str) -> Result<Vec<PackageInfo>> {
        let client = reqwest::Client::new();
        let url = format!("{}/api/v1/packages/search?q={}", registry.url, name);

        let mut request = client.get(&url);
        if let Some(token) = &registry.auth_token {
            request = request.header("Authorization", format!("Bearer {}", token));
        }

        let response = request.send().await?;
        if !response.status().is_success() {
            anyhow::bail!("Registry search failed: {}", response.status());
        }

        let packages: Vec<PackageInfo> = response.json().await?;
        Ok(packages)
    }

    /// Download package from registry
    pub async fn download(
        &self,
        name: &str,
        version: &str,
        dest: &Path,
    ) -> Result<PackageMetadata> {
        // Try each registry in order
        for registry in &self.registries {
            if !registry.enabled {
                continue;
            }

            match self.download_from_registry(registry, name, version, dest).await {
                Ok(metadata) => {
                    println!(
                        "📦 Downloaded {} v{} from {}",
                        name.cyan(),
                        version,
                        registry.name.green()
                    );
                    return Ok(metadata);
                }
                Err(e) => {
                    println!(
                        "⚠️  Failed to download from {}: {}",
                        registry.name.yellow(),
                        e
                    );
                }
            }
        }

        anyhow::bail!("Package not found in any registry")
    }

    /// Download from a specific registry
    async fn download_from_registry(
        &self,
        registry: &Registry,
        name: &str,
        version: &str,
        dest: &Path,
    ) -> Result<PackageMetadata> {
        let client = reqwest::Client::new();
        let url = format!("{}/api/v1/packages/{}/{}/download", registry.url, name, version);

        let mut request = client.get(&url);
        if let Some(token) = &registry.auth_token {
            request = request.header("Authorization", format!("Bearer {}", token));
        }

        let response = request.send().await?;
        if !response.status().is_success() {
            anyhow::bail!("Package download failed: {}", response.status());
        }

        // Save package archive
        let archive_path = self.cache_dir.join(format!("{}-{}.tar.gz", name, version));
        let bytes = response.bytes().await?;
        fs::write(&archive_path, bytes)?;

        // Extract archive
        self.extract_archive(&archive_path, dest)?;

        // Load metadata
        let metadata_path = dest.join("porters.toml");
        let metadata = if metadata_path.exists() {
            let content = fs::read_to_string(&metadata_path)?;
            toml::from_str(&content)?
        } else {
            PackageMetadata {
                name: name.to_string(),
                version: version.to_string(),
                description: None,
                repository: None,
                dependencies: HashMap::new(),
            }
        };

        Ok(metadata)
    }

    /// Extract tar.gz archive
    fn extract_archive(&self, archive: &Path, dest: &Path) -> Result<()> {
        use flate2::read::GzDecoder;
        use tar::Archive;

        let file = fs::File::open(archive)?;
        let decoder = GzDecoder::new(file);
        let mut archive = Archive::new(decoder);

        fs::create_dir_all(dest)?;
        archive.unpack(dest)?;

        Ok(())
    }

    /// Publish package to registry
    pub async fn publish(
        &self,
        registry_name: &str,
        package_path: &Path,
        version: &str,
    ) -> Result<()> {
        let registry = self
            .registries
            .iter()
            .find(|r| r.name == registry_name)
            .ok_or_else(|| anyhow::anyhow!("Registry {} not found", registry_name))?;

        if !registry.enabled {
            anyhow::bail!("Registry {} is disabled", registry_name);
        }

        let auth_token = registry
            .auth_token
            .as_ref()
            .ok_or_else(|| anyhow::anyhow!("No auth token for registry {}", registry_name))?;

        // Create package archive
        let archive_path = self.create_package_archive(package_path, version)?;

        // Upload to registry
        let client = reqwest::Client::new();
        let url = format!("{}/api/v1/packages/publish", registry.url);

        // Read file contents for upload
        let file_bytes = tokio::fs::read(&archive_path).await?;
        let body = reqwest::Body::from(file_bytes);

        let response = client
            .post(&url)
            .header("Authorization", format!("Bearer {}", auth_token))
            .header("Content-Type", "application/gzip")
            .body(body)
            .send()
            .await?;

        if !response.status().is_success() {
            let error: String = response.text().await?;
            anyhow::bail!("Publish failed: {}", error);
        }

        println!(
            "✅ Published to {} successfully",
            registry_name.green()
        );
        Ok(())
    }

    /// Create package archive for publishing
    fn create_package_archive(&self, package_path: &Path, version: &str) -> Result<PathBuf> {
        use flate2::write::GzEncoder;
        use flate2::Compression;

        let name = package_path
            .file_name()
            .and_then(|n| n.to_str())
            .unwrap_or("package");

        let archive_path = self.cache_dir.join(format!("{}-{}.tar.gz", name, version));

        let tar_gz = fs::File::create(&archive_path)?;
        let enc = GzEncoder::new(tar_gz, Compression::default());
        let mut tar = tar::Builder::new(enc);

        tar.append_dir_all(".", package_path)?;
        tar.finish()?;

        Ok(archive_path)
    }

    /// List all configured registries
    pub fn list_registries(&self) {
        if self.registries.is_empty() {
            println!("No registries configured");
            return;
        }

        println!("📋 Configured registries:\n");
        for registry in &self.registries {
            let status = if registry.enabled { "✓" } else { "✗" };
            println!(
                "  {} {} - {}",
                status,
                registry.name.cyan(),
                registry.url.dimmed()
            );
        }
    }
}

/// Package information from registry
#[derive(Debug, Clone, Serialize, Deserialize)]
#[allow(dead_code)]
pub struct PackageInfo {
    pub name: String,
    pub version: String,
    pub description: Option<String>,
    pub downloads: u64,
    pub repository: Option<String>,
}

/// Package metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
#[allow(dead_code)]
pub struct PackageMetadata {
    pub name: String,
    pub version: String,
    pub description: Option<String>,
    pub repository: Option<String>,
    #[serde(default)]
    pub dependencies: HashMap<String, String>,
}
