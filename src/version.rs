//! Version requirement checking and validation
//!
//! This module provides functionality for checking version requirements
//! of tools and dependencies, ensuring compatibility.

use anyhow::{Context, Result, bail};
use std::cmp::Ordering;
use std::process::Command;

/// Semantic version comparator
#[derive(Debug, Clone, PartialEq)]
pub enum VersionReq {
    /// Exact version (==1.2.3)
    Exact(Version),
    /// Greater than or equal (>=1.2.3)
    GreaterEq(Version),
    /// Less than or equal (<=1.2.3)
    LessEq(Version),
    /// Greater than (>1.2.3)
    Greater(Version),
    /// Less than (<1.2.3)
    Less(Version),
    /// Compatible (^1.2.3 - allows patch/minor updates)
    Compatible(Version),
    /// Tilde (~1.2.3 - allows patch updates only)
    Tilde(Version),
    /// Any version (*)
    Any,
}

/// Semantic version (major.minor.patch)
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Version {
    pub major: u32,
    pub minor: u32,
    pub patch: u32,
}

impl Version {
    /// Parse a version string like "1.2.3"
    pub fn parse(s: &str) -> Result<Self> {
        let parts: Vec<&str> = s.trim().split('.').collect();

        if parts.is_empty() {
            bail!("Empty version string");
        }

        let major = parts[0]
            .parse::<u32>()
            .with_context(|| format!("Invalid major version: {}", parts[0]))?;

        let minor = if parts.len() > 1 {
            parts[1]
                .parse::<u32>()
                .with_context(|| format!("Invalid minor version: {}", parts[1]))?
        } else {
            0
        };

        let patch = if parts.len() > 2 {
            parts[2]
                .parse::<u32>()
                .with_context(|| format!("Invalid patch version: {}", parts[2]))?
        } else {
            0
        };

        Ok(Version {
            major,
            minor,
            patch,
        })
    }
}

impl PartialOrd for Version {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Version {
    fn cmp(&self, other: &Self) -> Ordering {
        match self.major.cmp(&other.major) {
            Ordering::Equal => match self.minor.cmp(&other.minor) {
                Ordering::Equal => self.patch.cmp(&other.patch),
                ord => ord,
            },
            ord => ord,
        }
    }
}

impl std::fmt::Display for Version {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}.{}.{}", self.major, self.minor, self.patch)
    }
}

impl VersionReq {
    /// Parse a version requirement string
    pub fn parse(s: &str) -> Result<Self> {
        let s = s.trim();

        if s == "*" {
            return Ok(VersionReq::Any);
        }

        if let Some(ver) = s.strip_prefix(">=") {
            return Ok(VersionReq::GreaterEq(Version::parse(ver)?));
        }

        if let Some(ver) = s.strip_prefix("<=") {
            return Ok(VersionReq::LessEq(Version::parse(ver)?));
        }

        if let Some(ver) = s.strip_prefix('>') {
            return Ok(VersionReq::Greater(Version::parse(ver)?));
        }

        if let Some(ver) = s.strip_prefix('<') {
            return Ok(VersionReq::Less(Version::parse(ver)?));
        }

        if let Some(ver) = s.strip_prefix('^') {
            return Ok(VersionReq::Compatible(Version::parse(ver)?));
        }

        if let Some(ver) = s.strip_prefix('~') {
            return Ok(VersionReq::Tilde(Version::parse(ver)?));
        }

        if let Some(ver) = s.strip_prefix("==") {
            return Ok(VersionReq::Exact(Version::parse(ver)?));
        }

        // Default to exact match
        Ok(VersionReq::Exact(Version::parse(s)?))
    }

    /// Check if a version satisfies this requirement
    pub fn matches(&self, version: &Version) -> bool {
        match self {
            VersionReq::Any => true,
            VersionReq::Exact(req) => version == req,
            VersionReq::GreaterEq(req) => version >= req,
            VersionReq::LessEq(req) => version <= req,
            VersionReq::Greater(req) => version > req,
            VersionReq::Less(req) => version < req,
            VersionReq::Compatible(req) => {
                // ^1.2.3 allows >=1.2.3, <2.0.0
                version.major == req.major && version >= req
            }
            VersionReq::Tilde(req) => {
                // ~1.2.3 allows >=1.2.3, <1.3.0
                version.major == req.major
                    && version.minor == req.minor
                    && version.patch >= req.patch
            }
        }
    }
}

/// Tool version checker
pub struct ToolVersionChecker;

impl ToolVersionChecker {
    /// Get the installed version of a tool
    pub fn get_tool_version(tool: &str) -> Result<Version> {
        let output = match tool {
            "c" | "gcc" => {
                // Try gcc first, then clang
                if let Ok(out) = Command::new("gcc").arg("--version").output() {
                    out
                } else {
                    Command::new("clang").arg("--version").output()?
                }
            }
            "cpp" | "g++" => {
                // Try g++ first, then clang++
                if let Ok(out) = Command::new("g++").arg("--version").output() {
                    out
                } else {
                    Command::new("clang++").arg("--version").output()?
                }
            }
            "clang" => Command::new("clang").arg("--version").output()?,
            "cmake" => Command::new("cmake").arg("--version").output()?,
            "ninja" => Command::new("ninja").arg("--version").output()?,
            "make" => Command::new("make").arg("--version").output()?,
            "xmake" => Command::new("xmake").arg("--version").output()?,
            "meson" => Command::new("meson").arg("--version").output()?,
            "bazel" => Command::new("bazel").arg("--version").output()?,
            "conan" => Command::new("conan").arg("--version").output()?,
            "vcpkg" => Command::new("vcpkg").arg("version").output()?,
            "msvc" => {
                // MSVC version detection (cl.exe)
                Command::new("cl").output()?
            }
            _ => bail!("Unknown tool: {}", tool),
        };

        let stdout = String::from_utf8_lossy(&output.stdout);
        let stderr = String::from_utf8_lossy(&output.stderr);
        let combined = format!("{}{}", stdout, stderr);

        Self::parse_version_from_output(&combined, tool)
    }

    /// Parse version from tool output
    fn parse_version_from_output(output: &str, tool: &str) -> Result<Version> {
        // Common patterns for version extraction
        let patterns = vec![
            // "version 1.2.3"
            r"version\s+(\d+)\.(\d+)\.(\d+)",
            // "cmake version 3.20.0"
            r"cmake\s+version\s+(\d+)\.(\d+)\.(\d+)",
            // "gcc (GCC) 11.2.0"
            r"\(GCC\)\s+(\d+)\.(\d+)\.(\d+)",
            // "g++ (GCC) 11.2.0"
            r"g\+\+\s+\(GCC\)\s+(\d+)\.(\d+)\.(\d+)",
            // "clang version 13.0.0"
            r"clang\s+version\s+(\d+)\.(\d+)\.(\d+)",
            // Just "1.2.3" (ninja, meson)
            r"^(\d+)\.(\d+)\.(\d+)",
            // "Bazel 5.0.0"
            r"Bazel\s+(\d+)\.(\d+)\.(\d+)",
            // "Conan version 1.50.0"
            r"Conan\s+version\s+(\d+)\.(\d+)\.(\d+)",
        ];

        for pattern in patterns {
            if let Ok(re) = regex::Regex::new(pattern) {
                if let Some(caps) = re.captures(output) {
                    if caps.len() >= 4 {
                        let major = caps[1].parse::<u32>()?;
                        let minor = caps[2].parse::<u32>()?;
                        let patch = caps[3].parse::<u32>()?;
                        return Ok(Version {
                            major,
                            minor,
                            patch,
                        });
                    }
                }
            }
        }

        bail!(
            "Could not parse version for tool '{}' from output:\n{}",
            tool,
            output
        )
    }

    /// Check if a tool is installed
    pub fn is_tool_installed(tool: &str) -> bool {
        Self::get_tool_version(tool).is_ok()
    }

    /// Check if a tool version satisfies a requirement
    pub fn check_requirement(tool: &str, requirement: &str) -> Result<bool> {
        let req = VersionReq::parse(requirement)?;

        if matches!(req, VersionReq::Any) {
            return Ok(true);
        }

        let installed = Self::get_tool_version(tool)
            .with_context(|| format!("Tool '{}' is not installed or not found in PATH", tool))?;

        Ok(req.matches(&installed))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_version_parsing() {
        let v = Version::parse("1.2.3").unwrap();
        assert_eq!(v.major, 1);
        assert_eq!(v.minor, 2);
        assert_eq!(v.patch, 3);

        let v2 = Version::parse("10").unwrap();
        assert_eq!(v2.major, 10);
        assert_eq!(v2.minor, 0);
        assert_eq!(v2.patch, 0);
    }

    #[test]
    fn test_version_comparison() {
        let v1 = Version::parse("1.2.3").unwrap();
        let v2 = Version::parse("1.2.4").unwrap();
        let v3 = Version::parse("1.3.0").unwrap();
        let v4 = Version::parse("2.0.0").unwrap();

        assert!(v1 < v2);
        assert!(v2 < v3);
        assert!(v3 < v4);
        assert!(v1 == v1);
    }

    #[test]
    fn test_version_req_parsing() {
        assert!(matches!(VersionReq::parse("*").unwrap(), VersionReq::Any));
        assert!(matches!(
            VersionReq::parse(">=1.2.3").unwrap(),
            VersionReq::GreaterEq(_)
        ));
        assert!(matches!(
            VersionReq::parse("^1.2.3").unwrap(),
            VersionReq::Compatible(_)
        ));
        assert!(matches!(
            VersionReq::parse("~1.2.3").unwrap(),
            VersionReq::Tilde(_)
        ));
        assert!(matches!(
            VersionReq::parse("1.2.3").unwrap(),
            VersionReq::Exact(_)
        ));
    }

    #[test]
    fn test_version_req_matching() {
        let v = Version::parse("1.2.3").unwrap();

        assert!(VersionReq::parse("*").unwrap().matches(&v));
        assert!(VersionReq::parse(">=1.2.0").unwrap().matches(&v));
        assert!(VersionReq::parse("<=1.2.5").unwrap().matches(&v));
        assert!(VersionReq::parse("^1.2.0").unwrap().matches(&v));
        assert!(VersionReq::parse("~1.2.0").unwrap().matches(&v));
        assert!(VersionReq::parse("1.2.3").unwrap().matches(&v));

        assert!(!VersionReq::parse(">=1.3.0").unwrap().matches(&v));
        assert!(!VersionReq::parse("<1.2.3").unwrap().matches(&v));
        assert!(!VersionReq::parse("^2.0.0").unwrap().matches(&v));
    }
}
