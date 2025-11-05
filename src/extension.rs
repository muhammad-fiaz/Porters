//! Extension/Plugin system for Porters
//!
//! This module provides a flexible extension system allowing users to:
//! - Add custom build hooks (pre/post build/install)
//! - Define custom commands
//! - Extend Porters functionality with Lua or WASM plugins
//! - Share extensions via extension.toml manifests

use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};

/// Extension manifest definition (extension.toml)
///
/// Defines metadata, hooks, and commands for a Porters extension.
/// Extensions can provide custom build steps and additional CLI commands.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExtensionManifest {
    pub name: String,
    pub version: String,
    pub description: String,
    pub authors: Vec<String>,
    pub license: Option<String>,
    pub repository: Option<String>,
    pub homepage: Option<String>,

    #[serde(default)]
    pub hooks: ExtensionHooks,

    #[serde(default)]
    pub commands: Vec<ExtensionCommand>,
}

/// Extension hooks
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ExtensionHooks {
    pub pre_build: Option<String>,
    pub post_build: Option<String>,
    pub pre_install: Option<String>,
    pub post_install: Option<String>,
    pub pre_sync: Option<String>,
    pub post_sync: Option<String>,
}

/// Custom command provided by extension
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExtensionCommand {
    pub name: String,
    pub description: String,
    pub script: String,
}

/// Extension manager
pub struct ExtensionManager {
    extensions_dir: PathBuf,
    loaded_extensions: Vec<Extension>,
}

/// Loaded extension
pub struct Extension {
    pub manifest: ExtensionManifest,
    pub path: PathBuf,
}

impl ExtensionManager {
    pub fn new() -> Result<Self> {
        let extensions_dir = Self::extensions_dir()?;
        std::fs::create_dir_all(&extensions_dir)?;

        Ok(Self {
            extensions_dir,
            loaded_extensions: Vec::new(),
        })
    }

    /// Get extensions directory
    pub fn extensions_dir() -> Result<PathBuf> {
        let home = if cfg!(windows) {
            std::env::var("USERPROFILE")?
        } else {
            std::env::var("HOME")?
        };

        Ok(PathBuf::from(home).join(".porters").join("extensions"))
    }

    /// Load all available extensions
    pub fn load_extensions(&mut self) -> Result<()> {
        if !self.extensions_dir.exists() {
            return Ok(());
        }

        for entry in std::fs::read_dir(&self.extensions_dir)? {
            let entry = entry?;
            let path = entry.path();

            if path.is_dir() {
                if let Ok(extension) = self.load_extension(&path) {
                    self.loaded_extensions.push(extension);
                }
            }
        }

        Ok(())
    }

    /// Load a single extension
    fn load_extension(&self, path: &Path) -> Result<Extension> {
        let manifest_path = path.join("extension.toml");
        let content = std::fs::read_to_string(&manifest_path)?;
        let manifest: ExtensionManifest = toml::from_str(&content)?;

        Ok(Extension {
            manifest,
            path: path.to_path_buf(),
        })
    }

    /// Install extension from various sources
    ///
    /// Extensions can be installed from:
    /// - Git repositories (user provides URL)
    /// - Local paths (user provides path)
    /// - Listed in porters.toml extensions array (auto-installed)
    pub fn install_extension(&mut self, name: &str, source: ExtensionSource) -> Result<()> {
        match source {
            ExtensionSource::CratesIo => {
                // User should install extensions manually via cargo or from git
                // We just verify if it exists in the extensions directory
                println!(
                    "📦 Looking for extension '{}' in extensions directory...",
                    name
                );

                let dest = self.extensions_dir.join(name);
                if dest.exists() {
                    let extension = self.load_extension(&dest)?;
                    self.loaded_extensions.push(extension);
                    println!("✅ Extension '{}' loaded successfully", name);
                } else {
                    println!("⚠️  Extension '{}' not found.", name);
                    println!(
                        "   To install: Clone the extension to ~/.porters/extensions/{}",
                        name
                    );
                    println!("   Or add it to [extensions] in porters.toml for auto-loading");
                }
            }
            ExtensionSource::Git(url) => {
                // Clone from git
                println!("📦 Installing extension '{}' from {}...", name, url);
                let dest = self.extensions_dir.join(name);

                use git2::Repository;
                Repository::clone(&url, &dest)?;

                // Load the extension
                let extension = self.load_extension(&dest)?;
                self.loaded_extensions.push(extension);
                println!("✅ Extension '{}' installed successfully", name);
            }
            ExtensionSource::Path(path) => {
                // Copy from local path
                println!("📦 Installing extension '{}' from path...", name);
                let dest = self.extensions_dir.join(name);

                Self::copy_dir(&path, &dest)?;

                let extension = self.load_extension(&dest)?;
                self.loaded_extensions.push(extension);
                println!("✅ Extension '{}' installed successfully", name);
            }
        }

        Ok(())
    }

    /// Uninstall extension
    pub fn uninstall_extension(&mut self, name: &str) -> Result<()> {
        let extension_path = self.extensions_dir.join(name);

        if extension_path.exists() {
            std::fs::remove_dir_all(&extension_path)?;
            self.loaded_extensions.retain(|e| e.manifest.name != name);
            println!("✅ Extension '{}' uninstalled", name);
        } else {
            println!("⚠️  Extension '{}' not found", name);
        }

        Ok(())
    }

    /// List installed extensions
    pub fn list_extensions(&self) -> &[Extension] {
        &self.loaded_extensions
    }

    /// Execute hook
    pub fn execute_hook(&self, hook_name: &str, context: &HookContext) -> Result<()> {
        for extension in &self.loaded_extensions {
            let script = match hook_name {
                "pre_build" => &extension.manifest.hooks.pre_build,
                "post_build" => &extension.manifest.hooks.post_build,
                "pre_install" => &extension.manifest.hooks.pre_install,
                "post_install" => &extension.manifest.hooks.post_install,
                "pre_sync" => &extension.manifest.hooks.pre_sync,
                "post_sync" => &extension.manifest.hooks.post_sync,
                _ => &None,
            };

            if let Some(script) = script {
                println!(
                    "🔌 Running {} hook from '{}'...",
                    hook_name, extension.manifest.name
                );
                self.execute_script(script, &extension.path, context)?;
            }
        }

        Ok(())
    }

    /// Execute custom command
    #[allow(dead_code)]
    pub fn execute_command(&self, command_name: &str, args: &[String]) -> Result<bool> {
        for extension in &self.loaded_extensions {
            for cmd in &extension.manifest.commands {
                if cmd.name == command_name {
                    println!(
                        "🔌 Running command '{}' from '{}'...",
                        command_name, extension.manifest.name
                    );

                    let context = HookContext {
                        project_dir: std::env::current_dir()?,
                        args: args.to_vec(),
                    };

                    self.execute_script(&cmd.script, &extension.path, &context)?;
                    return Ok(true);
                }
            }
        }

        Ok(false)
    }

    /// Execute script
    fn execute_script(
        &self,
        script: &str,
        extension_path: &Path,
        context: &HookContext,
    ) -> Result<()> {
        use std::process::Command;

        let script_path = extension_path.join(script);

        if !script_path.exists() {
            anyhow::bail!("Script not found: {}", script_path.display());
        }

        let output = if cfg!(windows) {
            Command::new("cmd")
                .args(["/C", script])
                .current_dir(&context.project_dir)
                .output()?
        } else {
            Command::new("sh")
                .arg(&script_path)
                .current_dir(&context.project_dir)
                .output()?
        };

        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            anyhow::bail!("Extension script failed: {}", stderr);
        }

        Ok(())
    }

    /// Copy directory recursively
    fn copy_dir(src: &Path, dst: &Path) -> Result<()> {
        std::fs::create_dir_all(dst)?;

        for entry in std::fs::read_dir(src)? {
            let entry = entry?;
            let src_path = entry.path();
            let dst_path = dst.join(entry.file_name());

            if src_path.is_dir() {
                Self::copy_dir(&src_path, &dst_path)?;
            } else {
                std::fs::copy(&src_path, &dst_path)?;
            }
        }

        Ok(())
    }

    /// Create extension template
    pub fn create_template(name: &str, path: &Path) -> Result<()> {
        let extension_dir = path.join(name);
        std::fs::create_dir_all(&extension_dir)?;

        // Create extension.toml
        let manifest = ExtensionManifest {
            name: name.to_string(),
            version: "0.1.0".to_string(),
            description: format!("{} extension for Porters", name),
            authors: vec!["Your Name <you@example.com>".to_string()],
            license: Some("MIT".to_string()),
            repository: None,
            homepage: None,
            hooks: ExtensionHooks::default(),
            commands: vec![],
        };

        let manifest_content = toml::to_string_pretty(&manifest)?;
        std::fs::write(extension_dir.join("extension.toml"), manifest_content)?;

        // Create README.md
        let readme = format!(
            "# {} Extension\n\n{}\n\n## Installation\n\n```bash\nporters extension install {}\n```\n",
            name, manifest.description, name
        );
        std::fs::write(extension_dir.join("README.md"), readme)?;

        // Create example hook script
        let hooks_dir = extension_dir.join("hooks");
        std::fs::create_dir_all(&hooks_dir)?;

        let example_hook = "#!/bin/sh\necho \"Running custom hook from extension\"\n";
        std::fs::write(hooks_dir.join("example.sh"), example_hook)?;

        println!(
            "✅ Extension template created at {}",
            extension_dir.display()
        );
        println!("\n📝 Next steps:");
        println!("   1. Edit extension.toml to configure your extension");
        println!("   2. Add hook scripts in the hooks/ directory");
        println!(
            "   3. Test locally with: porters extension install --path {}",
            extension_dir.display()
        );
        println!("   4. Publish to crates.io when ready!");

        Ok(())
    }
}

/// Extension source
pub enum ExtensionSource {
    CratesIo,
    Git(String),
    Path(PathBuf),
}

/// Hook execution context
#[allow(dead_code)]
pub struct HookContext {
    pub project_dir: PathBuf,
    #[allow(dead_code)]
    pub args: Vec<String>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_extension_manifest() {
        let manifest = ExtensionManifest {
            name: "test-ext".to_string(),
            version: "0.1.0".to_string(),
            description: "Test extension".to_string(),
            authors: vec!["Test Author".to_string()],
            license: Some("MIT".to_string()),
            repository: None,
            homepage: None,
            hooks: ExtensionHooks::default(),
            commands: vec![],
        };

        assert_eq!(manifest.name, "test-ext");
    }
}
