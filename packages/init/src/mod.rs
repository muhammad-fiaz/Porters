mod project_metadata;

use std::env;
use std::collections::HashMap;
use colored::*;
use dialoguer::{theme::ColorfulTheme, Input, Select};
use serde::{Serialize, Deserialize, ser::SerializeStruct};
use std::fs::{self, File};
use std::io::{ Write};

#[derive(Serialize, Deserialize, Debug)]
struct PortersConfig {
    build_system: BuildSystem,
    project: Project,
    dependencies: Option<HashMap<String, Dependency>>,
    optional_dependencies: Option<HashMap<String, Dependency>>,
    build: Build,
    scripts: Option<Scripts>,
    env: Option<Env>,
}

#[derive(Serialize, Deserialize, Debug)]
struct BuildSystem {
    requires: Vec<String>,
    build_backend: String,
}

#[derive( Deserialize, Debug)]
struct Project {
    name: String,
    description: Option<String>,
    project_type: String,
    version: String,
    license: Option<String>,
    requires_cpp: Option<String>,
    keywords: Option<Vec<String>>,
    maintainers: Option<Vec<Contact>>,
    authors: Option<Vec<Contact>>,
    homepage: Option<String>,
    documentation: Option<String>,
    repository: Option<String>,
    readme: Option<String>,
}

impl Serialize for Project {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut s = serializer.serialize_struct("Project", 4)?;
        s.serialize_field("name", &self.name)?;
        s.serialize_field("project_type", &self.project_type)?;
        s.serialize_field("version", &self.version)?;
        if let Some(description) = &self.description {
            s.serialize_field("description", description)?;
        }
        if let Some(license) = &self.license {
            s.serialize_field("license", license)?;
        }
        if let Some(requires_cpp) = &self.requires_cpp {
            s.serialize_field("requires_cpp", requires_cpp)?;
        }
        if let Some(keywords) = &self.keywords {
            s.serialize_field("keywords", keywords)?;
        }
        if let Some(maintainers) = &self.maintainers {
            s.serialize_field("maintainers", maintainers)?;
        }
        if let Some(authors) = &self.authors {
            s.serialize_field("authors", authors)?;
        }
        if let Some(homepage) = &self.homepage {
            s.serialize_field("homepage", homepage)?;
        }
        if let Some(documentation) = &self.documentation {
            s.serialize_field("documentation", documentation)?;
        }
        if let Some(repository) = &self.repository {
            s.serialize_field("repository", repository)?;
        }
        if let Some(readme) = &self.readme {
            s.serialize_field("readme", readme)?;
        }
        s.end()
    }
}

#[derive(Serialize, Deserialize, Debug)]
struct Contact {
    name: String,
    email: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct Dependency {
    version: Option<String>,
    git: Option<String>,
    rev: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
struct Build {
    toolchain: String,
    flags: Vec<String>,
    targets: Vec<BuildTarget>,
}

#[derive(Serialize, Deserialize, Debug)]
struct BuildTarget {
    name: String,
    platform: Option<String>,
    type_: String,
    sources: Option<Vec<String>>,
}

#[derive(Serialize, Deserialize, Debug)]
struct Scripts {
    build: String,
    test: String,
    run: String,
    clean: String,
    exec: String,
    install: String,
    uninstall: String,
    package: String,
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

    let project_type = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Select project type")
        .default(0)
        .item("Library")
        .item("Executable")
        .interact()
        .unwrap();

    let project_description: Option<String> = optional_input("Project description", "A modern C++ library for high-performance computing.");
    let project_version: String = Input::with_theme(&ColorfulTheme::default())
        .with_prompt("Project version")
        .default("0.1.0".into())
        .interact_text()
        .unwrap();

    let project_license: Option<String> = optional_input("Project license", "MIT");
    let requires_cpp: Option<String> = optional_input("Minimum C++ version", ">=17");

    let maintainer_name: Option<String> = optional_input("Maintainer name", "Your Name");
    let maintainer_email: Option<String> = optional_input("Maintainer email", "your-email@example.com");

    let author_name: Option<String> = optional_input("Author name", "Your Name");
    let author_email: Option<String> = optional_input("Author email", "your-email@example.com");

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

    let requires: Vec<String> = vec![
        "cmake>=3.20".to_string(),
        "ninja>=1.10".to_string(),
        "porters".to_string(),
    ];
    let build_backend: String = "porters.build".to_string();

    let homepage: Option<String> = optional_input("Homepage URL", "https://github.com/owner/repo");
    let documentation: Option<String> = optional_input("Documentation URL", "https://github.com/owner/repo");
    let repository: Option<String> = optional_input("Repository URL", "https://github.com/owner/repo");
    let readme: Option<String> = optional_input("Readme file path", "README.md");

    let targets: Vec<BuildTarget> = vec![
        BuildTarget {
            name: "porters".to_string(),
            platform: None,
            type_: "library".to_string(),
            sources: Some(vec!["src/lib.rs".to_string()]),
        },
        BuildTarget {
            name: "porters".to_string(),
            platform: Some("linux".to_string()),
            type_: "binary".to_string(),
            sources: None,
        },
    ];

    let scripts = Scripts {
        build: "porters build".to_string(),
        test: "porters test".to_string(),
        run: "porters run".to_string(),
        clean: "porters clean".to_string(),
        exec: "porters exec".to_string(),
        install: "porters install".to_string(),
        uninstall: "porters uninstall".to_string(),
        package: "porters package".to_string(),
    };

    let build_system = BuildSystem {
        requires,
        build_backend,
    };

    let project = Project {
        name: project_name,
        description: project_description,
        project_type: project_type_to_str(project_type),
        version: project_version,
        license: project_license,
        requires_cpp,
        keywords: None,
        maintainers: if let (Some(name), Some(email)) = (maintainer_name, maintainer_email) {
            Some(vec![Contact { name, email }])
        } else {
            None
        },
        authors: if let (Some(name), Some(email)) = (author_name, author_email) {
            Some(vec![Contact { name, email }])
        } else {
            None
        },
        homepage,
        documentation,
        repository,
        readme,
    };

    let build = Build {
        toolchain: build_toolchain,
        flags: build_flags.split(',').map(|s| s.trim().to_string()).collect(),
        targets,
    };

    let porters_config = PortersConfig {
        build_system,
        project,
        dependencies: None,
        optional_dependencies: None,
        build,
        scripts: Some(scripts),
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

    //let project_type_str = if project_type == 0 { "C" } else { "C++" };

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

fn optional_input(prompt: &str, default: &str) -> Option<String> {
    let input: String = Input::with_theme(&ColorfulTheme::default())
        .with_prompt(prompt)
        .default(default.into())
        .allow_empty(true)
        .interact_text()
        .unwrap();

    // Return None if input matches default or is empty
    if input == default || input.is_empty() {
        None
    } else {
        Some(input)
    }
}

fn project_type_to_str(project_type: usize) -> String {
    match project_type {
        0 => "Library".to_string(),
        1 => "Executable".to_string(),
        _ => unreachable!(),
    }
}