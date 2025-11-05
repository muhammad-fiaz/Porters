use std::env;
use std::fs;
use std::path::PathBuf;

fn main() {
    println!("cargo:rerun-if-changed=build.rs");
    
    // Only run post-install on actual install, not regular builds
    if env::var("CARGO_INSTALL").is_ok() {
        // Create post-install hook script
        create_post_install_script();
    }
}

fn create_post_install_script() {
    let out_dir = env::var("OUT_DIR").unwrap();
    let mut script_path = PathBuf::from(out_dir);
    script_path.push("post_install.txt");
    
    let message = r#"
================================================================================
  Porters installed successfully! 🎉
================================================================================

To use the 'porters' command, ensure cargo bin is in your PATH:

Windows (PowerShell - Run as Administrator):
  [Environment]::SetEnvironmentVariable("Path", $env:Path + ";$env:USERPROFILE\.cargo\bin", "User")
  
Windows (Command Prompt):
  setx PATH "%PATH%;%USERPROFILE%\.cargo\bin"

Linux/macOS (add to ~/.bashrc or ~/.zshrc):
  export PATH="$HOME/.cargo/bin:$PATH"
  
Then restart your terminal or run:
  source ~/.bashrc    # Linux/macOS

Current session (temporary):
  Windows: $env:Path += ";$env:USERPROFILE\.cargo\bin"
  Linux/macOS: export PATH="$HOME/.cargo/bin:$PATH"

Quick test:
  porters --version
  porters --help

================================================================================
"#;
    
    let _ = fs::write(script_path, message);
}
