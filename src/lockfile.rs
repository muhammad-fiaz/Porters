//! Lock file management for dependency resolution
//!
//! This module handles reading, writing, and managing the porters.lock file
//! which tracks resolved dependency versions and checksums.

use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::Path;

/// Lock file to track resolved dependencies
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct LockFile {
    /// Version of the lock file format
    pub version: String,
    
    /// Resolved dependencies
    pub dependencies: HashMap<String, ResolvedDependency>,
    
    /// When the lock file was last updated
    pub updated_at: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResolvedDependency {
    pub name: String,
    pub version: String,
    pub source: DependencySource,
    pub checksum: Option<String>,
    pub dependencies: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "lowercase")]
pub enum DependencySource {
    Git {
        url: String,
        rev: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        branch: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        tag: Option<String>,
    },
    Path {
        path: String,
    },
    Registry {
        registry: String,
        version: String,
    },
}

impl LockFile {
    pub const VERSION: &'static str = "1";
    
    /// Create a new lock file
    pub fn new() -> Self {
        Self {
            version: Self::VERSION.to_string(),
            dependencies: HashMap::new(),
            updated_at: chrono::Utc::now().to_rfc3339(),
        }
    }
    
    /// Load lock file from path
    pub fn load<P: AsRef<Path>>(path: P) -> Result<Self> {
        if !path.as_ref().exists() {
            return Ok(Self::new());
        }
        
        let content = std::fs::read_to_string(path.as_ref())
            .with_context(|| format!("Failed to read {}", path.as_ref().display()))?;
        
        let lock: LockFile = toml::from_str(&content)
            .with_context(|| "Failed to parse lock file")?;
        
        Ok(lock)
    }
    
    /// Save lock file to path
    pub fn save<P: AsRef<Path>>(&self, path: P) -> Result<()> {
        let content = toml::to_string_pretty(self)
            .context("Failed to serialize lock file")?;
        
        std::fs::write(path.as_ref(), content)
            .with_context(|| format!("Failed to write {}", path.as_ref().display()))?;
        
        Ok(())
    }
    
    /// Add a resolved dependency
    pub fn add_dependency(&mut self, name: String, dep: ResolvedDependency) {
        self.dependencies.insert(name, dep);
        self.updated_at = chrono::Utc::now().to_rfc3339();
    }
    
    /// Remove a dependency
    #[allow(dead_code)]
    pub fn remove_dependency(&mut self, name: &str) {
        self.dependencies.remove(name);
        self.updated_at = chrono::Utc::now().to_rfc3339();
    }
    
    /// Get a resolved dependency
    #[allow(dead_code)]
    pub fn get_dependency(&self, name: &str) -> Option<&ResolvedDependency> {
        self.dependencies.get(name)
    }
    
    /// Update the timestamp
    #[allow(dead_code)]
    pub fn touch(&mut self) {
        self.updated_at = chrono::Utc::now().to_rfc3339();
    }
}
