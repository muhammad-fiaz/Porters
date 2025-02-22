use std::process::Command;
use std::fs;
use colored::Colorize;

pub fn run() {
    println!("{}", "Building the project...".green());

    let config_content = fs::read_to_string("porters.toml").expect("Failed to read porters.toml");
    let config: toml::Value = toml::from_str(&config_content).expect("Failed to parse porters.toml");

    let toolchain = config["build"]["toolchain"].as_str().unwrap();
    let flags = config["build"]["flags"].as_array().unwrap();

    let entry_point = config["project"]["entrypoint"].as_str().unwrap_or("src/main.c");

    if !fs::metadata(entry_point).is_ok() {
        eprintln!("{}", format!("Entry point file '{}' not found!", entry_point).red());
        return;
    }

    let mut command = Command::new(toolchain);
    for flag in flags {
        command.arg(flag.as_str().unwrap());
    }
    command.arg(entry_point);

    let output = command.output().expect("Failed to execute build command");

    if output.status.success() {
        println!("{}", "Build completed successfully!".green());
    } else {
        eprintln!("{}", "Build failed!".red());
        eprintln!("{}", String::from_utf8_lossy(&output.stderr));
    }
}