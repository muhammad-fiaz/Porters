//! Build system abstraction layer
//!
//! This module provides a unified trait-based interface for all supported
//! build systems. It includes adapters for CMake, Make, Ninja, Meson, Bazel,
//! XMake, and 10+ other build tools, enabling consistent build orchestration.

use anyhow::{Result, anyhow};
use std::path::Path;

pub mod autotools;
pub mod bazel;
pub mod buck2;
pub mod cmake;
pub mod conan;
pub mod custom;
pub mod make;
pub mod meson;
pub mod ninja;
pub mod premake;
pub mod qmake;
pub mod scons;
pub mod vcpkg;
pub mod xmake;

use crate::config::PortersConfig;
use crate::deps::ResolvedDependency;
use crate::scan::ProjectSources;

/// Trait that all build system adapters must implement
pub trait BuildSystem {
    /// Get the name of the build system
    fn name(&self) -> &str;

    /// Check if this build system is present in the project
    fn detect(root: &Path) -> bool
    where
        Self: Sized;

    /// Configure the build (if needed)
    fn configure(&self, sources: &ProjectSources, deps: &[ResolvedDependency]) -> Result<()>;

    /// Build the project
    fn build(
        &self,
        sources: &ProjectSources,
        deps: &[ResolvedDependency],
        args: &[String],
    ) -> Result<()>;

    /// Run the built executable
    fn run(&self, args: &[String]) -> Result<()>;

    /// Run tests
    fn test(&self, sources: &ProjectSources, deps: &[ResolvedDependency]) -> Result<()>;

    /// Clean build artifacts
    fn clean(&self) -> Result<()>;
}

/// Detect which build system to use
pub fn detect_build_system(root: &str, config: &PortersConfig) -> Result<Box<dyn BuildSystem>> {
    let root_path = Path::new(root);

    // Check if custom build is configured
    if let Some(ref custom_build) = config.build.custom {
        return Ok(Box::new(custom::CustomBuildSystem::new(
            root,
            custom_build.clone(),
        )));
    }

    // Check if build system is explicitly specified
    if let Some(ref system) = config.build.system {
        return match system.as_str() {
            "cmake" => Ok(Box::new(cmake::CMakeBuildSystem::new(root))),
            "xmake" => Ok(Box::new(xmake::XMakeBuildSystem::new(root))),
            "meson" => Ok(Box::new(meson::MesonBuildSystem::new(root))),
            "make" => Ok(Box::new(make::MakeBuildSystem::new(root))),
            "ninja" => Ok(Box::new(ninja::NinjaBuildSystem::new(root))),
            "autotools" => Ok(Box::new(autotools::AutotoolsBuildSystem::new(root))),
            "scons" => Ok(Box::new(scons::SConsBuildSystem::new(root))),
            "bazel" => Ok(Box::new(bazel::BazelBuildSystem::new(root))),
            "buck2" => Ok(Box::new(buck2::Buck2BuildSystem::new(root))),
            "premake" => Ok(Box::new(premake::PremakeBuildSystem::new(root))),
            "qmake" => Ok(Box::new(qmake::QMakeBuildSystem::new(root))),
            "conan" => Ok(Box::new(conan::ConanBuildSystem::new(root))),
            "vcpkg" => Ok(Box::new(vcpkg::VcpkgBuildSystem::new(root))),
            _ => Err(anyhow!("Unknown build system: {}", system)),
        };
    }

    // Auto-detect build system (priority order)

    // Package managers first (they might wrap other build systems)
    if conan::ConanBuildSystem::detect(root_path) {
        return Ok(Box::new(conan::ConanBuildSystem::new(root)));
    }

    if vcpkg::VcpkgBuildSystem::detect(root_path) {
        return Ok(Box::new(vcpkg::VcpkgBuildSystem::new(root)));
    }

    // Modern build systems
    if bazel::BazelBuildSystem::detect(root_path) {
        return Ok(Box::new(bazel::BazelBuildSystem::new(root)));
    }

    if buck2::Buck2BuildSystem::detect(root_path) {
        return Ok(Box::new(buck2::Buck2BuildSystem::new(root)));
    }

    if cmake::CMakeBuildSystem::detect(root_path) {
        return Ok(Box::new(cmake::CMakeBuildSystem::new(root)));
    }

    if xmake::XMakeBuildSystem::detect(root_path) {
        return Ok(Box::new(xmake::XMakeBuildSystem::new(root)));
    }

    if meson::MesonBuildSystem::detect(root_path) {
        return Ok(Box::new(meson::MesonBuildSystem::new(root)));
    }

    if premake::PremakeBuildSystem::detect(root_path) {
        return Ok(Box::new(premake::PremakeBuildSystem::new(root)));
    }

    if qmake::QMakeBuildSystem::detect(root_path) {
        return Ok(Box::new(qmake::QMakeBuildSystem::new(root)));
    }

    // Traditional build systems
    if ninja::NinjaBuildSystem::detect(root_path) {
        return Ok(Box::new(ninja::NinjaBuildSystem::new(root)));
    }

    if autotools::AutotoolsBuildSystem::detect(root_path) {
        return Ok(Box::new(autotools::AutotoolsBuildSystem::new(root)));
    }

    if scons::SConsBuildSystem::detect(root_path) {
        return Ok(Box::new(scons::SConsBuildSystem::new(root)));
    }

    if make::MakeBuildSystem::detect(root_path) {
        return Ok(Box::new(make::MakeBuildSystem::new(root)));
    }

    // Default to CMake
    Ok(Box::new(cmake::CMakeBuildSystem::new(root)))
}
