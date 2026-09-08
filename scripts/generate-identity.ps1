# Generate Stellar identity for deployment (Windows)

Write-Host "🔑 Generating Stellar identity for WasteFi deployment...`n" -ForegroundColor Cyan

# Check if soroban is installed
if (!(Get-Command soroban -ErrorAction SilentlyContinue)) {
    Write-Host "Soroban CLI not found. Please run .\scripts\setup.ps1 first" -ForegroundColor Yellow
    exit 1
}

# Generate identity
Write-Host "Generating deployer identity..." -ForegroundColor Yellow
soroban keys generate deployer --network testnet

# Get the public key
$PUBLIC_KEY = soroban keys address deployer
Write-Host "✓ Identity generated" -ForegroundColor Green
Write-Host "Public Key: $PUBLIC_KEY`n" -ForegroundColor Blue

# Fund the account
Write-Host "Funding account with testnet XLM..." -ForegroundColor Yellow
soroban keys fund deployer --network testnet

Write-Host "✓ Account funded with testnet XLM`n" -ForegroundColor Green

# Get secret key for .env
$SECRET_KEY = soroban keys show deployer

Write-Host "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━" -ForegroundColor Green
Write-Host "✨ Deployer identity ready!" -ForegroundColor Green
Write-Host "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━`n" -ForegroundColor Green

Write-Host "Important:" -ForegroundColor Yellow
Write-Host "1. Your identity has been saved in Soroban CLI"
Write-Host "2. Public Key: " -NoNewline
Write-Host "$PUBLIC_KEY" -ForegroundColor Blue
Write-Host "3. Update your .env file with:"
Write-Host "   DEPLOYER_SECRET=$SECRET_KEY" -ForegroundColor Green
Write-Host "`n⚠️  Keep your secret key secure! Never commit it to version control." -ForegroundColor Yellow
