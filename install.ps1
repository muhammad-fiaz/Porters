# Porters Installation Script for Windows
# This script installs Porters and automatically adds it to PATH

param(
    [switch]$SkipPathUpdate = $false
)

Write-Host "==================================" -ForegroundColor Cyan
Write-Host "  Porters Installation Script" -ForegroundColor Cyan
Write-Host "==================================" -ForegroundColor Cyan
Write-Host ""

# Check if Rust is installed
Write-Host "[1/4] Checking Rust installation..." -ForegroundColor Yellow
if (-not (Get-Command cargo -ErrorAction SilentlyContinue)) {
    Write-Host "X Rust is not installed!" -ForegroundColor Red
    Write-Host "Please install Rust from: https://rustup.rs/" -ForegroundColor Yellow
    Write-Host "After installing Rust, run this script again." -ForegroundColor Yellow
    exit 1
}

$rustVersion = cargo --version
Write-Host "OK Rust detected: $rustVersion" -ForegroundColor Green
Write-Host ""

# Install Porters
Write-Host "[2/4] Installing Porters..." -ForegroundColor Yellow
$installOutput = cargo install --path . 2>&1
if ($LASTEXITCODE -eq 0) {
    Write-Host "OK Porters installed successfully!" -ForegroundColor Green
} else {
    Write-Host "X Failed to install Porters" -ForegroundColor Red
    Write-Host $installOutput
    exit 1
}
Write-Host ""

# Get cargo bin directory
$cargoBinDir = if ($env:CARGO_HOME) {
    Join-Path $env:CARGO_HOME "bin"
} else {
    Join-Path $env:USERPROFILE ".cargo\bin"
}

Write-Host "Cargo bin directory: $cargoBinDir" -ForegroundColor Cyan

# Check if already in PATH
Write-Host "[3/4] Checking PATH configuration..." -ForegroundColor Yellow
$userPath = [Environment]::GetEnvironmentVariable("Path", "User")
$pathContainsCargoBin = $userPath -split ';' | Where-Object { $_ -eq $cargoBinDir }

if ($pathContainsCargoBin) {
    Write-Host "OK Cargo bin directory already in PATH" -ForegroundColor Green
    $SkipPathUpdate = $true
} else {
    Write-Host "! Cargo bin directory not found in PATH" -ForegroundColor Yellow
}
Write-Host ""

# Add to PATH if needed
if (-not $SkipPathUpdate) {
    Write-Host "[4/4] Adding Porters to PATH..." -ForegroundColor Yellow
    Write-Host "This will add: $cargoBinDir" -ForegroundColor Cyan
    Write-Host "To your User PATH environment variable." -ForegroundColor Cyan
    Write-Host ""
    
    $response = Read-Host "Do you want to add Porters to PATH automatically? (Y/n)"
    if ($response -eq '' -or $response -eq 'Y' -or $response -eq 'y') {
        try {
            # Add to User PATH
            $newPath = if ($userPath) {
                "$userPath;$cargoBinDir"
            } else {
                $cargoBinDir
            }
            
            [Environment]::SetEnvironmentVariable("Path", $newPath, "User")
            
            # Update current session PATH
            $env:Path = "$env:Path;$cargoBinDir"
            
            Write-Host "OK Successfully added to PATH!" -ForegroundColor Green
            Write-Host ""
            Write-Host "! IMPORTANT:" -ForegroundColor Yellow
            Write-Host "  - Current terminal: PATH updated (porters available now)" -ForegroundColor White
            Write-Host "  - Other terminals: Restart required to use 'porters'" -ForegroundColor White
            Write-Host ""
        } catch {
            Write-Host "X Failed to update PATH automatically" -ForegroundColor Red
            Write-Host "Error: $_" -ForegroundColor Red
            Write-Host ""
            Write-Host "Please add manually:" -ForegroundColor Yellow
            Write-Host "  1. Open System Properties > Environment Variables" -ForegroundColor White
            Write-Host "  2. Edit User PATH variable" -ForegroundColor White
            Write-Host "  3. Add: $cargoBinDir" -ForegroundColor White
            Write-Host ""
        }
    } else {
        Write-Host "Skipped PATH update." -ForegroundColor Yellow
        Write-Host ""
        Write-Host "To use 'porters' command, add to PATH manually:" -ForegroundColor Yellow
        Write-Host "  $cargoBinDir" -ForegroundColor Cyan
        Write-Host ""
    }
} else {
    Write-Host "[4/4] PATH configuration" -ForegroundColor Yellow
    Write-Host "OK Already configured" -ForegroundColor Green
    Write-Host ""
}

# Verify installation
Write-Host "==================================" -ForegroundColor Cyan
Write-Host "  Verifying Installation" -ForegroundColor Cyan
Write-Host "==================================" -ForegroundColor Cyan
Write-Host ""

# Refresh environment in current session
$env:Path = [Environment]::GetEnvironmentVariable("Path", "User") + ";" + [Environment]::GetEnvironmentVariable("Path", "Machine")

if (Get-Command porters -ErrorAction SilentlyContinue) {
    $portersVersion = porters --version 2>&1
    Write-Host "OK Porters is ready!" -ForegroundColor Green
    Write-Host "  Version: $portersVersion" -ForegroundColor Cyan
    Write-Host ""
    Write-Host "Try it now:" -ForegroundColor Yellow
    Write-Host "  porters --help" -ForegroundColor Cyan
    Write-Host "  porters init" -ForegroundColor Cyan
    Write-Host "  porters execute myfile.c" -ForegroundColor Cyan
    Write-Host ""
} else {
    Write-Host "! Installation complete, but 'porters' command not found" -ForegroundColor Yellow
    Write-Host ""
    Write-Host "Please:" -ForegroundColor Yellow
    Write-Host "  1. Restart your terminal" -ForegroundColor White
    Write-Host "  2. Or run: `$env:Path += ';$cargoBinDir'" -ForegroundColor White
    Write-Host "  3. Then try: porters --help" -ForegroundColor White
    Write-Host ""
}

Write-Host "==================================" -ForegroundColor Cyan
Write-Host "  Installation Complete!" -ForegroundColor Cyan
Write-Host "==================================" -ForegroundColor Cyan
Write-Host ""
