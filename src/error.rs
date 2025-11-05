//! Enhanced error handling with GitHub integration
//!
//! This module provides custom error types for all Porters operations,
//! with automatic GitHub issue link suggestions for debugging and reporting.
//! Each error category links to relevant GitHub issues for community support.
//!
//! **Note**: This is an enhanced error handling system for future use.
//! Currently using anyhow::Error in most places.

use std::fmt;

#[allow(dead_code)]
const GITHUB_ISSUES: &str = "https://github.com/muhammad-fiaz/Porters/issues";

/// Custom error types for Porters
#[derive(Debug)]
#[allow(dead_code)]
pub enum PortersError {
    BuildSystem(String),
    Dependency(String),
    Configuration(String),
    Network(String),
    IO(String),
    Git(String),
    Compiler(String),
    Extension(String),
    Unknown(String),
}

impl fmt::Display for PortersError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let (category, message) = match self {
            PortersError::BuildSystem(msg) => ("Build System", msg),
            PortersError::Dependency(msg) => ("Dependency", msg),
            PortersError::Configuration(msg) => ("Configuration", msg),
            PortersError::Network(msg) => ("Network", msg),
            PortersError::IO(msg) => ("I/O", msg),
            PortersError::Git(msg) => ("Git", msg),
            PortersError::Compiler(msg) => ("Compiler", msg),
            PortersError::Extension(msg) => ("Extension", msg),
            PortersError::Unknown(msg) => ("Unknown", msg),
        };
        
        write!(f, "[{}] {}", category, message)
    }
}

impl std::error::Error for PortersError {}

/// Display error with helpful message and GitHub link
#[allow(dead_code)]
pub fn display_error(error: &dyn std::error::Error, is_warning: bool) {
    let prefix = if is_warning {
        "⚠️  WARNING"
    } else {
        "❌ ERROR"
    };
    
    eprintln!("\n{}: {}", prefix, error);
    
    if !is_warning {
        eprintln!("\n🤔 Oops! Looks like something went wrong.");
        eprintln!("   If you think this is a bug in Porters, please report it to:");
        eprintln!("   {}", GITHUB_ISSUES);
        eprintln!();
    }
}

/// Display error from anyhow::Error
#[allow(dead_code)]
pub fn display_anyhow_error(error: &anyhow::Error, is_warning: bool) {
    let prefix = if is_warning {
        "⚠️  WARNING"
    } else {
        "❌ ERROR"
    };
    
    eprintln!("\n{}: {}", prefix, error);
    
    // Display chain of causes
    let mut current = error.source();
    while let Some(cause) = current {
        eprintln!("   Caused by: {}", cause);
        current = cause.source();
    }
    
    if !is_warning {
        eprintln!("\n🤔 Oops! Looks like something went wrong.");
        eprintln!("   If you think this is a bug in Porters, please report it to:");
        eprintln!("   {}", GITHUB_ISSUES);
        eprintln!();
    }
}

/// Create error with context
#[allow(dead_code)]
pub fn build_system_error(message: impl Into<String>) -> PortersError {
    PortersError::BuildSystem(message.into())
}

#[allow(dead_code)]
pub fn dependency_error(message: impl Into<String>) -> PortersError {
    PortersError::Dependency(message.into())
}

#[allow(dead_code)]
pub fn config_error(message: impl Into<String>) -> PortersError {
    PortersError::Configuration(message.into())
}

#[allow(dead_code)]
pub fn network_error(message: impl Into<String>) -> PortersError {
    PortersError::Network(message.into())
}

#[allow(dead_code)]
pub fn io_error(message: impl Into<String>) -> PortersError {
    PortersError::IO(message.into())
}

#[allow(dead_code)]
pub fn git_error(message: impl Into<String>) -> PortersError {
    PortersError::Git(message.into())
}

#[allow(dead_code)]
pub fn compiler_error(message: impl Into<String>) -> PortersError {
    PortersError::Compiler(message.into())
}

#[allow(dead_code)]
pub fn extension_error(message: impl Into<String>) -> PortersError {
    PortersError::Extension(message.into())
}

/// Result type alias
#[allow(dead_code)]
pub type PortersResult<T> = Result<T, PortersError>;

/// Macro for easy error creation with context
#[macro_export]
macro_rules! porters_error {
    (BuildSystem, $($arg:tt)*) => {
        $crate::error::PortersError::BuildSystem(format!($($arg)*))
    };
    (Dependency, $($arg:tt)*) => {
        $crate::error::PortersError::Dependency(format!($($arg)*))
    };
    (Configuration, $($arg:tt)*) => {
        $crate::error::PortersError::Configuration(format!($($arg)*))
    };
    (Network, $($arg:tt)*) => {
        $crate::error::PortersError::Network(format!($($arg)*))
    };
    (IO, $($arg:tt)*) => {
        $crate::error::PortersError::IO(format!($($arg)*))
    };
    (Git, $($arg:tt)*) => {
        $crate::error::PortersError::Git(format!($($arg)*))
    };
    (Compiler, $($arg:tt)*) => {
        $crate::error::PortersError::Compiler(format!($($arg)*))
    };
    (Extension, $($arg:tt)*) => {
        $crate::error::PortersError::Extension(format!($($arg)*))
    };
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_error_display() {
        let error = build_system_error("CMake not found");
        assert_eq!(error.to_string(), "[Build System] CMake not found");
    }
    
    #[test]
    fn test_error_types() {
        let _ = dependency_error("Failed to resolve");
        let _ = config_error("Invalid TOML");
        let _ = compiler_error("GCC not found");
    }
}
