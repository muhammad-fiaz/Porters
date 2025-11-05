//! Build artifact management
//!
//! This module provides functionality for tracking, packaging, and installing
//! build artifacts (executables, libraries, headers, etc.).
//!
//! **Note**: This is a future feature for build artifact tracking.

use anyhow::Result;
use colored::Colorize;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};
use walkdir::WalkDir;

/// Build artifact types
///
/// Categorizes different types of build outputs.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[allow(dead_code)]
pub enum ArtifactType {
    Executable,
    StaticLibrary,
    SharedLibrary,
    Object,
    Header,
    Archive,
    Unknown,
}

#[allow(dead_code)]
impl ArtifactType {
    /// Detect artifact type from file extension
    pub fn from_extension(ext: &str) -> Self {
        match ext.to_lowercase().as_str() {
            "exe" | "" => ArtifactType::Executable, // Unix executables have no extension
            "a" => ArtifactType::StaticLibrary,
            "so" | "dylib" | "dll" => ArtifactType::SharedLibrary,
            "o" | "obj" => ArtifactType::Object,
            "h" | "hpp" | "hxx" | "h++" => ArtifactType::Header,
            "tar" | "gz" | "zip" => ArtifactType::Archive,
            _ => ArtifactType::Unknown,
        }
    }
}

/// Build artifact information
#[derive(Debug, Clone, Serialize, Deserialize)]
#[allow(dead_code)]
pub struct Artifact {
    pub path: PathBuf,
    pub artifact_type: ArtifactType,
    pub size: u64,
    pub hash: String,
}

/// Artifact manifest for tracking build outputs
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[allow(dead_code)]
pub struct ArtifactManifest {
    pub artifacts: HashMap<String, Artifact>,
    pub build_time: Option<String>,
    pub target: Option<String>,
}

/// Artifact manager
#[allow(dead_code)]
pub struct ArtifactManager {
    manifest_path: PathBuf,
    manifest: ArtifactManifest,
}

#[allow(dead_code)]
impl ArtifactManager {
    /// Create a new artifact manager
    pub fn new(manifest_path: PathBuf) -> Result<Self> {
        let manifest = if manifest_path.exists() {
            let content = fs::read_to_string(&manifest_path)?;
            toml::from_str(&content).unwrap_or_default()
        } else {
            ArtifactManifest::default()
        };

        Ok(Self {
            manifest_path,
            manifest,
        })
    }

    /// Scan build directory for artifacts
    pub fn scan_build_dir(&mut self, build_dir: &Path, target: Option<&str>) -> Result<()> {
        println!("🔍 Scanning build artifacts...");

        self.manifest.artifacts.clear();
        self.manifest.target = target.map(|s| s.to_string());
        self.manifest.build_time = Some(chrono::Utc::now().to_rfc3339());

        // Scan for artifacts
        for entry in WalkDir::new(build_dir)
            .max_depth(5)
            .into_iter()
            .filter_map(|e| e.ok())
        {
            if entry.file_type().is_file() {
                self.process_file(entry.path())?;
            }
        }

        println!("✅ Found {} artifacts", self.manifest.artifacts.len());
        Ok(())
    }

    /// Process a single file
    fn process_file(&mut self, path: &Path) -> Result<()> {
        let ext = path.extension().and_then(|e| e.to_str()).unwrap_or("");

        let artifact_type = ArtifactType::from_extension(ext);

        // Skip object files and unknown types unless they're executables
        if matches!(artifact_type, ArtifactType::Object | ArtifactType::Unknown) {
            // Check if it's an executable on Unix (no extension)
            #[cfg(unix)]
            {
                use std::os::unix::fs::PermissionsExt;
                let metadata = fs::metadata(path)?;
                let permissions = metadata.permissions();
                if permissions.mode() & 0o111 == 0 {
                    return Ok(()); // Not executable, skip
                }
            }

            #[cfg(not(unix))]
            if matches!(artifact_type, ArtifactType::Unknown) {
                return Ok(());
            }
        }

        let metadata = fs::metadata(path)?;
        let hash = crate::hash::calculate_file_hash(path)?;

        let artifact = Artifact {
            path: path.to_path_buf(),
            artifact_type,
            size: metadata.len(),
            hash,
        };

        let name = path
            .file_name()
            .and_then(|n| n.to_str())
            .unwrap_or("unknown")
            .to_string();

        self.manifest.artifacts.insert(name, artifact);
        Ok(())
    }

    /// Save manifest to disk
    pub fn save(&self) -> Result<()> {
        let content = toml::to_string_pretty(&self.manifest)?;
        if let Some(parent) = self.manifest_path.parent() {
            fs::create_dir_all(parent)?;
        }
        fs::write(&self.manifest_path, content)?;
        Ok(())
    }

    /// List all artifacts
    pub fn list_artifacts(&self) {
        if self.manifest.artifacts.is_empty() {
            println!("No artifacts found");
            return;
        }

        println!("📦 Build Artifacts:\n");
        if let Some(target) = &self.manifest.target {
            println!("Target: {}\n", target.cyan());
        }

        for (name, artifact) in &self.manifest.artifacts {
            let type_str = match artifact.artifact_type {
                ArtifactType::Executable => "Executable".green(),
                ArtifactType::StaticLibrary => "Static Lib".blue(),
                ArtifactType::SharedLibrary => "Shared Lib".blue(),
                ArtifactType::Object => "Object".dimmed(),
                ArtifactType::Header => "Header".yellow(),
                ArtifactType::Archive => "Archive".magenta(),
                ArtifactType::Unknown => "Unknown".dimmed(),
            };

            println!(
                "  {} {} ({})",
                type_str,
                name.cyan(),
                Self::human_size(artifact.size)
            );
            println!("    Path: {}", artifact.path.display().to_string().dimmed());
            println!("    Hash: {}\n", artifact.hash[..16].dimmed());
        }
    }

    /// Package artifacts for distribution
    pub fn package(&self, output_path: &Path, name: &str, version: &str) -> Result<()> {
        use flate2::Compression;
        use flate2::write::GzEncoder;

        println!("📦 Packaging artifacts...");

        let archive_name = format!("{}-{}.tar.gz", name, version);
        let archive_path = output_path.join(&archive_name);

        let tar_gz = fs::File::create(&archive_path)?;
        let enc = GzEncoder::new(tar_gz, Compression::default());
        let mut tar = tar::Builder::new(enc);

        // Add executables and libraries
        for (artifact_name, artifact) in &self.manifest.artifacts {
            if matches!(
                artifact.artifact_type,
                ArtifactType::Executable
                    | ArtifactType::StaticLibrary
                    | ArtifactType::SharedLibrary
            ) {
                tar.append_path_with_name(&artifact.path, artifact_name)?;
            }
        }

        tar.finish()?;

        println!("✅ Created {}", archive_name.green());
        println!("   Location: {}", archive_path.display());

        Ok(())
    }

    /// Install artifacts to a destination
    pub fn install(&self, dest: &Path, artifact_types: &[ArtifactType]) -> Result<()> {
        println!("📦 Installing artifacts to {}...", dest.display());

        let bin_dir = dest.join("bin");
        let lib_dir = dest.join("lib");
        let include_dir = dest.join("include");

        fs::create_dir_all(&bin_dir)?;
        fs::create_dir_all(&lib_dir)?;
        fs::create_dir_all(&include_dir)?;

        let mut count = 0;
        for (name, artifact) in &self.manifest.artifacts {
            if !artifact_types.contains(&artifact.artifact_type) {
                continue;
            }

            let dest_path = match artifact.artifact_type {
                ArtifactType::Executable => bin_dir.join(name),
                ArtifactType::StaticLibrary | ArtifactType::SharedLibrary => lib_dir.join(name),
                ArtifactType::Header => include_dir.join(name),
                _ => continue,
            };

            fs::copy(&artifact.path, &dest_path)?;
            count += 1;
        }

        println!("✅ Installed {} artifacts", count);
        Ok(())
    }

    /// Clean artifacts from build directory
    pub fn clean(&self, build_dir: &Path) -> Result<()> {
        println!("🗑️  Cleaning build artifacts...");

        let mut count = 0;
        for artifact in self.manifest.artifacts.values() {
            if artifact.path.exists() && artifact.path.starts_with(build_dir) {
                fs::remove_file(&artifact.path)?;
                count += 1;
            }
        }

        println!("✅ Removed {} artifacts", count);
        Ok(())
    }

    /// Get artifact statistics
    pub fn stats(&self) -> ArtifactStats {
        let mut stats = ArtifactStats::default();

        for artifact in self.manifest.artifacts.values() {
            stats.total_count += 1;
            stats.total_size += artifact.size;

            match artifact.artifact_type {
                ArtifactType::Executable => stats.executables += 1,
                ArtifactType::StaticLibrary => stats.static_libs += 1,
                ArtifactType::SharedLibrary => stats.shared_libs += 1,
                ArtifactType::Object => stats.objects += 1,
                ArtifactType::Header => stats.headers += 1,
                ArtifactType::Archive => stats.archives += 1,
                ArtifactType::Unknown => stats.unknown += 1,
            }
        }

        stats
    }

    fn human_size(size: u64) -> String {
        let size = size as f64;
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

#[derive(Debug, Default)]
#[allow(dead_code)]
pub struct ArtifactStats {
    pub total_count: usize,
    pub total_size: u64,
    pub executables: usize,
    pub static_libs: usize,
    pub shared_libs: usize,
    pub objects: usize,
    pub headers: usize,
    pub archives: usize,
    pub unknown: usize,
}
