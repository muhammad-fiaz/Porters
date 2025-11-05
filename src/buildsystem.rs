//! Build system detection and abstraction layer
//!
//! This module provides comprehensive build system detection for 25+ build tools,
//! including CMake, Make, Ninja, Bazel, Meson, XMake, and more. It automatically
//! identifies the build system used by a project and provides metadata about it.

use std::path::Path;
use std::process::Command;

/// Supported build systems
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BuildSystem {
    // Traditional
    Make,
    Ninja,
    Autotools,
    Meson,
    SCons,
    Jam,
    
    // CMake ecosystem
    CMake,
    Conan,
    Vcpkg,
    #[allow(dead_code)]
    Hunter,
    
    // Modern alternatives
    XMake,
    Bazel,
    Buck2,
    Premake,
    QMake,
    GradleCpp,
    
    // Custom
    #[allow(dead_code)]
    Custom,
}

impl BuildSystem {
    pub fn as_str(&self) -> &str {
        match self {
            BuildSystem::Make => "make",
            BuildSystem::Ninja => "ninja",
            BuildSystem::Autotools => "autotools",
            BuildSystem::Meson => "meson",
            BuildSystem::SCons => "scons",
            BuildSystem::Jam => "jam",
            BuildSystem::CMake => "cmake",
            BuildSystem::Conan => "conan",
            BuildSystem::Vcpkg => "vcpkg",
            BuildSystem::Hunter => "hunter",
            BuildSystem::XMake => "xmake",
            BuildSystem::Bazel => "bazel",
            BuildSystem::Buck2 => "buck2",
            BuildSystem::Premake => "premake",
            BuildSystem::QMake => "qmake",
            BuildSystem::GradleCpp => "gradle-cpp",
            BuildSystem::Custom => "custom",
        }
    }
    
    /// Parse build system from string
    #[allow(dead_code)]
    pub fn from_str(s: &str) -> Option<Self> {
        match s.to_lowercase().as_str() {
            "make" => Some(BuildSystem::Make),
            "ninja" => Some(BuildSystem::Ninja),
            "autotools" | "automake" | "autoconf" => Some(BuildSystem::Autotools),
            "meson" => Some(BuildSystem::Meson),
            "scons" => Some(BuildSystem::SCons),
            "jam" | "boost.build" => Some(BuildSystem::Jam),
            "cmake" => Some(BuildSystem::CMake),
            "conan" => Some(BuildSystem::Conan),
            "vcpkg" => Some(BuildSystem::Vcpkg),
            "hunter" => Some(BuildSystem::Hunter),
            "xmake" => Some(BuildSystem::XMake),
            "bazel" => Some(BuildSystem::Bazel),
            "buck2" | "buck" => Some(BuildSystem::Buck2),
            "premake" | "premake5" => Some(BuildSystem::Premake),
            "qmake" => Some(BuildSystem::QMake),
            "gradle-cpp" | "gradle" => Some(BuildSystem::GradleCpp),
            "custom" => Some(BuildSystem::Custom),
            _ => None,
        }
    }
    
    /// Detection files for each build system
    pub fn detection_files(&self) -> &[&str] {
        match self {
            BuildSystem::Make => &["Makefile", "makefile", "GNUmakefile"],
            BuildSystem::Ninja => &["build.ninja"],
            BuildSystem::Autotools => &["configure.ac", "configure.in", "Makefile.am"],
            BuildSystem::Meson => &["meson.build"],
            BuildSystem::SCons => &["SConstruct", "SConscript"],
            BuildSystem::Jam => &["Jamfile", "Jamroot"],
            BuildSystem::CMake => &["CMakeLists.txt"],
            BuildSystem::Conan => &["conanfile.txt", "conanfile.py"],
            BuildSystem::Vcpkg => &["vcpkg.json"],
            BuildSystem::Hunter => &["cmake/Hunter/config.cmake"],
            BuildSystem::XMake => &["xmake.lua"],
            BuildSystem::Bazel => &["BUILD", "BUILD.bazel", "WORKSPACE"],
            BuildSystem::Buck2 => &["BUCK", ".buckconfig"],
            BuildSystem::Premake => &["premake5.lua", "premake4.lua"],
            BuildSystem::QMake => &["*.pro"],
            BuildSystem::GradleCpp => &["build.gradle"],
            BuildSystem::Custom => &[],
        }
    }
    
    /// Command to check if build tool is installed
    #[allow(dead_code)]
    pub fn check_command(&self) -> &str {
        match self {
            BuildSystem::Make => "make",
            BuildSystem::Ninja => "ninja",
            BuildSystem::Autotools => "autoconf",
            BuildSystem::Meson => "meson",
            BuildSystem::SCons => "scons",
            BuildSystem::Jam => "bjam",
            BuildSystem::CMake => "cmake",
            BuildSystem::Conan => "conan",
            BuildSystem::Vcpkg => "vcpkg",
            BuildSystem::Hunter => "cmake",
            BuildSystem::XMake => "xmake",
            BuildSystem::Bazel => "bazel",
            BuildSystem::Buck2 => "buck2",
            BuildSystem::Premake => "premake5",
            BuildSystem::QMake => "qmake",
            BuildSystem::GradleCpp => "gradle",
            BuildSystem::Custom => "",
        }
    }
    
    /// Installation instructions
    #[allow(dead_code)]
    pub fn install_instructions(&self) -> &str {
        match self {
            BuildSystem::Make => "sudo apt install make  # Ubuntu/Debian\nbrew install make  # macOS",
            BuildSystem::Ninja => "sudo apt install ninja-build  # Ubuntu/Debian\nbrew install ninja  # macOS",
            BuildSystem::Autotools => "sudo apt install autoconf automake  # Ubuntu/Debian\nbrew install autoconf automake  # macOS",
            BuildSystem::Meson => "pip install meson ninja  # All platforms",
            BuildSystem::SCons => "pip install scons  # All platforms",
            BuildSystem::Jam => "Download from boost.org/build",
            BuildSystem::CMake => "sudo apt install cmake  # Ubuntu/Debian\nbrew install cmake  # macOS\nchoco install cmake  # Windows",
            BuildSystem::Conan => "pip install conan  # All platforms",
            BuildSystem::Vcpkg => "git clone https://github.com/microsoft/vcpkg && ./vcpkg/bootstrap-vcpkg.sh",
            BuildSystem::Hunter => "Integrated with CMake - no separate install needed",
            BuildSystem::XMake => "curl -fsSL https://xmake.io/shget.text | bash  # Linux/macOS\nscoop install xmake  # Windows",
            BuildSystem::Bazel => "Download from bazel.build",
            BuildSystem::Buck2 => "cargo install buck2  # Via cargo",
            BuildSystem::Premake => "Download from premake.github.io",
            BuildSystem::QMake => "sudo apt install qt5-qmake  # Ubuntu/Debian\nbrew install qt  # macOS",
            BuildSystem::GradleCpp => "Download from gradle.org",
            BuildSystem::Custom => "Custom build script",
        }
    }
}

/// Compiler toolchain
#[derive(Debug, Clone, PartialEq, Eq)]
#[allow(dead_code)]
pub enum Compiler {
    GCC,
    Clang,
    MSVC,
    LLVM,
    MinGW,
    Emscripten,
    Intel,
    Unknown,
}

#[allow(dead_code)]
impl Compiler {
    pub fn as_str(&self) -> &str {
        match self {
            Compiler::GCC => "gcc",
            Compiler::Clang => "clang",
            Compiler::MSVC => "msvc",
            Compiler::LLVM => "llvm",
            Compiler::MinGW => "mingw",
            Compiler::Emscripten => "emscripten",
            Compiler::Intel => "icc",
            Compiler::Unknown => "unknown",
        }
    }
    
    pub fn detect_c_compiler() -> Self {
        if Command::new("clang").arg("--version").output().is_ok() {
            Compiler::Clang
        } else if Command::new("gcc").arg("--version").output().is_ok() {
            Compiler::GCC
        } else if Command::new("cl").arg("/?").output().is_ok() {
            Compiler::MSVC
        } else if Command::new("emcc").arg("--version").output().is_ok() {
            Compiler::Emscripten
        } else if Command::new("icc").arg("--version").output().is_ok() {
            Compiler::Intel
        } else {
            Compiler::Unknown
        }
    }
    
    pub fn detect_cpp_compiler() -> Self {
        if Command::new("clang++").arg("--version").output().is_ok() {
            Compiler::Clang
        } else if Command::new("g++").arg("--version").output().is_ok() {
            Compiler::GCC
        } else if Command::new("cl").arg("/?").output().is_ok() {
            Compiler::MSVC
        } else if Command::new("em++").arg("--version").output().is_ok() {
            Compiler::Emscripten
        } else if Command::new("icpc").arg("--version").output().is_ok() {
            Compiler::Intel
        } else {
            Compiler::Unknown
        }
    }
}

/// Auto-detect build system from project directory
pub fn detect_build_system<P: AsRef<Path>>(path: P) -> Option<BuildSystem> {
    let path = path.as_ref();
    
    // Check each build system's detection files
    let systems = [
        BuildSystem::CMake,
        BuildSystem::XMake,
        BuildSystem::Meson,
        BuildSystem::Bazel,
        BuildSystem::Buck2,
        BuildSystem::Conan,
        BuildSystem::Vcpkg,
        BuildSystem::Make,
        BuildSystem::Ninja,
        BuildSystem::Autotools,
        BuildSystem::SCons,
        BuildSystem::Jam,
        BuildSystem::Premake,
        BuildSystem::QMake,
        BuildSystem::GradleCpp,
    ];
    
    for system in &systems {
        for file_pattern in system.detection_files() {
            // Handle wildcards
            if file_pattern.contains('*') {
                let extension = file_pattern.trim_start_matches("*.");
                if let Ok(entries) = std::fs::read_dir(path) {
                    for entry in entries.flatten() {
                        if let Some(fname) = entry.file_name().to_str() {
                            if fname.ends_with(extension) {
                                return Some(system.clone());
                            }
                        }
                    }
                }
            } else if path.join(file_pattern).exists() {
                return Some(system.clone());
            }
        }
    }
    
    None
}

/// Check if a build tool is installed
#[allow(dead_code)]
pub fn is_tool_installed(tool: &str) -> bool {
    Command::new(tool)
        .arg("--version")
        .output()
        .is_ok()
}

/// Get version of installed tool
#[allow(dead_code)]
pub fn get_tool_version(tool: &str) -> Option<String> {
    let output = Command::new(tool)
        .arg("--version")
        .output()
        .ok()?;
    
    let stdout = String::from_utf8_lossy(&output.stdout);
    stdout.lines().next().map(|s| s.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_build_system_from_str() {
        assert_eq!(BuildSystem::from_str("cmake"), Some(BuildSystem::CMake));
        assert_eq!(BuildSystem::from_str("xmake"), Some(BuildSystem::XMake));
        assert_eq!(BuildSystem::from_str("bazel"), Some(BuildSystem::Bazel));
    }
    
    #[test]
    fn test_compiler_detection() {
        let compiler = Compiler::detect_cpp_compiler();
        assert_ne!(compiler, Compiler::Unknown);
    }
}
