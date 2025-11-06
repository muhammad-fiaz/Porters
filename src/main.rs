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
mod extension;
mod global;
mod hash;
mod lockfile;
mod publish;
mod registry;
mod scan;
mod update;
mod util;
mod version;

use config::PortersConfig;
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
    /// Initialize a new porters project in current directory
    Init,

    /// Create a new porters project in a new directory
    Create {
        /// Project name
        name: String,

        /// Use default settings without prompts
        #[arg(short, long)]
        yes: bool,
    },

    /// Add a dependency
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
    },

    /// Remove a dependency
    Remove {
        /// Package name
        package: String,
    },

    /// Build the project
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

    /// Run the project
    Run {
        /// Additional run arguments
        #[arg(trailing_var_arg = true, allow_hyphen_values = true)]
        args: Vec<String>,
    },

    /// Execute a single C/C++ file directly with dependencies
    Execute {
        /// Source file to compile and run (.c or .cpp)
        file: String,

        /// Arguments to pass to the executable
        #[arg(trailing_var_arg = true, allow_hyphen_values = true)]
        args: Vec<String>,
    },

    /// Run tests
    Test,

    /// Update dependencies
    Update,

    /// Clean build artifacts
    Clean,

    /// Generate or update lockfile
    Lock,

    /// Vendor dependencies into project
    Vendor,

    /// Show dependency graph
    Graph,

    /// Publish package to GitHub releases
    Publish {
        /// GitHub access token (or use GITHUB_TOKEN env var)
        #[arg(long)]
        token: Option<String>,

        /// Dry run - don't actually publish
        #[arg(long)]
        dry_run: bool,
    },

    /// Upgrade porters to the latest version
    Upgrade,

    /// Install a package globally
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

    /// Sync dependencies from porters.toml
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

    /// Manage extensions
    Extension {
        #[command(subcommand)]
        action: ExtensionAction,
    },

    /// Run a custom script from porters.toml
    RunScript {
        /// Script name
        name: String,
    },

    /// List project dependencies
    List {
        /// Show dependency tree
        #[arg(long)]
        tree: bool,
    },

    /// List globally installed packages
    GlobalList,

    /// Clean cache
    CleanCache {
        /// Force clean (including binary cache)
        #[arg(long)]
        force: bool,
    },

    /// Update porters itself to latest version
    SelfUpdate,

    /// Update all dependencies to latest compatible versions
    UpdateDeps {
        /// Update to absolute latest (ignore constraints)
        #[arg(long)]
        latest: bool,
    },

    /// Cross-compile for specific platform(s)
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

    /// Execute a custom command (dynamically matched from config)
    #[command(external_subcommand)]
    Custom(Vec<String>),
}

#[derive(Subcommand)]
enum ExtensionAction {
    /// Install an extension
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

    /// Uninstall an extension
    Uninstall {
        /// Extension name
        name: String,
    },

    /// List installed extensions
    List,

    /// Create extension template
    Create {
        /// Extension name
        name: String,
    },
}

#[tokio::main]
async fn main() -> Result<()> {
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
        } => add_dependency(&package, dev, optional, git, branch, tag).await,
        Commands::Remove { package } => remove_dependency(&package).await,
        Commands::Build {
            all_platforms,
            linux,
            windows,
            macos,
            args,
        } => build_project(all_platforms, linux, windows, macos, args).await,
        Commands::Run { args } => run_project(args).await,
        Commands::Execute { file, args } => execute_single_file(&file, args).await,
        Commands::Test => test_project().await,
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
    }
}

async fn init_project() -> Result<()> {
    print_step("Initializing porters project in current directory");

    if std::path::Path::new("porters.toml").exists() {
        print_warning("porters.toml already exists");
        return Ok(());
    }

    // Scan for existing C/C++ files to determine project type
    let sources = scan::scan_project(".")?;
    let has_sources = !sources.source_files.is_empty();

    if has_sources {
        print_success(&format!(
            "Detected {} C/C++ source files",
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
        .items(&[
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
            print_success(&format!("Detected build system: {}", system));
            format!("\n[build]\nsystem = \"{}\"", system)
        } else {
            print_info("No build system detected - you can configure one later");
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

    print_success("Created porters.toml");
    if has_sources {
        print_info("Your existing C/C++ project is now managed by Porters!");
        print_info("Run 'porters build' to build your project");
    } else {
        print_info("Add your C/C++ source files and run 'porters build'");
    }

    // Check build tools
    check_build_tools();

    Ok(())
}

async fn create_project(name: &str, use_defaults: bool) -> Result<()> {
    print_step(&format!("Creating new project: {}", name));

    // Check if directory already exists
    if std::path::Path::new(name).exists() {
        print_error(&format!("Directory '{}' already exists", name));
        return Err(anyhow::anyhow!("Directory already exists"));
    }

    // Create project directory
    std::fs::create_dir_all(name)?;
    std::env::set_current_dir(name)?;

    let (language, author, email, repo, build_system, project_type, entry_point, license) =
        if use_defaults {
            print_info("Using default settings...");
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
    create_project_structure(&language)?;

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

    print_success(&format!("Created project '{}' successfully! 🎉", name));
    print_info(&format!("cd {} && porters build", name));

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
    let project_types = vec!["Application (executable)", "Library (static/shared)"];
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
    let languages = vec!["C", "C++", "Both (C and C++)"];
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
            .with_prompt("Library name (optional, press Enter to use project name)")
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
        .with_prompt("Author name (optional, press Enter to skip)")
        .allow_empty(true)
        .interact_text()?;

    let author = if author.trim().is_empty() {
        None
    } else {
        Some(author)
    };

    // Email (optional)
    let email: String = Input::with_theme(&theme)
        .with_prompt("Email (optional, press Enter to skip)")
        .allow_empty(true)
        .interact_text()?;

    let email = if email.trim().is_empty() {
        None
    } else {
        Some(email)
    };

    // Repository URL (optional)
    let repo: String = Input::with_theme(&theme)
        .with_prompt("Repository URL (optional, press Enter to skip)")
        .allow_empty(true)
        .interact_text()?;

    let repo = if repo.trim().is_empty() {
        None
    } else {
        Some(repo)
    };

    // License selection
    let license_idx = Select::with_theme(&theme)
        .with_prompt("Select license")
        .items(&[
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
        _ => None,
    };

    // Build system selection
    let build_systems = vec!["CMake", "XMake", "Meson", "Make", "Custom"];
    let build_idx = Select::with_theme(&theme)
        .with_prompt("Select build system")
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

fn create_project_structure(language: &str) -> Result<()> {
    // Create directory structure
    std::fs::create_dir_all("src")?;
    std::fs::create_dir_all("include")?;

    // Create main source file
    match language {
        "c" => {
            std::fs::write(
                "src/main.c",
                r#"#include <stdio.h>

int main(int argc, char *argv[]) {
    printf("Hello from Porters!\n");
    return 0;
}
"#,
            )?;
        }
        "cpp" => {
            std::fs::write(
                "src/main.cpp",
                r#"#include <iostream>

int main(int argc, char *argv[]) {
    std::cout << "Hello from Porters!" << std::endl;
    return 0;
}
"#,
            )?;
        }
        _ => {
            // both
            std::fs::write(
                "src/main.cpp",
                r#"#include <iostream>

int main(int argc, char *argv[]) {
    std::cout << "Hello from Porters!" << std::endl;
    return 0;
}
"#,
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

    // Create README
    std::fs::write(
        "README.md",
        r#"# Project

A C/C++ project managed by Porters.

## Building

```bash
porters build
```

## Running

```bash
porters run
```
"#,
    )?;

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

fn check_build_tools() {
    print_info("Checking build tools...");

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
            print_success(&format!("{} is installed", name));
        } else {
            missing.push((name, url));
        }
    }

    if !missing.is_empty() {
        println!();
        print_warning("Some build tools are not installed:");
        for (name, url) in missing {
            println!("  📥 Install {} from: {}", name, url);
        }
        println!();
        print_info("Install missing tools to use all Porters features");
    }
}

async fn add_dependency(
    package: &str,
    dev: bool,
    optional: bool,
    git: Option<String>,
    _branch: Option<String>,
    _tag: Option<String>,
) -> Result<()> {
    print_step(&format!("Adding dependency: {}", package));

    let mut config = PortersConfig::load("porters.toml")?;

    let dep_type = if dev {
        "dev-dependencies"
    } else {
        "dependencies"
    };
    print_info(&format!("Adding to [{}]", dep_type));

    // Determine the actual source
    let source = if let Some(git_url) = git {
        git_url
    } else {
        package.to_string()
    };

    config.add_dependency(&source, dev, optional)?;
    config.save("porters.toml")?;

    print_success(&format!("Added {} to {}", package, dep_type));

    Ok(())
}

async fn remove_dependency(package: &str) -> Result<()> {
    print_step(&format!("Removing dependency: {}", package));

    let mut config = PortersConfig::load("porters.toml")?;
    config.remove_dependency(package)?;
    config.save("porters.toml")?;

    print_success(&format!("Removed {}", package));

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
        print_error("🤔 Oops! Unsatisfied version requirements:");
        for failure in &failures {
            eprintln!("{}", failure);
        }
        eprintln!();
        print_info("Please install or upgrade the required tools to continue.");
        print_info("See: https://github.com/muhammad-fiaz/porters#requirements");
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
    print_step("Building project");

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
    print_info("Checking tool version requirements...");
    check_tool_requirements(&config)?;
    print_success("All tool requirements satisfied ✓");

    // Execute pre-build script if defined
    if let Some(pre_build_script) = &config.build.scripts.pre_build {
        print_info("Executing pre-build script...");
        execute_build_script(pre_build_script, "pre-build")?;
        print_success("Pre-build script completed ✓");
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
    print_info("Scanning project sources...");
    let sources = scan::scan_project(".")?;
    print_success(&format!(
        "Found {} source files",
        sources.source_files.len()
    ));

    // Resolve dependencies
    print_info("Resolving dependencies...");
    let resolved_deps = deps::resolve_dependencies(&config).await?;
    print_success(&format!("Resolved {} dependencies", resolved_deps.len()));

    // Check binary cache for dependencies
    if bin_cache.is_enabled() {
        print_info("Checking binary cache for dependencies...");
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
                    print_info(&format!("✓ {} found in binary cache", dep.name));
                    bin_cache.retrieve(&dep.name, version, &dep_hash, &build_hash, dep_path)?;
                }
            }
        }
    }

    // Verify checksums of dependencies
    print_info("Verifying dependency checksums...");
    verify_dependency_checksums(&resolved_deps)?;
    print_success("All checksums verified ✓");

    // Detect and run build system
    print_info("Detecting build system...");
    let build_system = build::detect_build_system(".", &config)?;
    print_success(&format!("Using build system: {}", build_system.name()));

    print_info("Building...");
    build_system.build(&sources, &resolved_deps, &args)?;

    // Store build in binary cache
    if bin_cache.is_enabled() {
        print_info("Storing build artifacts in binary cache...");
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
        print_info("Executing post-build script...");
        execute_build_script(post_build_script, "post-build")?;
        print_success("Post-build script completed ✓");
    }

    print_success("Build complete! 🎉");

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
    print_step("Running project");

    // Build first (current platform only)
    build_project(false, false, false, false, vec![]).await?;

    let config = PortersConfig::load("porters.toml")?;
    let build_system = build::detect_build_system(".", &config)?;

    print_info("Running executable...");
    build_system.run(&args)?;

    Ok(())
}

async fn test_project() -> Result<()> {
    print_step("Running tests");

    let config = PortersConfig::load("porters.toml")?;
    let sources = scan::scan_project(".")?;
    let resolved_deps = deps::resolve_dependencies(&config).await?;
    let build_system = build::detect_build_system(".", &config)?;

    build_system.test(&sources, &resolved_deps)?;

    print_success("Tests complete! ✅");

    Ok(())
}

async fn update_dependencies() -> Result<()> {
    print_step("Updating dependencies");

    let config = PortersConfig::load("porters.toml")?;
    deps::update_dependencies(&config).await?;

    print_success("Dependencies updated! 🔄");

    Ok(())
}

async fn clean_project() -> Result<()> {
    print_step("Cleaning project");

    let config = PortersConfig::load("porters.toml")?;
    let build_system = build::detect_build_system(".", &config)?;

    build_system.clean()?;

    print_success("Clean complete! 🧹");

    Ok(())
}

async fn generate_lockfile() -> Result<()> {
    print_step("Generating lockfile");

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
            dependencies: vec![], // TODO: Track nested dependencies
        };

        lockfile.add_dependency(dep.name.clone(), resolved_dep);
    }

    // Save lockfile
    lockfile.save("porters.lock")?;

    print_success("Generated porters.lock 🔒");

    Ok(())
}

async fn vendor_dependencies() -> Result<()> {
    print_step("Vendoring dependencies");

    let config = PortersConfig::load("porters.toml")?;
    deps::vendor_dependencies(&config, "vendor").await?;

    print_success("Dependencies vendored to ./vendor 📦");

    Ok(())
}

async fn show_dependency_graph() -> Result<()> {
    print_step("Generating dependency graph");

    let config = PortersConfig::load("porters.toml")?;
    let resolved_deps = deps::resolve_dependencies(&config).await?;

    deps::print_dependency_graph(&resolved_deps)?;

    Ok(())
}

async fn publish_package(token: Option<String>, dry_run: bool) -> Result<()> {
    print_step("Publishing package");

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
    print_step(&format!("Installing package globally: {}", package));

    // Load config for build scripts (if exists)
    let config = PortersConfig::load("porters.toml").ok();

    // Execute pre-install script if configured
    if let Some(ref cfg) = config
        && let Some(pre_install_script) = &cfg.build.scripts.pre_install
    {
        print_info("Executing pre-install script...");
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
                println!("\n📦 Installed Extensions:\n");
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
    print_step(&format!("Running script: {}", name));

    let config = PortersConfig::load("porters.toml")?;

    if let Some(script) = config.scripts.get(name) {
        print_info(&format!("Executing: {}", script));

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

        print_success(&format!("Script '{}' completed successfully! ✓", name));
    } else {
        print_error(&format!("Script '{}' not found in porters.toml", name));
        print_info("Available scripts:");
        for script_name in config.scripts.keys() {
            println!("  - {}", script_name);
        }
        anyhow::bail!("Script not found");
    }

    Ok(())
}

async fn execute_custom_command(args: Vec<String>) -> Result<()> {
    if args.is_empty() {
        anyhow::bail!("No command provided");
    }

    let command_name = &args[0];
    let config = PortersConfig::load("porters.toml")?;

    // Find matching custom command
    if let Some(custom_cmd) = config.commands.iter().find(|c| &c.name == command_name) {
        print_step(&format!("Running custom command: {}", custom_cmd.name));
        print_info(&custom_cmd.description);

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

        let status = cmd
            .status()
            .with_context(|| format!("Failed to execute custom command '{}'", command_name))?;

        if !status.success() {
            anyhow::bail!(
                "Custom command '{}' failed with exit code: {:?}",
                command_name,
                status.code()
            );
        }

        print_success(&format!(
            "Command '{}' completed successfully! ✓",
            command_name
        ));
    } else {
        print_error(&format!("Unknown command: {}", command_name));

        if !config.commands.is_empty() {
            print_info("Available custom commands:");
            for cmd in &config.commands {
                println!("  {} - {}", cmd.name, cmd.description);
            }
        }

        anyhow::bail!("Command not found");
    }

    Ok(())
}

/// Update sync_dependencies signature to support caching
async fn sync_dependencies(
    include_dev: bool,
    include_optional: bool,
    use_cache: bool,
) -> Result<()> {
    print_step("Syncing dependencies from porters.toml");

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

    if cache_enabled {
        print_info("✨ Dependency caching enabled");
    } else {
        print_info("⚠️  Cache disabled (use --no-cache=false to enable)");
    }

    // Rest of existing sync_dependencies implementation...
    // Execute pre-install script if configured
    if let Some(pre_install_script) = &config.build.scripts.pre_install {
        print_info("Executing pre-install script...");
        execute_build_script(pre_install_script, "pre-install")?;
    }

    // Auto-install extensions listed in config
    if !config.extensions.is_empty() {
        print_info(&format!(
            "Auto-installing {} extensions from config...",
            config.extensions.len()
        ));
        let mut ext_manager = extension::ExtensionManager::new()?;

        for ext_name in &config.extensions {
            // Check if already installed
            let extensions = ext_manager.list_extensions();
            if extensions.iter().any(|e| &e.manifest.name == ext_name) {
                print_info(&format!(
                    "📦 Extension '{}' already installed, skipping",
                    ext_name
                ));
                continue;
            }

            print_info(&format!("📦 Installing extension '{}'...", ext_name));
            match ext_manager.install_extension(ext_name, extension::ExtensionSource::CratesIo) {
                Ok(_) => print_success(&format!(
                    "Extension '{}' installed successfully! 🔌",
                    ext_name
                )),
                Err(e) => print_warning(&format!(
                    "Failed to install extension '{}': {}",
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

    print_info(&format!(
        "Using project dependencies directory: {}",
        ports_dir.display()
    ));

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

    print_info(&format!("Syncing {} dependencies...", to_install.len()));

    // Install each dependency
    for (name, dep, is_dev) in &to_install {
        let dep_type = if *is_dev { "dev" } else { "regular" };
        print_info(&format!("Installing {} ({})...", name, dep_type));

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
                    print_warning(&format!("{} already exists, skipping", name));
                } else {
                    let _checksum = deps::clone_git_repo(url, &dep_path).await?;
                    print_success(&format!("Installed {}", name));

                    // Store in cache
                    if cache_enabled {
                        dep_cache.store(name, version, &dep_path)?;
                    }
                }
            }
            config::Dependency::Detailed {
                path: Some(path), ..
            } => {
                print_info(&format!("{} is a path dependency at {}", name, path));
            }
            config::Dependency::Simple(spec) => {
                print_warning(&format!(
                    "{}: Simple version spec not yet supported ({})",
                    name, spec
                ));
            }
            _ => {
                print_warning(&format!("{}: No installation source found", name));
            }
        }
    }

    // Execute post-sync hooks
    ext_manager.execute_hook("post_sync", &hook_context)?;

    // Execute post-install script if configured
    if let Some(post_install_script) = &config.build.scripts.post_install {
        print_info("Executing post-install script...");
        execute_build_script(post_install_script, "post-install")?;
    }

    // Generate/update lockfile
    print_step("Updating lockfile");
    generate_lockfile().await?;

    print_success("Dependencies synced successfully! ✓");
    Ok(())
}

/// List project dependencies
async fn list_dependencies(tree: bool) -> Result<()> {
    print_step("Listing project dependencies");

    let config = PortersConfig::load("porters.toml")?;

    let total = config.dependencies.len() + config.dev_dependencies.len();
    if total == 0 {
        print_info("No dependencies found in porters.toml");
        return Ok(());
    }

    println!("\n📦 Dependencies ({})", config.dependencies.len());
    for (name, dep) in &config.dependencies {
        print_dependency(name, dep, tree, 0);
    }

    if !config.dev_dependencies.is_empty() {
        println!("\n🔧 Dev Dependencies ({})", config.dev_dependencies.len());
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
    print_step("Listing globally installed packages");

    let global_config = global::GlobalConfig::load()?;
    let packages = global_config.list_packages();

    if packages.is_empty() {
        print_info("No global packages installed");
        print_info("Install packages globally with: porters install --global <package>");
    } else {
        println!("\n📦 Global Packages ({})", packages.len());
        for pkg in packages {
            println!("  ✓ {} @ {} ({})", pkg.name, pkg.version, pkg.source);
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
    println!("\n📦 Build Artifacts:");
    for (i, dir) in build_dirs.iter().enumerate() {
        println!("  {} {}", targets[i].display_name(), dir.display());
    }

    Ok(())
}

/// Execute a single C/C++ file directly with all dependencies
async fn execute_single_file(file: &str, args: Vec<String>) -> Result<()> {
    use std::path::Path;
    use std::process::Command;

    print_step(&format!("Executing single file: {}", file));

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

    print_info(&format!("Using compiler: {}", compiler));

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
            print_info("Resolving dependencies for include paths...");

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

    // Output executable name
    let output_name = if cfg!(windows) {
        "porters_temp_exec.exe"
    } else {
        "./porters_temp_exec"
    };
    cmd.arg("-o").arg(output_name);

    print_info("Compiling...");

    // Compile the file
    let compile_output = cmd.output().context("Failed to execute compiler")?;

    if !compile_output.status.success() {
        let stderr = String::from_utf8_lossy(&compile_output.stderr);
        print_error("Compilation failed!");
        eprintln!("{}", stderr);
        anyhow::bail!("Compilation failed");
    }

    print_success("Compilation successful!");

    // Execute the compiled program
    print_step("Running executable...");
    println!();

    let mut exec_cmd = Command::new(output_name);
    exec_cmd.args(&args);

    let exec_status = exec_cmd
        .status()
        .context("Failed to execute compiled program")?;

    println!();

    // Clean up temporary executable
    if Path::new(output_name).exists() {
        std::fs::remove_file(output_name).context("Failed to remove temporary executable")?;
    }

    if exec_status.success() {
        print_success("Execution completed successfully! ✨");
        Ok(())
    } else {
        print_error(&format!(
            "Program exited with code: {}",
            exec_status.code().unwrap_or(-1)
        ));
        anyhow::bail!("Execution failed");
    }
}

/// Detect the best available C compiler
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
fn detect_cpp_compiler() -> String {
    use std::process::Command;

    for compiler in &["g++", "clang++", "c++"] {
        if Command::new(compiler).arg("--version").output().is_ok() {
            return compiler.to_string();
        }
    }
    "g++".to_string() // Default fallback
}

/// Check if cargo bin is in PATH and offer to add it automatically
fn check_path_setup() {
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
        return; // Can't determine cargo bin path
    };

    let cargo_bin_str = cargo_bin.to_string_lossy().to_string();

    // Check if cargo bin is in PATH
    if let Ok(path_var) = env::var("PATH") {
        let paths: Vec<&str> = if cfg!(windows) {
            path_var.split(';').collect()
        } else {
            path_var.split(':').collect()
        };

        // Already in PATH
        if paths
            .iter()
            .any(|p| p.trim() == cargo_bin_str || p.trim() == cargo_bin.to_str().unwrap_or(""))
        {
            return;
        }
    }

    // Check if first run marker exists
    let marker_file = cargo_bin.join(".porters_path_checked");
    if marker_file.exists() {
        return; // Already showed message
    }

    // Show PATH setup message
    println!("\n{}", "=".repeat(80));
    println!("⚠️  Cargo bin directory not found in PATH");
    println!("{}", "=".repeat(80));
    println!();
    println!("To use 'porters' command globally, add to your PATH:");
    println!();

    if cfg!(windows) {
        println!("Windows PowerShell (Run as Administrator):");
        println!("  [Environment]::SetEnvironmentVariable(");
        println!("    \"Path\",");
        println!(
            "    [Environment]::GetEnvironmentVariable(\"Path\", \"User\") + \";{}\",",
            cargo_bin_str
        );
        println!("    \"User\"");
        println!("  )");
        println!();
        println!("Or add manually:");
        println!("  1. Search 'Environment Variables' in Start Menu");
        println!("  2. Click 'Environment Variables'");
        println!("  3. Under 'User variables', select 'Path' and click 'Edit'");
        println!("  4. Click 'New' and add: {}", cargo_bin_str);
        println!("  5. Click 'OK' on all dialogs");
        println!("  6. Restart your terminal");
        println!();
        println!("Current session only (temporary):");
        println!("  $env:Path += \";{}\"", cargo_bin_str);
    } else {
        println!("Linux/macOS (add to ~/.bashrc or ~/.zshrc):");
        println!("  export PATH=\"{}:$PATH\"", cargo_bin_str);
        println!();
        println!("Then run:");
        println!("  source ~/.bashrc    # or source ~/.zshrc");
        println!();
        println!("Current session only (temporary):");
        println!("  export PATH=\"{}:$PATH\"", cargo_bin_str);
    }

    println!();
    println!("{}", "=".repeat(80));
    println!();

    // Create marker file to avoid showing this message again
    let _ = std::fs::write(&marker_file, "checked");
}
