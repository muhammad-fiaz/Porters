//! Cross-compilation support
//!
//! This module provides cross-compilation capabilities for building
//! C/C++ projects for multiple target platforms and architectures.

use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::process::Command;
use colored::Colorize;

/// Cross-compilation target
/// 
/// Represents a specific platform and architecture combination
/// for cross-compilation.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum Target {
    /// Linux x86_64 (64-bit Intel/AMD)
    #[serde(rename = "linux-x86_64")]
    LinuxX8664,
    
    /// Linux aarch64 (64-bit ARM)
    #[serde(rename = "linux-aarch64")]
    LinuxAarch64,
    
    /// Linux armv7 (32-bit ARM)
    #[serde(rename = "linux-armv7")]
    LinuxArmv7,
    #[serde(rename = "windows-x86_64")]
    WindowsX8664,
    #[serde(rename = "windows-i686")]
    WindowsI686,
    #[serde(rename = "macos-x86_64")]
    MacosX8664,
    #[serde(rename = "macos-aarch64")]
    MacosAarch64,
    #[serde(rename = "baremetal-arm")]
    BaremetalArm,
    #[serde(rename = "baremetal-riscv")]
    BaremetalRiscv,
    #[serde(rename = "android-aarch64")]
    AndroidAarch64,
    #[serde(rename = "ios-aarch64")]
    IosAarch64,
    #[serde(rename = "wasm32")]
    Wasm32,
}

impl Target {
    /// Get all targets for a platform
    pub fn for_platform(platform: &str) -> Vec<Target> {
        match platform.to_lowercase().as_str() {
            "linux" => vec![Target::LinuxX8664, Target::LinuxAarch64, Target::LinuxArmv7],
            "windows" => vec![Target::WindowsX8664, Target::WindowsI686],
            "macos" => vec![Target::MacosX8664, Target::MacosAarch64],
            "baremetal" => vec![Target::BaremetalArm, Target::BaremetalRiscv],
            "android" => vec![Target::AndroidAarch64],
            "ios" => vec![Target::IosAarch64],
            "wasm" => vec![Target::Wasm32],
            _ => vec![],
        }
    }

    /// Get all supported targets
    pub fn all() -> Vec<Target> {
        vec![
            Target::LinuxX8664,
            Target::LinuxAarch64,
            Target::LinuxArmv7,
            Target::WindowsX8664,
            Target::WindowsI686,
            Target::MacosX8664,
            Target::MacosAarch64,
            Target::BaremetalArm,
            Target::BaremetalRiscv,
            Target::AndroidAarch64,
            Target::IosAarch64,
            Target::Wasm32,
        ]
    }

    /// Get target triple (CMake style)
    pub fn triple(&self) -> &'static str {
        match self {
            Target::LinuxX8664 => "x86_64-unknown-linux-gnu",
            Target::LinuxAarch64 => "aarch64-unknown-linux-gnu",
            Target::LinuxArmv7 => "armv7-unknown-linux-gnueabihf",
            Target::WindowsX8664 => "x86_64-pc-windows-msvc",
            Target::WindowsI686 => "i686-pc-windows-msvc",
            Target::MacosX8664 => "x86_64-apple-darwin",
            Target::MacosAarch64 => "aarch64-apple-darwin",
            Target::BaremetalArm => "arm-none-eabi",
            Target::BaremetalRiscv => "riscv32-unknown-none-elf",
            Target::AndroidAarch64 => "aarch64-linux-android",
            Target::IosAarch64 => "aarch64-apple-ios",
            Target::Wasm32 => "wasm32-unknown-unknown",
        }
    }

    /// Get display name
    pub fn display_name(&self) -> &'static str {
        match self {
            Target::LinuxX8664 => "Linux x86_64",
            Target::LinuxAarch64 => "Linux ARM64",
            Target::LinuxArmv7 => "Linux ARMv7",
            Target::WindowsX8664 => "Windows x86_64",
            Target::WindowsI686 => "Windows x86",
            Target::MacosX8664 => "macOS Intel",
            Target::MacosAarch64 => "macOS Apple Silicon",
            Target::BaremetalArm => "Bare Metal ARM",
            Target::BaremetalRiscv => "Bare Metal RISC-V",
            Target::AndroidAarch64 => "Android ARM64",
            Target::IosAarch64 => "iOS ARM64",
            Target::Wasm32 => "WebAssembly",
        }
    }

    /// Get recommended toolchain
    pub fn toolchain(&self) -> Vec<&'static str> {
        match self {
            Target::LinuxX8664 => vec!["gcc", "g++"],
            Target::LinuxAarch64 => vec!["aarch64-linux-gnu-gcc", "aarch64-linux-gnu-g++"],
            Target::LinuxArmv7 => vec!["arm-linux-gnueabihf-gcc", "arm-linux-gnueabihf-g++"],
            Target::WindowsX8664 => vec!["x86_64-w64-mingw32-gcc", "x86_64-w64-mingw32-g++"],
            Target::WindowsI686 => vec!["i686-w64-mingw32-gcc", "i686-w64-mingw32-g++"],
            Target::MacosX8664 => vec!["clang", "clang++"],
            Target::MacosAarch64 => vec!["clang", "clang++"],
            Target::BaremetalArm => vec!["arm-none-eabi-gcc", "arm-none-eabi-g++"],
            Target::BaremetalRiscv => vec!["riscv32-unknown-elf-gcc", "riscv32-unknown-elf-g++"],
            Target::AndroidAarch64 => vec!["aarch64-linux-android-clang", "aarch64-linux-android-clang++"],
            Target::IosAarch64 => vec!["clang", "clang++"],
            Target::Wasm32 => vec!["emcc", "em++"],
        }
    }
}

/// Cross-compilation configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CrossCompileConfig {
    #[serde(default)]
    pub targets: HashMap<String, TargetConfig>,
    #[serde(default)]
    pub default_target: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TargetConfig {
    pub toolchain: Option<String>,
    pub sysroot: Option<PathBuf>,
    pub linker: Option<String>,
    #[serde(default)]
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

/// Cross-compilation builder
pub struct CrossCompiler {
    config: CrossCompileConfig,
    project_root: PathBuf,
}

impl CrossCompiler {
    /// Create a new cross-compiler
    pub fn new(config: CrossCompileConfig, project_root: PathBuf) -> Self {
        Self {
            config,
            project_root,
        }
    }

    /// Compile for specific target
    pub fn compile(&self, target: &Target, build_system: &str) -> Result<PathBuf> {
        println!("🔨 Cross-compiling for {}...", target.display_name().cyan());

        // Get target configuration
        let target_config = self.config.targets.get(target.triple());

        // Check toolchain availability
        self.check_toolchain(target)?;

        // Create build directory
        let build_dir = self.project_root.join("build").join(target.triple());
        std::fs::create_dir_all(&build_dir)?;

        // Compile based on build system
        match build_system.to_lowercase().as_str() {
            "cmake" => self.compile_cmake(target, target_config, &build_dir)?,
            "make" => self.compile_make(target, target_config, &build_dir)?,
            "meson" => self.compile_meson(target, target_config, &build_dir)?,
            "xmake" => self.compile_xmake(target, target_config, &build_dir)?,
            _ => anyhow::bail!("Cross-compilation not supported for build system: {}", build_system),
        }

        println!("✅ Cross-compilation successful for {}", target.display_name().green());
        Ok(build_dir)
    }

    /// Compile all targets
    pub fn compile_all(&self, targets: &[Target], build_system: &str) -> Result<Vec<PathBuf>> {
        let mut build_dirs = Vec::new();

        for target in targets {
            match self.compile(target, build_system) {
                Ok(dir) => build_dirs.push(dir),
                Err(e) => {
                    println!("❌ Failed to compile for {}: {}", target.display_name().red(), e);
                }
            }
        }

        Ok(build_dirs)
    }

    /// Check if toolchain is available
    fn check_toolchain(&self, target: &Target) -> Result<()> {
        let toolchain = target.toolchain();
        let cc = toolchain[0];

        let output = Command::new("which")
            .arg(cc)
            .output();

        if output.is_err() || !output.unwrap().status.success() {
            println!("⚠️  Toolchain {} not found. Install instructions:", cc.yellow());
            self.print_install_instructions(target);
            anyhow::bail!("Required toolchain not available");
        }

        Ok(())
    }

    /// Print toolchain installation instructions
    fn print_install_instructions(&self, target: &Target) {
        match target {
            Target::LinuxAarch64 => {
                println!("  sudo apt-get install gcc-aarch64-linux-gnu g++-aarch64-linux-gnu");
            }
            Target::LinuxArmv7 => {
                println!("  sudo apt-get install gcc-arm-linux-gnueabihf g++-arm-linux-gnueabihf");
            }
            Target::WindowsX8664 | Target::WindowsI686 => {
                println!("  sudo apt-get install mingw-w64");
            }
            Target::BaremetalArm => {
                println!("  sudo apt-get install gcc-arm-none-eabi");
            }
            Target::Wasm32 => {
                println!("  Install Emscripten SDK: https://emscripten.org/");
            }
            _ => {
                println!("  Check documentation for {} toolchain", target.display_name());
            }
        }
    }

    /// Compile with CMake
    fn compile_cmake(
        &self,
        target: &Target,
        config: Option<&TargetConfig>,
        build_dir: &Path,
    ) -> Result<()> {
        let mut cmd = Command::new("cmake");
        cmd.current_dir(build_dir);
        cmd.arg("..");

        // Set toolchain file if provided
        if let Some(cfg) = config {
            if let Some(toolchain_file) = &cfg.cmake_toolchain_file {
                cmd.arg(format!("-DCMAKE_TOOLCHAIN_FILE={}", toolchain_file.display()));
            }
        }

        // Set target triple
        cmd.arg(format!("-DCMAKE_SYSTEM_NAME={}", self.get_cmake_system_name(target)));
        cmd.arg(format!("-DCMAKE_SYSTEM_PROCESSOR={}", self.get_cmake_processor(target)));

        // Set compilers
        let toolchain = target.toolchain();
        cmd.arg(format!("-DCMAKE_C_COMPILER={}", toolchain[0]));
        cmd.arg(format!("-DCMAKE_CXX_COMPILER={}", toolchain[1]));

        // Add custom flags
        if let Some(cfg) = config {
            for (key, value) in &cfg.env {
                cmd.env(key, value);
            }

            if !cfg.flags.cflags.is_empty() {
                cmd.arg(format!("-DCMAKE_C_FLAGS={}", cfg.flags.cflags.join(" ")));
            }
            if !cfg.flags.cxxflags.is_empty() {
                cmd.arg(format!("-DCMAKE_CXX_FLAGS={}", cfg.flags.cxxflags.join(" ")));
            }
        }

        let output = cmd.output()?;
        if !output.status.success() {
            anyhow::bail!("CMake configuration failed: {}", String::from_utf8_lossy(&output.stderr));
        }

        // Build
        let build_output = Command::new("cmake")
            .current_dir(build_dir)
            .arg("--build")
            .arg(".")
            .output()?;

        if !build_output.status.success() {
            anyhow::bail!("Build failed: {}", String::from_utf8_lossy(&build_output.stderr));
        }

        Ok(())
    }

    /// Compile with Make
    fn compile_make(
        &self,
        target: &Target,
        config: Option<&TargetConfig>,
        _build_dir: &Path,
    ) -> Result<()> {
        let toolchain = target.toolchain();
        let mut cmd = Command::new("make");
        cmd.current_dir(&self.project_root);
        cmd.arg(format!("CC={}", toolchain[0]));
        cmd.arg(format!("CXX={}", toolchain[1]));

        if let Some(cfg) = config {
            if !cfg.flags.cflags.is_empty() {
                cmd.arg(format!("CFLAGS={}", cfg.flags.cflags.join(" ")));
            }
            if !cfg.flags.cxxflags.is_empty() {
                cmd.arg(format!("CXXFLAGS={}", cfg.flags.cxxflags.join(" ")));
            }
        }

        let output = cmd.output()?;
        if !output.status.success() {
            anyhow::bail!("Make failed: {}", String::from_utf8_lossy(&output.stderr));
        }

        Ok(())
    }

    /// Compile with Meson
    fn compile_meson(
        &self,
        target: &Target,
        config: Option<&TargetConfig>,
        build_dir: &Path,
    ) -> Result<()> {
        // Meson requires cross-file for cross-compilation
        let cross_file = self.generate_meson_cross_file(target, config)?;

        let mut cmd = Command::new("meson");
        cmd.arg("setup");
        cmd.arg(build_dir);
        cmd.arg("--cross-file");
        cmd.arg(&cross_file);

        let output = cmd.output()?;
        if !output.status.success() {
            anyhow::bail!("Meson setup failed: {}", String::from_utf8_lossy(&output.stderr));
        }

        let build_output = Command::new("ninja")
            .current_dir(build_dir)
            .output()?;

        if !build_output.status.success() {
            anyhow::bail!("Ninja build failed: {}", String::from_utf8_lossy(&build_output.stderr));
        }

        Ok(())
    }

    /// Compile with XMake
    fn compile_xmake(
        &self,
        target: &Target,
        _config: Option<&TargetConfig>,
        _build_dir: &Path,
    ) -> Result<()> {
        let mut cmd = Command::new("xmake");
        cmd.current_dir(&self.project_root);
        cmd.arg("f");
        cmd.arg("-p");
        cmd.arg(self.get_xmake_platform(target));
        cmd.arg("-a");
        cmd.arg(self.get_xmake_arch(target));

        let output = cmd.output()?;
        if !output.status.success() {
            anyhow::bail!("XMake config failed: {}", String::from_utf8_lossy(&output.stderr));
        }

        let build_output = Command::new("xmake")
            .current_dir(&self.project_root)
            .output()?;

        if !build_output.status.success() {
            anyhow::bail!("XMake build failed: {}", String::from_utf8_lossy(&build_output.stderr));
        }

        Ok(())
    }

    /// Generate Meson cross-compilation file
    fn generate_meson_cross_file(
        &self,
        target: &Target,
        _config: Option<&TargetConfig>,
    ) -> Result<PathBuf> {
        let cross_file = self.project_root.join(format!("meson-cross-{}.ini", target.triple()));
        let toolchain = target.toolchain();

        let content = format!(
            "[binaries]\n\
             c = '{}'\n\
             cpp = '{}'\n\
             ar = 'ar'\n\
             strip = 'strip'\n\
             \n\
             [host_machine]\n\
             system = '{}'\n\
             cpu_family = '{}'\n\
             cpu = '{}'\n\
             endian = 'little'\n",
            toolchain[0],
            toolchain[1],
            self.get_meson_system(target),
            self.get_meson_cpu_family(target),
            self.get_meson_cpu(target)
        );

        std::fs::write(&cross_file, content)?;
        Ok(cross_file)
    }

    fn get_cmake_system_name(&self, target: &Target) -> &'static str {
        match target {
            Target::LinuxX8664 | Target::LinuxAarch64 | Target::LinuxArmv7 => "Linux",
            Target::WindowsX8664 | Target::WindowsI686 => "Windows",
            Target::MacosX8664 | Target::MacosAarch64 => "Darwin",
            Target::AndroidAarch64 => "Android",
            Target::IosAarch64 => "iOS",
            _ => "Generic",
        }
    }

    fn get_cmake_processor(&self, target: &Target) -> &'static str {
        match target {
            Target::LinuxX8664 | Target::WindowsX8664 | Target::MacosX8664 => "x86_64",
            Target::LinuxAarch64 | Target::MacosAarch64 | Target::AndroidAarch64 | Target::IosAarch64 => "aarch64",
            Target::LinuxArmv7 => "armv7",
            Target::WindowsI686 => "i686",
            Target::BaremetalArm => "arm",
            Target::BaremetalRiscv => "riscv32",
            Target::Wasm32 => "wasm32",
        }
    }

    fn get_meson_system(&self, target: &Target) -> &'static str {
        match target {
            Target::LinuxX8664 | Target::LinuxAarch64 | Target::LinuxArmv7 => "linux",
            Target::WindowsX8664 | Target::WindowsI686 => "windows",
            Target::MacosX8664 | Target::MacosAarch64 => "darwin",
            _ => "unknown",
        }
    }

    fn get_meson_cpu_family(&self, target: &Target) -> &'static str {
        match target {
            Target::LinuxX8664 | Target::WindowsX8664 | Target::MacosX8664 => "x86_64",
            Target::LinuxAarch64 | Target::MacosAarch64 => "aarch64",
            Target::LinuxArmv7 => "arm",
            Target::WindowsI686 => "x86",
            _ => "unknown",
        }
    }

    fn get_meson_cpu(&self, target: &Target) -> &'static str {
        self.get_cmake_processor(target)
    }

    fn get_xmake_platform(&self, target: &Target) -> &'static str {
        match target {
            Target::LinuxX8664 | Target::LinuxAarch64 | Target::LinuxArmv7 => "linux",
            Target::WindowsX8664 | Target::WindowsI686 => "windows",
            Target::MacosX8664 | Target::MacosAarch64 => "macosx",
            Target::AndroidAarch64 => "android",
            Target::IosAarch64 => "iphoneos",
            Target::Wasm32 => "wasm",
            _ => "cross",
        }
    }

    fn get_xmake_arch(&self, target: &Target) -> &'static str {
        match target {
            Target::LinuxX8664 | Target::WindowsX8664 | Target::MacosX8664 => "x86_64",
            Target::LinuxAarch64 | Target::MacosAarch64 | Target::AndroidAarch64 | Target::IosAarch64 => "arm64",
            Target::LinuxArmv7 => "armv7",
            Target::WindowsI686 => "i386",
            _ => "unknown",
        }
    }
}
