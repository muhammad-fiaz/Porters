//! Project source file scanning and analysis
//!
//! This module scans C/C++ projects to discover source files, headers,
//! and include directories. It intelligently excludes common directories
//! like vendor, build, and node_modules for efficient project analysis.

use anyhow::{Context, Result};
use std::collections::HashSet;
use std::path::{Path, PathBuf};
use walkdir::WalkDir;

/// Supported C/C++ source file extensions
const SOURCE_EXTENSIONS: &[&str] = &["c", "cc", "cpp", "cxx"];

/// Supported C/C++ header file extensions
const HEADER_EXTENSIONS: &[&str] = &["h", "hh", "hpp", "hxx"];

/// Directories to exclude from scanning
const EXCLUDED_DIRS: &[&str] = &[
    "vendor",
    "third_party",
    "build",
    "out",
    "target",
    ".git",
    ".svn",
    "node_modules",
];

#[derive(Debug, Clone)]
pub struct ProjectSources {
    pub source_files: Vec<PathBuf>,
    #[allow(dead_code)]
    pub header_files: Vec<PathBuf>,
    pub include_paths: Vec<PathBuf>,
    #[allow(dead_code)]
    pub root: PathBuf,
}

/// Scan a project directory for C/C++ source and header files
pub fn scan_project<P: AsRef<Path>>(root: P) -> Result<ProjectSources> {
    let root = root
        .as_ref()
        .canonicalize()
        .with_context(|| format!("Failed to canonicalize path: {}", root.as_ref().display()))?;

    let mut source_files = Vec::new();
    let mut header_files = Vec::new();
    let mut include_dirs = HashSet::new();

    for entry in WalkDir::new(&root)
        .follow_links(true)
        .into_iter()
        .filter_entry(|e| !is_excluded_dir(e.path()))
    {
        let entry = entry?;
        let path = entry.path();

        if !path.is_file() {
            continue;
        }

        if let Some(ext) = path.extension().and_then(|e| e.to_str()) {
            if SOURCE_EXTENSIONS.contains(&ext) {
                source_files.push(path.to_path_buf());
            } else if HEADER_EXTENSIONS.contains(&ext) {
                header_files.push(path.to_path_buf());

                // Add parent directory as include path
                if let Some(parent) = path.parent() {
                    include_dirs.insert(parent.to_path_buf());
                }
            }
        }
    }

    // Add common include directories
    let common_includes = vec!["include", "src", "inc"];
    for dir_name in common_includes {
        let inc_path = root.join(dir_name);
        if inc_path.exists() && inc_path.is_dir() {
            include_dirs.insert(inc_path);
        }
    }

    let include_paths: Vec<PathBuf> = include_dirs.into_iter().collect();

    Ok(ProjectSources {
        source_files,
        header_files,
        include_paths,
        root,
    })
}

/// Check if a directory should be excluded from scanning
fn is_excluded_dir(path: &Path) -> bool {
    path.file_name()
        .and_then(|name| name.to_str())
        .map(|name| EXCLUDED_DIRS.contains(&name))
        .unwrap_or(false)
}

/// Find all source files matching a pattern
#[allow(dead_code)]
pub fn find_sources<P: AsRef<Path>>(root: P, pattern: &str) -> Result<Vec<PathBuf>> {
    let root = root.as_ref();
    let regex = regex::Regex::new(pattern)?;

    let mut matches = Vec::new();

    for entry in WalkDir::new(root)
        .follow_links(true)
        .into_iter()
        .filter_entry(|e| !is_excluded_dir(e.path()))
    {
        let entry = entry?;
        let path = entry.path();

        if path.is_file() {
            if let Some(file_name) = path.file_name().and_then(|n| n.to_str()) {
                if regex.is_match(file_name) {
                    matches.push(path.to_path_buf());
                }
            }
        }
    }

    Ok(matches)
}

/// Scan for test files (files with "test" in the name or in test directories)
#[allow(dead_code)]
pub fn scan_test_files<P: AsRef<Path>>(root: P) -> Result<Vec<PathBuf>> {
    let root = root.as_ref();
    let mut test_files = Vec::new();

    for entry in WalkDir::new(root)
        .follow_links(true)
        .into_iter()
        .filter_entry(|e| !is_excluded_dir(e.path()))
    {
        let entry = entry?;
        let path = entry.path();

        if !path.is_file() {
            continue;
        }

        // Check if file is in a test directory or has "test" in its name
        let is_test = path.components().any(|c| {
            c.as_os_str()
                .to_str()
                .map(|s| s.contains("test") || s == "tests")
                .unwrap_or(false)
        }) || path
            .file_name()
            .and_then(|n| n.to_str())
            .map(|n| n.contains("test"))
            .unwrap_or(false);

        if is_test {
            if let Some(ext) = path.extension().and_then(|e| e.to_str()) {
                if SOURCE_EXTENSIONS.contains(&ext) {
                    test_files.push(path.to_path_buf());
                }
            }
        }
    }

    Ok(test_files)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_excluded_dirs() {
        assert!(is_excluded_dir(Path::new("vendor")));
        assert!(is_excluded_dir(Path::new("build")));
        assert!(is_excluded_dir(Path::new(".git")));
        assert!(!is_excluded_dir(Path::new("src")));
    }
}
