# Installation script for Engram CLI on Windows
# Usage: irm https://raw.githubusercontent.com/blackfall-labs/engram-cli/main/scripts/install.ps1 | iex

$ErrorActionPreference = "Stop"

$REPO = "blackfall-labs/engram-cli"
$BINARY_NAME = "engram.exe"
$INSTALL_DIR = "$env:LOCALAPPDATA\engram\bin"

Write-Host "===================================" -ForegroundColor Blue
Write-Host "   Engram CLI Installer" -ForegroundColor Blue
Write-Host "===================================" -ForegroundColor Blue
Write-Host ""

# Detect architecture
$ARCH = if ([Environment]::Is64BitOperatingSystem) { "x86_64" } else { "x86" }
if ($ARCH -ne "x86_64") {
    Write-Host "Unsupported architecture: $ARCH" -ForegroundColor Red
    Write-Host "Only x86_64 (64-bit) Windows is supported" -ForegroundColor Red
    exit 1
}

$BINARY_FILE = "engram-Windows-x86_64.exe"

Write-Host "Detected platform: Windows $ARCH" -ForegroundColor Yellow
Write-Host "Binary to download: $BINARY_FILE" -ForegroundColor Yellow
Write-Host ""

# Get latest release
Write-Host "Fetching latest release..." -ForegroundColor Blue
try {
    $release = Invoke-RestMethod -Uri "https://api.github.com/repos/$REPO/releases/latest"
    $LATEST_RELEASE = $release.tag_name
} catch {
    Write-Host "Failed to fetch latest release" -ForegroundColor Red
    exit 1
}

Write-Host "Latest version: $LATEST_RELEASE" -ForegroundColor Green
Write-Host ""

# Download URLs
$DOWNLOAD_URL = "https://github.com/$REPO/releases/download/$LATEST_RELEASE/$BINARY_FILE"
$CHECKSUM_URL = "https://github.com/$REPO/releases/download/$LATEST_RELEASE/$BINARY_FILE.sha256"

# Create install directory
New-Item -ItemType Directory -Force -Path $INSTALL_DIR | Out-Null

# Download binary
Write-Host "Downloading $BINARY_FILE..." -ForegroundColor Blue
$TempFile = "$env:TEMP\$BINARY_NAME"
try {
    Invoke-WebRequest -Uri $DOWNLOAD_URL -OutFile $TempFile -UseBasicParsing
} catch {
    Write-Host "Failed to download binary" -ForegroundColor Red
    Write-Host $_.Exception.Message -ForegroundColor Red
    exit 1
}

# Download checksum
Write-Host "Downloading checksum..." -ForegroundColor Blue
$TempChecksum = "$env:TEMP\$BINARY_NAME.sha256"
try {
    Invoke-WebRequest -Uri $CHECKSUM_URL -OutFile $TempChecksum -UseBasicParsing
} catch {
    Write-Host "Warning: Failed to download checksum, skipping verification" -ForegroundColor Yellow
    $TempChecksum = $null
}

# Verify checksum
if ($TempChecksum -and (Test-Path $TempChecksum)) {
    Write-Host "Verifying checksum..." -ForegroundColor Blue
    $ExpectedHash = (Get-Content $TempChecksum).Trim().ToLower()
    $ActualHash = (Get-FileHash -Path $TempFile -Algorithm SHA256).Hash.ToLower()

    if ($ExpectedHash -eq $ActualHash) {
        Write-Host "✓ Checksum verified" -ForegroundColor Green
    } else {
        Write-Host "✗ Checksum mismatch!" -ForegroundColor Red
        Write-Host "Expected: $ExpectedHash" -ForegroundColor Red
        Write-Host "Got:      $ActualHash" -ForegroundColor Red
        Remove-Item $TempFile -Force
        Remove-Item $TempChecksum -Force -ErrorAction SilentlyContinue
        exit 1
    }
}

# Install binary
Write-Host "Installing to $INSTALL_DIR\$BINARY_NAME..." -ForegroundColor Blue
Move-Item -Path $TempFile -Destination "$INSTALL_DIR\$BINARY_NAME" -Force

# Clean up
if ($TempChecksum) {
    Remove-Item $TempChecksum -Force -ErrorAction SilentlyContinue
}

Write-Host ""
Write-Host "===================================" -ForegroundColor Green
Write-Host "   Installation successful!" -ForegroundColor Green
Write-Host "===================================" -ForegroundColor Green
Write-Host ""
Write-Host "Binary installed to: $INSTALL_DIR\$BINARY_NAME" -ForegroundColor Yellow
Write-Host ""

# Check if directory is in PATH
$envPath = [Environment]::GetEnvironmentVariable("Path", "User")
if ($envPath -notlike "*$INSTALL_DIR*") {
    Write-Host "⚠ $INSTALL_DIR is not in your PATH" -ForegroundColor Yellow
    Write-Host ""
    Write-Host "Adding to PATH..." -ForegroundColor Blue

    # Add to user PATH
    $newPath = "$envPath;$INSTALL_DIR"
    [Environment]::SetEnvironmentVariable("Path", $newPath, "User")

    Write-Host "✓ Added to PATH (restart your terminal for changes to take effect)" -ForegroundColor Green
    Write-Host ""
    Write-Host "You can now run: engram --help" -ForegroundColor Blue
    Write-Host "(Note: You may need to restart your terminal)" -ForegroundColor Yellow
} else {
    Write-Host "✓ $INSTALL_DIR is already in your PATH" -ForegroundColor Green
    Write-Host ""
    Write-Host "You can now run: engram --help" -ForegroundColor Blue
}

Write-Host ""
