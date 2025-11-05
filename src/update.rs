//! Update management for Porters
//!
//! This module handles checking for updates from GitHub releases and
//! performing automatic self-updates of the porters binary.

use crate::util::pretty::*;
use anyhow::{Context, Result};
use reqwest::blocking::Client;
use semver::Version;
use serde::Deserialize;

/// GitHub repository owner
const REPO_OWNER: &str = "muhammad-fiaz";

/// GitHub repository name
const REPO_NAME: &str = "porters";

/// Current version of porters (from Cargo.toml)
const CURRENT_VERSION: &str = env!("CARGO_PKG_VERSION");

/// Represents a GitHub release from the API
#[derive(Debug, Deserialize)]
#[allow(dead_code)]
struct GitHubRelease {
    /// Tag name (e.g., "v0.1.0")
    tag_name: String,

    /// Release name/title
    name: String,

    /// URL to the release page
    html_url: String,

    /// Release notes/body
    body: String,

    /// Release assets (binaries, archives, etc.)
    assets: Vec<GitHubAsset>,
}

/// Represents a release asset (downloadable file)
#[derive(Debug, Deserialize)]
#[allow(dead_code)]
struct GitHubAsset {
    /// Filename
    name: String,

    /// Direct download URL
    browser_download_url: String,
}

/// Check for updates on GitHub releases
///
/// Queries the GitHub API for the latest release and compares it
/// with the current version.
///
/// # Returns
/// * `Ok(Some(version))` - A newer version is available
/// * `Ok(None)` - Current version is up to date or check failed
/// * `Err(...)` - Error occurred during the check
pub fn check_for_updates() -> Result<Option<String>> {
    let client = Client::new();
    let url = format!(
        "https://api.github.com/repos/{}/{}/releases/latest",
        REPO_OWNER, REPO_NAME
    );

    let response = client
        .get(&url)
        .header("User-Agent", "porters")
        .header("Accept", "application/vnd.github.v3+json")
        .send()
        .context("Failed to check for updates")?;

    if !response.status().is_success() {
        return Ok(None);
    }

    let release: GitHubRelease = response.json()?;

    // Parse versions
    let current = Version::parse(CURRENT_VERSION)?;
    let latest_tag = release.tag_name.trim_start_matches('v');
    let latest = Version::parse(latest_tag)?;

    if latest > current {
        Ok(Some(latest.to_string()))
    } else {
        Ok(None)
    }
}

/// Perform self-update using the self_update crate
///
/// Downloads and installs the latest version of porters from GitHub releases.
/// Shows download progress and notifies the user when complete.
///
/// # Returns
/// * `Ok(())` - Update completed successfully or already up to date
/// * `Err(...)` - Error occurred during update
pub fn perform_update() -> Result<()> {
    print_step("Checking for updates");

    let current_version = Version::parse(CURRENT_VERSION)?;
    print_info(&format!("Current version: {}", current_version));

    // Use self_update crate for automatic binary update
    let status = self_update::backends::github::Update::configure()
        .repo_owner(REPO_OWNER)
        .repo_name(REPO_NAME)
        .bin_name("porters")
        .show_download_progress(true)
        .current_version(CURRENT_VERSION)
        .build()?
        .update()?;

    match status {
        self_update::Status::UpToDate(version) => {
            print_success(&format!("Already up to date (v{})", version));
        }
        self_update::Status::Updated(version) => {
            print_success(&format!("Updated to v{}", version));
            print_info("Please restart porters to use the new version");
        }
    }

    Ok(())
}

/// Display available update information with release notes
///
/// Shows a formatted notification box with version information
/// and release notes from GitHub.
///
/// # Arguments
/// * `latest_version` - The latest available version string
///
/// # Returns
/// * `Ok(())` - Information displayed successfully
/// * `Err(...)` - Error fetching release information
#[allow(dead_code)]
pub fn display_update_available(latest_version: &str) -> Result<()> {
    let client = Client::new();
    let url = format!(
        "https://api.github.com/repos/{}/{}/releases/latest",
        REPO_OWNER, REPO_NAME
    );

    let response = client
        .get(&url)
        .header("User-Agent", "porters")
        .header("Accept", "application/vnd.github.v3+json")
        .send()?;

    if response.status().is_success() {
        let release: GitHubRelease = response.json()?;

        println!("\n┌─────────────────────────────────────────────────────────┐");
        println!("│          🎉 New Version Available! 🎉                  │");
        println!("├─────────────────────────────────────────────────────────┤");
        println!(
            "│ Current: v{}                                        │",
            CURRENT_VERSION
        );
        println!(
            "│ Latest:  v{}                                        │",
            latest_version
        );
        println!("├─────────────────────────────────────────────────────────┤");
        println!("│ Update with:                                            │");
        println!("│   porters upgrade                                       │");
        println!("│                                                         │");
        println!("│ Or via cargo:                                           │");
        println!("│   cargo install porters --force                         │");
        println!("└─────────────────────────────────────────────────────────┘\n");

        if !release.body.is_empty() {
            print_info("Release Notes:");
            println!("{}", release.body);
        }
    }

    Ok(())
}

/// Silent check for updates (used on startup)
///
/// Checks GitHub releases for a new version and displays a notification
/// if one is available. Respects the `auto-update-check` setting in config.
/// Fails silently if checking is disabled or if there's an error.
pub fn silent_update_check() {
    // Try to load config, but don't fail if it doesn't exist
    if let Ok(config) = crate::config::PortersConfig::load("porters.toml") {
        // Respect the auto_update_check setting
        if !config.auto_update_check {
            return;
        }
    }

    // Perform the update check
    if let Ok(Some(latest)) = check_for_updates() {
        println!("\n💡 A new version of porters is available: v{}", latest);
        println!("   Run 'porters upgrade' to update");
        println!("   To disable this check, set 'auto-update-check = false' in porters.toml\n");
    }
}
