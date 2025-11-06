//! Advanced dependency resolution with constraint solving
//!
//! This module handles complex dependency resolution scenarios including:
//! - Version constraints and conflict detection
//! - Transitive dependency tracking
//! - Dependency graph analysis with topological sorting
//! - Support for Git, path, and registry sources

use crate::config::{Dependency, PortersConfig};
use anyhow::{Context, Result, anyhow};
use petgraph::algo::toposort;
use petgraph::graph::{DiGraph, NodeIndex};
use semver::Version;
use std::collections::HashMap;
use std::path::PathBuf;

/// Dependency resolver with constraint solving and graph analysis
///
/// Builds a dependency graph, detects conflicts, and produces a
/// topologically sorted resolution order for proper build sequencing.
///
/// **Note**: Advanced feature for future constraint-based resolution
#[allow(dead_code)]
pub struct DependencyResolver {
    resolved: HashMap<String, ResolvedDependency>,
    conflicts: Vec<DependencyConflict>,
    dep_graph: DiGraph<String, ()>,
    node_map: HashMap<String, NodeIndex>,
}

/// Resolved dependency with version and features
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct ResolvedDependency {
    pub name: String,
    pub version: Option<Version>,
    pub source: DependencySource,
    pub features: Vec<String>,
    pub transitive_deps: Vec<String>,
}

/// Dependency source specification
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub enum DependencySource {
    Git { url: String, rev: String },
    Path { path: String },
    Registry { version: String },
}

/// Dependency version conflict information
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct DependencyConflict {
    pub package: String,
    pub requested_versions: Vec<String>,
}

#[allow(dead_code)]
impl DependencyResolver {
    pub fn new() -> Self {
        Self {
            resolved: HashMap::new(),
            conflicts: Vec::new(),
            dep_graph: DiGraph::new(),
            node_map: HashMap::new(),
        }
    }

    /// Resolve all dependencies with constraint checking and transitive resolution
    pub async fn resolve(
        &mut self,
        config: &PortersConfig,
    ) -> Result<HashMap<String, ResolvedDependency>> {
        let current_platform = get_current_platform();

        // Collect all dependencies (regular + dev)
        let mut all_deps = config.dependencies.clone();
        all_deps.extend(config.dev_dependencies.clone());

        // Resolve dependencies recursively
        for (name, dep) in all_deps {
            self.resolve_dependency_recursive(&name, &dep, &current_platform, None)
                .await?;
        }

        // Topologically sort to get dependency order
        match toposort(&self.dep_graph, None) {
            Ok(sorted) => {
                // Dependencies are now in correct build order
                let build_order: Vec<String> = sorted
                    .iter()
                    .filter_map(|idx| self.dep_graph.node_weight(*idx).cloned())
                    .collect();

                if !build_order.is_empty() {
                    println!(
                        "📦 Resolved dependency build order: {}",
                        build_order.join(" → ")
                    );
                }
            }
            Err(cycle) => {
                let unknown = "unknown".to_string();
                let cycle_node = self
                    .dep_graph
                    .node_weight(cycle.node_id())
                    .unwrap_or(&unknown);
                return Err(anyhow!(
                    "Circular dependency detected involving: {}",
                    cycle_node
                ));
            }
        }

        // Check for conflicts
        if !self.conflicts.is_empty() {
            return Err(anyhow!(
                "Dependency conflicts detected:\n{}",
                self.format_conflicts()
            ));
        }

        Ok(self.resolved.clone())
    }

    /// Recursively resolve a dependency and its transitive dependencies
    async fn resolve_dependency_recursive(
        &mut self,
        name: &str,
        dep: &Dependency,
        platform: &str,
        parent: Option<&str>,
    ) -> Result<()> {
        // Skip if already resolved
        if self.resolved.contains_key(name) {
            // Add edge in dependency graph if parent exists
            if let Some(parent_name) = parent {
                self.add_dependency_edge(parent_name, name);
            }
            return Ok(());
        }

        match dep {
            Dependency::Simple(version) => {
                let transitive_deps = Vec::new();
                self.add_resolved(
                    name,
                    None,
                    DependencySource::Registry {
                        version: version.clone(),
                    },
                    vec![],
                    transitive_deps,
                )?;

                if let Some(parent_name) = parent {
                    self.add_dependency_edge(parent_name, name);
                }
            }
            Dependency::Detailed {
                version,
                git,
                path,
                features,
                platforms,
                constraints,
                ..
            } => {
                // Check platform compatibility
                if let Some(allowed_platforms) = platforms
                    && !allowed_platforms.contains(&platform.to_string())
                {
                    // Skip platform-specific dependency
                    return Ok(());
                }

                // Check version constraints
                if let Some(constraint) = &**constraints
                    && !self.check_constraint(constraint)?
                {
                    return Err(anyhow!(
                        "Constraint '{}' not satisfied for {}",
                        constraint,
                        name
                    ));
                }

                // Determine source and resolve transitively
                let (source, transitive_deps) = if let Some(git_url) = git {
                    // For git dependencies, could parse their config (future enhancement)
                    let deps = Vec::new(); // Simplified for now
                    (
                        DependencySource::Git {
                            url: git_url.clone(),
                            rev: "HEAD".to_string(),
                        },
                        deps,
                    )
                } else if let Some(path_str) = path {
                    // For path dependencies, parse their porters.toml
                    let deps = self.resolve_path_transitive(path_str)?;
                    (
                        DependencySource::Path {
                            path: path_str.clone(),
                        },
                        deps,
                    )
                } else {
                    // Registry dependencies (future feature)
                    (
                        DependencySource::Registry {
                            version: version.clone().unwrap_or("*".to_string()),
                        },
                        Vec::new(),
                    )
                };

                let ver = version.as_ref().and_then(|v| Version::parse(v).ok());

                self.add_resolved(
                    name,
                    ver,
                    source.clone(),
                    features.clone(),
                    transitive_deps.clone(),
                )?;

                if let Some(parent_name) = parent {
                    self.add_dependency_edge(parent_name, name);
                }

                // Recursively resolve transitive dependencies
                for trans_dep_name in &transitive_deps {
                    if let Some(trans_dep_spec) =
                        self.get_transitive_dep_spec(trans_dep_name, &source).await
                    {
                        Box::pin(self.resolve_dependency_recursive(
                            trans_dep_name,
                            &trans_dep_spec,
                            platform,
                            Some(name),
                        ))
                        .await?;
                    }
                }
            }
        }

        Ok(())
    }

    /// Resolve transitive dependencies from a path source
    fn resolve_path_transitive(&self, path: &str) -> Result<Vec<String>> {
        let dep_config_path = PathBuf::from(path).join("porters.toml");

        if !dep_config_path.exists() {
            // Dependency doesn't have porters.toml, no transitive deps
            return Ok(Vec::new());
        }

        // Load the dependency's configuration
        match PortersConfig::load(&dep_config_path) {
            Ok(dep_config) => {
                // Extract all dependency names
                let mut trans_deps = Vec::new();
                trans_deps.extend(dep_config.dependencies.keys().cloned());
                // Don't include dev dependencies of dependencies by default
                Ok(trans_deps)
            }
            Err(_) => {
                // Failed to parse config, assume no transitive deps
                Ok(Vec::new())
            }
        }
    }

    /// Get transitive dependency specification
    async fn get_transitive_dep_spec(
        &self,
        trans_dep_name: &str,
        source: &DependencySource,
    ) -> Option<Dependency> {
        match source {
            DependencySource::Path { path } => {
                let dep_config_path = PathBuf::from(path).join("porters.toml");
                if let Ok(dep_config) = PortersConfig::load(&dep_config_path) {
                    // Try regular dependencies
                    if let Some(dep) = dep_config.dependencies.get(trans_dep_name) {
                        return Some(dep.clone());
                    }
                }
                None
            }
            _ => {
                // Git and Registry not yet implemented for transitive resolution
                None
            }
        }
    }

    /// Add an edge in the dependency graph
    fn add_dependency_edge(&mut self, parent: &str, child: &str) {
        let parent_idx = *self
            .node_map
            .entry(parent.to_string())
            .or_insert_with(|| self.dep_graph.add_node(parent.to_string()));

        let child_idx = *self
            .node_map
            .entry(child.to_string())
            .or_insert_with(|| self.dep_graph.add_node(child.to_string()));

        self.dep_graph.add_edge(parent_idx, child_idx, ());
    }

    fn add_resolved(
        &mut self,
        name: &str,
        version: Option<Version>,
        source: DependencySource,
        features: Vec<String>,
        transitive_deps: Vec<String>,
    ) -> Result<()> {
        if let Some(existing) = self.resolved.get(name) {
            // Check for version conflicts
            if let (Some(existing_ver), Some(new_ver)) = (&existing.version, &version)
                && existing_ver != new_ver
            {
                self.conflicts.push(DependencyConflict {
                    package: name.to_string(),
                    requested_versions: vec![existing_ver.to_string(), new_ver.to_string()],
                });
            }
        } else {
            self.resolved.insert(
                name.to_string(),
                ResolvedDependency {
                    name: name.to_string(),
                    version,
                    source,
                    features,
                    transitive_deps,
                },
            );
        }

        Ok(())
    }

    fn check_constraint(&self, constraint: &str) -> Result<bool> {
        // Simple constraint checking - can be extended
        // Format: "min_version >= 1.0.0", "platform == linux", etc.

        if constraint.contains(">=") {
            // Version constraint (simplified)
            Ok(true)
        } else if constraint.contains("==") {
            // Platform or other equality check
            let parts: Vec<&str> = constraint.split("==").map(|s| s.trim()).collect();
            if parts.len() == 2 && parts[0] == "platform" {
                let current_platform = get_current_platform();
                Ok(parts[1] == current_platform)
            } else {
                Ok(true)
            }
        } else {
            Ok(true)
        }
    }

    fn format_conflicts(&self) -> String {
        self.conflicts
            .iter()
            .map(|c| {
                format!(
                    "  {} requires versions: {}",
                    c.package,
                    c.requested_versions.join(", ")
                )
            })
            .collect::<Vec<_>>()
            .join("\n")
    }
}

/// Get current platform identifier (utility function)
#[allow(dead_code)]
pub fn get_current_platform() -> String {
    #[cfg(target_os = "windows")]
    return "windows".to_string();

    #[cfg(target_os = "macos")]
    return "macos".to_string();

    #[cfg(target_os = "linux")]
    return "linux".to_string();

    #[cfg(not(any(target_os = "windows", target_os = "macos", target_os = "linux")))]
    return "unknown".to_string();
}

/// Check if current environment satisfies platform requirements
#[allow(dead_code)]
pub fn check_platform_compatibility(allowed_platforms: &[String]) -> bool {
    let current = get_current_platform();
    allowed_platforms.is_empty() || allowed_platforms.contains(&current)
}
