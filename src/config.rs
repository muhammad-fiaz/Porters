use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::{Path, PathBuf};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PortersConfig {
    pub project: ProjectInfo,

    #[serde(default)]
    pub dependencies: HashMap<String, Dependency>,

    #[serde(default, rename = "dev-dependencies")]
    pub dev_dependencies: HashMap<String, Dependency>,

    #[serde(default)]
    pub features: HashMap<String, Vec<String>>,

    #[serde(default)]
    pub build: BuildConfig,

    #[serde(default)]
    pub requires: ToolRequirements,

    #[serde(default)]
    pub extensions: Vec<String>,

    #[serde(default)]
    pub commands: Vec<CustomCommand>,

    #[serde(default)]
    pub scripts: HashMap<String, String>,

    #[serde(default)]
    pub cache: CacheConfig,

    #[serde(default)]
    pub registries: Vec<RegistryConfig>,

    #[serde(default, rename = "cross-compile")]
    pub cross_compile: CrossCompileConfig,

    #[serde(default)]
    pub run: RunConfig,

    #[serde(default = "default_true", rename = "auto-update-check")]
    pub auto_update_check: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProjectInfo {
    pub name: String,
    pub version: String,

    #[serde(default)]
    pub authors: Vec<String>,

    #[serde(default)]
    pub description: Option<String>,

    #[serde(default)]
    pub license: Option<String>,

    #[serde(default)]
    pub homepage: Option<String>,

    #[serde(default)]
    pub repository: Option<String>,

    #[serde(default)]
    pub readme: Option<String>,

    #[serde(default)]
    pub keywords: Vec<String>,

    #[serde(default)]
    pub categories: Vec<String>,

    #[serde(default, rename = "project-type")]
    pub project_type: ProjectType,

    #[serde(default)]
    pub entry_point: Option<String>,

    #[serde(default)]
    pub platforms: Vec<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum ProjectType {
    #[default]
    Application,
    Library,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Dependency {
    Simple(String),
    Detailed {
        #[serde(skip_serializing_if = "Option::is_none")]
        version: Option<String>,

        #[serde(skip_serializing_if = "Option::is_none")]
        git: Option<String>,

        #[serde(skip_serializing_if = "Option::is_none")]
        branch: Option<String>,

        #[serde(skip_serializing_if = "Option::is_none")]
        tag: Option<String>,

        #[serde(skip_serializing_if = "Option::is_none")]
        rev: Option<String>,

        #[serde(skip_serializing_if = "Option::is_none")]
        path: Option<String>,

        #[serde(default)]
        optional: bool,

        #[serde(default)]
        features: Vec<String>,

        #[serde(skip_serializing_if = "Option::is_none")]
        platforms: Option<Vec<String>>,

        #[serde(skip_serializing_if = "Option::is_none")]
        constraints: Option<String>,

        #[serde(skip_serializing_if = "Option::is_none")]
        checksum: Box<Option<String>>,
    },
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct BuildConfig {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub system: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom: Option<CustomBuild>,

    #[serde(default)]
    pub flags: BuildFlags,

    #[serde(default)]
    pub include: Vec<String>,

    #[serde(default)]
    pub linking: LinkingConfig,

    #[serde(default)]
    pub scripts: BuildScripts,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct BuildFlags {
    #[serde(default)]
    pub cflags: Vec<String>,

    #[serde(default)]
    pub cxxflags: Vec<String>,

    #[serde(default)]
    pub ldflags: Vec<String>,

    #[serde(default)]
    pub defines: Vec<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct LinkingConfig {
    #[serde(default)]
    pub libraries: Vec<String>,

    #[serde(default)]
    pub library_paths: Vec<String>,

    #[serde(default)]
    pub frameworks: Vec<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct BuildScripts {
    #[serde(skip_serializing_if = "Option::is_none", rename = "pre-build")]
    pub pre_build: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none", rename = "post-build")]
    pub post_build: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none", rename = "pre-install")]
    pub pre_install: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none", rename = "post-install")]
    pub post_install: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CustomBuild {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub configure: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub build: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub install: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub test: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub clean: Option<String>,
}

/// Tool version requirements
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ToolRequirements {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub c: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub cpp: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub cmake: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub gcc: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub clang: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub msvc: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub ninja: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub make: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub xmake: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub meson: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub bazel: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub conan: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub vcpkg: Option<String>,
}

/// Custom command definition
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CustomCommand {
    pub name: String,
    pub description: String,
    pub script: String,

    #[serde(default)]
    pub env: HashMap<String, String>,
}

impl PortersConfig {
    /// Load configuration from a TOML file
    pub fn load<P: AsRef<Path>>(path: P) -> Result<Self> {
        let content = std::fs::read_to_string(path.as_ref())
            .with_context(|| format!("Failed to read {}", path.as_ref().display()))?;

        let config: PortersConfig =
            toml::from_str(&content).with_context(|| "Failed to parse porters.toml")?;

        Ok(config)
    }

    /// Save configuration to a TOML file
    pub fn save<P: AsRef<Path>>(&self, path: P) -> Result<()> {
        let content =
            toml::to_string_pretty(self).with_context(|| "Failed to serialize configuration")?;

        std::fs::write(path.as_ref(), content)
            .with_context(|| format!("Failed to write {}", path.as_ref().display()))?;

        Ok(())
    }

    /// Add a dependency to the configuration
    pub fn add_dependency(&mut self, package: &str, dev: bool, optional: bool) -> Result<()> {
        let dep = if package.starts_with("http://")
            || package.starts_with("https://")
            || package.starts_with("git@")
        {
            // Git dependency
            Dependency::Detailed {
                version: None,
                git: Some(package.to_string()),
                branch: None,
                tag: None,
                rev: None,
                path: None,
                optional,
                features: vec![],
                platforms: None,
                constraints: None,
                checksum: Box::new(None::<String>),
            }
        } else if Path::new(package).exists() {
            // Path dependency
            Dependency::Detailed {
                version: None,
                git: None,
                branch: None,
                tag: None,
                rev: None,
                path: Some(package.to_string()),
                optional,
                features: vec![],
                platforms: None,
                constraints: None,
                checksum: Box::new(None::<String>),
            }
        } else {
            // Simple package name (for future registry support)
            Dependency::Detailed {
                version: Some("*".to_string()),
                git: None,
                branch: None,
                tag: None,
                rev: None,
                path: None,
                optional,
                features: vec![],
                platforms: None,
                constraints: None,
                checksum: Box::new(None::<String>),
            }
        };

        let deps = if dev {
            &mut self.dev_dependencies
        } else {
            &mut self.dependencies
        };

        // Extract package name from git URL if needed
        let pkg_name = if package.contains("://") {
            package
                .split('/')
                .next_back()
                .unwrap_or(package)
                .trim_end_matches(".git")
        } else {
            package
        };

        deps.insert(pkg_name.to_string(), dep);

        Ok(())
    }

    /// Remove a dependency from the configuration
    pub fn remove_dependency(&mut self, package: &str) -> Result<()> {
        self.dependencies.remove(package);
        self.dev_dependencies.remove(package);
        Ok(())
    }

    /// Get all dependencies (including dev dependencies)
    pub fn all_dependencies(&self) -> HashMap<String, &Dependency> {
        let mut all = HashMap::new();

        for (name, dep) in &self.dependencies {
            all.insert(name.clone(), dep);
        }

        for (name, dep) in &self.dev_dependencies {
            all.insert(name.clone(), dep);
        }

        all
    }
}

/// Cache configuration
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CacheConfig {
    #[serde(default = "default_true")]
    pub enabled: bool,

    #[serde(default = "default_true", rename = "binary-cache")]
    pub binary_cache: bool,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub dir: Option<PathBuf>,
}

/// Direct execution configuration for single files
///
/// **Automatic Features:**
/// - Compiler auto-detection (gcc/clang/g++/clang++)
/// - Dependency include/lib paths auto-resolved from porters.toml
/// - File type detection (.c vs .cpp)
///
/// **Manual Configuration (Optional):**
/// - Custom include directories
/// - Exclude patterns for files
/// - Custom compiler/linker flags
/// - Override default compilers
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct RunConfig {
    /// Additional include directories for direct execution (OPTIONAL)
    ///
    /// **Automatic**: Dependency includes are added automatically
    /// **Manual**: Add extra paths here (e.g., system includes)
    #[serde(default, rename = "include-dirs")]
    pub include_dirs: Vec<String>,

    /// File patterns to exclude from direct execution (OPTIONAL)
    ///
    /// Example: ["test_*", "*_backup.c"]
    #[serde(default, rename = "exclude-patterns")]
    pub exclude_patterns: Vec<String>,

    /// Additional compiler flags for C/C++ files (OPTIONAL)
    ///
    /// **Automatic**: Basic compilation works without flags
    /// **Manual**: Add custom flags like ["-Wall", "-O2", "-std=c17"]
    #[serde(default, rename = "compiler-flags")]
    pub compiler_flags: Vec<String>,

    /// Additional linker flags (OPTIONAL)
    ///
    /// Example: ["-lm", "-lpthread"]
    #[serde(default, rename = "linker-flags")]
    pub linker_flags: Vec<String>,

    /// Default C compiler (OPTIONAL - defaults to auto-detected gcc/clang)
    #[serde(skip_serializing_if = "Option::is_none", rename = "c-compiler")]
    pub c_compiler: Option<String>,

    /// Default C++ compiler (OPTIONAL - defaults to auto-detected g++/clang++)
    #[serde(skip_serializing_if = "Option::is_none", rename = "cpp-compiler")]
    pub cpp_compiler: Option<String>,
}

fn default_true() -> bool {
    true
}

/// Registry configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RegistryConfig {
    pub name: String,
    pub url: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auth_token: Option<String>,
    #[serde(default = "default_true")]
    pub enabled: bool,
}

/// Cross-compilation configuration
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CrossCompileConfig {
    #[serde(default)]
    pub targets: HashMap<String, TargetConfig>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "default-target")]
    pub default_target: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct TargetConfig {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub toolchain: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sysroot: Option<PathBuf>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub linker: Option<String>,
    #[serde(
        skip_serializing_if = "Option::is_none",
        rename = "cmake-toolchain-file"
    )]
    pub cmake_toolchain_file: Option<PathBuf>,
    #[serde(default)]
    pub env: HashMap<String, String>,
    #[serde(default)]
    pub flags: TargetFlags,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct TargetFlags {
    #[serde(default)]
    pub cflags: Vec<String>,
    #[serde(default)]
    pub cxxflags: Vec<String>,
    #[serde(default)]
    pub ldflags: Vec<String>,
}
