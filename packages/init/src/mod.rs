mod project_metadata;

use std::env;
use colored::*;
use dialoguer::{theme::ColorfulTheme, Input, Select};
use serde::{Serialize, Deserialize};
use std::fs::{self, File};
use std::io::{self, Write};

#[derive(Serialize, Deserialize, Debug)]
struct PortersConfig {
    build_system: BuildSystem,
    project: Project,
    dependencies: Option<Dependencies>,
    optional_dependencies: Option<Dependencies>,
    build: Build,
    scripts: Option<Scripts>,
    env: Option<Env>,
}

#[derive(Serialize, Deserialize, Debug)]
struct BuildSystem {
    requires: Vec<String>,
    build_backend: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct Project {
    name: String,
    description: String,
    version: String,
    license: String,
    requires_cpp: String,
    keywords: Vec<String>,
    maintainers: Vec<Contact>,
    authors: Vec<Contact>,
}

#[derive(Serialize, Deserialize, Debug)]
struct Contact {
    name: String,
    email: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct Dependencies {
    dependencies: Vec<Dependency>,
}

#[derive(Serialize, Deserialize, Debug)]
struct Dependency {
    name: String,
    version: Option<String>,
    git: Option<String>,
    rev: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
struct Build {
    toolchain: String,
    flags: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug)]
struct Scripts {
    pre_build: Option<Vec<String>>,
    post_build: Option<Vec<String>>,
}

#[derive(Serialize, Deserialize, Debug)]
struct Env {
    variables: Vec<EnvVar>,
}

#[derive(Serialize, Deserialize, Debug)]
struct EnvVar {
    key: String,
    value: String,
}

pub fn run() {
    println!("{}", "Initializing a new Porters project...".green());

    let project_name: String = Input::with_theme(&ColorfulTheme::default())
        .with_prompt("Project name")
        .default("porters project".into())
        .interact_text()
        .unwrap();

    let project_description: String = Input::with_theme(&ColorfulTheme::default())
        .with_prompt("Project description")
        .default("A modern C++ library for high-performance computing.".into())
        .interact_text()
        .unwrap();

    let project_version: String = Input::with_theme(&ColorfulTheme::default())
        .with_prompt("Project version")
        .default("0.1.0".into())
        .interact_text()
        .unwrap();

    let project_license: String = Input::with_theme(&ColorfulTheme::default())
        .with_prompt("Project license")
        .default("MIT".into())
        .interact_text()
        .unwrap();

    let requires_cpp: String = Input::with_theme(&ColorfulTheme::default())
        .with_prompt("Minimum C++ version")
        .default(">=17".into())
        .interact_text()
        .unwrap();

    let maintainer_name: String = Input::with_theme(&ColorfulTheme::default())
        .with_prompt("Maintainer name")
        .default("Your Name".into())
        .interact_text()
        .unwrap();

    let maintainer_email: String = Input::with_theme(&ColorfulTheme::default())
        .with_prompt("Maintainer email")
        .default("your-email@example.com".into())
        .interact_text()
        .unwrap();

    let author_name: String = Input::with_theme(&ColorfulTheme::default())
        .with_prompt("Author name")
        .default("Your Name".into())
        .interact_text()
        .unwrap();

    let author_email: String = Input::with_theme(&ColorfulTheme::default())
        .with_prompt("Author email")
        .default("your-email@example.com".into())
        .interact_text()
        .unwrap();

    let build_toolchain: String = Input::with_theme(&ColorfulTheme::default())
        .with_prompt("Build toolchain (e.g., gcc, clang)")
        .default("clang".into())
        .interact_text()
        .unwrap();

    let build_flags: String = Input::with_theme(&ColorfulTheme::default())
        .with_prompt("Build flags (comma separated)")
        .default("-O2,-Wall".into())
        .interact_text()
        .unwrap();

    let build_system = BuildSystem {
        requires: vec!["cmake>=3.20".to_string(), "ninja>=1.10".to_string()],
        build_backend: "porters.build".to_string(),
    };

    let project = Project {
        name: project_name,
        description: project_description,
        version: project_version,
        license: project_license,
        requires_cpp,
        keywords: vec!["c++".to_string(), "high-performance".to_string(), "library".to_string()],
        maintainers: vec![Contact {
            name: maintainer_name,
            email: maintainer_email,
        }],
        authors: vec![Contact {
            name: author_name,
            email: author_email,
        }],
    };

    let build = Build {
        toolchain: build_toolchain,
        flags: build_flags.split(',').map(|s| s.trim().to_string()).collect(),
    };

    let porters_config = PortersConfig {
        build_system,
        project,
        dependencies: None,
        optional_dependencies: None,
        build,
        scripts: None,
        env: None,
    };

    let toml = toml::to_string(&porters_config).unwrap();
    let mut file = File::create("porters.toml").unwrap();
    file.write_all(toml.as_bytes()).unwrap();

    println!("{}", "porters.toml has been created successfully!".green());
}

pub fn create_new_project(project_name: &str) {
    println!("{}", format!("Creating a new project: {}", project_name).green());
    fs::create_dir(project_name).unwrap();
    env::set_current_dir(project_name).unwrap();

    let project_type = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Select project type")
        .default(0)
        .item("C")
        .item("C++")
        .interact()
        .unwrap();

    let project_type_str = if project_type == 0 { "C" } else { "C++" };

    run();

    let entry_point: String = Input::with_theme(&ColorfulTheme::default())
        .with_prompt("Entry point file")
        .default(if project_type == 0 { "src/main.c" } else { "src/main.cpp" }.into())
        .interact_text()
        .unwrap();

    let main_file_content = if project_type == 0 {
        r#"#include <stdio.h>

int main() {
    printf("Hello, World!\n");
    return 0;
}
"#
    } else {
        r#"#include <iostream>

int main() {
    std::cout << "Hello, World!" << std::endl;
    return 0;
}
"#
    };

    fs::create_dir_all("src").unwrap();
    let mut file = File::create(entry_point).unwrap();
    file.write_all(main_file_content.as_bytes()).unwrap();

    println!(
        "{}: {}",
        "Created".green(),
        if project_type == 0 { "src/main.c" } else { "src/main.cpp" }
    );
}

fn prompt(message: &str, default: &str) -> String {
    print!("{} [{}]: ", message.blue(), default);
    io::stdout().flush().unwrap();
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let input = input.trim();
    if input.is_empty() {
        default.to_string()
    } else {
        input.to_string()
    }
}