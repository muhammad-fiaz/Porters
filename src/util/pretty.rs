//! Pretty-printing utilities with colored output
//!
//! This module provides consistent, emoji-enhanced terminal output
//! for different message types: steps, success, info, warnings, and errors.

use colored::*;

/// Print a step header with arrow emoji
pub fn print_step(msg: &str) {
    println!("{}  {}", "➡️".bright_blue(), msg.bold().bright_white());
}

/// Print a success message with checkmark emoji
pub fn print_success(msg: &str) {
    println!("{}  {}", "✅".green(), msg.green());
}

/// Print an info message
pub fn print_info(msg: &str) {
    println!("{}  {}", "ℹ️".cyan(), msg.cyan());
}

/// Print a warning message
pub fn print_warning(msg: &str) {
    println!("{}  {}", "⚠️".yellow(), msg.yellow());
}

/// Print an error message with fire emoji
pub fn print_error(msg: &str) {
    eprintln!("{}  {}", "🔥".red(), msg.red().bold());
}

/// Print a build message
pub fn print_build(msg: &str) {
    println!("{}  {}", "🔨".bright_yellow(), msg);
}

/// Print a package message
pub fn print_package(msg: &str) {
    println!("{}  {}", "📦".bright_magenta(), msg);
}

/// Print a graph node
pub fn print_graph_node(name: &str, version: &str, depth: usize) {
    let indent = "  ".repeat(depth);
    if depth == 0 {
        println!(
            "{}{}  {}",
            indent,
            "📦".bright_magenta(),
            format!("{} ({})", name, version).bold()
        );
    } else {
        println!(
            "{}├─ {}  {} ({})",
            indent,
            "📦".bright_blue(),
            name,
            version
        );
    }
}
