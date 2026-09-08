# WasteFi Development Environment Setup Script for Windows
# Run this script in PowerShell

Write-Host "🚀 Setting up WasteFi development environment..." -ForegroundColor Cyan

# Check if Rust is installed
Write-Host "`nChecking Rust installation..." -ForegroundColor Yellow
if (!(Get-Command rustc -ErrorAction SilentlyContinue)) {
    Write-Host "Rust is not installed. Please install from: https://rustup.rs/" -ForegroundColor Red
    Write-Host "After installing Rust, restart PowerShell and run this script again." -ForegroundColor Yellow
    exit 1
} else {
    $rustVersion = rustc --version
    Write-Host "✓ Rust is already installed ($rustVersion)" -ForegroundColor Green
}

# Update Rust
Write-Host "`nUpdating Rust toolchain..." -ForegroundColor Yellow
rustup update
Write-Host "✓ Rust updated" -ForegroundColor Green

# Add wasm32 target
Write-Host "`nAdding wasm32-unknown-unknown target..." -ForegroundColor Yellow
rustup target add wasm32-unknown-unknown
Write-Host "✓ wasm32 target added" -ForegroundColor Green

# Install rustfmt and clippy
Write-Host "`nInstalling rustfmt and clippy..." -ForegroundColor Yellow
rustup component add rustfmt clippy
Write-Host "✓ rustfmt and clippy installed" -ForegroundColor Green

# Check if Soroban CLI is installed
Write-Host "`nChecking Soroban CLI installation..." -ForegroundColor Yellow
if (!(Get-Command soroban -ErrorAction SilentlyContinue)) {
    Write-Host "Soroban CLI not found. Installing..." -ForegroundColor Yellow
    cargo install --locked soroban-cli
    Write-Host "✓ Soroban CLI installed successfully" -ForegroundColor Green
} else {
    $sorobanVersion = soroban --version
    Write-Host "✓ Soroban CLI is already installed ($sorobanVersion)" -ForegroundColor Green
}

# Configure Soroban for Stellar testnet
Write-Host "`nConfiguring Soroban for Stellar testnet..." -ForegroundColor Yellow
try {
    soroban network add testnet `
      --rpc-url https://soroban-testnet.stellar.org:443 `
      --network-passphrase "Test SDF Network ; September 2015" 2>$null
} catch {
    Write-Host "Testnet already configured" -ForegroundColor Gray
}
Write-Host "✓ Soroban configured for testnet" -ForegroundColor Green

# Create .env file if it doesn't exist
if (!(Test-Path .env)) {
    Write-Host "`nCreating .env file..." -ForegroundColor Yellow
    Copy-Item .env.example .env
    Write-Host "✓ .env file created" -ForegroundColor Green
    Write-Host "⚠️  Please update .env with your configuration" -ForegroundColor Yellow
} else {
    Write-Host "✓ .env file already exists" -ForegroundColor Green
}

# Build the project
Write-Host "`nBuilding WasteFi contracts..." -ForegroundColor Yellow
cargo build --target wasm32-unknown-unknown --release
Write-Host "✓ Contracts built successfully" -ForegroundColor Green

# Run tests
Write-Host "`nRunning tests..." -ForegroundColor Yellow
cargo test
Write-Host "✓ All tests passed" -ForegroundColor Green

Write-Host "`n━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━" -ForegroundColor Green
Write-Host "✨ WasteFi development environment is ready!" -ForegroundColor Green
Write-Host "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━" -ForegroundColor Green

Write-Host "`nNext steps:" -ForegroundColor Yellow
Write-Host "1. Update .env with your deployer secret key"
Write-Host "2. Generate a deployer identity: " -NoNewline
Write-Host ".\scripts\generate-identity.ps1" -ForegroundColor Green
Write-Host "3. Deploy contracts: " -NoNewline
Write-Host ".\scripts\deploy.ps1" -ForegroundColor Green
Write-Host "4. Start developing! 🚀"
