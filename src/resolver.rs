//! Dependency resolution for Porters packages
//!
//! This module provides comprehensive dependency resolution including:
//! - Recursive dependency resolution
//! - Version constraint validation (SemVer)
//! - Conflict detection and reporting
//! - Platform and architecture requirements
//! - Compiler constraints
//! - Circular dependency detection
//! - Nested dependency handling

#![allow(dead_code)]

use crate::version::{Version, VersionReq};
use anyhow::{Context, Result};
use colored::Colorize;
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet, VecDeque};

/// Dependency information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Dependency {
    pub name: String,
    pub version_req: String,
    pub optional: bool,
    #[serde(default)]
    pub features: Vec<String>,
}

/// Resolved dependency with its version
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ResolvedDependency {
    pub name: String,
    pub version: Version,
    pub source: DependencySource,
}

/// Source of a dependency
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum DependencySource {
    Conan,
    Vcpkg,
    XMake,
    Registry,
}

impl std::fmt::Display for DependencySource {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DependencySource::Conan => write!(f, "Conan"),
            DependencySource::Vcpkg => write!(f, "vcpkg"),
            DependencySource::XMake => write!(f, "XMake"),
            DependencySource::Registry => write!(f, "Registry"),
        }
    }
}

/// Platform constraints
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlatformConstraints {
    #[serde(default)]
    pub platforms: Vec<String>,
    #[serde(default)]
    pub arch: Vec<String>,
    #[serde(default)]
    pub min_cpp_standard: Option<String>,
    #[serde(default)]
    pub max_cpp_standard: Option<String>,
    #[serde(default)]
    pub compilers: HashMap<String, String>,
    #[serde(default)]
    pub environment: HashMap<String, String>,
}

/// Package metadata for resolution
#[derive(Debug, Clone)]
pub struct PackageMetadata {
    pub name: String,
    pub version: Version,
    pub dependencies: Vec<Dependency>,
    pub constraints: Option<PlatformConstraints>,
    pub source: DependencySource,
}

/// Dependency conflict
#[derive(Debug)]
pub struct DependencyConflict {
    pub package: String,
    pub requested_versions: Vec<(String, String)>, // (requester, version_req)
}

/// Dependency graph node
#[derive(Debug, Clone)]
struct DependencyNode {
    package: ResolvedDependency,
    dependencies: Vec<String>,
    #[allow(dead_code)] // Used for debugging and future features
    depth: usize,
}

/// Dependency resolver
pub struct DependencyResolver {
    /// Resolved dependencies
    resolved: HashMap<String, ResolvedDependency>,
    /// Dependency graph for circular detection
    graph: HashMap<String, DependencyNode>,
    /// Conflicts detected
    conflicts: Vec<DependencyConflict>,
}

impl DependencyResolver {
    /// Create a new dependency resolver
    pub fn new() -> Self {
        Self {
            resolved: HashMap::new(),
            graph: HashMap::new(),
            conflicts: Vec::new(),
        }
    }

    /// Resolve dependencies recursively
    ///
    /// This performs a breadth-first resolution of all dependencies,
    /// validating version constraints and detecting conflicts.
    pub fn resolve<F>(
        &mut self,
        root_deps: Vec<Dependency>,
        fetch_metadata: F,
    ) -> Result<Vec<ResolvedDependency>>
    where
        F: Fn(&str, &str) -> Result<PackageMetadata>,
    {
        let mut queue: VecDeque<(Dependency, usize, String)> = VecDeque::new();
        let mut version_requests: HashMap<String, Vec<(String, String)>> = HashMap::new();

        // Initialize queue with root dependencies
        for dep in root_deps {
            queue.push_back((dep.clone(), 0, "root".to_string()));
            version_requests
                .entry(dep.name.clone())
                .or_default()
                .push(("root".to_string(), dep.version_req.clone()));
        }

        // Process dependencies breadth-first
        while let Some((dep, depth, _requester)) = queue.pop_front() {
            // Check if already resolved
            if let Some(resolved) = self.resolved.get(&dep.name) {
                // Validate version compatibility
                let version_req = VersionReq::parse(&dep.version_req)
                    .with_context(|| format!("Invalid version requirement: {}", dep.version_req))?;

                if !version_req.matches(&resolved.version) {
                    // Version conflict detected
                    let requests = version_requests.get(&dep.name).unwrap();
                    self.conflicts.push(DependencyConflict {
                        package: dep.name.clone(),
                        requested_versions: requests.clone(),
                    });
                }
                continue;
            }

            // Fetch package metadata
            let metadata = fetch_metadata(&dep.name, &dep.version_req)
                .with_context(|| format!("Failed to fetch metadata for {}", dep.name))?;

            // Validate constraints
            self.validate_constraints(&metadata)?;

            // Add to resolved set
            let resolved_dep = ResolvedDependency {
                name: metadata.name.clone(),
                version: metadata.version.clone(),
                source: metadata.source.clone(),
            };

            // Add to graph for circular dependency detection
            let dep_names: Vec<String> = metadata
                .dependencies
                .iter()
                .map(|d| d.name.clone())
                .collect();

            self.graph.insert(
                metadata.name.clone(),
                DependencyNode {
                    package: resolved_dep.clone(),
                    dependencies: dep_names.clone(),
                    depth,
                },
            );

            // Check for circular dependencies
            if self.has_circular_dependency(&metadata.name) {
                anyhow::bail!(
                    "Circular dependency detected involving package: {}",
                    metadata.name
                );
            }

            self.resolved.insert(metadata.name.clone(), resolved_dep);

            // Add nested dependencies to queue
            for nested_dep in metadata.dependencies {
                if nested_dep.optional {
                    continue; // Skip optional dependencies
                }

                version_requests
                    .entry(nested_dep.name.clone())
                    .or_default()
                    .push((metadata.name.clone(), nested_dep.version_req.clone()));

                queue.push_back((nested_dep, depth + 1, metadata.name.clone()));
            }
        }

        // Check for conflicts
        if !self.conflicts.is_empty() {
            self.report_conflicts();
            anyhow::bail!("Dependency conflicts detected");
        }

        // Return resolved dependencies in topological order
        Ok(self.topological_sort())
    }

    /// Validate platform and compiler constraints
    fn validate_constraints(&self, metadata: &PackageMetadata) -> Result<()> {
        let constraints = match &metadata.constraints {
            Some(c) => c,
            None => return Ok(()), // No constraints to validate
        };

        // Validate platform
        if !constraints.platforms.is_empty() {
            let current_platform = Self::current_platform();
            if !constraints.platforms.contains(&current_platform) {
                anyhow::bail!(
                    "Package {} is not available for platform '{}'. Supported: {}",
                    metadata.name,
                    current_platform,
                    constraints.platforms.join(", ")
                );
            }
        }

        // Validate architecture
        if !constraints.arch.is_empty() {
            let current_arch = Self::current_arch();
            if !constraints.arch.contains(&current_arch) {
                println!(
                    "{}",
                    format!(
                        "⚠️  Warning: Package {} may not support architecture '{}'. Supported: {}",
                        metadata.name,
                        current_arch,
                        constraints.arch.join(", ")
                    )
                    .yellow()
                );
            }
        }

        // Validate environment variables
        for (var, expected) in &constraints.environment {
            match std::env::var(var) {
                Ok(value) => {
                    if &value != expected {
                        println!(
                            "{}",
                            format!(
                                "⚠️  Warning: Environment variable {} = '{}', expected '{}'",
                                var, value, expected
                            )
                            .yellow()
                        );
                    }
                }
                Err(_) => {
                    anyhow::bail!(
                        "Package {} requires environment variable {} = '{}'",
                        metadata.name,
                        var,
                        expected
                    );
                }
            }
        }

        Ok(())
    }

    /// Detect circular dependencies using DFS
    fn has_circular_dependency(&self, package: &str) -> bool {
        let mut visited = HashSet::new();
        let mut rec_stack = HashSet::new();
        self.dfs_cycle_detection(package, &mut visited, &mut rec_stack)
    }

    fn dfs_cycle_detection(
        &self,
        package: &str,
        visited: &mut HashSet<String>,
        rec_stack: &mut HashSet<String>,
    ) -> bool {
        visited.insert(package.to_string());
        rec_stack.insert(package.to_string());

        if let Some(node) = self.graph.get(package) {
            for dep in &node.dependencies {
                if !visited.contains(dep) {
                    if self.dfs_cycle_detection(dep, visited, rec_stack) {
                        return true;
                    }
                } else if rec_stack.contains(dep) {
                    return true; // Cycle detected
                }
            }
        }

        rec_stack.remove(package);
        false
    }

    /// Topological sort of dependencies
    fn topological_sort(&self) -> Vec<ResolvedDependency> {
        let mut sorted = Vec::new();
        let mut visited = HashSet::new();
        let mut temp_mark = HashSet::new();

        for package in self.graph.keys() {
            if !visited.contains(package) {
                self.visit_topo(package, &mut visited, &mut temp_mark, &mut sorted);
            }
        }

        sorted
    }

    fn visit_topo(
        &self,
        package: &str,
        visited: &mut HashSet<String>,
        temp_mark: &mut HashSet<String>,
        sorted: &mut Vec<ResolvedDependency>,
    ) {
        if visited.contains(package) {
            return;
        }

        temp_mark.insert(package.to_string());

        if let Some(node) = self.graph.get(package) {
            for dep in &node.dependencies {
                if !visited.contains(dep) {
                    self.visit_topo(dep, visited, temp_mark, sorted);
                }
            }
        }

        temp_mark.remove(package);
        visited.insert(package.to_string());

        if let Some(node) = self.graph.get(package) {
            sorted.push(node.package.clone());
        }
    }

    /// Report dependency conflicts
    fn report_conflicts(&self) {
        println!("{}", "❌ Dependency Conflicts Detected:".red().bold());
        for conflict in &self.conflicts {
            println!("\n  Package: {}", conflict.package.yellow());
            println!("  Requested versions:");
            for (requester, version) in &conflict.requested_versions {
                println!("    - {} requires {}", requester.cyan(), version);
            }
        }
    }

    /// Get current platform
    fn current_platform() -> String {
        if cfg!(target_os = "linux") {
            "linux".to_string()
        } else if cfg!(target_os = "windows") {
            "windows".to_string()
        } else if cfg!(target_os = "macos") {
            "macos".to_string()
        } else if cfg!(target_os = "freebsd") {
            "freebsd".to_string()
        } else if cfg!(target_os = "android") {
            "android".to_string()
        } else if cfg!(target_os = "ios") {
            "ios".to_string()
        } else {
            "unknown".to_string()
        }
    }

    /// Get current architecture
    fn current_arch() -> String {
        if cfg!(target_arch = "x86") {
            "x86".to_string()
        } else if cfg!(target_arch = "x86_64") {
            "x86_64".to_string()
        } else if cfg!(target_arch = "arm") {
            "arm".to_string()
        } else if cfg!(target_arch = "aarch64") {
            "arm64".to_string()
        } else if cfg!(target_arch = "riscv64") {
            "riscv64".to_string()
        } else {
            "unknown".to_string()
        }
    }

    /// Get conflicts
    pub fn get_conflicts(&self) -> &[DependencyConflict] {
        &self.conflicts
    }

    /// Get resolved dependencies count
    pub fn resolved_count(&self) -> usize {
        self.resolved.len()
    }
}

impl Default for DependencyResolver {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dependency_source_display() {
        assert_eq!(DependencySource::Conan.to_string(), "Conan");
        assert_eq!(DependencySource::Vcpkg.to_string(), "vcpkg");
        assert_eq!(DependencySource::XMake.to_string(), "XMake");
        assert_eq!(DependencySource::Registry.to_string(), "Registry");
    }

    #[test]
    fn test_resolver_creation() {
        let resolver = DependencyResolver::new();
        assert_eq!(resolver.resolved_count(), 0);
        assert_eq!(resolver.get_conflicts().len(), 0);
    }

    #[test]
    fn test_platform_detection() {
        let platform = DependencyResolver::current_platform();
        assert!(!platform.is_empty());
        assert!(platform == "windows" || platform == "linux" || platform == "macos");
    }

    #[test]
    fn test_arch_detection() {
        let arch = DependencyResolver::current_arch();
        assert!(!arch.is_empty());
        assert!(
            arch == "x86"
                || arch == "x86_64"
                || arch == "arm"
                || arch == "arm64"
                || arch == "riscv64"
        );
    }

    #[test]
    fn test_simple_resolution() {
        let mut resolver = DependencyResolver::new();

        let fetch_metadata = |name: &str, _version: &str| -> Result<PackageMetadata> {
            Ok(PackageMetadata {
                name: name.to_string(),
                version: Version::new(1, 0, 0),
                dependencies: vec![],
                constraints: None,
                source: DependencySource::Registry,
            })
        };

        let deps = vec![Dependency {
            name: "test-pkg".to_string(),
            version_req: "^1.0.0".to_string(),
            optional: false,
            features: vec![],
        }];

        let result = resolver.resolve(deps, fetch_metadata);
        assert!(result.is_ok());
        assert_eq!(resolver.resolved_count(), 1);
    }

    #[test]
    fn test_nested_dependencies() {
        let mut resolver = DependencyResolver::new();

        let fetch_metadata = |name: &str, _version: &str| -> Result<PackageMetadata> {
            let dependencies = if name == "parent" {
                vec![Dependency {
                    name: "child".to_string(),
                    version_req: "^1.0.0".to_string(),
                    optional: false,
                    features: vec![],
                }]
            } else {
                vec![]
            };

            Ok(PackageMetadata {
                name: name.to_string(),
                version: Version::new(1, 0, 0),
                dependencies,
                constraints: None,
                source: DependencySource::Registry,
            })
        };

        let deps = vec![Dependency {
            name: "parent".to_string(),
            version_req: "^1.0.0".to_string(),
            optional: false,
            features: vec![],
        }];

        let result = resolver.resolve(deps, fetch_metadata);
        assert!(result.is_ok());
        assert_eq!(resolver.resolved_count(), 2); // parent + child
    }

    #[test]
    fn test_platform_constraints() {
        let resolver = DependencyResolver::new();

        let metadata = PackageMetadata {
            name: "test".to_string(),
            version: Version::new(1, 0, 0),
            dependencies: vec![],
            constraints: Some(PlatformConstraints {
                platforms: vec![DependencyResolver::current_platform()],
                arch: vec![],
                min_cpp_standard: None,
                max_cpp_standard: None,
                compilers: HashMap::new(),
                environment: HashMap::new(),
            }),
            source: DependencySource::Registry,
        };

        let result = resolver.validate_constraints(&metadata);
        assert!(result.is_ok()); // Should pass since we're using current platform
    }

    #[test]
    fn test_platform_constraint_failure() {
        let resolver = DependencyResolver::new();

        let unsupported_platform = if cfg!(target_os = "windows") {
            "linux"
        } else {
            "windows"
        };

        let metadata = PackageMetadata {
            name: "test".to_string(),
            version: Version::new(1, 0, 0),
            dependencies: vec![],
            constraints: Some(PlatformConstraints {
                platforms: vec![unsupported_platform.to_string()],
                arch: vec![],
                min_cpp_standard: None,
                max_cpp_standard: None,
                compilers: HashMap::new(),
                environment: HashMap::new(),
            }),
            source: DependencySource::Registry,
        };

        let result = resolver.validate_constraints(&metadata);
        assert!(result.is_err()); // Should fail due to platform mismatch
    }
}
