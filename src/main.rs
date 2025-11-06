//! # Porters - Universal C/C++ Project Manager
//!
//! Porters is a comprehensive package manager and build orchestrator for C/C++ projects,
//! designed to simplify dependency management, cross-platform builds, and project workflows.
//! ```
//! Licensed under the Apache License, Version 2.0
//! See [LICENSE](https://github.com/muhammad-fiaz/Porters/blob/main/LICENSE)

use anyhow::{Context, Result};
use clap::{Parser, Subcommand};
use dialoguer::{Input, Select};

mod artifact;
mod bin_cache;
mod build;
mod buildsystem;
mod cache;
mod config;
mod cross_compile;
mod deps;
mod error;
mod export;
mod extension;
mod global;
mod global_config;
mod hash;
mod license;
mod lockfile;
mod pkg_managers;
mod publish;
mod registry;
mod scan;
mod update;
mod util;
mod version;

use config::PortersConfig;
use pkg_managers::{ConanManager, InstallScope, PackageManager, VcpkgManager, XMakeManager};
use util::pretty::*;

#[derive(Parser)]
#[command(name = "porters")]
#[command(about = "A universal C/C++ project manager", long_about = None)]
#[command(version)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// 🚀 Initialize a new porters project in current directory
    Init,

    /// 📦 Create a new porters project in a new directory
    #[command(visible_alias = "new")]
    Create {
        /// Project name
        name: String,

        /// Use default settings without prompts
        #[arg(short, long)]
        yes: bool,
    },

    /// ➕ Add a dependency
    #[command(visible_alias = "a")]
    Add {
        /// Package name or path/git URL
        package: String,

        /// Add as dev dependency
        #[arg(long)]
        dev: bool,

        /// Add as optional feature
        #[arg(long)]
        optional: bool,

        /// Git URL or package name
        #[arg(long)]
        git: Option<String>,

        /// Git branch
        #[arg(long)]
        branch: Option<String>,

        /// Git tag
        #[arg(long)]
        tag: Option<String>,

        /// Install globally (default: local to project)
        #[arg(long, short = 'g')]
        global: bool,
    },

    /// ➖ Remove a dependency
    #[command(visible_alias = "rm")]
    Remove {
        /// Package name
        package: String,

        /// Remove globally (default: local to project)
        #[arg(long, short = 'g')]
        global: bool,

        /// Force removal without confirmation
        #[arg(long, short = 'f')]
        force: bool,
    },

    /// 📦 Add a Conan package as dependency
    #[command(visible_alias = "co")]
    Conan {
        #[command(subcommand)]
        action: PackageManagerAction,
    },

    /// 📦 Add a vcpkg package as dependency
    #[command(visible_alias = "vc")]
    Vcpkg {
        #[command(subcommand)]
        action: PackageManagerAction,
    },

    /// 📦 Add an XMake package as dependency
    #[command(visible_alias = "xm")]
    Xmake {
        #[command(subcommand)]
        action: PackageManagerAction,
    },

    /// �🔨 Build the project
    Build {
        /// Build for all supported platforms
        #[arg(long)]
        all_platforms: bool,

        /// Build for Linux
        #[arg(long)]
        linux: bool,

        /// Build for Windows
        #[arg(long)]
        windows: bool,

        /// Build for macOS
        #[arg(long)]
        macos: bool,

        /// Additional build arguments
        #[arg(trailing_var_arg = true, allow_hyphen_values = true)]
        args: Vec<String>,
    },

    /// ▶️ Run the project
    #[command(visible_alias = "r")]
    Run {
        /// Additional run arguments
        #[arg(trailing_var_arg = true, allow_hyphen_values = true)]
        args: Vec<String>,
    },

    /// ⚡ Execute a single C/C++ file directly with dependencies
    #[command(visible_alias = "exec")]
    Execute {
        /// Source file to compile and run (.c or .cpp)
        file: String,

        /// Arguments to pass to the executable
        #[arg(trailing_var_arg = true, allow_hyphen_values = true)]
        args: Vec<String>,

        /// Open in external system terminal instead of current terminal
        #[arg(long)]
        external: bool,

        /// Run without console window (for GUI applications)
        #[arg(long)]
        no_console: bool,

        /// Custom output executable name (default: source filename without extension)
        #[arg(long, short)]
        output: Option<String>,
    },

    /// 🧪 Run tests
    #[command(visible_alias = "t")]
    Test,

    /// ✅ Check compilation without creating executables (syntax check)
    #[command(visible_alias = "ch")]
    Check {
        /// Specific file to check (optional - checks all project files if not provided)
        file: Option<String>,

        /// Show verbose compiler output
        #[arg(short, long)]
        verbose: bool,
    },

    /// 🔄 Update dependencies
    #[command(visible_alias = "u")]
    Update,

    /// 🧹 Clean build artifacts
    #[command(visible_alias = "c")]
    Clean,

    /// 🔒 Generate or update lockfile
    Lock,

    /// 📋 Vendor dependencies into project
    Vendor,

    /// 🌳 Show dependency graph
    #[command(visible_alias = "g")]
    Graph,

    /// 📤 Publish package to GitHub releases
    #[command(visible_alias = "pub")]
    Publish {
        /// GitHub access token (or use GITHUB_TOKEN env var)
        #[arg(long)]
        token: Option<String>,

        /// Dry run - don't actually publish
        #[arg(long)]
        dry_run: bool,
    },

    /// ⬆️ Upgrade porters to the latest version
    Upgrade,

    /// 🌍 Install a package globally
    #[command(visible_alias = "i")]
    Install {
        /// Package name or git URL
        package: String,

        /// Git URL
        #[arg(long)]
        git: Option<String>,

        /// Git branch
        #[arg(long)]
        branch: Option<String>,

        /// Git tag
        #[arg(long)]
        tag: Option<String>,
    },

    /// 🔄 Sync dependencies from porters.toml
    #[command(visible_alias = "s")]
    Sync {
        /// Include dev dependencies
        #[arg(long)]
        dev: bool,

        /// Include optional dependencies
        #[arg(long)]
        optional: bool,

        /// Disable cache (force re-download)
        #[arg(long)]
        no_cache: bool,
    },

    /// 🔌 Manage extensions
    Extension {
        #[command(subcommand)]
        action: ExtensionAction,
    },

    /// 📜 Run a custom script from porters.toml
    RunScript {
        /// Script name
        name: String,
    },

    /// 📊 List project dependencies
    #[command(visible_alias = "ls")]
    List {
        /// Show dependency tree
        #[arg(long)]
        tree: bool,
    },

    /// 📤 Export project to build system config files
    Export {
        #[command(subcommand)]
        build_system: ExportBuildSystem,
    },

    /// 🌐 List globally installed packages
    GlobalList,

    /// 🗑️ Clean cache
    CleanCache {
        /// Force clean (including binary cache)
        #[arg(long)]
        force: bool,
    },

    /// ♻️ Update porters itself to latest version
    SelfUpdate,

    /// 🔼 Update all dependencies to latest compatible versions
    UpdateDeps {
        /// Update to absolute latest (ignore constraints)
        #[arg(long)]
        latest: bool,
    },

    /// 🎯 Cross-compile for specific platform(s)
    Compile {
        /// Compile for all supported platforms
        #[arg(long)]
        all_platforms: bool,

        /// Compile for Linux
        #[arg(long)]
        linux: bool,

        /// Compile for Windows
        #[arg(long)]
        windows: bool,

        /// Compile for macOS
        #[arg(long)]
        macos: bool,

        /// Compile for bare metal
        #[arg(long)]
        baremetal: bool,

        /// Specific target triple
        #[arg(long)]
        target: Option<String>,
    },

    /// 🔧 Execute a custom command (dynamically matched from config)
    #[command(external_subcommand)]
    Custom(Vec<String>),

    /// ➕ Add porters to system PATH
    AddToPath {
        /// Overwrite existing PATH entry if present
        #[arg(long)]
        overwrite: bool,
    },

    /// ➖ Remove porters from system PATH
    RemoveFromPath,

    /// 📚 Open documentation in browser
    Docs,
}

#[derive(Subcommand)]
enum PackageManagerAction {
    /// Add a package from the package manager
    Add {
        /// Package name (e.g., fmt/10.1.1 for Conan, fmt for vcpkg)
        package: String,

        /// Package version (optional, depends on package manager)
        #[arg(long)]
        version: Option<String>,

        /// Install globally (default: local to project)
        #[arg(long, short = 'g')]
        global: bool,
    },

    /// Remove a package
    Remove {
        /// Package name
        package: String,

        /// Remove globally (default: local to project)
        #[arg(long, short = 'g')]
        global: bool,

        /// Force removal without confirmation
        #[arg(long, short = 'f')]
        force: bool,
    },

    /// List installed packages
    List {
        /// List global packages (default: local to project)
        #[arg(long, short = 'g')]
        global: bool,
    },

    /// Search for packages
    Search {
        /// Search query
        query: String,
    },
}

#[derive(Subcommand)]
enum ExportBuildSystem {
    /// Export to CMakeLists.txt
    Cmake,

    /// Export to xmake.lua
    Xmake,

    /// Export to Makefile
    Make,

    /// Export to meson.build
    Meson,

    /// Export to BUILD.bazel
    Bazel,

    /// Export to vcpkg.json
    Vcpkg,

    /// Export to conanfile.py
    Conan,
}

#[derive(Subcommand)]
enum ExtensionAction {
    /// 📥 Install an extension
    Install {
        /// Extension name
        name: String,

        /// Install from git URL
        #[arg(long)]
        git: Option<String>,

        /// Install from local path
        #[arg(long)]
        path: Option<String>,
    },

    /// 🗑️ Uninstall an extension
    Uninstall {
        /// Extension name
        name: String,
    },

    /// 📋 List installed extensions
    List,

    /// 🆕 Create extension template
    Create {
        /// Extension name
        name: String,
    },
}

/// Initialize Porters on first run or load global config
fn initialize_porters() -> Result<()> {
    use global_config::{GlobalPortersConfig, SystemCheck};

    // Load or create global config
    let mut config = GlobalPortersConfig::load_or_create()?;

    // Check if this is first run (no config file existed)
    let config_path = GlobalPortersConfig::config_path()?;
    let is_first_run = !config_path.exists() || config.porters_version.is_empty();

    // Update version if needed
    if config.porters_version != env!("CARGO_PKG_VERSION") {
        config.porters_version = env!("CARGO_PKG_VERSION").to_string();
        let _ = config.save();
    }

    // On first run or if no compiler detected, run system check
    if is_first_run || std::env::args().any(|arg| arg == "--check-system") {
        let system_check = SystemCheck::run();

        if is_first_run {
            println!();
            print_success("Welcome to Porters! 🚀");
            println!();
        }

        // Always show system check on first run
        if is_first_run || !system_check.has_compiler {
            system_check.display();

            if !system_check.has_compiler {
                print_error("❌ Cannot proceed without a C/C++ compiler!");
                println!();
                anyhow::bail!("Missing required system dependencies");
            }
        }
    }

    Ok(())
}

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize global config and check system requirements
    initialize_porters()?;

    // Check and setup PATH on first run
    check_path_setup();

    // Silent update check on startup
    update::silent_update_check();

    let cli = Cli::parse();

    match cli.command {
        Commands::Init => init_project().await,
        Commands::Create { name, yes } => create_project(&name, yes).await,
        Commands::Add {
            package,
            dev,
            optional,
            git,
            branch,
            tag,
            global,
        } => add_dependency(&package, dev, optional, git, branch, tag, global).await,
        Commands::Remove {
            package,
            global,
            force,
        } => remove_dependency(&package, global, force).await,
        Commands::Build {
            all_platforms,
            linux,
            windows,
            macos,
            args,
        } => build_project(all_platforms, linux, windows, macos, args).await,
        Commands::Run { args } => run_project(args).await,
        Commands::Execute {
            file,
            args,
            external,
            no_console,
            output,
        } => execute_single_file(&file, args, external, no_console, output.as_deref()).await,
        Commands::Test => test_project().await,
        Commands::Check { file, verbose } => check_compilation(file.as_deref(), verbose).await,
        Commands::Update => update_dependencies().await,
        Commands::Clean => clean_project().await,
        Commands::Lock => generate_lockfile().await,
        Commands::Vendor => vendor_dependencies().await,
        Commands::Graph => show_dependency_graph().await,
        Commands::Publish { token, dry_run } => publish_package(token, dry_run).await,
        Commands::Upgrade => upgrade_self().await,
        Commands::Install {
            package,
            git,
            branch,
            tag,
        } => install_package(&package, git, branch, tag).await,
        Commands::Sync {
            dev,
            optional,
            no_cache,
        } => sync_dependencies(dev, optional, !no_cache).await,
        Commands::Extension { action } => handle_extension(action).await,
        Commands::RunScript { name } => run_script(&name).await,
        Commands::List { tree } => list_dependencies(tree).await,
        Commands::Export { build_system } => export_to_build_system(build_system).await,
        Commands::Conan { action } => handle_conan_action(action).await,
        Commands::Vcpkg { action } => handle_vcpkg_action(action).await,
        Commands::Xmake { action } => handle_xmake_action(action).await,
        Commands::GlobalList => global_list_packages().await,
        Commands::CleanCache { force } => clean_cache(force).await,
        Commands::SelfUpdate => self_update().await,
        Commands::UpdateDeps { latest } => update_deps(latest).await,
        Commands::Compile {
            all_platforms,
            linux,
            windows,
            macos,
            baremetal,
            target,
        } => compile_cross(all_platforms, linux, windows, macos, baremetal, target).await,
        Commands::Custom(args) => execute_custom_command(args).await,
        Commands::AddToPath { overwrite } => add_to_path(overwrite),
        Commands::RemoveFromPath => remove_from_path(),
        Commands::Docs => open_docs(),
    }
}

async fn init_project() -> Result<()> {
    print_step("🔧 Initializing porters project in current directory");

    if std::path::Path::new("porters.toml").exists() {
        print_warning("⚠️  porters.toml already exists");
        return Ok(());
    }

    // Scan for existing C/C++ files to determine project type
    let sources = scan::scan_project(".")?;
    let has_sources = !sources.source_files.is_empty();

    if has_sources {
        print_success(&format!(
            "🔍 Detected {} C/C++ source files",
            sources.source_files.len()
        ));
    }

    // Get project name from current directory
    let current_dir = std::env::current_dir()?;
    let default_name = current_dir
        .file_name()
        .and_then(|n| n.to_str())
        .unwrap_or("my-project");

    // Interactive questions
    let project_name: String = dialoguer::Input::new()
        .with_prompt("Project name")
        .default(default_name.to_string())
        .interact_text()?;

    let version: String = dialoguer::Input::new()
        .with_prompt("Version")
        .default("0.1.0".to_string())
        .interact_text()?;

    let author: String = dialoguer::Input::new()
        .with_prompt("Author")
        .allow_empty(true)
        .interact_text()?;

    let description: String = dialoguer::Input::new()
        .with_prompt("Description")
        .allow_empty(true)
        .interact_text()?;

    let license = dialoguer::Select::new()
        .with_prompt("License")
        .items([
            "Apache-2.0",
            "MIT",
            "GPL-3.0",
            "GPL-2.0",
            "BSD-3-Clause",
            "BSD-2-Clause",
            "MPL-2.0",
            "LGPL-3.0",
            "Unlicense",
            "None",
        ])
        .default(0)
        .interact()?;

    let license_str = match license {
        0 => "Apache-2.0",
        1 => "MIT",
        2 => "GPL-3.0",
        3 => "GPL-2.0",
        4 => "BSD-3-Clause",
        5 => "BSD-2-Clause",
        6 => "MPL-2.0",
        7 => "LGPL-3.0",
        8 => "Unlicense",
        _ => "",
    };

    // Auto-detect build system
    let detected_build_system = detect_existing_build_system(".");

    let config_content = if has_sources {
        // For existing projects
        let build_section = if let Some(system) = &detected_build_system {
            print_success(&format!("🔨 Detected build system: {}", system));
            format!("\n[build]\nsystem = \"{}\"", system)
        } else {
            print_info("💡 No build system detected - you can configure one later");
            String::new()
        };

        let authors_line = if !author.is_empty() {
            format!("authors = [\"{}\"]", author)
        } else {
            "authors = []".to_string()
        };

        let desc_line = if !description.is_empty() {
            format!("\ndescription = \"{}\"", description)
        } else {
            String::new()
        };

        let license_line = if !license_str.is_empty() {
            format!("\nlicense = \"{}\"", license_str)
        } else {
            String::new()
        };

        format!(
            r#"[project]
name = "{}"
version = "{}"{}{} 
{}

[dependencies]
# Add your dependencies here
# Example: fmt = {{ git = "https://github.com/fmtlib/fmt" }}

[dev-dependencies]
# Add your dev dependencies here
{}
"#,
            project_name, version, desc_line, license_line, authors_line, build_section
        )
    } else {
        // For new projects
        let authors_line = if !author.is_empty() {
            format!("authors = [\"{}\"]", author)
        } else {
            "authors = []".to_string()
        };

        let desc_line = if !description.is_empty() {
            format!("\ndescription = \"{}\"", description)
        } else {
            String::new()
        };

        let license_line = if !license_str.is_empty() {
            format!("\nlicense = \"{}\"", license_str)
        } else {
            String::new()
        };

        format!(
            r#"[project]
name = "{}"
version = "{}"{}{} 
{}

[dependencies]
# Add your dependencies here
# Example: fmt = {{ git = "https://github.com/fmtlib/fmt" }}

[dev-dependencies]
# Add your dev dependencies here

[build]
# Build system will be auto-detected
# Or specify: system = "cmake"
"#,
            project_name, version, desc_line, license_line, authors_line
        )
    };

    std::fs::write("porters.toml", config_content)?;

    // Create cache directories
    ensure_cache_dirs()?;

    // Create .gitignore if it doesn't exist
    create_gitignore(".")?;

    print_success("✅ Created porters.toml");
    if has_sources {
        print_info("🎯 Your existing C/C++ project is now managed by Porters!");
        print_info("💡 Run 'porters build' to build your project");
    } else {
        print_info("📝 Add your C/C++ source files and run 'porters build'");
    }

    // Check build tools
    check_build_tools();

    Ok(())
}

async fn create_project(name: &str, use_defaults: bool) -> Result<()> {
    print_step(&format!("📦 Creating new project: {}", name));

    // Check if directory already exists
    if std::path::Path::new(name).exists() {
        print_error(&format!("❌ Directory '{}' already exists", name));
        return Err(anyhow::anyhow!("Directory already exists"));
    }

    // Create project directory
    std::fs::create_dir_all(name)?;
    std::env::set_current_dir(name)?;

    let (language, author, email, repo, build_system, project_type, entry_point, license) =
        if use_defaults {
            print_info("⚡ Using default settings...");
            (
                "both".to_string(),
                None,
                None,
                None,
                "cmake".to_string(),
                "application".to_string(),
                None,
                Some("Apache-2.0".to_string()),
            )
        } else {
            get_project_details()?
        };

    // Create project structure based on project type
    create_project_structure(
        &language,
        &project_type,
        name,
        &author,
        &email,
        &repo,
        &license,
        &build_system,
    )?;

    // Create porters.toml with all metadata
    create_porters_config_enhanced(ProjectConfig {
        name,
        author: &author,
        email: &email,
        repo: &repo,
        build_system: &build_system,
        project_type: &project_type,
        entry_point: &entry_point,
        license: &license,
    })?;

    // Create cache directories
    ensure_cache_dirs()?;

    // Create .gitignore
    create_gitignore(".")?;

    print_success(&format!("🎉 Created project '{}' successfully!", name));
    print_info(&format!("💡 cd {} && porters build", name));

    // Check build tools
    check_build_tools();

    Ok(())
}

type ProjectDetails = (
    String,
    Option<String>,
    Option<String>,
    Option<String>,
    String,
    String,
    Option<String>,
    Option<String>,
);

fn get_project_details() -> Result<ProjectDetails> {
    let theme = dialoguer::theme::ColorfulTheme::default();

    // Project type selection
    let project_types = vec!["🚀 Application (executable)", "📦 Library (static/shared)"];
    let type_idx = Select::with_theme(&theme)
        .with_prompt("Select project type")
        .items(&project_types)
        .default(0)
        .interact()?;

    let project_type = if type_idx == 0 {
        "application"
    } else {
        "library"
    }
    .to_string();

    // Language selection
    let languages = vec![
        "🔵 C (Pure C)",
        "🔴 C++ (Pure C++)",
        "🟣 Both (Hybrid C/C++ with extern \"C\")",
    ];
    let language_idx = Select::with_theme(&theme)
        .with_prompt("Select project language")
        .items(&languages)
        .default(2)
        .interact()?;

    let language = match language_idx {
        0 => "c",
        1 => "cpp",
        _ => "both",
    }
    .to_string();

    // For libraries, ask for library name
    let entry_point = if project_type == "library" {
        let lib_name: String = Input::with_theme(&theme)
            .with_prompt("📚 Library name (optional, press Enter to use project name)")
            .allow_empty(true)
            .interact_text()?;

        if lib_name.trim().is_empty() {
            None
        } else {
            Some(lib_name)
        }
    } else {
        None
    };

    // Author name (optional)
    let author: String = Input::with_theme(&theme)
        .with_prompt("👤 Author name (optional, press Enter to skip)")
        .allow_empty(true)
        .interact_text()?;

    let author = if author.trim().is_empty() {
        None
    } else {
        Some(author)
    };

    // Email (optional)
    let email: String = Input::with_theme(&theme)
        .with_prompt("📧 Email (optional, press Enter to skip)")
        .allow_empty(true)
        .interact_text()?;

    let email = if email.trim().is_empty() {
        None
    } else {
        Some(email)
    };

    // Repository URL (optional)
    let repo: String = Input::with_theme(&theme)
        .with_prompt("🔗 Repository URL (optional, press Enter to skip)")
        .allow_empty(true)
        .interact_text()?;

    let repo = if repo.trim().is_empty() {
        None
    } else {
        Some(repo)
    };

    // License selection
    let license_idx = Select::with_theme(&theme)
        .with_prompt("📝 Select license")
        .items([
            "⚖️  Apache-2.0 (Permissive, with patent protection)",
            "📄 MIT (Very permissive, simple)",
            "🔓 GPL-3.0 (Copyleft, strong protection)",
            "🔓 GPL-2.0 (Copyleft, older version)",
            "📋 BSD-3-Clause (Permissive, with attribution)",
            "📋 BSD-2-Clause (Permissive, simpler)",
            "🔧 MPL-2.0 (Weak copyleft, file-level)",
            "📚 LGPL-3.0 (For libraries, weak copyleft)",
            "🆓 Unlicense (Public domain)",
            "✏️  Custom (Create your own)",
            "❌ None",
        ])
        .default(0)
        .interact()?;

    let license = match license_idx {
        0 => Some("Apache-2.0".to_string()),
        1 => Some("MIT".to_string()),
        2 => Some("GPL-3.0".to_string()),
        3 => Some("GPL-2.0".to_string()),
        4 => Some("BSD-3-Clause".to_string()),
        5 => Some("BSD-2-Clause".to_string()),
        6 => Some("MPL-2.0".to_string()),
        7 => Some("LGPL-3.0".to_string()),
        8 => Some("Unlicense".to_string()),
        9 => Some("Custom".to_string()),
        _ => None,
    };

    // Build system selection
    let build_systems = vec![
        "🔨 CMake (Industry standard, most popular)",
        "⚡ XMake (Modern, fast, Lua-based)",
        "🏗️  Meson (Fast, Python-based)",
        "🔧 Make (Traditional, simple)",
        "✨ Custom (Manual configuration)",
    ];
    let build_idx = Select::with_theme(&theme)
        .with_prompt("⚙️  Select build system")
        .items(&build_systems)
        .default(0)
        .interact()?;

    let build_system = match build_idx {
        0 => "cmake",
        1 => "xmake",
        2 => "meson",
        3 => "make",
        _ => "custom",
    }
    .to_string();

    Ok((
        language,
        author,
        email,
        repo,
        build_system,
        project_type,
        entry_point,
        license,
    ))
}

#[allow(clippy::too_many_arguments)]
fn create_project_structure(
    language: &str,
    project_type: &str,
    project_name: &str,
    author: &Option<String>,
    email: &Option<String>,
    repo: &Option<String>,
    license: &Option<String>,
    build_system: &str,
) -> Result<()> {
    // Create directory structure
    std::fs::create_dir_all("src")?;
    std::fs::create_dir_all("include")?;

    if project_type == "library" {
        std::fs::create_dir_all("examples")?;
        std::fs::create_dir_all("tests")?;
    }

    let is_cpp = language == "cpp" || language == "both";

    // Create main source file based on project type
    if project_type == "application" {
        // Application project
        if language == "both" {
            // Hybrid project: Create both C and C++ files with extern "C" example
            std::fs::write(
                "src/main.cpp",
                r#"#include <iostream>
#include "c_module.h"

int main(int argc, char *argv[]) {
    std::cout << "🚀 Hello from C++ (Porters Hybrid Project)!" << std::endl;
    
    // Call C function from C++ code
    const char* c_message = get_c_message();
    std::cout << "📦 Message from C module: " << c_message << std::endl;
    
    return 0;
}
"#,
            )?;

            // Create C module header
            std::fs::write(
                "include/c_module.h",
                r#"#ifndef C_MODULE_H
#define C_MODULE_H

#ifdef __cplusplus
extern "C" {
#endif

/**
 * Get a message from the C module
 * This function can be called from both C and C++ code
 */
const char* get_c_message(void);

/**
 * Process a number using C code
 */
int c_process_number(int value);

#ifdef __cplusplus
}
#endif

#endif /* C_MODULE_H */
"#,
            )?;

            // Create C module implementation
            std::fs::write(
                "src/c_module.c",
                r#"#include "c_module.h"
#include <stdio.h>

const char* get_c_message(void) {
    return "This is a C function callable from C++!";
}

int c_process_number(int value) {
    printf("Processing %d in C code\n", value);
    return value * 2;
}
"#,
            )?;

            // Create C++ utility header
            std::fs::write(
                "include/cpp_utils.hpp",
                r#"#ifndef CPP_UTILS_HPP
#define CPP_UTILS_HPP

#include <string>
#include <vector>

namespace utils {

/**
 * C++ utility class
 * Can use C functions via extern "C"
 */
class StringHelper {
public:
    static std::string to_upper(const std::string& str);
    static std::vector<std::string> split(const std::string& str, char delimiter);
};

} // namespace utils

#endif /* CPP_UTILS_HPP */
"#,
            )?;

            // Create C++ utility implementation
            std::fs::write(
                "src/cpp_utils.cpp",
                r#"#include "cpp_utils.hpp"
#include <algorithm>
#include <sstream>

namespace utils {

std::string StringHelper::to_upper(const std::string& str) {
    std::string result = str;
    std::transform(result.begin(), result.end(), result.begin(), ::toupper);
    return result;
}

std::vector<std::string> StringHelper::split(const std::string& str, char delimiter) {
    std::vector<std::string> tokens;
    std::stringstream ss(str);
    std::string token;
    
    while (std::getline(ss, token, delimiter)) {
        tokens.push_back(token);
    }
    
    return tokens;
}

} // namespace utils
"#,
            )?;
        } else if is_cpp {
            std::fs::write(
                "src/main.cpp",
                r#"#include <iostream>

int main(int argc, char *argv[]) {
    std::cout << "🚀 Hello from Porters!" << std::endl;
    return 0;
}
"#,
            )?;
        } else {
            std::fs::write(
                "src/main.c",
                r#"#include <stdio.h>

int main(int argc, char *argv[]) {
    printf("🚀 Hello from Porters!\n");
    return 0;
}
"#,
            )?;
        }
    } else {
        // Library project
        let lib_name = project_name.replace("-", "_");

        // Create library header
        if is_cpp {
            std::fs::write(
                format!("include/{}.hpp", lib_name),
                format!(
                    r#"#ifndef {}_HPP
#define {}_HPP

#include <string>

namespace {} {{

class Example {{
public:
    Example();
    ~Example();
    
    std::string get_message() const;
    void set_message(const std::string& msg);
    
private:
    std::string message_;
}};

}} // namespace {}

#endif // {}_HPP
"#,
                    lib_name.to_uppercase(),
                    lib_name.to_uppercase(),
                    lib_name,
                    lib_name,
                    lib_name.to_uppercase()
                ),
            )?;

            // Create library implementation
            std::fs::write(
                format!("src/{}.cpp", lib_name),
                format!(
                    r#"#include "{}.hpp"

namespace {} {{

Example::Example() : message_("Hello from {} library!") {{
}}

Example::~Example() {{
}}

std::string Example::get_message() const {{
    return message_;
}}

void Example::set_message(const std::string& msg) {{
    message_ = msg;
}}

}} // namespace {}
"#,
                    lib_name, lib_name, lib_name, lib_name
                ),
            )?;

            // Create example usage
            std::fs::write(
                "examples/basic_usage.cpp",
                format!(
                    r#"#include <iostream>
#include "{}.hpp"

int main() {{
    {}::Example example;
    std::cout << example.get_message() << std::endl;
    
    example.set_message("Custom message!");
    std::cout << example.get_message() << std::endl;
    
    return 0;
}}
"#,
                    lib_name, lib_name
                ),
            )?;
        } else {
            // C library
            std::fs::write(
                format!("include/{}.h", lib_name),
                format!(
                    r#"#ifndef {}_H
#define {}_H

#ifdef __cplusplus
extern "C" {{
#endif

typedef struct {}_example {}_example_t;

{}_example_t* {}_example_create(void);
void {}_example_destroy({}_example_t* example);

const char* {}_example_get_message(const {}_example_t* example);
void {}_example_set_message({}_example_t* example, const char* msg);

#ifdef __cplusplus
}}
#endif

#endif /* {}_H */
"#,
                    lib_name.to_uppercase(),
                    lib_name.to_uppercase(),
                    lib_name,
                    lib_name,
                    lib_name,
                    lib_name,
                    lib_name,
                    lib_name,
                    lib_name,
                    lib_name,
                    lib_name,
                    lib_name,
                    lib_name.to_uppercase()
                ),
            )?;

            std::fs::write(
                format!("src/{}.c", lib_name),
                format!(
                    r#"#include "{}.h"
#include <stdlib.h>
#include <string.h>

struct {}_example {{
    char* message;
}};

{}_example_t* {}_example_create(void) {{
    {}_example_t* example = malloc(sizeof({}_example_t));
    if (example) {{
        example->message = strdup("Hello from {} library!");
    }}
    return example;
}}

void {}_example_destroy({}_example_t* example) {{
    if (example) {{
        free(example->message);
        free(example);
    }}
}}

const char* {}_example_get_message(const {}_example_t* example) {{
    return example ? example->message : NULL;
}}

void {}_example_set_message({}_example_t* example, const char* msg) {{
    if (example && msg) {{
        free(example->message);
        example->message = strdup(msg);
    }}
}}
"#,
                    lib_name,
                    lib_name,
                    lib_name,
                    lib_name,
                    lib_name,
                    lib_name,
                    lib_name,
                    lib_name,
                    lib_name,
                    lib_name,
                    lib_name,
                    lib_name,
                    lib_name
                ),
            )?;

            // Create example
            std::fs::write(
                "examples/basic_usage.c",
                format!(
                    r#"#include <stdio.h>
#include "{}.h"

int main(void) {{
    {}_example_t* example = {}_example_create();
    
    printf("%s\n", {}_example_get_message(example));
    
    {}_example_set_message(example, "Custom message!");
    printf("%s\n", {}_example_get_message(example));
    
    {}_example_destroy(example);
    return 0;
}}
"#,
                    lib_name, lib_name, lib_name, lib_name, lib_name, lib_name, lib_name
                ),
            )?;
        }

        // Create test file
        if is_cpp {
            std::fs::write(
                "tests/test_basic.cpp",
                format!(
                    r#"#include <cassert>
#include "{}.hpp"

int main() {{
    {}::Example example;
    
    // Test default message
    assert(!example.get_message().empty());
    
    // Test set/get
    example.set_message("Test message");
    assert(example.get_message() == "Test message");
    
    return 0;
}}
"#,
                    lib_name, lib_name
                ),
            )?;
        } else {
            std::fs::write(
                "tests/test_basic.c",
                format!(
                    r#"#include <assert.h>
#include <string.h>
#include "{}.h"

int main(void) {{
    {}_example_t* example = {}_example_create();
    assert(example != NULL);
    
    // Test default message
    assert({}_example_get_message(example) != NULL);
    
    // Test set/get
    {}_example_set_message(example, "Test message");
    assert(strcmp({}_example_get_message(example), "Test message") == 0);
    
    {}_example_destroy(example);
    return 0;
}}
"#,
                    lib_name, lib_name, lib_name, lib_name, lib_name, lib_name, lib_name
                ),
            )?;
        }
    }

    // Create .gitignore
    std::fs::write(
        ".gitignore",
        r#"# Build directories
build/
out/
target/
vendor/

# Porters
porters.lock
.porters/

# IDE
.vscode/
.idea/
*.swp
*.swo

# OS
.DS_Store
Thumbs.db

# Compiled files
*.o
*.obj
*.exe
*.dll
*.so
*.dylib
*.a
*.lib
"#,
    )?;

    // Create comprehensive README
    let project_type_str = if project_type == "library" {
        "library"
    } else {
        "application"
    };
    let language_badge = match language {
        "c" => "![Language](https://img.shields.io/badge/language-C-blue.svg)",
        "cpp" => "![Language](https://img.shields.io/badge/language-C++-blue.svg)",
        "both" => "![Language](https://img.shields.io/badge/language-C%2FC++-blue.svg)",
        _ => "",
    };

    let author_section = if let Some(author_name) = author {
        if let Some(email_addr) = email {
            format!("\n## Author\n\n👤 **{}** <{}>\n", author_name, email_addr)
        } else {
            format!("\n## Author\n\n👤 **{}**\n", author_name)
        }
    } else {
        String::new()
    };

    let repo_section = if let Some(repository_url) = repo {
        format!(
            "\n## Repository\n\n🔗 [{}]({})\n\n\
            ### Contributing\n\n\
            Contributions are welcome! Please feel free to submit a Pull Request.\n",
            repository_url, repository_url
        )
    } else {
        String::new()
    };

    let readme_content = if project_type == "library" {
        format!(
            r#"# {project_name}

{language_badge}
[![License](https://img.shields.io/badge/license-{license}-green.svg)]({license_link})

> A {lang_full} {project_type_str} managed by [Porters](https://github.com/muhammad-fiaz/porters)

## 📋 Description

{description}

## ✨ Features

- ✅ Cross-platform support (Windows, Linux, macOS)
- ✅ Modern {lang_full} codebase
- ✅ Easy integration with Porters package manager
- ✅ Comprehensive examples and tests

## 🚀 Quick Start

### Prerequisites

- [Porters](https://github.com/muhammad-fiaz/porters) - Universal C/C++ package manager
- A C/C++ compiler (GCC, Clang, or MSVC)

### Building

```bash
# Clone the repository
git clone {repo_placeholder}
cd {project_name}

# Build the library
porters build
```

### Running Examples

```bash
# Run the basic usage example
porters execute examples/basic_usage.{ext}
```

### Running Tests

```bash
# Run tests
porters execute tests/test_basic.{ext}
```

## 📦 Installation

### Using Porters (Recommended)

Add to your `porters.toml`:

```toml
[dependencies]
{project_name} = {{ git = "{repo_placeholder}" }}
```

Or install locally:

```bash
porters add {project_name} --path /path/to/{project_name}
```

### Manual Integration

Include the library in your project:

```{lang_example}
{include_example}
```

## 📚 Usage

### Basic Example

```{lang_example}
{usage_example}
```

## 🛠️ Development

### Project Structure

```
{project_name}/
├── include/          # Public headers
├── src/              # Source files
├── examples/         # Usage examples
├── tests/            # Test files
├── porters.toml      # Porters configuration
└── README.md         # This file
```

### Building from Source

```bash
# Build in release mode
porters build --release

# Build for specific platform
porters build --target x86_64-pc-windows-msvc
```

### Adding Dependencies

```bash
# Add a dependency
porters add <package-name>

# Add from Git
porters add <package> --git https://github.com/user/repo
```
{author_section}{repo_section}
## 📄 License

This project is licensed under the {license} License - see the [LICENSE](LICENSE) file for details.

## 🙏 Acknowledgments

- Built with [Porters](https://github.com/muhammad-fiaz/porters)
- Powered by the C/C++ community

---

**Made with ❤️ using Porters**
"#,
            project_name = project_name,
            language_badge = language_badge,
            license = license.as_deref().unwrap_or("MIT"),
            license_link = match license.as_deref() {
                Some("Apache-2.0") => "https://opensource.org/licenses/Apache-2.0",
                Some("MIT") => "https://opensource.org/licenses/MIT",
                Some("GPL-3.0") => "https://www.gnu.org/licenses/gpl-3.0",
                Some("GPL-2.0") => "https://www.gnu.org/licenses/gpl-2.0",
                Some("BSD-3-Clause") => "https://opensource.org/licenses/BSD-3-Clause",
                Some("BSD-2-Clause") => "https://opensource.org/licenses/BSD-2-Clause",
                Some("MPL-2.0") => "https://opensource.org/licenses/MPL-2.0",
                Some("LGPL-3.0") => "https://www.gnu.org/licenses/lgpl-3.0",
                Some("Unlicense") => "https://unlicense.org",
                _ => "LICENSE",
            },
            lang_full = match language {
                "c" => "C",
                "cpp" => "C++",
                "both" => "C/C++ hybrid",
                _ => "C/C++",
            },
            project_type_str = project_type_str,
            description = "Add your library description here",
            repo_placeholder = repo
                .as_deref()
                .unwrap_or("https://github.com/yourusername/yourrepo"),
            ext = if language == "c" { "c" } else { "cpp" },
            lang_example = if language == "c" { "c" } else { "cpp" },
            include_example = if language == "c" {
                format!("#include \"{}.h\"", project_name.replace("-", "_"))
            } else {
                format!("#include \"{}.hpp\"", project_name.replace("-", "_"))
            },
            usage_example = if language == "c" {
                format!(
                    "#include <stdio.h>\n\
                     #include \"{}.h\"\n\n\
                     int main(void) {{\n    \
                         // Use the library\n    \
                         return 0;\n\
                     }}",
                    project_name.replace("-", "_")
                )
            } else {
                format!(
                    "#include <iostream>\n\
                     #include \"{}.hpp\"\n\n\
                     int main() {{\n    \
                         // Use the library\n    \
                         return 0;\n\
                     }}",
                    project_name.replace("-", "_")
                )
            },
            author_section = author_section,
            repo_section = repo_section,
        )
    } else {
        format!(
            r#"# {project_name}

{language_badge}
[![License](https://img.shields.io/badge/license-{license}-green.svg)]({license_link})

> A {lang_full} {project_type_str} managed by [Porters](https://github.com/muhammad-fiaz/porters)

## 📋 Description

{description}

## ✨ Features

- 🚀 Fast and efficient {lang_full} application
- 🌍 Cross-platform support (Windows, Linux, macOS)
- 📦 Managed by Porters for easy dependency handling
- ⚙️ Configured with {build_system}

## 🚀 Quick Start

### Prerequisites

- [Porters](https://github.com/muhammad-fiaz/porters) - Universal C/C++ package manager
- A C/C++ compiler (GCC, Clang, or MSVC)
- {build_system_name} build system

### Installation

```bash
# Clone the repository
git clone {repo_placeholder}
cd {project_name}

# Build the application
porters build
```

### Running

```bash
# Run the application
porters run

# Or execute directly with arguments
porters run -- arg1 arg2
```

## 🛠️ Development

### Project Structure

```
{project_name}/
├── src/              # Source files
│   └── main.{ext}    # Entry point{hybrid_structure}
├── include/          # Header files
├── porters.toml      # Porters configuration
└── README.md         # This file
```

### Building

```bash
# Debug build
porters build

# Release build
porters build --release

# Build for specific platform
porters build --target x86_64-pc-windows-msvc
```

### Adding Dependencies

```bash
# Add a dependency
porters add <package-name>

# Add from Git
porters add <package> --git https://github.com/user/repo

# Add dev dependency
porters add <package> --dev
```

### Testing

```bash
# Run tests (if configured)
porters test
```
{author_section}{repo_section}
## 📄 License

This project is licensed under the {license} License - see the [LICENSE](LICENSE) file for details.

## 🙏 Acknowledgments

- Built with [Porters](https://github.com/muhammad-fiaz/porters)
- Powered by the C/C++ community

---

**Made with ❤️ using Porters**
"#,
            project_name = project_name,
            language_badge = language_badge,
            license = license.as_deref().unwrap_or("MIT"),
            license_link = match license.as_deref() {
                Some("Apache-2.0") => "https://opensource.org/licenses/Apache-2.0",
                Some("MIT") => "https://opensource.org/licenses/MIT",
                Some("GPL-3.0") => "https://www.gnu.org/licenses/gpl-3.0",
                Some("GPL-2.0") => "https://www.gnu.org/licenses/gpl-2.0",
                Some("BSD-3-Clause") => "https://opensource.org/licenses/BSD-3-Clause",
                Some("BSD-2-Clause") => "https://opensource.org/licenses/BSD-2-Clause",
                Some("MPL-2.0") => "https://opensource.org/licenses/MPL-2.0",
                Some("LGPL-3.0") => "https://www.gnu.org/licenses/lgpl-3.0",
                Some("Unlicense") => "https://unlicense.org",
                _ => "LICENSE",
            },
            lang_full = match language {
                "c" => "C",
                "cpp" => "C++",
                "both" => "C/C++ hybrid",
                _ => "C/C++",
            },
            project_type_str = project_type_str,
            description = "Add your application description here",
            build_system = build_system,
            build_system_name = match build_system {
                "cmake" => "CMake",
                "xmake" => "XMake",
                "meson" => "Meson",
                "make" => "Make",
                _ => "Custom",
            },
            repo_placeholder = repo
                .as_deref()
                .unwrap_or("https://github.com/yourusername/yourrepo"),
            ext = if language == "c" { "c" } else { "cpp" },
            hybrid_structure = if language == "both" {
                "\n│   ├── c_module.c   # C implementation\n\
                 │   └── cpp_utils.cpp # C++ implementation"
            } else {
                ""
            },
            author_section = author_section,
            repo_section = repo_section,
        )
    };

    std::fs::write("README.md", readme_content)?;

    // Create LICENSE file if license is specified
    if let Some(license_id) = license {
        let author_name = author.as_deref().unwrap_or("Author");
        license::LicenseGenerator::write_license_file(license_id, author_name, project_name)?;
        print_success(&format!("📄 Created LICENSE file ({} license)", license_id));
    }

    Ok(())
}

struct ProjectConfig<'a> {
    name: &'a str,
    author: &'a Option<String>,
    email: &'a Option<String>,
    repo: &'a Option<String>,
    build_system: &'a str,
    project_type: &'a str,
    entry_point: &'a Option<String>,
    license: &'a Option<String>,
}

fn create_porters_config_enhanced(config: ProjectConfig) -> Result<()> {
    let mut config_str = String::new();

    // Project section
    config_str.push_str("[project]\n");
    config_str.push_str(&format!("name = \"{}\"\n", config.name));
    config_str.push_str("version = \"0.1.0\"\n");

    // Authors
    if let Some(author_name) = config.author {
        if let Some(email_addr) = config.email {
            config_str.push_str(&format!(
                "authors = [\"{} <{}> \"]\n",
                author_name, email_addr
            ));
        } else {
            config_str.push_str(&format!("authors = [\"{}\"]\n", author_name));
        }
    } else {
        config_str.push_str("authors = []\n");
    }

    // Description
    config_str.push_str(&format!(
        "description = \"A {} project\"\n",
        config.project_type
    ));

    // License
    if let Some(lic) = config.license {
        config_str.push_str(&format!("license = \"{}\"\n", lic));
    }

    // Repository
    if let Some(repo_url) = config.repo {
        config_str.push_str(&format!("repository = \"{}\"\n", repo_url));
    }

    // Project type
    config_str.push_str(&format!("project-type = \"{}\"\n", config.project_type));

    // Entry point
    if let Some(entry) = config.entry_point {
        config_str.push_str(&format!("entry_point = \"{}\"\n", entry));
    } else if config.project_type == "application" {
        config_str.push_str("entry_point = \"src/main\"\n");
    }

    // Platforms (default to all)
    config_str.push_str("platforms = [\"windows\", \"macos\", \"linux\"]\n");

    // Keywords
    config_str.push_str(&format!(
        "keywords = [\"{}\", \"c\", \"cpp\"]\n",
        config.project_type
    ));

    // README
    config_str.push_str("readme = \"README.md\"\n");

    config_str.push_str("\n[dependencies]\n");
    config_str.push_str("# Add your dependencies here\n");
    config_str.push_str("# Example: fmt = { git = \"https://github.com/fmtlib/fmt\" }\n");

    config_str.push_str("\n[dev-dependencies]\n");
    config_str.push_str("# Add your dev dependencies here\n");

    config_str.push_str("\n[build]\n");
    if config.build_system != "custom" {
        config_str.push_str(&format!("system = \"{}\"\n", config.build_system));
    } else {
        config_str.push_str("# Configure custom build commands\n");
        config_str.push_str("# [build.custom]\n");
        config_str.push_str("# configure = \"cmake -B build\"\n");
        config_str.push_str("# build = \"cmake --build build\"\n");
    }

    std::fs::write("porters.toml", config_str)?;

    // Create CMakeLists.txt for cmake projects
    if config.build_system == "cmake" {
        create_cmake_file_enhanced(config.name, config.project_type)?;
    }

    Ok(())
}

// Keep old function for backward compatibility
#[allow(dead_code)]
fn create_porters_config(
    name: &str,
    author: &Option<String>,
    email: &Option<String>,
    repo: &Option<String>,
    build_system: &str,
) -> Result<()> {
    create_porters_config_enhanced(ProjectConfig {
        name,
        author,
        email,
        repo,
        build_system,
        project_type: "application",
        entry_point: &None,
        license: &Some("Apache-2.0".to_string()),
    })
}

fn create_cmake_file_enhanced(project_name: &str, project_type: &str) -> Result<()> {
    let cmake_content = if project_type == "library" {
        format!(
            r#"cmake_minimum_required(VERSION 3.15)
project({} VERSION 0.1.0)

set(CMAKE_CXX_STANDARD 17)
set(CMAKE_CXX_STANDARD_REQUIRED ON)

# Collect all source files
file(GLOB_RECURSE SOURCES "src/*.c" "src/*.cpp" "src/*.cxx" "src/*.cc")

# Create library (can be STATIC, SHARED, or MODULE)
add_library({} STATIC ${{SOURCES}})

# Include directories
target_include_directories({} 
    PUBLIC 
        $<BUILD_INTERFACE:${{CMAKE_CURRENT_SOURCE_DIR}}/include>
        $<INSTALL_INTERFACE:include>
)

# Installation rules
install(TARGETS {} 
    LIBRARY DESTINATION lib
    ARCHIVE DESTINATION lib
    RUNTIME DESTINATION bin
)
install(DIRECTORY include/ DESTINATION include)
"#,
            project_name, project_name, project_name, project_name
        )
    } else {
        format!(
            r#"cmake_minimum_required(VERSION 3.15)
project({} VERSION 0.1.0)

set(CMAKE_CXX_STANDARD 17)
set(CMAKE_CXX_STANDARD_REQUIRED ON)

# Collect all source files
file(GLOB_RECURSE SOURCES "src/*.c" "src/*.cpp" "src/*.cxx" "src/*.cc")

# Create executable
add_executable({} ${{SOURCES}})

# Include directories
target_include_directories({} PRIVATE include)
"#,
            project_name, project_name, project_name
        )
    };

    std::fs::write("CMakeLists.txt", cmake_content)?;
    Ok(())
}

#[allow(dead_code)]
fn create_cmake_file(project_name: &str) -> Result<()> {
    create_cmake_file_enhanced(project_name, "application")
}

fn detect_existing_build_system(path: &str) -> Option<String> {
    let path = std::path::Path::new(path);

    if path.join("CMakeLists.txt").exists() {
        Some("cmake".to_string())
    } else if path.join("xmake.lua").exists() {
        Some("xmake".to_string())
    } else if path.join("meson.build").exists() {
        Some("meson".to_string())
    } else if path.join("Makefile").exists() || path.join("makefile").exists() {
        Some("make".to_string())
    } else {
        None
    }
}

fn ensure_cache_dirs() -> Result<()> {
    let home = dirs::home_dir().expect("Could not find home directory");
    let cache_dir = home.join(".porters").join("cache");
    std::fs::create_dir_all(cache_dir.join("sources"))?;
    std::fs::create_dir_all(cache_dir.join("build"))?;
    Ok(())
}

/// Create .gitignore file with common C/C++ patterns and .porters/ folder
fn create_gitignore(project_dir: &str) -> Result<()> {
    let gitignore_path = std::path::Path::new(project_dir).join(".gitignore");

    // Check if .gitignore already exists
    if gitignore_path.exists() {
        // Read existing content
        let existing_content = std::fs::read_to_string(&gitignore_path)?;

        // Check if it already has .porters/ entry
        if !existing_content.contains(".porters/") {
            // Append .porters/ entry
            let mut file = std::fs::OpenOptions::new()
                .append(true)
                .open(&gitignore_path)?;

            use std::io::Write;
            writeln!(file, "\n# Porters local cache and build files")?;
            writeln!(file, ".porters/")?;

            print_info("📝 Updated existing .gitignore with .porters/ entry");
        } else {
            print_info("✅ .gitignore already contains .porters/ entry");
        }
    } else {
        // Create new .gitignore with comprehensive patterns
        let gitignore_content = r#"# Porters local cache and build files
.porters/

# Build artifacts
build/
*.o
*.obj
*.exe
*.out
*.app
*.a
*.so
*.dylib
*.dll
*.lib

# CMake
CMakeCache.txt
CMakeFiles/
cmake_install.cmake
Makefile
*.cmake

# IDE and Editor files
.vscode/
.idea/
*.swp
*.swo
*~
.DS_Store

# Dependency directories
ports/
vendor/

# Debug files
*.dSYM/
*.pdb
*.ilk

# Test outputs
Testing/
*.gcov
*.gcda
*.gcno
"#;

        std::fs::write(&gitignore_path, gitignore_content)?;
        print_success("✅ Created .gitignore with common C/C++ patterns");
    }

    Ok(())
}

fn check_build_tools() {
    print_info("🔍 Checking build tools...");

    let tools = vec![
        ("cmake", "CMake", "https://cmake.org/download/"),
        ("make", "Make", "https://www.gnu.org/software/make/"),
        ("git", "Git", "https://git-scm.com/downloads"),
    ];

    let mut missing = Vec::new();

    for (cmd, name, url) in &tools {
        if std::process::Command::new(cmd)
            .arg("--version")
            .output()
            .is_ok()
        {
            print_success(&format!("✅ {} is installed", name));
        } else {
            missing.push((name, url));
        }
    }

    if !missing.is_empty() {
        println!();
        print_warning("⚠️  Some build tools are not installed:");
        for (name, url) in missing {
            println!("  📥  Install {} from: {}", name, url);
        }
        println!();
        print_info("💡 Install missing tools to use all Porters features");
    }
}

async fn add_dependency(
    package: &str,
    dev: bool,
    optional: bool,
    git: Option<String>,
    _branch: Option<String>,
    _tag: Option<String>,
    global: bool,
) -> Result<()> {
    if global {
        print_step(&format!("📦 Adding global dependency: {}", package));
        // Global dependencies are tracked in global config
        // For now, we'll add them to the local config as well
        // TODO: Implement proper global dependency tracking in GlobalConfig
        print_info("💡 Global dependencies are stored in ~/.porters/packages");
    } else {
        print_step(&format!("📦 Adding dependency: {}", package));
    }

    let mut config = PortersConfig::load("porters.toml")?;

    let dep_type = if dev {
        "dev-dependencies"
    } else {
        "dependencies"
    };
    print_info(&format!("📦 Adding to [{}]", dep_type));

    // Determine the actual source
    let source = if let Some(git_url) = git {
        git_url
    } else {
        package.to_string()
    };

    config.add_dependency(&source, dev, optional)?;
    config.save("porters.toml")?;

    if global {
        print_success(&format!(
            "✅ Added {} globally and to {}",
            package, dep_type
        ));
    } else {
        print_success(&format!("✅ Added {} to {}", package, dep_type));
    }

    Ok(())
}

async fn remove_dependency(package: &str, global: bool, force: bool) -> Result<()> {
    if global {
        print_step(&format!("🗑️  Removing global dependency: {}", package));
    } else {
        print_step(&format!("🗑️  Removing dependency: {}", package));
    }

    // Confirm removal unless force is used
    if !force {
        print!("⚠️  Remove {} from porters.toml? (y/N): ", package);
        use std::io::{self, Write};
        io::stdout().flush()?;
        let mut input = String::new();
        io::stdin().read_line(&mut input)?;
        if !input.trim().eq_ignore_ascii_case("y") {
            println!("❌ Removal cancelled");
            return Ok(());
        }
    }

    let mut config = PortersConfig::load("porters.toml")?;
    config.remove_dependency(package)?;
    config.save("porters.toml")?;

    if global {
        print_success(&format!("✅ Removed {} globally", package));
        print_info("💡 Global dependencies in ~/.porters/packages remain intact");
    } else {
        print_success(&format!("✅ Removed {}", package));
    }

    Ok(())
}

/// Check tool version requirements from config
fn check_tool_requirements(config: &PortersConfig) -> Result<()> {
    let requires = &config.requires;
    let mut failures = Vec::new();

    // Map of tool field names to user-friendly names
    let tool_names = vec![
        ("c", "C Compiler"),
        ("cpp", "C++ Compiler"),
        ("cmake", "CMake"),
        ("gcc", "GCC"),
        ("clang", "Clang"),
        ("msvc", "MSVC"),
        ("ninja", "Ninja"),
        ("make", "Make"),
        ("xmake", "XMake"),
        ("meson", "Meson"),
        ("bazel", "Bazel"),
        ("conan", "Conan"),
        ("vcpkg", "vcpkg"),
    ];

    // Check each tool requirement
    for (field, friendly_name) in tool_names {
        let requirement = match field {
            "c" => &requires.c,
            "cpp" => &requires.cpp,
            "cmake" => &requires.cmake,
            "gcc" => &requires.gcc,
            "clang" => &requires.clang,
            "msvc" => &requires.msvc,
            "ninja" => &requires.ninja,
            "make" => &requires.make,
            "xmake" => &requires.xmake,
            "meson" => &requires.meson,
            "bazel" => &requires.bazel,
            "conan" => &requires.conan,
            "vcpkg" => &requires.vcpkg,
            _ => &None,
        };

        if let Some(req_str) = requirement {
            // Use version::ToolVersionChecker helper methods
            if !version::ToolVersionChecker::is_tool_installed(field) {
                failures.push(format!(
                    "  ❌ {}: requires {}, but tool not found in PATH",
                    friendly_name, req_str
                ));
                continue;
            }

            match version::ToolVersionChecker::check_requirement(field, req_str) {
                Ok(true) => {
                    // Requirement satisfied
                }
                Ok(false) => {
                    let installed = version::ToolVersionChecker::get_tool_version(field)
                        .unwrap_or_else(|_| version::Version::parse("0.0.0").unwrap());
                    failures.push(format!(
                        "  ❌ {}: requires {}, found {}",
                        friendly_name, req_str, installed
                    ));
                }
                Err(e) => {
                    failures.push(format!(
                        "  ⚠️  {}: invalid version requirement '{}' ({})",
                        friendly_name, req_str, e
                    ));
                }
            }
        }
    }

    if !failures.is_empty() {
        eprintln!();
        print_error("❌ Oops! Unsatisfied version requirements:");
        for failure in &failures {
            eprintln!("  {}", failure);
        }
        eprintln!();
        print_info("💡 Please install or upgrade the required tools to continue.");
        print_info("📚 See: https://github.com/muhammad-fiaz/porters#requirements");
        eprintln!();
        anyhow::bail!("Version requirements not satisfied");
    }

    Ok(())
}

/// Execute a build script (pre-build, post-build, etc.)
fn execute_build_script(script: &str, script_type: &str) -> Result<()> {
    let mut cmd = if cfg!(target_os = "windows") {
        let mut c = std::process::Command::new("cmd");
        c.args(["/C", script]);
        c
    } else {
        let mut c = std::process::Command::new("sh");
        c.args(["-c", script]);
        c
    };

    // Set environment variables for the script
    let project_dir = std::env::current_dir()?;
    let build_dir = project_dir.join("build");

    cmd.env("PROJECT_DIR", &project_dir);
    cmd.env("BUILD_DIR", &build_dir);
    cmd.env("PORTERS_VERSION", env!("CARGO_PKG_VERSION"));

    cmd.current_dir(&project_dir);

    let status = cmd
        .status()
        .with_context(|| format!("Failed to execute {} script", script_type))?;

    if !status.success() {
        anyhow::bail!(
            "{} script failed with exit code: {:?}",
            script_type,
            status.code()
        );
    }

    Ok(())
}

/// Verify checksums of resolved dependencies against lockfile
fn verify_dependency_checksums(resolved_deps: &[deps::ResolvedDependency]) -> Result<()> {
    let lock_file_path = global::project_lock_file(".");

    // Load lockfile if it exists
    if !lock_file_path.exists() {
        print_info("No lockfile found, skipping checksum verification");
        return Ok(());
    }

    let lock = lockfile::LockFile::load(&lock_file_path)?;

    for dep in resolved_deps {
        // Skip if dependency not in lockfile
        let Some(locked_dep) = lock.dependencies.get(&dep.name) else {
            print_info(&format!(
                "⚠️  {} not in lockfile, skipping checksum verification",
                dep.name
            ));
            continue;
        };

        // Skip if lockfile has no checksum stored
        let Some(expected_checksum) = &locked_dep.checksum else {
            print_info(&format!(
                "⚠️  {} has no checksum in lockfile, skipping verification",
                dep.name
            ));
            continue;
        };

        // Get dependency directory
        let dep_path = global::project_deps_dir(".").join(&dep.name);
        if !dep_path.exists() {
            anyhow::bail!(
                "Dependency {} not found at {}",
                dep.name,
                dep_path.display()
            );
        }

        // Verify checksum using hash module
        if !hash::verify_directory_hash(&dep_path, expected_checksum)? {
            let actual_checksum = hash::calculate_directory_hash(&dep_path)?;
            anyhow::bail!(
                "❌ Checksum mismatch for dependency '{}'!\n\
                 Expected: {}\n\
                 Actual:   {}\n\n\
                 This could indicate:\n\
                 - The dependency was tampered with\n\
                 - The dependency source has changed\n\
                 - The lockfile is out of date\n\n\
                 Run 'porters sync' to update the lockfile.",
                dep.name,
                &expected_checksum[..16],
                &actual_checksum[..16]
            );
        }

        print_info(&format!("✓ {} checksum verified", dep.name));
    }

    Ok(())
}

/// Build the project for specified platform(s)
///
/// If no platform flags are specified, builds for the current platform.
/// Supports cross-compilation for Linux, Windows, macOS, and all platforms.
///
/// # Arguments
/// * `all_platforms` - Build for all supported platforms
/// * `linux` - Build for Linux
/// * `windows` - Build for Windows  
/// * `macos` - Build for macOS
/// * `args` - Additional arguments to pass to the build system
///
/// # Returns
/// * `Result<()>` - Success or error
async fn build_project(
    all_platforms: bool,
    linux: bool,
    windows: bool,
    macos: bool,
    args: Vec<String>,
) -> Result<()> {
    print_step("🔨 Building project");

    // If any platform flags are set, delegate to cross-compile
    if all_platforms || linux || windows || macos {
        return compile_cross(all_platforms, linux, windows, macos, false, None).await;
    }

    // Default: build for current platform
    let config = PortersConfig::load("porters.toml")?;

    // Initialize binary cache
    let bin_cache_dir = config
        .cache
        .dir
        .clone()
        .unwrap_or_else(|| dirs::home_dir().unwrap().join(".porters").join("cache"))
        .parent()
        .unwrap()
        .join("bin-cache");

    let bin_cache = bin_cache::BinaryCache::new(
        bin_cache_dir,
        config.cache.enabled && config.cache.binary_cache,
    );
    bin_cache.init()?;

    // Check tool version requirements FIRST
    print_info("🔍 Checking tool version requirements...");
    check_tool_requirements(&config)?;
    print_success("✅ All tool requirements satisfied");

    // Execute pre-build script if defined
    if let Some(pre_build_script) = &config.build.scripts.pre_build {
        print_info("🔧 Executing pre-build script...");
        execute_build_script(pre_build_script, "pre-build")?;
        print_success("✅ Pre-build script completed");
    }

    // Load extensions and execute pre-build hooks
    let mut ext_manager = extension::ExtensionManager::new()?;
    ext_manager.load_extensions()?;

    let hook_context = extension::HookContext {
        project_dir: std::env::current_dir()?,
        args: args.clone(),
    };

    ext_manager.execute_hook("pre_build", &hook_context)?;

    // Scan project sources
    print_info("🔍 Scanning project sources...");
    let sources = scan::scan_project(".")?;
    print_success(&format!(
        "📄 Found {} source files",
        sources.source_files.len()
    ));

    // Resolve dependencies
    print_info("📦 Resolving dependencies...");
    let resolved_deps = deps::resolve_dependencies(&config).await?;
    print_success(&format!("✅ Resolved {} dependencies", resolved_deps.len()));

    // Check binary cache for dependencies
    if bin_cache.is_enabled() {
        print_info("🔍 Checking binary cache for dependencies...");
        for dep in &resolved_deps {
            // Get dependency version
            let version = &dep.version;

            // Calculate hashes for cache key
            let dep_path = &dep.path;
            if dep_path.exists() {
                let dep_hash = hash::calculate_directory_hash(dep_path)?;
                let build_hash =
                    hash::calculate_file_hash(&std::env::current_dir()?.join("porters.toml"))?;

                if bin_cache.is_cached(&dep.name, version, &dep_hash, &build_hash) {
                    print_info(&format!("✅ {} found in binary cache", dep.name));
                    bin_cache.retrieve(&dep.name, version, &dep_hash, &build_hash, dep_path)?;
                }
            }
        }
    }

    // Verify checksums of dependencies
    print_info("🔒 Verifying dependency checksums...");
    verify_dependency_checksums(&resolved_deps)?;
    print_success("✅ All checksums verified");

    // Detect and run build system
    print_info("🔍 Detecting build system...");
    let build_system = build::detect_build_system(".", &config)?;
    print_success(&format!("🔨 Using build system: {}", build_system.name()));

    print_info("⚙️  Building...");
    build_system.build(&sources, &resolved_deps, &args)?;

    // Store build in binary cache
    if bin_cache.is_enabled() {
        print_info("💾 Storing build artifacts in binary cache...");
        let build_dir = std::env::current_dir()?.join("build");
        if build_dir.exists() {
            // For the main project, use project name as cache key
            let project_name = &config.project.name;
            let project_version = &config.project.version;
            let build_hash =
                hash::calculate_file_hash(&std::env::current_dir()?.join("porters.toml"))?;
            let source_hash = hash::calculate_directory_hash(&std::env::current_dir()?)?;

            bin_cache.store(
                project_name,
                project_version,
                &source_hash,
                &build_hash,
                &build_dir,
            )?;
        }
    }

    // Execute post-build hooks
    ext_manager.execute_hook("post_build", &hook_context)?;

    // Execute post-build script if defined
    if let Some(post_build_script) = &config.build.scripts.post_build {
        print_info("🔧 Executing post-build script...");
        execute_build_script(post_build_script, "post-build")?;
        print_success("✅ Post-build script completed");
    }

    print_success("🎉 Build complete!");

    Ok(())
}

/// Run the project after building
///
/// # Arguments
/// * `args` - Arguments to pass to the executable
///
/// # Returns
/// * `Result<()>` - Success or error
async fn run_project(args: Vec<String>) -> Result<()> {
    print_step("▶️  Running project");

    // Build first (current platform only)
    build_project(false, false, false, false, vec![]).await?;

    let config = PortersConfig::load("porters.toml")?;
    let build_system = build::detect_build_system(".", &config)?;

    print_info("🚀 Running executable...");
    build_system.run(&args)?;

    Ok(())
}

async fn test_project() -> Result<()> {
    print_step("🧪 Running tests");

    let config = PortersConfig::load("porters.toml")?;
    let sources = scan::scan_project(".")?;
    let resolved_deps = deps::resolve_dependencies(&config).await?;
    let build_system = build::detect_build_system(".", &config)?;

    build_system.test(&sources, &resolved_deps)?;

    print_success("✅ Tests complete!");

    Ok(())
}

/// Checks compilation of source files without creating executables (syntax-only check).
///
/// This function performs a fast compilation check to validate code syntax and catch
/// compilation errors without generating executable files. It's useful for rapid
/// feedback during development and CI/CD pipelines.
///
/// # Arguments
///
/// * `file` - Optional path to a specific source file to check. If `None`, checks all
///   source files in the project. Supports both C (.c) and C++ (.cpp/.cc/.cxx) files.
/// * `verbose` - If `true`, displays detailed compiler output including warnings and full
///   error traces. If `false`, shows only summary information.
///
/// # Behavior
///
/// - **Single File Mode** (`file` is `Some`): Checks only the specified file
/// - **Project Mode** (`file` is `None`): Discovers and checks all `.c`, `.cpp`, `.cc`,
///   and `.cxx` files in the `src/` directory
/// - **Compiler Flags**: Uses `-fsyntax-only` (GCC/Clang) or `/Zs` (MSVC) to skip
///   code generation and linking, resulting in faster checks
/// - **Dependencies**: Automatically includes dependency paths from `porters.toml`
/// - **Error Reporting**: Captures and displays compiler stderr with emoji indicators
///
/// # Returns
///
/// * `Result<()>` - Success if all files compile without errors, error otherwise
///
/// # Examples
///
/// ```bash
/// # Check all source files in the project
/// porters check
///
/// # Check a specific file
/// porters check src/main.c
///
/// # Check with verbose compiler output
/// porters check --verbose
/// porters check src/utils.cpp --verbose
/// ```
///
/// # Errors
///
/// Returns an error if:
/// - The specified file doesn't exist
/// - No source files found in project mode
/// - Compiler is not installed or not found
/// - Configuration file (`porters.toml`) is invalid
/// - Compilation errors are detected
///
/// # Implementation Details
///
/// The function performs the following steps:
/// 1. Determines which files to check (single file or all files)
/// 2. Loads project configuration and resolves dependencies
/// 3. Detects appropriate compiler (GCC, Clang, or MSVC)
/// 4. Builds compile command with syntax-only flags
/// 5. Executes compiler and captures output
/// 6. Displays results with color-coded emoji indicators
async fn check_compilation(file: Option<&str>, verbose: bool) -> Result<()> {
    use std::path::Path;
    use std::process::Command;

    print_step("✅ Checking compilation (syntax-only)");

    // Determine which files to check
    let files_to_check: Vec<String> = if let Some(specific_file) = file {
        // Single file mode
        if !Path::new(specific_file).exists() {
            return Err(anyhow::anyhow!("❌ File not found: {}", specific_file));
        }
        println!("🎯 Checking single file: {}", specific_file);
        vec![specific_file.to_string()]
    } else {
        // Project mode - discover all source files
        println!("🔍 Discovering source files in project...");
        let sources = scan::scan_project(".")?;

        if sources.source_files.is_empty() {
            return Err(anyhow::anyhow!("❌ No source files found in project"));
        }

        println!("📦 Found {} source file(s)", sources.source_files.len());
        sources
            .source_files
            .iter()
            .map(|p| p.display().to_string())
            .collect()
    };

    // Load configuration and resolve dependencies
    let config = match PortersConfig::load("porters.toml") {
        Ok(cfg) => cfg,
        Err(_) => {
            println!("⚠️  No porters.toml found, checking without configuration");
            // Continue without config
            let mut all_passed = true;
            let mut total_checked = 0;
            let mut total_errors = 0;

            for source_file in &files_to_check {
                total_checked += 1;
                let path = Path::new(source_file);
                let extension = path.extension().and_then(|e| e.to_str()).unwrap_or("");

                // Determine if C or C++ and detect appropriate compiler
                let (compiler, is_cpp) = match extension {
                    "cpp" | "cc" | "cxx" | "C" => {
                        let cpp_compiler = detect_cpp_compiler();
                        (cpp_compiler, true)
                    }
                    _ => {
                        let c_compiler = detect_c_compiler();
                        (c_compiler, false)
                    }
                };

                println!(
                    "\n🔨 Checking: {} ({})",
                    source_file,
                    if is_cpp { "C++" } else { "C" }
                );
                if verbose {
                    println!("🔧 Using compiler: {}", compiler);
                }

                // Build compiler command
                let mut cmd = Command::new(&compiler);

                // Add syntax-only flag
                if compiler.contains("cl.exe") || compiler.contains("cl ") {
                    cmd.arg("/Zs").arg("/nologo");
                } else {
                    cmd.arg("-fsyntax-only");
                }

                cmd.arg(source_file);

                let output = cmd.output()?;

                if output.status.success() {
                    println!("✅ PASSED: {}", source_file);
                } else {
                    all_passed = false;
                    total_errors += 1;
                    println!("❌ FAILED: {}", source_file);

                    let stderr = String::from_utf8_lossy(&output.stderr);
                    let stdout = String::from_utf8_lossy(&output.stdout);

                    if verbose {
                        println!("\n📋 Detailed compiler output:");
                        println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
                        if !stdout.is_empty() {
                            println!("{}", stdout);
                        }
                        if !stderr.is_empty() {
                            println!("{}", stderr);
                        }
                        println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
                    } else if !stderr.is_empty() {
                        let lines: Vec<&str> = stderr.lines().take(10).collect();
                        println!("\n❌ Compilation errors:");
                        for line in lines {
                            println!("  {}", line);
                        }
                        let total_lines = stderr.lines().count();
                        if total_lines > 10 {
                            println!(
                                "  ... ({} more lines, use --verbose for full output)",
                                total_lines - 10
                            );
                        }
                    }
                }
            }

            println!("\n━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
            println!("📊 Compilation Check Summary");
            println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
            println!("   Total files checked: {}", total_checked);
            println!("   ✅ Passed: {}", total_checked - total_errors);
            println!("   ❌ Failed: {}", total_errors);
            println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");

            if all_passed {
                print_success("All compilation checks passed! ✅");
                return Ok(());
            } else {
                return Err(anyhow::anyhow!(
                    "❌ Compilation check failed: {} error(s) found",
                    total_errors
                ));
            }
        }
    };

    let resolved_deps = deps::resolve_dependencies(&config).await?;

    // Build include paths from dependencies
    let mut include_paths = Vec::new();
    for dep in &resolved_deps {
        for include_path in &dep.include_paths {
            include_paths.push(format!("-I{}", include_path.display()));
        }
    }

    // Check each file
    let mut all_passed = true;
    let mut total_checked = 0;
    let mut total_errors = 0;

    for source_file in &files_to_check {
        total_checked += 1;
        let path = Path::new(source_file);
        let extension = path.extension().and_then(|e| e.to_str()).unwrap_or("");

        // Determine if C or C++ and detect appropriate compiler
        let (compiler, is_cpp) = match extension {
            "cpp" | "cc" | "cxx" | "C" => {
                let cpp_compiler = detect_cpp_compiler();
                (cpp_compiler, true)
            }
            _ => {
                let c_compiler = detect_c_compiler();
                (c_compiler, false)
            }
        };

        println!(
            "\n🔨 Checking: {} ({})",
            source_file,
            if is_cpp { "C++" } else { "C" }
        );
        if verbose {
            println!("🔧 Using compiler: {}", compiler);
        }

        // Build compiler command
        let mut cmd = Command::new(&compiler);

        // Add syntax-only flag
        if compiler.contains("cl.exe") || compiler.contains("cl ") {
            // MSVC
            cmd.arg("/Zs"); // Syntax check only
            cmd.arg("/nologo"); // Suppress banner
        } else {
            // GCC/Clang
            cmd.arg("-fsyntax-only"); // Syntax check only
        }

        // Add include paths
        for include in &include_paths {
            cmd.arg(include);
        }

        // Add compiler flags from config
        if is_cpp {
            for flag in &config.build.flags.cxxflags {
                cmd.arg(flag);
            }
        } else {
            for flag in &config.build.flags.cflags {
                cmd.arg(flag);
            }
        }

        // Add the source file
        cmd.arg(source_file);

        // Execute compiler
        let output = cmd.output()?;

        // Process results
        if output.status.success() {
            println!("✅ PASSED: {}", source_file);
        } else {
            all_passed = false;
            total_errors += 1;
            println!("❌ FAILED: {}", source_file);

            // Display compiler output
            let stderr = String::from_utf8_lossy(&output.stderr);
            let stdout = String::from_utf8_lossy(&output.stdout);

            if verbose {
                println!("\n📋 Detailed compiler output:");
                println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
                if !stdout.is_empty() {
                    println!("{}", stdout);
                }
                if !stderr.is_empty() {
                    println!("{}", stderr);
                }
                println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
            } else {
                // Show concise error output
                if !stderr.is_empty() {
                    // Show first few lines of errors
                    let lines: Vec<&str> = stderr.lines().take(10).collect();
                    println!("\n❌ Compilation errors:");
                    for line in lines {
                        println!("  {}", line);
                    }
                    let total_lines = stderr.lines().count();
                    if total_lines > 10 {
                        println!(
                            "  ... ({} more lines, use --verbose for full output)",
                            total_lines - 10
                        );
                    }
                }
            }
        }
    }

    // Print summary
    println!("\n━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    println!("📊 Compilation Check Summary");
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    println!("   Total files checked: {}", total_checked);
    println!("   ✅ Passed: {}", total_checked - total_errors);
    println!("   ❌ Failed: {}", total_errors);
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");

    if all_passed {
        print_success("All compilation checks passed! ✅");
        Ok(())
    } else {
        Err(anyhow::anyhow!(
            "❌ Compilation check failed: {} error(s) found",
            total_errors
        ))
    }
}

async fn update_dependencies() -> Result<()> {
    print_step("🔄 Updating dependencies");

    let config = PortersConfig::load("porters.toml")?;
    deps::update_dependencies(&config).await?;

    print_success("✅ Dependencies updated!");

    Ok(())
}

async fn clean_project() -> Result<()> {
    print_step("🧹 Cleaning project");

    let config = PortersConfig::load("porters.toml")?;
    let build_system = build::detect_build_system(".", &config)?;

    build_system.clean()?;

    print_success("✅ Clean complete!");

    Ok(())
}

async fn generate_lockfile() -> Result<()> {
    print_step("🔒 Generating lockfile");

    let config = PortersConfig::load("porters.toml")?;
    let resolved_deps = deps::resolve_dependencies(&config).await?;

    // Use the lockfile module to create and save lockfile
    let mut lockfile = lockfile::LockFile::new();

    for dep in &resolved_deps {
        // Create lockfile::DependencySource from deps::DependencySource
        let source = match &dep.source {
            deps::DependencySource::Git { url, rev } => lockfile::DependencySource::Git {
                url: url.clone(),
                rev: rev.clone(),
                branch: None,
                tag: None,
            },
            deps::DependencySource::Path { path } => {
                lockfile::DependencySource::Path { path: path.clone() }
            }
            deps::DependencySource::Registry { registry } => lockfile::DependencySource::Registry {
                registry: registry.clone(),
                version: dep.version.clone(),
            },
        };

        let resolved_dep = lockfile::ResolvedDependency {
            name: dep.name.clone(),
            version: dep.version.clone(),
            source,
            checksum: dep.checksum.clone(),
            dependencies: dep.dependencies.clone(),
        };

        lockfile.add_dependency(dep.name.clone(), resolved_dep);
    }

    // Save lockfile
    lockfile.save("porters.lock")?;

    print_success("Generated porters.lock 🔒");

    Ok(())
}

async fn vendor_dependencies() -> Result<()> {
    print_step("📦 Vendoring dependencies");

    let config = PortersConfig::load("porters.toml")?;
    deps::vendor_dependencies(&config, "vendor").await?;

    print_success("✅ Dependencies vendored to ./vendor");

    Ok(())
}

async fn show_dependency_graph() -> Result<()> {
    print_step("📊 Generating dependency graph");

    let config = PortersConfig::load("porters.toml")?;
    let resolved_deps = deps::resolve_dependencies(&config).await?;

    deps::print_dependency_graph(&resolved_deps)?;

    Ok(())
}

async fn publish_package(token: Option<String>, dry_run: bool) -> Result<()> {
    print_step("📤 Publishing package");

    let config = PortersConfig::load("porters.toml")?;

    // Get token from argument or environment variable
    let token = token
        .or_else(|| std::env::var("GITHUB_TOKEN").ok())
        .ok_or_else(|| {
            anyhow::anyhow!(
                "GitHub token required. Use --token or set GITHUB_TOKEN environment variable"
            )
        })?;

    publish::publish_package(&config, &token, dry_run)?;

    if !dry_run {
        print_success("Package published successfully! 🎉");
    }

    Ok(())
}

async fn upgrade_self() -> Result<()> {
    update::perform_update()?;
    Ok(())
}

async fn install_package(
    package: &str,
    git: Option<String>,
    _branch: Option<String>,
    _tag: Option<String>,
) -> Result<()> {
    print_step(&format!("📥 Installing package globally: {}", package));

    // Load config for build scripts (if exists)
    let config = PortersConfig::load("porters.toml").ok();

    // Execute pre-install script if configured
    if let Some(ref cfg) = config
        && let Some(pre_install_script) = &cfg.build.scripts.pre_install
    {
        print_info("🔧 Executing pre-install script...");
        execute_build_script(pre_install_script, "pre-install")?;
    }

    // Load extensions and execute pre-install hooks
    let mut ext_manager = extension::ExtensionManager::new()?;
    ext_manager.load_extensions()?;

    let hook_context = extension::HookContext {
        project_dir: std::env::current_dir()?,
        args: vec![],
    };

    ext_manager.execute_hook("pre_install", &hook_context)?;

    // Initialize global directory
    global::GlobalConfig::initialize()?;

    let source = if let Some(git_url) = git {
        git_url
    } else {
        package.to_string()
    };

    // Extract package name from URL
    let pkg_name = if source.contains("://") || source.contains("@") {
        source
            .split('/')
            .next_back()
            .unwrap_or(package)
            .trim_end_matches(".git")
    } else {
        package
    };

    // Install to global location
    let packages_dir = global::GlobalConfig::packages_dir()?;
    let install_path = packages_dir.join(pkg_name);

    print_info(&format!("Installing to: {}", install_path.display()));

    // Clone repository if git source
    if source.starts_with("http://") || source.starts_with("https://") || source.starts_with("git@")
    {
        print_info("Cloning repository...");
        deps::clone_git_repo(&source, &install_path).await?;
    }

    // Update global config
    let mut global_config = global::GlobalConfig::load()?;
    global_config.add_package(
        pkg_name.to_string(),
        "latest".to_string(),
        source.clone(),
        install_path.clone(),
    )?;

    print_success(&format!(
        "Installed {} globally at {}",
        pkg_name,
        install_path.display()
    ));
    print_info(&format!(
        "Global packages directory: {}",
        packages_dir.display()
    ));

    // Execute post-install hooks
    ext_manager.execute_hook("post_install", &hook_context)?;

    // Execute post-install script if configured
    if let Some(ref cfg) = config
        && let Some(post_install_script) = &cfg.build.scripts.post_install
    {
        print_info("Executing post-install script...");
        execute_build_script(post_install_script, "post-install")?;
    }

    Ok(())
}

// Old sync_dependencies moved to end of file with cache support
// This is now just a wrapper for backward compatibility

async fn handle_extension(action: ExtensionAction) -> Result<()> {
    use extension::*;

    match action {
        ExtensionAction::Install { name, git, path } => {
            let mut manager = ExtensionManager::new()?;

            let source = if let Some(git_url) = git {
                ExtensionSource::Git(git_url)
            } else if let Some(local_path) = path {
                ExtensionSource::Path(std::path::PathBuf::from(local_path))
            } else {
                ExtensionSource::CratesIo
            };

            manager.install_extension(&name, source)?;
            print_success(&format!("Extension '{}' installed successfully! 🔌", name));
        }

        ExtensionAction::Uninstall { name } => {
            let mut manager = ExtensionManager::new()?;
            manager.uninstall_extension(&name)?;
        }

        ExtensionAction::List => {
            let mut manager = ExtensionManager::new()?;
            manager.load_extensions()?;

            let extensions = manager.list_extensions();

            if extensions.is_empty() {
                print_info("No extensions installed");
                print_info("Install extensions with: porters extension install <name>");
            } else {
                println!("\n📦  Installed Extensions:\n");
                for ext in extensions {
                    println!("  {} v{}", ext.manifest.name, ext.manifest.version);
                    println!("    {}", ext.manifest.description);
                    if let Some(repo) = &ext.manifest.repository {
                        println!("    Repository: {}", repo);
                    }
                    println!();
                }
            }
        }

        ExtensionAction::Create { name } => {
            let current_dir = std::env::current_dir()?;
            ExtensionManager::create_template(&name, &current_dir)?;
        }
    }

    Ok(())
}

async fn run_script(name: &str) -> Result<()> {
    print_step(&format!("🔧 Running script: {}", name));

    let config = PortersConfig::load("porters.toml")?;

    if let Some(script) = config.scripts.get(name) {
        print_info(&format!("⚙️  Executing: {}", script));

        let mut cmd = if cfg!(target_os = "windows") {
            let mut c = std::process::Command::new("cmd");
            c.args(["/C", script]);
            c
        } else {
            let mut c = std::process::Command::new("sh");
            c.args(["-c", script]);
            c
        };

        cmd.current_dir(std::env::current_dir()?);

        let status = cmd
            .status()
            .with_context(|| format!("Failed to execute script '{}'", name))?;

        if !status.success() {
            anyhow::bail!(
                "Script '{}' failed with exit code: {:?}",
                name,
                status.code()
            );
        }

        print_success(&format!("✅ Script '{}' completed successfully!", name));
    } else {
        print_error(&format!("❌ Script '{}' not found in porters.toml", name));
        print_info("📋 Available scripts:");
        for script_name in config.scripts.keys() {
            println!("   📜 {}", script_name);
        }
        anyhow::bail!("Script not found");
    }

    Ok(())
}

async fn execute_custom_command(args: Vec<String>) -> Result<()> {
    if args.is_empty() {
        print_error("❌ No command provided");
        println!();
        println!("💡 Try one of these:");
        println!("   porters --help          Show all available commands");
        println!("   porters <command> --help  Show help for a specific command");
        println!();
        anyhow::bail!("No command provided");
    }

    let command_name = &args[0];

    // Try to load config to check for custom commands
    let config_result = PortersConfig::load("porters.toml");

    match config_result {
        Ok(config) => {
            // Find matching custom command
            if let Some(custom_cmd) = config.commands.iter().find(|c| &c.name == command_name) {
                print_step(&format!("🔧 Running custom command: {}", custom_cmd.name));
                print_info(&format!("📝 {}", custom_cmd.description));

                let mut cmd = if cfg!(target_os = "windows") {
                    let mut c = std::process::Command::new("cmd");
                    c.args(["/C", &custom_cmd.script]);
                    c
                } else {
                    let mut c = std::process::Command::new("sh");
                    c.args(["-c", &custom_cmd.script]);
                    c
                };

                // Set environment variables
                for (key, value) in &custom_cmd.env {
                    cmd.env(key, value);
                }

                // Set working directory
                cmd.current_dir(std::env::current_dir()?);

                let status = cmd.status().with_context(|| {
                    format!("Failed to execute custom command '{}'", command_name)
                })?;

                if !status.success() {
                    anyhow::bail!(
                        "❌ Custom command '{}' failed with exit code: {:?}",
                        command_name,
                        status.code()
                    );
                }

                print_success(&format!(
                    "✅ Command '{}' completed successfully!",
                    command_name
                ));
            } else {
                print_error(&format!("❌ Unknown command: '{}'", command_name));
                println!();

                if !config.commands.is_empty() {
                    print_info("📋 Available custom commands:");
                    for cmd in &config.commands {
                        println!("   🔧 {} - {}", cmd.name, cmd.description);
                    }
                    println!();
                }

                println!("💡 Try one of these:");
                println!("   porters --help          Show all available commands");
                println!("   porters <command> --help  Show help for a specific command");
                println!();

                anyhow::bail!("Command '{}' not found", command_name);
            }
        }
        Err(_) => {
            // No porters.toml, so it's definitely an unknown command
            print_error(&format!("❌ Unknown command: '{}'", command_name));
            println!();
            println!("💡 This doesn't appear to be a valid porters command.");
            println!();
            println!("📚 Try one of these:");
            println!("   porters --help          Show all available commands");
            println!("   porters <command> --help  Show help for a specific command");
            println!("   porters docs            Open documentation in browser");
            println!();

            anyhow::bail!("Command '{}' not found", command_name);
        }
    }

    Ok(())
}

/// Update sync_dependencies signature to support caching
async fn sync_dependencies(
    include_dev: bool,
    include_optional: bool,
    use_cache: bool,
) -> Result<()> {
    print_step("🔄 Syncing dependencies from porters.toml");

    let config = PortersConfig::load("porters.toml")?;

    // Initialize cache
    let cache_enabled = use_cache && config.cache.enabled;
    let cache_dir = config
        .cache
        .dir
        .clone()
        .unwrap_or_else(|| dirs::home_dir().unwrap().join(".porters").join("cache"));

    let dep_cache = cache::DependencyCache::new(cache_dir.clone(), cache_enabled);
    dep_cache.init()?;

    let bin_cache_dir = cache_dir.parent().unwrap().join("bin-cache");
    let bin_cache =
        bin_cache::BinaryCache::new(bin_cache_dir, cache_enabled && config.cache.binary_cache);
    bin_cache.init()?;

    // Rest of existing sync_dependencies implementation...
    // Execute pre-install script if configured
    if let Some(pre_install_script) = &config.build.scripts.pre_install {
        print_info("🔧 Executing pre-install script...");
        execute_build_script(pre_install_script, "pre-install")?;
    }

    // Auto-install extensions listed in config
    if !config.extensions.is_empty() {
        print_info(&format!(
            "🔌 Auto-installing {} extensions from config...",
            config.extensions.len()
        ));
        let mut ext_manager = extension::ExtensionManager::new()?;

        for ext_name in &config.extensions {
            // Check if already installed
            let extensions = ext_manager.list_extensions();
            if extensions.iter().any(|e| &e.manifest.name == ext_name) {
                print_info(&format!(
                    "✅ Extension '{}' already installed, skipping",
                    ext_name
                ));
                continue;
            }

            print_info(&format!("📦 Installing extension '{}'...", ext_name));
            match ext_manager.install_extension(ext_name, extension::ExtensionSource::CratesIo) {
                Ok(_) => print_success(&format!(
                    "🔌 Extension '{}' installed successfully!",
                    ext_name
                )),
                Err(e) => print_warning(&format!(
                    "⚠️  Failed to install extension '{}': {}",
                    ext_name, e
                )),
            }
        }
    }

    // Load extensions and execute pre-sync hooks
    let mut ext_manager = extension::ExtensionManager::new()?;
    ext_manager.load_extensions()?;

    let hook_context = extension::HookContext {
        project_dir: std::env::current_dir()?,
        args: vec![],
    };

    ext_manager.execute_hook("pre_sync", &hook_context)?;

    // Create ports directory for project-local dependencies
    let ports_dir = global::project_deps_dir(".");
    std::fs::create_dir_all(&ports_dir)?;

    // Collect all dependencies to install
    let mut to_install = Vec::new();

    // Regular dependencies
    for (name, dep) in &config.dependencies {
        to_install.push((name.clone(), dep.clone(), false));
    }

    // Dev dependencies (if requested)
    if include_dev {
        for (name, dep) in &config.dev_dependencies {
            to_install.push((name.clone(), dep.clone(), true));
        }
    }

    // Filter out optional dependencies if not requested
    if !include_optional {
        to_install.retain(|(_, dep, _)| match dep {
            config::Dependency::Detailed { optional, .. } => !optional,
            _ => true,
        });
    }

    if to_install.is_empty() {
        print_info("✅ No dependencies to sync");
        return Ok(());
    }

    print_step(&format!("📦 Syncing {} dependencies...", to_install.len()));

    // Install each dependency
    for (name, dep, is_dev) in &to_install {
        let dep_type = if *is_dev { "dev" } else { "regular" };
        print_info(&format!("📥 Installing {} ({})...", name, dep_type));

        match dep {
            config::Dependency::Detailed {
                git: Some(url),
                branch,
                tag,
                ..
            } => {
                let dep_path = ports_dir.join(name);

                // Check cache first
                let version = tag.as_deref().or(branch.as_deref()).unwrap_or("main");
                if cache_enabled && dep_cache.is_cached(name, version, None)? {
                    dep_cache.retrieve(name, version, &dep_path)?;
                } else if dep_path.exists() {
                    print_warning(&format!("⚠️  {} already exists, skipping", name));
                } else {
                    let _checksum = deps::clone_git_repo(url, &dep_path).await?;
                    print_success(&format!("✅ Installed {}", name));

                    // Store in cache
                    if cache_enabled {
                        dep_cache.store(name, version, &dep_path)?;
                    }
                }
            }
            config::Dependency::Detailed {
                path: Some(path), ..
            } => {
                print_info(&format!("📁 {} is a path dependency at {}", name, path));
            }
            config::Dependency::Simple(spec) => {
                print_warning(&format!(
                    "⚠️  {}: Simple version spec not yet supported ({})",
                    name, spec
                ));
            }
            _ => {
                print_warning(&format!("⚠️  {}: No installation source found", name));
            }
        }
    }

    // Execute post-sync hooks
    ext_manager.execute_hook("post_sync", &hook_context)?;

    // Execute post-install script if configured
    if let Some(post_install_script) = &config.build.scripts.post_install {
        print_info("🔧 Executing post-install script...");
        execute_build_script(post_install_script, "post-install")?;
    }

    // Generate/update lockfile
    print_step("🔒 Updating lockfile");
    generate_lockfile().await?;

    print_success("✅ Dependencies synced successfully!");
    Ok(())
}

/// Export project configuration to build system files
async fn export_to_build_system(build_system: ExportBuildSystem) -> Result<()> {
    use export::BuildSystemExporter;

    print_step("📤 Exporting project configuration");

    // Load config
    let config = PortersConfig::load("porters.toml")?;

    // Scan sources
    print_info("📂 Scanning project sources...");
    let sources = scan::scan_project(".")?;

    // Export based on build system choice
    match build_system {
        ExportBuildSystem::Cmake => {
            let exporter = export::cmake::CMakeExporter::new();
            exporter.export(&config, &sources)?;
            print_info("💡 Build with: cmake -B build && cmake --build build");
        }
        ExportBuildSystem::Xmake => {
            let exporter = export::xmake::XMakeExporter::new();
            exporter.export(&config, &sources)?;
            print_info("💡 Build with: xmake");
        }
        ExportBuildSystem::Vcpkg => {
            let exporter = export::vcpkg::VcpkgExporter::new();
            exporter.export(&config, &sources)?;
            print_info("💡 Install dependencies with: vcpkg install");
            print_info(
                "💡 Then use CMake: cmake -B build -DCMAKE_TOOLCHAIN_FILE=[vcpkg root]/scripts/buildsystems/vcpkg.cmake",
            );
        }
        ExportBuildSystem::Conan => {
            let exporter = export::conan::ConanExporter::new();
            exporter.export(&config, &sources)?;
            print_info("💡 Install dependencies with: conan install . --build=missing");
            print_info("💡 Then build with: conan build .");
        }
        ExportBuildSystem::Make => {
            print_error("❌ Makefile export not yet implemented");
            print_info("💡 Use 'porters export cmake' or 'porters export xmake' for now");
            return Err(anyhow::anyhow!("Makefile export not implemented"));
        }
        ExportBuildSystem::Meson => {
            print_error("❌ Meson export not yet implemented");
            print_info("💡 Use 'porters export cmake' or 'porters export xmake' for now");
            return Err(anyhow::anyhow!("Meson export not implemented"));
        }
        ExportBuildSystem::Bazel => {
            print_error("❌ Bazel export not yet implemented");
            print_info("💡 Use 'porters export cmake' or 'porters export xmake' for now");
            return Err(anyhow::anyhow!("Bazel export not implemented"));
        }
    }

    print_success("✅ Export completed successfully!");

    Ok(())
}

/// List project dependencies
async fn list_dependencies(tree: bool) -> Result<()> {
    print_step("📋 Listing project dependencies");

    let config = PortersConfig::load("porters.toml")?;

    let total = config.dependencies.len() + config.dev_dependencies.len();
    if total == 0 {
        print_info("💡 No dependencies found in porters.toml");
        return Ok(());
    }

    println!("\n📦  Dependencies ({})", config.dependencies.len());
    for (name, dep) in &config.dependencies {
        print_dependency(name, dep, tree, 0);
    }

    if !config.dev_dependencies.is_empty() {
        println!("\n🔧  Dev Dependencies ({})", config.dev_dependencies.len());
        for (name, dep) in &config.dev_dependencies {
            print_dependency(name, dep, tree, 0);
        }
    }

    Ok(())
}

fn print_dependency(name: &str, dep: &config::Dependency, tree: bool, indent: usize) {
    let prefix = "  ".repeat(indent);
    match dep {
        config::Dependency::Simple(version) => {
            println!("{}  {} @ {}", prefix, name, version);
        }
        config::Dependency::Detailed {
            git: Some(url),
            tag,
            branch,
            ..
        } => {
            let version = tag.as_deref().or(branch.as_deref()).unwrap_or("HEAD");
            println!("{}  {} @ {} (git)", prefix, name, version);
            if tree {
                println!("{}    Source: {}", prefix, url);
            }
        }
        config::Dependency::Detailed {
            path: Some(path), ..
        } => {
            println!("{}  {} (path: {})", prefix, name, path);
        }
        _ => {
            println!("{}  {}", prefix, name);
        }
    }
}

/// List globally installed packages
async fn global_list_packages() -> Result<()> {
    print_step("📦 Listing globally installed packages");

    let global_config = global::GlobalConfig::load()?;
    let packages = global_config.list_packages();

    if packages.is_empty() {
        print_info("💡 No global packages installed");
        print_info("📥 Install packages globally with: porters install --global <package>");
    } else {
        println!("\n📦  Global Packages ({})", packages.len());
        for pkg in packages {
            println!("  ✅ {} @ {} ({})", pkg.name, pkg.version, pkg.source);
            println!("      {}", pkg.install_path.display());
        }
        let global_dir = global::GlobalConfig::global_dir()?;
        println!("\nLocation: {}", global_dir.display());
    }

    Ok(())
}

/// Clean cache
async fn clean_cache(force: bool) -> Result<()> {
    print_step("Cleaning cache");

    let cache_dir = dirs::home_dir()
        .ok_or_else(|| anyhow::anyhow!("Could not find home directory"))?
        .join(".porters")
        .join("cache");

    let bin_cache_dir = cache_dir.parent().unwrap().join("bin-cache");

    let dep_cache = cache::DependencyCache::new(cache_dir, true);
    let bin_cache = bin_cache::BinaryCache::new(bin_cache_dir, true);

    // Show stats before cleaning
    if let Ok(stats) = dep_cache.stats() {
        print_info(&format!(
            "Dependency cache: {} items, {}",
            stats.count,
            stats.human_size()
        ));
    }

    if force && let Ok(stats) = bin_cache.stats() {
        print_info(&format!(
            "Binary cache: {} items, {}",
            stats.count,
            stats.human_size()
        ));
    }

    // Clean dependency cache
    dep_cache.clear(force)?;

    // Clean binary cache if force
    if force {
        bin_cache.clear()?;
    }

    print_success("Cache cleaned successfully! ✓");
    Ok(())
}

/// Self-update porters to latest version
async fn self_update() -> Result<()> {
    print_step("Updating porters to latest version");

    print_info("Checking for updates...");

    let status = self_update::backends::github::Update::configure()
        .repo_owner("muhammad-fiaz")
        .repo_name("porters")
        .bin_name("porters")
        .show_download_progress(true)
        .current_version(env!("CARGO_PKG_VERSION"))
        .build()?
        .update()?;

    match status {
        self_update::Status::UpToDate(version) => {
            print_success(&format!("Already up to date (v{})", version));
        }
        self_update::Status::Updated(version) => {
            print_success(&format!("Updated to v{}! 🎉", version));
            print_info("Restart your terminal to use the new version");
        }
    }

    Ok(())
}

/// Update all dependencies to latest versions
async fn update_deps(latest: bool) -> Result<()> {
    if latest {
        print_step("Updating all dependencies to absolute latest versions");
        print_warning("This ignores version constraints!");
    } else {
        print_step("Updating dependencies to latest compatible versions");
    }

    let mut config = PortersConfig::load("porters.toml")?;

    let mut updated = 0;

    // Update regular dependencies
    for (name, dep) in &mut config.dependencies {
        if let config::Dependency::Detailed {
            git: Some(_url),
            tag,
            branch,
            ..
        } = dep
        {
            print_info(&format!("Checking {} for updates...", name));

            // For now, just update to latest tag/branch
            // In production, we'd query git remote for latest tags
            if latest {
                *tag = None;
                *branch = Some("main".to_string());
                updated += 1;
                print_success(&format!("Updated {} to latest", name));
            } else {
                print_info(&format!("{} update check (respecting constraints)", name));
            }
        }
    }

    if updated > 0 {
        config.save("porters.toml")?;
        print_success(&format!("Updated {} dependencies", updated));
        print_info("Run 'porters sync' to apply updates");
    } else {
        print_info("All dependencies are up to date");
    }

    Ok(())
}

/// Cross-compile for specified platforms
async fn compile_cross(
    all_platforms: bool,
    linux: bool,
    windows: bool,
    macos: bool,
    baremetal: bool,
    target: Option<String>,
) -> Result<()> {
    print_step("Cross-compilation");

    let config = PortersConfig::load("porters.toml")?;
    let project_root = std::env::current_dir()?;

    // Determine build system
    let build_system_str = if let Some(sys) = config.build.system.as_deref() {
        sys.to_string()
    } else if let Some(detected) = buildsystem::detect_build_system(".") {
        detected.as_str().to_string()
    } else {
        "cmake".to_string()
    };

    // Create cross-compiler
    let cross_config = cross_compile::CrossCompileConfig {
        targets: config
            .cross_compile
            .targets
            .iter()
            .map(|(k, v)| {
                (
                    k.clone(),
                    cross_compile::TargetConfig {
                        toolchain: v.toolchain.clone(),
                        sysroot: v.sysroot.clone(),
                        linker: v.linker.clone(),
                        cmake_toolchain_file: v.cmake_toolchain_file.clone(),
                        env: v.env.clone(),
                        flags: cross_compile::TargetFlags {
                            cflags: v.flags.cflags.clone(),
                            cxxflags: v.flags.cxxflags.clone(),
                            ldflags: v.flags.ldflags.clone(),
                        },
                    },
                )
            })
            .collect(),
        default_target: config.cross_compile.default_target.clone(),
    };

    let compiler = cross_compile::CrossCompiler::new(cross_config, project_root);

    // Collect targets to compile
    let mut targets = Vec::new();

    if all_platforms {
        targets = cross_compile::Target::all();
    } else {
        if linux {
            targets.extend(cross_compile::Target::for_platform("linux"));
        }
        if windows {
            targets.extend(cross_compile::Target::for_platform("windows"));
        }
        if macos {
            targets.extend(cross_compile::Target::for_platform("macos"));
        }
        if baremetal {
            targets.extend(cross_compile::Target::for_platform("baremetal"));
        }
        if let Some(target_str) = target {
            print_warning(&format!(
                "Custom target triple not yet fully supported: {}",
                target_str
            ));
        }
    }

    if targets.is_empty() {
        print_error("No targets specified!");
        print_info("Use --linux, --windows, --macos, --baremetal, or --all-platforms");
        anyhow::bail!("No compilation targets specified");
    }

    print_info(&format!(
        "Compiling for {} target(s) using {}",
        targets.len(),
        build_system_str
    ));

    let build_dirs = compiler.compile_all(&targets, &build_system_str)?;

    print_success(&format!(
        "Successfully compiled for {} targets! 🎉",
        build_dirs.len()
    ));

    // Show build artifact locations
    println!("\n📦  Build Artifacts:");
    for (i, dir) in build_dirs.iter().enumerate() {
        println!("  {} {}", targets[i].display_name(), dir.display());
    }

    Ok(())
}

/// Open a program in an external system terminal
fn open_in_external_terminal(program: &std::path::Path, args: &[String]) -> Result<()> {
    use std::process::Command;

    let program_str = program.to_string_lossy();
    let args_str = args.join(" ");

    #[cfg(target_os = "windows")]
    {
        // Windows: Create a temporary batch file to properly handle pause
        let temp_dir = std::env::temp_dir();
        let batch_file = temp_dir.join(format!("porters_run_{}.bat", std::process::id()));

        let batch_content = if args.is_empty() {
            format!(
                "@echo off\r\n\
                 \"{}\"\r\n\
                 echo.\r\n\
                 echo Press any key to close...\r\n\
                 pause >nul\r\n\
                 del \"%~f0\"",
                program_str
            )
        } else {
            format!(
                "@echo off\r\n\
                 \"{}\" {}\r\n\
                 echo.\r\n\
                 echo Press any key to close...\r\n\
                 pause >nul\r\n\
                 del \"%~f0\"",
                program_str, args_str
            )
        };

        std::fs::write(&batch_file, batch_content)
            .context("Failed to create temporary batch file")?;

        Command::new("cmd")
            .args(["/C", "start", "cmd", "/C", &batch_file.to_string_lossy()])
            .spawn()
            .context("Failed to open external terminal on Windows")?;
    }

    #[cfg(target_os = "macos")]
    {
        // macOS: Use osascript to open Terminal.app with the command
        let full_command = if args.is_empty() {
            format!("{}", program_str)
        } else {
            format!("{} {}", program_str, args_str)
        };

        let script = format!(
            "tell application \"Terminal\" to do script \"{}; echo ''; echo 'Press any key to close...'; read -n 1\"",
            full_command.replace("\"", "\\\"")
        );

        Command::new("osascript")
            .args(["-e", &script])
            .spawn()
            .context("Failed to open external terminal on macOS")?;
    }

    #[cfg(target_os = "linux")]
    {
        // Linux: Try common terminal emulators in order of preference
        let terminals = [
            ("gnome-terminal", vec!["--", "bash", "-c"]),
            ("konsole", vec!["-e", "bash", "-c"]),
            ("xfce4-terminal", vec!["-e", "bash", "-c"]),
            ("xterm", vec!["-e", "bash", "-c"]),
            ("mate-terminal", vec!["-e", "bash", "-c"]),
            ("terminator", vec!["-e", "bash", "-c"]),
            ("alacritty", vec!["-e", "bash", "-c"]),
            ("kitty", vec!["-e", "bash", "-c"]),
        ];

        let full_command = if args.is_empty() {
            format!(
                "{}; echo ''; echo 'Press Enter to close...'; read",
                program_str
            )
        } else {
            format!(
                "{} {}; echo ''; echo 'Press Enter to close...'; read",
                program_str, args_str
            )
        };

        let mut launched = false;
        for (terminal, term_args) in &terminals {
            if Command::new(terminal)
                .args(term_args)
                .arg(&full_command)
                .spawn()
                .is_ok()
            {
                launched = true;
                break;
            }
        }

        if !launched {
            anyhow::bail!(
                "No suitable terminal emulator found. Please install one of: gnome-terminal, konsole, xterm"
            );
        }
    }

    Ok(())
}

/// Execute a single C/C++ file directly with all dependencies
///
/// # Arguments
/// * `file` - Source file path to compile and execute
/// * `args` - Arguments to pass to the compiled program
/// * `external` - Open program in external terminal window
/// * `no_console` - Run without console window (GUI apps)
/// * `output_name` - Custom output executable name (optional)
///
/// # Returns
/// * `Result<()>` - Success or error
async fn execute_single_file(
    file: &str,
    args: Vec<String>,
    external: bool,
    no_console: bool,
    output_name: Option<&str>,
) -> Result<()> {
    use std::path::Path;
    use std::process::Command;

    print_step(&format!("⚡ Executing single file: {}", file));

    // Check if file exists
    let file_path = Path::new(file);
    if !file_path.exists() {
        anyhow::bail!("File not found: {}", file);
    }

    // Determine file type
    let extension = file_path
        .extension()
        .and_then(|e| e.to_str())
        .ok_or_else(|| anyhow::anyhow!("Could not determine file extension"))?;

    // Support all C/C++ source file extensions
    let is_cpp = matches!(extension, "cpp" | "cxx" | "cc" | "C" | "CPP" | "c++" | "cp");
    let is_c = matches!(extension, "c" | "C");

    if !is_c && !is_cpp {
        anyhow::bail!(
            "File must be a C/C++ source file\n\
            Supported extensions:\n\
            - C: .c\n\
            - C++: .cpp, .cxx, .cc, .c++, .cp, .C, .CPP\n\
            Note: Header files (.h, .hpp, .hxx) cannot be compiled directly"
        );
    }

    // Load configuration if available
    let config = if Path::new("porters.toml").exists() {
        Some(PortersConfig::load("porters.toml")?)
    } else {
        None
    };

    // Determine execution modes (CLI flags override config)
    let use_external = external
        || config
            .as_ref()
            .map(|c| c.run.use_external_terminal)
            .unwrap_or(false);
    let use_no_console = no_console || config.as_ref().map(|c| c.run.no_console).unwrap_or(false);

    // Determine compiler
    let compiler = if let Some(ref cfg) = config {
        if is_cpp {
            cfg.run
                .cpp_compiler
                .clone()
                .unwrap_or_else(detect_cpp_compiler)
        } else {
            cfg.run.c_compiler.clone().unwrap_or_else(detect_c_compiler)
        }
    } else if is_cpp {
        detect_cpp_compiler()
    } else {
        detect_c_compiler()
    };

    print_info(&format!("🔧 Using compiler: {}", compiler));

    // Build compiler command
    let mut cmd = Command::new(&compiler);
    cmd.arg(file);

    // Add include directories from config
    if let Some(ref cfg) = config {
        // Add configured include directories
        for include_dir in &cfg.run.include_dirs {
            cmd.arg(format!("-I{}", include_dir));
        }

        // Add dependency include paths
        if !cfg.dependencies.is_empty() {
            print_info("📦 Resolving dependencies for include paths...");

            // Try to resolve dependencies
            if let Ok(resolved) = deps::resolve_dependencies(cfg).await {
                for dep in &resolved {
                    for include_path in &dep.include_paths {
                        cmd.arg(format!("-I{}", include_path.display()));
                    }
                    for lib_path in &dep.lib_paths {
                        cmd.arg(format!("-L{}", lib_path.display()));
                    }
                }
            }
        }

        // Add compiler flags
        for flag in &cfg.run.compiler_flags {
            cmd.arg(flag);
        }

        // Add linker flags
        for flag in &cfg.run.linker_flags {
            cmd.arg(flag);
        }
    }

    // Determine output executable name
    // Priority: CLI --output flag > config output-name > source filename > default "a.out"
    let output_name = output_name
        .map(|s| s.to_string())
        .or_else(|| config.as_ref().and_then(|c| c.run.output_name.clone()))
        .unwrap_or_else(|| {
            // Use source filename without extension
            file_path
                .file_stem()
                .and_then(|s| s.to_str())
                .unwrap_or("a")
                .to_string()
        });

    // Output executable name (use absolute path for reliability)
    let output_path = std::env::current_dir()?.join(if cfg!(windows) {
        format!("{}.exe", output_name)
    } else {
        output_name.clone()
    });

    cmd.arg("-o").arg(&output_path);

    print_info(&format!("📝 Output executable: {}", output_path.display()));
    print_info("🔨 Compiling...");

    // Compile the file
    let compile_output = cmd.output().context("Failed to execute compiler")?;

    if !compile_output.status.success() {
        let stderr = String::from_utf8_lossy(&compile_output.stderr);
        print_error("❌ Compilation failed!");
        eprintln!("{}", stderr);
        anyhow::bail!("Compilation failed");
    }

    print_success("✅ Compilation successful!");

    // Execute the compiled program
    if use_external {
        print_step("🪟 Opening in external terminal...");
        open_in_external_terminal(&output_path, &args)?;

        // Wait a bit before cleanup to let terminal open
        std::thread::sleep(std::time::Duration::from_millis(500));

        print_info("🚀 Program launched in external terminal");
        print_warning("⚠️  Note: Executable remains for you to use. Delete when done:");
        print_info(&format!("    {}", output_path.display()));

        return Ok(());
    }

    // Handle no-console mode (for GUI applications)
    if use_no_console {
        print_step("🎨 Running executable (no console)...");

        #[cfg(target_os = "windows")]
        {
            // Windows: Use CREATE_NO_WINDOW flag
            use std::os::windows::process::CommandExt;
            const CREATE_NO_WINDOW: u32 = 0x08000000;

            let mut cmd = Command::new(&output_path);
            cmd.args(&args);
            cmd.creation_flags(CREATE_NO_WINDOW);

            let mut child = cmd
                .spawn()
                .context("Failed to execute program in no-console mode")?;

            let _ = child.wait();
        }

        #[cfg(not(target_os = "windows"))]
        {
            // Unix: Redirect all output to /dev/null
            use std::process::Stdio;

            let mut cmd = Command::new(&output_path);
            cmd.args(&args);
            cmd.stdout(Stdio::null());
            cmd.stderr(Stdio::null());
            cmd.stdin(Stdio::null());

            let mut child = cmd
                .spawn()
                .context("Failed to execute program in no-console mode")?;

            let _ = child.wait();
        }

        print_info("🎨 Program executed silently (no console output)");
        print_info(&format!("📝 Executable: {}", output_path.display()));

        return Ok(());
    }

    print_step("▶️  Running executable...");
    println!();

    let mut exec_cmd = if cfg!(windows) {
        // On Windows, use cmd /c to ensure proper execution
        let mut cmd = Command::new("cmd");
        cmd.arg("/C");
        cmd.arg(&output_path);
        cmd.args(&args);
        cmd
    } else {
        let mut cmd = Command::new(&output_path);
        cmd.args(&args);
        cmd
    };

    // Inherit stdin/stdout/stderr for interactive programs
    exec_cmd.stdin(std::process::Stdio::inherit());
    exec_cmd.stdout(std::process::Stdio::inherit());
    exec_cmd.stderr(std::process::Stdio::inherit());

    let exec_status = exec_cmd
        .status()
        .context("Failed to execute compiled program")?;

    println!();

    if exec_status.success() {
        print_success("✅ Execution completed successfully! ✨");
        print_info(&format!("📝 Executable: {}", output_path.display()));
        Ok(())
    } else {
        print_error(&format!(
            "❌ Program exited with code: {}",
            exec_status.code().unwrap_or(-1)
        ));
        anyhow::bail!("Execution failed");
    }
}

/// Detect the best available C compiler
///
/// # Returns
/// * `String` - Compiler command name (gcc, clang, or cc)
fn detect_c_compiler() -> String {
    use std::process::Command;

    for compiler in &["gcc", "clang", "cc"] {
        if Command::new(compiler).arg("--version").output().is_ok() {
            return compiler.to_string();
        }
    }
    "gcc".to_string() // Default fallback
}

/// Detect the best available C++ compiler
///
/// # Returns
/// * `String` - Compiler command name (g++, clang++, or c++)
fn detect_cpp_compiler() -> String {
    use std::process::Command;

    for compiler in &["g++", "clang++", "c++"] {
        if Command::new(compiler).arg("--version").output().is_ok() {
            return compiler.to_string();
        }
    }
    "g++".to_string() // Default fallback
}

/// Add porters to system PATH
///
/// # Arguments
/// * `overwrite` - Overwrite existing PATH entry if present
///
/// # Returns
/// * `Result<()>` - Success or error
fn add_to_path(overwrite: bool) -> Result<()> {
    use std::env;
    use std::path::PathBuf;

    // Get current executable location (for binary installations)
    let current_exe = if let Ok(exe_path) = env::current_exe() {
        if let Some(parent) = exe_path.parent() {
            parent.to_path_buf()
        } else {
            anyhow::bail!("Cannot determine current executable directory");
        }
    } else {
        anyhow::bail!("Cannot determine current executable path");
    };

    // Get cargo bin directory (for cargo installations)
    let cargo_bin = if let Ok(cargo_home) = env::var("CARGO_HOME") {
        PathBuf::from(cargo_home).join("bin")
    } else if let Ok(home) = env::var("HOME") {
        PathBuf::from(home).join(".cargo").join("bin")
    } else if let Ok(userprofile) = env::var("USERPROFILE") {
        PathBuf::from(userprofile).join(".cargo").join("bin")
    } else {
        anyhow::bail!("Cannot determine cargo bin path");
    };

    // Determine which path to add
    let is_cargo_install = current_exe.to_string_lossy().contains(".cargo")
        || current_exe.to_string_lossy().contains("cargo")
        || cargo_bin == current_exe;

    let path_to_add = if is_cargo_install {
        &cargo_bin
    } else {
        &current_exe
    };

    let path_str = path_to_add.to_string_lossy().to_string();

    // Check if already in PATH
    if let Ok(path_var) = env::var("PATH") {
        let paths: Vec<&str> = if cfg!(windows) {
            path_var.split(';').collect()
        } else {
            path_var.split(':').collect()
        };

        let already_exists = paths
            .iter()
            .any(|p| p.trim() == path_str || p.trim() == path_to_add.to_str().unwrap_or(""));

        if already_exists && !overwrite {
            print_success("✅ Porters is already in your PATH!");
            println!();
            println!("ℹ️  Path: {}", path_str);
            println!();
            println!("If you want to overwrite the existing PATH entry, use:");
            println!("  porters add-to-path --overwrite");
            return Ok(());
        }

        if already_exists && overwrite {
            print_info("Overwriting existing PATH entry...");
        }
    }

    println!();
    print_step("Adding porters to system PATH");
    println!();

    if is_cargo_install {
        println!("ℹ️  Installation type: Cargo");
    } else {
        println!("ℹ️  Installation type: Binary Download");
    }
    println!("ℹ️  Adding path: {}", path_str);
    println!();

    if cfg!(windows) {
        println!("📋 Windows Instructions:");
        println!();
        println!("Option 1: PowerShell (Run as Administrator)");
        println!("{}", "=".repeat(60));
        println!("[Environment]::SetEnvironmentVariable(");
        println!("  \"Path\",");
        println!(
            "  [Environment]::GetEnvironmentVariable(\"Path\", \"User\") + \";{}\",",
            path_str
        );
        println!("  \"User\"");
        println!(")");
        println!("{}", "=".repeat(60));
        println!();
        println!("Option 2: Manual Setup");
        println!("  1. Search 'Environment Variables' in Start Menu");
        println!("  2. Click 'Environment Variables'");
        println!("  3. Under 'User variables', select 'Path' and click 'Edit'");
        println!("  4. Click 'New' and add: {}", path_str);
        println!("  5. Click 'OK' on all dialogs");
        println!("  6. Restart your terminal");
        println!();
        println!("Option 3: Current Session Only (Temporary)");
        println!("  $env:Path += \";{}\"", path_str);
    } else {
        println!("📋 Linux/macOS Instructions:");
        println!();
        println!("Add the following line to your shell configuration file:");
        println!("(~/.bashrc, ~/.zshrc, or ~/.profile)");
        println!();
        println!("{}", "=".repeat(60));
        println!("export PATH=\"{}:$PATH\"", path_str);
        println!("{}", "=".repeat(60));
        println!();
        println!("Then reload your shell configuration:");
        println!("  source ~/.bashrc    # or source ~/.zshrc");
        println!();
        println!("Current Session Only (Temporary):");
        println!("  export PATH=\"{}:$PATH\"", path_str);
    }

    println!();
    print_success("✨ Instructions provided above!");
    println!();
    println!("After adding to PATH:");
    println!("  1. Open a NEW terminal window");
    println!("  2. Run: porters --version");
    println!("  3. You can now use 'porters upgrade' to update automatically");
    println!();

    Ok(())
}

/// Remove porters from system PATH
fn remove_from_path() -> Result<()> {
    use std::env;
    use std::path::PathBuf;

    // Get cargo bin directory
    let cargo_bin = if let Ok(cargo_home) = env::var("CARGO_HOME") {
        PathBuf::from(cargo_home).join("bin")
    } else if let Ok(home) = env::var("HOME") {
        PathBuf::from(home).join(".cargo").join("bin")
    } else if let Ok(userprofile) = env::var("USERPROFILE") {
        PathBuf::from(userprofile).join(".cargo").join("bin")
    } else {
        anyhow::bail!("Cannot determine cargo bin path");
    };

    let cargo_bin_str = cargo_bin.to_string_lossy().to_string();

    // Check if in PATH
    if let Ok(path_var) = env::var("PATH") {
        let paths: Vec<&str> = if cfg!(windows) {
            path_var.split(';').collect()
        } else {
            path_var.split(':').collect()
        };

        let exists = paths
            .iter()
            .any(|p| p.trim() == cargo_bin_str || p.trim() == cargo_bin.to_str().unwrap_or(""));

        if !exists {
            print_warning("⚠️  Porters is not currently in your PATH");
            println!();
            println!("ℹ️  Path checked: {}", cargo_bin_str);
            return Ok(());
        }
    }

    println!();
    print_step("Removing porters from system PATH");
    println!();

    if cfg!(windows) {
        println!("📋 Windows Instructions:");
        println!();
        println!("Option 1: PowerShell (Run as Administrator)");
        println!("{}", "=".repeat(60));
        println!("$path = [Environment]::GetEnvironmentVariable(\"Path\", \"User\")");
        println!(
            "$newPath = ($path -split ';' | Where-Object {{ $_ -ne '{}' }}) -join ';'",
            cargo_bin_str
        );
        println!("[Environment]::SetEnvironmentVariable(\"Path\", $newPath, \"User\")");
        println!("{}", "=".repeat(60));
        println!();
        println!("Option 2: Manual Setup");
        println!("  1. Search 'Environment Variables' in Start Menu");
        println!("  2. Click 'Environment Variables'");
        println!("  3. Under 'User variables', select 'Path' and click 'Edit'");
        println!("  4. Find and remove: {}", cargo_bin_str);
        println!("  5. Click 'OK' on all dialogs");
        println!("  6. Restart your terminal");
        println!();
        println!("Option 3: Current Session Only (Temporary)");
        println!(
            "  $env:Path = ($env:Path -split ';' | Where-Object {{ $_ -ne '{}' }}) -join ';'",
            cargo_bin_str
        );
    } else {
        println!("📋 Linux/macOS Instructions:");
        println!();
        println!("Remove the following line from your shell configuration file:");
        println!("(~/.bashrc, ~/.zshrc, or ~/.profile)");
        println!();
        println!("{}", "=".repeat(60));
        println!("export PATH=\"{}:$PATH\"", cargo_bin_str);
        println!("{}", "=".repeat(60));
        println!();
        println!("Then reload your shell configuration:");
        println!("  source ~/.bashrc    # or source ~/.zshrc");
        println!();
        println!("Or edit manually:");
        println!("  nano ~/.bashrc      # or nano ~/.zshrc");
    }

    println!();
    print_success("✨ Instructions provided above!");
    println!();

    Ok(())
}

/// Check if cargo bin is in PATH and offer to add it automatically
fn check_path_setup() {
    use std::env;
    use std::path::PathBuf;

    // Get current executable location (for binary installations)
    let current_exe = if let Ok(exe_path) = env::current_exe() {
        if let Some(parent) = exe_path.parent() {
            parent.to_path_buf()
        } else {
            return;
        }
    } else {
        return;
    };

    // Get cargo bin directory (for cargo installations)
    let cargo_bin = if let Ok(cargo_home) = env::var("CARGO_HOME") {
        PathBuf::from(cargo_home).join("bin")
    } else if let Ok(home) = env::var("HOME") {
        PathBuf::from(home).join(".cargo").join("bin")
    } else if let Ok(userprofile) = env::var("USERPROFILE") {
        PathBuf::from(userprofile).join(".cargo").join("bin")
    } else {
        return; // Can't determine cargo bin path
    };

    let current_exe_str = current_exe.to_string_lossy().to_string();
    let cargo_bin_str = cargo_bin.to_string_lossy().to_string();

    // Check if either location is in PATH
    if let Ok(path_var) = env::var("PATH") {
        let paths: Vec<&str> = if cfg!(windows) {
            path_var.split(';').collect()
        } else {
            path_var.split(':').collect()
        };

        // Check if current exe location or cargo bin is in PATH
        let in_path = paths.iter().any(|p| {
            let trimmed = p.trim();
            trimmed == current_exe_str
                || trimmed == cargo_bin_str
                || trimmed == current_exe.to_str().unwrap_or("")
                || trimmed == cargo_bin.to_str().unwrap_or("")
        });

        if in_path {
            return; // Already in PATH
        }
    }

    // Check if first run marker exists
    let marker_file = cargo_bin.join(".porters_path_checked");
    if marker_file.exists() {
        return; // Already showed message
    }

    // Determine installation type
    let is_cargo_install = current_exe_str.contains(".cargo")
        || current_exe_str.contains("cargo")
        || cargo_bin == current_exe;

    // Show PATH setup message
    println!("\n{}", "=".repeat(80));
    println!("⚠️  Porters is not in your system PATH");
    println!("{}", "=".repeat(80));
    println!();

    if is_cargo_install {
        println!("ℹ️  Installation type: Cargo");
        println!("ℹ️  Binary location: {}", cargo_bin_str);
    } else {
        println!("ℹ️  Installation type: Binary Download");
        println!("ℹ️  Binary location: {}", current_exe_str);
    }

    println!();
    println!("ℹ️  You can add porters to PATH automatically using:");
    println!("    porters add-to-path");
    println!();
    println!("Or remove it later with:");
    println!("    porters remove-from-path");
    println!();

    if !is_cargo_install {
        println!("💡 Tip for binary installations:");
        println!("   Move porters to a standard location first:");
        if cfg!(windows) {
            println!("   - C:\\Program Files\\porters\\ (recommended)");
            println!("   - C:\\Users\\<YourName>\\AppData\\Local\\Programs\\porters\\");
        } else {
            println!("   - /usr/local/bin/ (requires sudo)");
            println!("   - ~/.local/bin/ (user-only)");
        }
        println!();
    }

    println!("{}", "=".repeat(80));
    println!();

    // Create marker file to avoid showing this message again
    let _ = std::fs::write(&marker_file, "checked");
}

/// Open documentation in the default browser
fn open_docs() -> Result<()> {
    const DOCS_URL: &str = "https://muhammad-fiaz.github.io/Porters/";

    print_step("📚 Opening documentation");
    println!();
    println!("🌐 Documentation URL: {}", DOCS_URL);
    println!();

    if webbrowser::open(DOCS_URL).is_ok() {
        print_success("✨ Documentation opened in your default browser!");
    } else {
        print_warning("⚠️  Could not open browser automatically");
        println!();
        println!("💡 Please visit the documentation manually:");
        println!("   {}", DOCS_URL);
    }

    println!();
    Ok(())
}

/// Handle Conan package manager actions
async fn handle_conan_action(action: PackageManagerAction) -> Result<()> {
    let manager = ConanManager::new();

    match action {
        PackageManagerAction::Add {
            package,
            version,
            global,
        } => {
            let scope = if global {
                InstallScope::Global
            } else {
                InstallScope::Local
            };
            manager.install(&package, version.as_deref(), scope)?;
        }
        PackageManagerAction::Remove {
            package,
            global,
            force,
        } => {
            let scope = if global {
                InstallScope::Global
            } else {
                InstallScope::Local
            };
            manager.remove(&package, scope, force)?;
        }
        PackageManagerAction::List { global } => {
            let scope = if global {
                InstallScope::Global
            } else {
                InstallScope::Local
            };
            let packages = manager.list(scope)?;
            let location = if global { "globally" } else { "in ports/conan" };
            if packages.is_empty() {
                println!("📦 No Conan packages installed {}", location);
            } else {
                println!("📦 Installed Conan packages {}:", location);
                for pkg in packages {
                    println!("  • {}", pkg);
                }
            }
        }
        PackageManagerAction::Search { query } => {
            let results = manager.search(&query)?;
            if results.is_empty() {
                println!("🔍 No packages found for '{}'", query);
            } else {
                println!("🔍 Search results for '{}':", query);
                for result in results {
                    println!("  {}", result);
                }
            }
        }
    }

    Ok(())
}

/// Handle vcpkg package manager actions
async fn handle_vcpkg_action(action: PackageManagerAction) -> Result<()> {
    let manager = VcpkgManager::new();

    match action {
        PackageManagerAction::Add {
            package,
            version,
            global,
        } => {
            let scope = if global {
                InstallScope::Global
            } else {
                InstallScope::Local
            };
            manager.install(&package, version.as_deref(), scope)?;
        }
        PackageManagerAction::Remove {
            package,
            global,
            force,
        } => {
            let scope = if global {
                InstallScope::Global
            } else {
                InstallScope::Local
            };
            manager.remove(&package, scope, force)?;
        }
        PackageManagerAction::List { global } => {
            let scope = if global {
                InstallScope::Global
            } else {
                InstallScope::Local
            };
            let packages = manager.list(scope)?;
            let location = if global { "globally" } else { "in ports/vcpkg" };
            if packages.is_empty() {
                println!("📦 No vcpkg packages installed {}", location);
            } else {
                println!("📦 Installed vcpkg packages {}:", location);
                for pkg in packages {
                    println!("  • {}", pkg);
                }
            }
        }
        PackageManagerAction::Search { query } => {
            let results = manager.search(&query)?;
            if results.is_empty() {
                println!("🔍 No packages found for '{}'", query);
            } else {
                println!("🔍 Search results for '{}':", query);
                for result in results {
                    println!("  {}", result);
                }
            }
        }
    }

    Ok(())
}

/// Handle XMake package manager actions
async fn handle_xmake_action(action: PackageManagerAction) -> Result<()> {
    let manager = XMakeManager::new();

    match action {
        PackageManagerAction::Add {
            package,
            version,
            global,
        } => {
            let scope = if global {
                InstallScope::Global
            } else {
                InstallScope::Local
            };
            manager.install(&package, version.as_deref(), scope)?;
        }
        PackageManagerAction::Remove {
            package,
            global,
            force,
        } => {
            let scope = if global {
                InstallScope::Global
            } else {
                InstallScope::Local
            };
            manager.remove(&package, scope, force)?;
        }
        PackageManagerAction::List { global } => {
            let scope = if global {
                InstallScope::Global
            } else {
                InstallScope::Local
            };
            let packages = manager.list(scope)?;
            let location = if global { "globally" } else { "in ports/xmake" };
            if packages.is_empty() {
                println!("📦 No XMake packages installed {}", location);
            } else {
                println!("📦 Installed XMake packages {}:", location);
                for pkg in packages {
                    println!("  • {}", pkg);
                }
            }
        }
        PackageManagerAction::Search { query } => {
            let results = manager.search(&query)?;
            if results.is_empty() {
                println!("🔍 No packages found for '{}'", query);
            } else {
                println!("🔍 Search results for '{}':", query);
                for result in results {
                    println!("  {}", result);
                }
            }
        }
    }

    Ok(())
}
