//! File and directory hashing utilities
//!
//! This module provides SHA-256 hashing functionality for files and directories,
//! used for cache validation and dependency verification.

use anyhow::{Context, Result};
use sha2::{Digest, Sha256};
use std::fs::File;
use std::io::Read;
use std::path::Path;
use walkdir::WalkDir;

/// Calculate SHA-256 hash of a file
pub fn calculate_file_hash(path: &Path) -> Result<String> {
    let mut file = File::open(path)
        .with_context(|| format!("Failed to open file for hashing: {}", path.display()))?;

    let mut hasher = Sha256::new();
    let mut buffer = [0; 8192];

    loop {
        let bytes_read = file
            .read(&mut buffer)
            .with_context(|| format!("Failed to read file: {}", path.display()))?;

        if bytes_read == 0 {
            break;
        }

        hasher.update(&buffer[..bytes_read]);
    }

    Ok(format!("{:x}", hasher.finalize()))
}

/// Calculate SHA-256 hash of an entire directory
/// Includes all files recursively, sorted by path for consistency
pub fn calculate_directory_hash(dir: &Path) -> Result<String> {
    if !dir.exists() {
        return Err(anyhow::anyhow!(
            "Directory does not exist: {}",
            dir.display()
        ));
    }

    if !dir.is_dir() {
        return Err(anyhow::anyhow!(
            "Path is not a directory: {}",
            dir.display()
        ));
    }

    let mut hasher = Sha256::new();
    let mut entries = Vec::new();

    // Collect all files in sorted order for deterministic hashing
    for entry in WalkDir::new(dir).follow_links(false).sort_by_file_name() {
        let entry = entry
            .with_context(|| format!("Failed to read directory entry in {}", dir.display()))?;
        let path = entry.path();

        // Skip directories, only hash files
        if !path.is_file() {
            continue;
        }

        // Skip .git directory to avoid hash changes from git metadata
        if path.components().any(|c| c.as_os_str() == ".git") {
            continue;
        }

        entries.push(path.to_path_buf());
    }

    // Hash each file's relative path and contents
    for file_path in entries {
        // Hash the relative path first (for structure)
        let relative_path = file_path
            .strip_prefix(dir)
            .unwrap_or(&file_path)
            .to_string_lossy()
            .replace('\\', "/"); // Normalize path separators

        hasher.update(relative_path.as_bytes());
        hasher.update(b"\0"); // Separator

        // Hash the file contents
        let file_hash = calculate_file_hash(&file_path)?;
        hasher.update(file_hash.as_bytes());
        hasher.update(b"\0"); // Separator
    }

    Ok(format!("{:x}", hasher.finalize()))
}

/// Verify that a directory's hash matches the expected checksum
pub fn verify_directory_hash(dir: &Path, expected_hash: &str) -> Result<bool> {
    let actual_hash = calculate_directory_hash(dir)
        .with_context(|| format!("Failed to calculate hash for directory: {}", dir.display()))?;

    Ok(actual_hash == expected_hash)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use tempfile::TempDir;

    #[test]
    fn test_file_hash() {
        let temp_dir = TempDir::new().unwrap();
        let file_path = temp_dir.path().join("test.txt");
        fs::write(&file_path, b"Hello, World!").unwrap();

        let hash = calculate_file_hash(&file_path).unwrap();
        // SHA-256 of "Hello, World!"
        assert_eq!(
            hash,
            "dffd6021bb2bd5b0af676290809ec3a53191dd81c7f70a4b28688a362182986f"
        );
    }

    #[test]
    fn test_directory_hash() {
        let temp_dir = TempDir::new().unwrap();
        let dir_path = temp_dir.path();

        fs::write(dir_path.join("file1.txt"), b"content1").unwrap();
        fs::write(dir_path.join("file2.txt"), b"content2").unwrap();

        let hash1 = calculate_directory_hash(dir_path).unwrap();
        let hash2 = calculate_directory_hash(dir_path).unwrap();

        // Same directory should produce same hash
        assert_eq!(hash1, hash2);
    }

    #[test]
    fn test_verify_hash() {
        let temp_dir = TempDir::new().unwrap();
        let dir_path = temp_dir.path();

        fs::write(dir_path.join("test.txt"), b"test content").unwrap();

        let hash = calculate_directory_hash(dir_path).unwrap();
        assert!(verify_directory_hash(dir_path, &hash).unwrap());
        assert!(!verify_directory_hash(dir_path, "invalid_hash").unwrap());
    }
}
