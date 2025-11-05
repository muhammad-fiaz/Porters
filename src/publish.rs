//! Package publishing to GitHub releases
//!
//! This module handles publishing Porters packages to GitHub releases,
//! creating release tags, uploading artifacts, and generating changelogs.
//! Supports dry-run mode for testing before actual publication.

use anyhow::{Context, Result, anyhow};
use reqwest::blocking::Client;
use serde::{Deserialize, Serialize};
use std::fs;
use crate::config::PortersConfig;
use crate::util::pretty::*;

#[derive(Debug, Serialize)]
struct GitHubRelease {
    tag_name: String,
    name: String,
    body: String,
    draft: bool,
    prerelease: bool,
}

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
struct GitHubReleaseResponse {
    id: u64,
    upload_url: String,
    html_url: String,
}

/// Publish package to GitHub releases
pub fn publish_package(
    config: &PortersConfig,
    token: &str,
    dry_run: bool,
) -> Result<()> {
    print_step("Publishing package to GitHub");
    
    // Validate configuration
    let repo_url = config.project.repository.as_ref()
        .ok_or_else(|| anyhow!("No repository URL in porters.toml"))?;
    
    let (owner, repo) = parse_github_repo(repo_url)?;
    
    print_info(&format!("Repository: {}/{}", owner, repo));
    print_info(&format!("Version: {}", config.project.version));
    
    if dry_run {
        print_warning("Dry run mode - no release will be created");
        return Ok(());
    }
    
    // Create release
    let client = Client::new();
    let release_data = GitHubRelease {
        tag_name: format!("v{}", config.project.version),
        name: format!("{} v{}", config.project.name, config.project.version),
        body: generate_release_notes(config),
        draft: false,
        prerelease: false,
    };
    
    let url = format!("https://api.github.com/repos/{}/{}/releases", owner, repo);
    
    print_step("Creating GitHub release");
    
    let response = client
        .post(&url)
        .header("Authorization", format!("Bearer {}", token))
        .header("Accept", "application/vnd.github.v3+json")
        .header("User-Agent", "porters")
        .json(&release_data)
        .send()
        .context("Failed to create GitHub release")?;
    
    if !response.status().is_success() {
        let error_text = response.text()?;
        return Err(anyhow!("GitHub API error: {}", error_text));
    }
    
    let release: GitHubReleaseResponse = response.json()?;
    
    print_success(&format!("Created release: {}", release.html_url));
    
    Ok(())
}

/// Parse GitHub repository URL to extract owner and repo name
fn parse_github_repo(url: &str) -> Result<(String, String)> {
    // Handle various URL formats:
    // https://github.com/owner/repo
    // https://github.com/owner/repo.git
    // git@github.com:owner/repo.git
    
    let url = url.trim_end_matches(".git");
    
    if url.contains("github.com/") {
        let parts: Vec<&str> = url.split("github.com/").collect();
        if parts.len() == 2 {
            let repo_parts: Vec<&str> = parts[1].split('/').collect();
            if repo_parts.len() >= 2 {
                return Ok((repo_parts[0].to_string(), repo_parts[1].to_string()));
            }
        }
    } else if url.contains("github.com:") {
        let parts: Vec<&str> = url.split("github.com:").collect();
        if parts.len() == 2 {
            let repo_parts: Vec<&str> = parts[1].split('/').collect();
            if repo_parts.len() >= 2 {
                return Ok((repo_parts[0].to_string(), repo_parts[1].to_string()));
            }
        }
    }
    
    Err(anyhow!("Invalid GitHub repository URL: {}", url))
}

/// Generate release notes from config and changelog
fn generate_release_notes(config: &PortersConfig) -> String {
    let mut notes = String::new();
    
    if let Some(desc) = &config.project.description {
        notes.push_str(&format!("{}\n\n", desc));
    }
    
    notes.push_str("## Installation\n\n");
    notes.push_str("### Via Cargo\n");
    notes.push_str("```bash\n");
    notes.push_str("cargo install porters\n");
    notes.push_str("```\n\n");
    
    notes.push_str("### Download Binary\n");
    notes.push_str("Download the appropriate binary for your platform from the assets below.\n\n");
    
    notes.push_str("| Platform | Download |\n");
    notes.push_str("|----------|----------|\n");
    notes.push_str(&format!("| Windows (x64) | [porters-v{}-x86_64-pc-windows-msvc.zip](https://github.com/muhammad-fiaz/porters/releases/download/v{}/porters-v{}-x86_64-pc-windows-msvc.zip) |\n", 
        config.project.version, config.project.version, config.project.version));
    notes.push_str(&format!("| macOS (Intel) | [porters-v{}-x86_64-apple-darwin.tar.gz](https://github.com/muhammad-fiaz/porters/releases/download/v{}/porters-v{}-x86_64-apple-darwin.tar.gz) |\n",
        config.project.version, config.project.version, config.project.version));
    notes.push_str(&format!("| macOS (ARM) | [porters-v{}-aarch64-apple-darwin.tar.gz](https://github.com/muhammad-fiaz/porters/releases/download/v{}/porters-v{}-aarch64-apple-darwin.tar.gz) |\n",
        config.project.version, config.project.version, config.project.version));
    notes.push_str(&format!("| Linux (x64) | [porters-v{}-x86_64-unknown-linux-gnu.tar.gz](https://github.com/muhammad-fiaz/porters/releases/download/v{}/porters-v{}-x86_64-unknown-linux-gnu.tar.gz) |\n",
        config.project.version, config.project.version, config.project.version));
    
    notes.push_str("\n## What's Changed\n\n");
    
    // Try to read CHANGELOG.md
    if let Ok(changelog) = fs::read_to_string("CHANGELOG.md") {
        // Extract latest version section
        if let Some(section) = extract_latest_changelog(&changelog, &config.project.version) {
            notes.push_str(&section);
        } else {
            notes.push_str("See CHANGELOG.md for details.\n");
        }
    } else {
        notes.push_str("See git history for changes.\n");
    }
    
    notes
}

/// Extract the latest version section from CHANGELOG.md
fn extract_latest_changelog(changelog: &str, version: &str) -> Option<String> {
    let version_header = format!("## [{}]", version);
    
    if let Some(start) = changelog.find(&version_header) {
        let after_header = &changelog[start + version_header.len()..];
        
        // Find next version header
        if let Some(end) = after_header.find("## [") {
            return Some(after_header[..end].trim().to_string());
        } else {
            return Some(after_header.trim().to_string());
        }
    }
    
    None
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_parse_github_repo() {
        assert_eq!(
            parse_github_repo("https://github.com/muhammad-fiaz/porters").unwrap(),
            ("muhammad-fiaz".to_string(), "porters".to_string())
        );
        
        assert_eq!(
            parse_github_repo("https://github.com/muhammad-fiaz/porters.git").unwrap(),
            ("muhammad-fiaz".to_string(), "porters".to_string())
        );
        
        assert_eq!(
            parse_github_repo("git@github.com:muhammad-fiaz/porters.git").unwrap(),
            ("muhammad-fiaz".to_string(), "porters".to_string())
        );
    }
}
