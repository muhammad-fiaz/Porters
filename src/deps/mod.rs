//! Dependency management system
//!
//! This module handles dependency resolution, fetching, and validation.
//! It supports Git-based dependencies, local path dependencies, and optional
//! registry integration, ensuring proper version tracking and checksum verification.

use anyhow::{Context, Result};
use git2::Repository;
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::collections::HashMap;
use std::path::{Path, PathBuf};

use crate::config::{Dependency, PortersConfig};
use crate::scan;
use crate::util::pretty::*;

#[allow(unused_imports)]
pub mod resolver;

/// A resolved dependency with all necessary build information
///
/// Contains the dependency's location, version, source metadata,
/// and paths for includes and libraries needed during compilation.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResolvedDependency {
    pub name: String,
    pub version: String,
    pub source: DependencySource,
    pub path: PathBuf,
    pub include_paths: Vec<PathBuf>,
    pub lib_paths: Vec<PathBuf>,
    pub checksum: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum DependencySource {
    Git { url: String, rev: String },
    Path { path: String },
    Registry { registry: String },
}

/// Lockfile structure (legacy - see lockfile.rs module for current implementation)
#[allow(dead_code)]
#[derive(Debug, Serialize, Deserialize)]
pub struct Lockfile {
    pub version: String,
    pub dependencies: HashMap<String, LockedDependency>,
}

/// Locked dependency entry (legacy - see lockfile.rs module)
#[allow(dead_code)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LockedDependency {
    pub version: String,
    pub source: DependencySource,
    pub checksum: String,
}

/// Resolve all dependencies from the configuration
pub async fn resolve_dependencies(config: &PortersConfig) -> Result<Vec<ResolvedDependency>> {
    let mut resolved = Vec::new();
    let cache_dir = get_cache_dir()?;

    for (name, dep) in config.all_dependencies() {
        print_package(&format!("Resolving {}...", name));

        let resolved_dep = resolve_dependency(&name, dep, &cache_dir).await?;
        resolved.push(resolved_dep);
    }

    Ok(resolved)
}

/// Resolve a single dependency
async fn resolve_dependency(
    name: &str,
    dep: &Dependency,
    cache_dir: &Path,
) -> Result<ResolvedDependency> {
    match dep {
        Dependency::Simple(version) => {
            // For now, simple dependencies are not supported (future registry feature)
            print_warning(&format!(
                "Registry dependencies not yet supported: {} = {}",
                name, version
            ));
            Err(anyhow::anyhow!("Registry dependencies not yet supported"))
        }
        Dependency::Detailed {
            git,
            branch,
            tag,
            rev,
            path,
            version,
            ..
        } => {
            if let Some(git_url) = git {
                resolve_git_dependency(
                    name,
                    git_url,
                    branch.as_deref(),
                    tag.as_deref(),
                    rev.as_deref(),
                    cache_dir,
                )
                .await
            } else if let Some(local_path) = path {
                resolve_path_dependency(name, local_path)
            } else if let Some(_ver) = version {
                print_warning(&format!(
                    "Registry dependencies not yet supported: {}",
                    name
                ));
                Err(anyhow::anyhow!("Registry dependencies not yet supported"))
            } else {
                Err(anyhow::anyhow!(
                    "Invalid dependency specification for {}",
                    name
                ))
            }
        }
    }
}

/// Resolve a Git dependency
async fn resolve_git_dependency(
    name: &str,
    url: &str,
    branch: Option<&str>,
    tag: Option<&str>,
    rev: Option<&str>,
    cache_dir: &Path,
) -> Result<ResolvedDependency> {
    // Create a unique directory name for this dependency
    let mut hasher = Sha256::new();
    hasher.update(url.as_bytes());
    let hash = hex::encode(hasher.finalize());
    let short_hash = &hash[..12];

    let dep_dir = cache_dir
        .join("sources")
        .join(format!("{}-{}", name, short_hash));

    // Clone or update the repository
    if dep_dir.exists() {
        print_info(&format!("Updating {} from git...", name));
        let repo = Repository::open(&dep_dir)
            .with_context(|| format!("Failed to open repository at {}", dep_dir.display()))?;

        // Fetch updates
        {
            let mut remote = repo.find_remote("origin")?;
            remote.fetch(&["refs/heads/*:refs/heads/*"], None, None)?;
        }

        // Checkout the requested revision
        let commit_id = if let Some(rev_str) = rev {
            let oid = git2::Oid::from_str(rev_str)?;
            repo.find_commit(oid)?.id()
        } else if let Some(tag_str) = tag {
            let reference = repo.find_reference(&format!("refs/tags/{}", tag_str))?;
            reference.peel_to_commit()?.id()
        } else if let Some(branch_str) = branch {
            let reference = repo.find_reference(&format!("refs/heads/{}", branch_str))?;
            reference.peel_to_commit()?.id()
        } else {
            // Default to HEAD of default branch
            repo.head()?.peel_to_commit()?.id()
        };

        // Checkout the commit
        let commit = repo.find_commit(commit_id)?;
        repo.checkout_tree(commit.as_object(), None)?;
        repo.set_head_detached(commit_id)?;

        // Calculate checksum of the dependency
        print_info("Calculating checksum...");
        let checksum = crate::hash::calculate_directory_hash(&dep_dir)
            .with_context(|| format!("Failed to calculate checksum for {}", dep_dir.display()))?;

        // Scan for include paths
        let sources = scan::scan_project(&dep_dir)?;

        Ok(ResolvedDependency {
            name: name.to_string(),
            version: commit_id.to_string()[..8].to_string(),
            source: DependencySource::Git {
                url: url.to_string(),
                rev: commit_id.to_string(),
            },
            path: dep_dir,
            include_paths: sources.include_paths,
            lib_paths: vec![],
            checksum: Some(checksum),
        })
    } else {
        print_info(&format!("Cloning {} from {}...", name, url));
        std::fs::create_dir_all(&dep_dir)?;

        let repo = Repository::clone(url, &dep_dir)
            .with_context(|| format!("Failed to clone repository: {}", url))?;

        // Checkout the requested revision
        let commit_id = if let Some(rev_str) = rev {
            let oid = git2::Oid::from_str(rev_str)?;
            repo.find_commit(oid)?.id()
        } else if let Some(tag_str) = tag {
            let reference = repo.find_reference(&format!("refs/tags/{}", tag_str))?;
            reference.peel_to_commit()?.id()
        } else if let Some(branch_str) = branch {
            let reference = repo.find_reference(&format!("refs/heads/{}", branch_str))?;
            reference.peel_to_commit()?.id()
        } else {
            // Default to HEAD of default branch
            repo.head()?.peel_to_commit()?.id()
        };

        // Checkout the commit
        let commit = repo.find_commit(commit_id)?;
        repo.checkout_tree(commit.as_object(), None)?;
        repo.set_head_detached(commit_id)?;

        // Calculate checksum of the dependency
        print_info("Calculating checksum...");
        let checksum = crate::hash::calculate_directory_hash(&dep_dir)
            .with_context(|| format!("Failed to calculate checksum for {}", dep_dir.display()))?;

        // Scan for include paths
        let sources = scan::scan_project(&dep_dir)?;

        Ok(ResolvedDependency {
            name: name.to_string(),
            version: commit_id.to_string()[..8].to_string(),
            source: DependencySource::Git {
                url: url.to_string(),
                rev: commit_id.to_string(),
            },
            path: dep_dir,
            include_paths: sources.include_paths,
            lib_paths: vec![],
            checksum: Some(checksum),
        })
    }
}

/// Resolve a path dependency
fn resolve_path_dependency(name: &str, path: &str) -> Result<ResolvedDependency> {
    let path_buf = PathBuf::from(path);

    if !path_buf.exists() {
        return Err(anyhow::anyhow!("Path dependency not found: {}", path));
    }

    let abs_path = path_buf
        .canonicalize()
        .with_context(|| format!("Failed to canonicalize path: {}", path))?;

    // Calculate checksum for path dependency
    print_info("Calculating checksum...");
    let checksum = crate::hash::calculate_directory_hash(&abs_path)
        .with_context(|| format!("Failed to calculate checksum for {}", abs_path.display()))?;

    // Scan for include paths
    let sources = scan::scan_project(&abs_path)?;

    Ok(ResolvedDependency {
        name: name.to_string(),
        version: "local".to_string(),
        source: DependencySource::Path {
            path: path.to_string(),
        },
        path: abs_path,
        include_paths: sources.include_paths,
        lib_paths: vec![],
        checksum: Some(checksum),
    })
}

/// Update all dependencies
pub async fn update_dependencies(config: &PortersConfig) -> Result<()> {
    print_step("Updating dependencies");

    let cache_dir = get_cache_dir()?;

    for (name, dep) in config.all_dependencies() {
        if let Dependency::Detailed { git: Some(_), .. } = dep {
            print_package(&format!("Updating {}...", name));
            resolve_dependency(&name, dep, &cache_dir).await?;
        }
    }

    Ok(())
}

/// Generate a lockfile from resolved dependencies
/// Generate lockfile from resolved dependencies (legacy API)
///
/// **Note**: Current implementation uses lockfile.rs module
#[allow(dead_code)]
pub fn generate_lockfile(deps: &[ResolvedDependency], path: &str) -> Result<()> {
    let mut locked_deps = HashMap::new();

    for dep in deps {
        let checksum = compute_checksum(&dep.path)?;

        locked_deps.insert(
            dep.name.clone(),
            LockedDependency {
                version: dep.version.clone(),
                source: dep.source.clone(),
                checksum,
            },
        );
    }

    let lockfile = Lockfile {
        version: "1".to_string(),
        dependencies: locked_deps,
    };

    let content = toml::to_string_pretty(&lockfile)?;
    std::fs::write(path, content)?;

    Ok(())
}

/// Vendor dependencies into a local directory
pub async fn vendor_dependencies(config: &PortersConfig, vendor_dir: &str) -> Result<()> {
    let vendor_path = Path::new(vendor_dir);
    std::fs::create_dir_all(vendor_path)?;

    let deps = resolve_dependencies(config).await?;

    for dep in deps {
        print_package(&format!("Vendoring {}...", dep.name));

        let target_dir = vendor_path.join(&dep.name);

        if target_dir.exists() {
            std::fs::remove_dir_all(&target_dir)?;
        }

        copy_dir_all(&dep.path, &target_dir)?;
    }

    Ok(())
}

/// Print dependency graph
pub fn print_dependency_graph(deps: &[ResolvedDependency]) -> Result<()> {
    for dep in deps {
        print_graph_node(&dep.name, &dep.version, 0);

        match &dep.source {
            DependencySource::Git { url, rev } => {
                println!("    ├─ Source: git ({})", url);
                println!("    └─ Revision: {}", rev);
            }
            DependencySource::Path { path } => {
                println!("    └─ Source: path ({})", path);
            }
            DependencySource::Registry { registry } => {
                println!("    └─ Source: registry ({})", registry);
            }
        }
    }

    Ok(())
}

/// Get the cache directory
fn get_cache_dir() -> Result<PathBuf> {
    let home = dirs::home_dir().ok_or_else(|| anyhow::anyhow!("Could not find home directory"))?;
    let cache_dir = home.join(".porters").join("cache");
    std::fs::create_dir_all(&cache_dir)?;
    Ok(cache_dir)
}

/// Compute checksum of a directory
/// Compute checksum of directory (utility function)
#[allow(dead_code)]
fn compute_checksum(path: &Path) -> Result<String> {
    let mut hasher = Sha256::new();

    for entry in walkdir::WalkDir::new(path).sort_by_file_name() {
        let entry = entry?;
        if entry.file_type().is_file() {
            let content = std::fs::read(entry.path())?;
            hasher.update(&content);
        }
    }

    Ok(hex::encode(hasher.finalize()))
}

/// Copy a directory recursively
fn copy_dir_all(src: &Path, dst: &Path) -> Result<()> {
    std::fs::create_dir_all(dst)?;

    for entry in std::fs::read_dir(src)? {
        let entry = entry?;
        let ty = entry.file_type()?;
        let src_path = entry.path();
        let dst_path = dst.join(entry.file_name());

        if ty.is_dir() {
            copy_dir_all(&src_path, &dst_path)?;
        } else {
            std::fs::copy(&src_path, &dst_path)?;
        }
    }

    Ok(())
}

/// Clone a git repository to a destination path
pub async fn clone_git_repo(url: &str, dest: &Path) -> Result<String> {
    print_info(&format!("Cloning {} to {}...", url, dest.display()));

    std::fs::create_dir_all(dest)?;

    Repository::clone(url, dest).with_context(|| format!("Failed to clone repository: {}", url))?;

    print_success("Clone complete");

    // Calculate checksum of the cloned repository
    print_info("Calculating checksum...");
    let checksum = crate::hash::calculate_directory_hash(dest)
        .with_context(|| format!("Failed to calculate checksum for {}", dest.display()))?;

    print_info(&format!("Checksum: {}", &checksum[..16]));

    Ok(checksum)
}
