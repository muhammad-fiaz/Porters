use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

/// Global Porters configuration stored in ~/.porters/config.toml
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GlobalPortersConfig {
    /// Porters version when config was created
    #[serde(default)]
    pub porters_version: String,

    /// Last update check timestamp
    #[serde(default)]
    pub last_update_check: Option<String>,

    /// Auto-update check enabled
    #[serde(default = "default_true")]
    pub auto_update_check: bool,

    /// Default compiler preference
    #[serde(default)]
    pub default_compiler: Option<String>,

    /// Default build system preference
    #[serde(default)]
    pub default_build_system: Option<String>,

    /// User preferences
    #[serde(default)]
    pub preferences: UserPreferences,

    /// Installed extensions (global)
    #[serde(default)]
    pub installed_extensions: Vec<String>,

    /// Global dependencies cache info
    #[serde(default)]
    pub cache: CacheConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct UserPreferences {
    /// Default author name
    #[serde(default)]
    pub author: Option<String>,

    /// Default email
    #[serde(default)]
    pub email: Option<String>,

    /// Default license
    #[serde(default)]
    pub license: Option<String>,

    /// Default project language
    #[serde(default)]
    pub default_language: Option<String>,

    /// Use external terminal by default
    #[serde(default)]
    pub use_external_terminal: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CacheConfig {
    /// Enable dependency caching
    #[serde(default = "default_true")]
    pub enabled: bool,

    /// Cache size limit in MB
    #[serde(default = "default_cache_size")]
    pub max_size_mb: u64,

    /// Auto-clean old cache entries
    #[serde(default = "default_true")]
    pub auto_clean: bool,
}

fn default_true() -> bool {
    true
}

fn default_cache_size() -> u64 {
    1024 // 1GB default
}

impl Default for GlobalPortersConfig {
    fn default() -> Self {
        Self {
            porters_version: env!("CARGO_PKG_VERSION").to_string(),
            last_update_check: None,
            auto_update_check: true,
            default_compiler: None,
            default_build_system: None,
            preferences: UserPreferences::default(),
            installed_extensions: Vec::new(),
            cache: CacheConfig::default(),
        }
    }
}

impl GlobalPortersConfig {
    /// Get the global .porters directory path
    pub fn global_dir() -> Result<PathBuf> {
        let home = dirs::home_dir().context("Could not determine home directory")?;
        Ok(home.join(".porters"))
    }

    /// Get the global config file path
    pub fn config_path() -> Result<PathBuf> {
        Ok(Self::global_dir()?.join("config.toml"))
    }

    /// Ensure the global .porters directory exists
    pub fn ensure_global_dir() -> Result<PathBuf> {
        let dir = Self::global_dir()?;
        if !dir.exists() {
            fs::create_dir_all(&dir).context("Failed to create .porters directory")?;
        }
        Ok(dir)
    }

    /// Load global config or create default if it doesn't exist
    pub fn load_or_create() -> Result<Self> {
        let config_path = Self::config_path()?;

        if config_path.exists() {
            let content =
                fs::read_to_string(&config_path).context("Failed to read global config")?;
            let config: Self = toml::from_str(&content).context("Failed to parse global config")?;
            Ok(config)
        } else {
            // Create default config
            let config = Self::default();
            config.save()?;
            Ok(config)
        }
    }

    /// Save global config
    pub fn save(&self) -> Result<()> {
        Self::ensure_global_dir()?;
        let config_path = Self::config_path()?;
        let content = toml::to_string_pretty(self).context("Failed to serialize global config")?;
        fs::write(&config_path, content).context("Failed to write global config")?;
        Ok(())
    }

    /// Update last update check timestamp
    #[allow(dead_code)]
    pub fn update_last_check(&mut self) -> Result<()> {
        use chrono::Utc;
        self.last_update_check = Some(Utc::now().to_rfc3339());
        self.save()
    }

    /// Check if system requirements are met
    #[allow(dead_code)]
    pub fn check_system_requirements() -> SystemCheck {
        SystemCheck::run()
    }
}

/// System requirements check results
#[derive(Debug, Clone)]
pub struct SystemCheck {
    pub compilers: Vec<CompilerInfo>,
    pub build_systems: Vec<BuildSystemInfo>,
    pub has_compiler: bool,
    pub has_build_system: bool,
}

#[derive(Debug, Clone)]
pub struct CompilerInfo {
    pub name: String,
    pub path: String,
    pub version: Option<String>,
}

#[derive(Debug, Clone)]
pub struct BuildSystemInfo {
    pub name: String,
    pub path: String,
    pub version: Option<String>,
}

impl SystemCheck {
    /// Run system requirements check
    pub fn run() -> Self {
        let compilers = Self::detect_compilers();
        let build_systems = Self::detect_build_systems();

        Self {
            has_compiler: !compilers.is_empty(),
            has_build_system: !build_systems.is_empty(),
            compilers,
            build_systems,
        }
    }

    /// Detect available C/C++ compilers
    fn detect_compilers() -> Vec<CompilerInfo> {
        let mut compilers = Vec::new();

        // List of compilers to check
        let compiler_names = if cfg!(target_os = "windows") {
            vec!["g++", "gcc", "clang", "clang++", "cl"]
        } else {
            vec!["g++", "gcc", "clang", "clang++", "cc", "c++"]
        };

        for name in compiler_names {
            if let Ok(output) = std::process::Command::new(name).arg("--version").output()
                && output.status.success()
            {
                let version = String::from_utf8_lossy(&output.stdout)
                    .lines()
                    .next()
                    .map(|s| s.to_string());

                // Get full path
                let path = which::which(name)
                    .ok()
                    .and_then(|p| p.to_str().map(|s| s.to_string()))
                    .unwrap_or_else(|| name.to_string());

                compilers.push(CompilerInfo {
                    name: name.to_string(),
                    path,
                    version,
                });
            }
        }

        compilers
    }

    /// Detect available build systems
    fn detect_build_systems() -> Vec<BuildSystemInfo> {
        let mut build_systems = Vec::new();

        let system_names = vec!["cmake", "make", "xmake", "meson", "ninja"];

        for name in system_names {
            if let Ok(output) = std::process::Command::new(name).arg("--version").output()
                && output.status.success()
            {
                let version = String::from_utf8_lossy(&output.stdout)
                    .lines()
                    .next()
                    .map(|s| s.to_string());

                let path = which::which(name)
                    .ok()
                    .and_then(|p| p.to_str().map(|s| s.to_string()))
                    .unwrap_or_else(|| name.to_string());

                build_systems.push(BuildSystemInfo {
                    name: name.to_string(),
                    path,
                    version,
                });
            }
        }

        build_systems
    }

    /// Display system check results
    pub fn display(&self) {
        use crate::util::pretty::*;

        println!();
        print_step("System Requirements Check");
        println!();

        // Compilers
        println!("🔧  C/C++ Compilers:");
        if self.compilers.is_empty() {
            print_warning("  ⚠️  No C/C++ compiler found!");
            println!("     Porters requires at least one of:");
            println!("       - GCC (g++/gcc)");
            println!("       - Clang (clang++/clang)");
            if cfg!(target_os = "windows") {
                println!("       - MinGW-w64");
                println!("       - MSVC (Visual Studio)");
            }
            println!();
            print_info("  Install instructions:");
            if cfg!(target_os = "windows") {
                println!("    Windows: Install MinGW-w64 or Visual Studio");
                println!("             https://www.mingw-w64.org/");
                println!("             https://visualstudio.microsoft.com/");
            } else if cfg!(target_os = "macos") {
                println!("    macOS: Install Xcode Command Line Tools");
                println!("           xcode-select --install");
            } else {
                println!("    Linux: Install build-essential");
                println!("           sudo apt-get install build-essential  # Debian/Ubuntu");
                println!("           sudo yum groupinstall 'Development Tools'  # RHEL/CentOS");
            }
        } else {
            for compiler in &self.compilers {
                println!("  ✓  {} - {}", compiler.name, compiler.path);
                if let Some(version) = &compiler.version {
                    println!("     {}", version);
                }
            }
        }

        println!();

        // Build systems
        println!("🔨  Build Systems:");
        if self.build_systems.is_empty() {
            print_warning("  ⚠️  No build system found!");
            println!("     Recommended: CMake (most versatile)");
            println!("     Optional: Make, XMake, Meson, Ninja");
            println!();
            print_info("  Install CMake:");
            if cfg!(target_os = "windows") {
                println!("    Windows: Download from https://cmake.org/download/");
                println!("             Or use: winget install Kitware.CMake");
            } else if cfg!(target_os = "macos") {
                println!("    macOS: brew install cmake");
            } else {
                println!("    Linux: sudo apt-get install cmake  # Debian/Ubuntu");
                println!("           sudo yum install cmake       # RHEL/CentOS");
            }
        } else {
            for build_system in &self.build_systems {
                println!("  ✓  {} - {}", build_system.name, build_system.path);
                if let Some(version) = &build_system.version {
                    println!("     {}", version);
                }
            }
        }

        println!();

        if !self.has_compiler || !self.has_build_system {
            print_error("System requirements not met!");
            println!();
            if !self.has_compiler {
                println!("  ❌  Missing: C/C++ Compiler (REQUIRED)");
            }
            if !self.has_build_system {
                println!("  ⚠️  Missing: Build System (RECOMMENDED)");
                println!("      You can still use Porters for single-file execution");
            }
            println!();
        } else {
            print_success("All system requirements met! ✓");
            println!();
        }
    }

    /// Check and warn if requirements not met
    #[allow(dead_code)]
    pub fn check_and_warn(&self) -> bool {
        if !self.has_compiler {
            self.display();
            return false;
        }

        if !self.has_build_system {
            use crate::util::pretty::*;
            print_warning("No build system detected (CMake recommended)");
            println!("  Porters can execute single files, but projects require a build system");
            println!();
        }

        true
    }
}
