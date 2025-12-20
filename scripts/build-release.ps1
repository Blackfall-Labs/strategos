# Build script for cross-platform releases on Windows
# Requires: rustup, cargo

Write-Host "===================================" -ForegroundColor Cyan
Write-Host "Engram CLI - Cross-Platform Builder" -ForegroundColor Cyan
Write-Host "===================================" -ForegroundColor Cyan
Write-Host ""

# Targets to build
$targets = @(
    "x86_64-pc-windows-msvc",
    "x86_64-unknown-linux-gnu",
    "x86_64-unknown-linux-musl"
)

# Output directory
$outputDir = "dist"
New-Item -ItemType Directory -Force -Path $outputDir | Out-Null

Write-Host "Installing required targets..." -ForegroundColor Yellow
foreach ($target in $targets) {
    rustup target add $target 2>$null
}

Write-Host ""
Write-Host "Building for all platforms..." -ForegroundColor Yellow
Write-Host ""

function Build-Target {
    param (
        [string]$target
    )

    $binName = "engram"
    $ext = ""
    $platformName = ""

    # Determine extension and platform name
    switch -Wildcard ($target) {
        "*windows*" {
            $ext = ".exe"
            $platformName = "Windows-x86_64"
        }
        "*linux*" {
            if ($target -like "*musl*") {
                $platformName = "Linux-x86_64-musl"
            } else {
                $platformName = "Linux-x86_64"
            }
        }
    }

    Write-Host "Building $platformName ($target)..." -ForegroundColor Green

    # Build
    if ($target -like "*windows*") {
        cargo build --release --target $target
    } else {
        # Try cross for Linux targets
        if (Get-Command cross -ErrorAction SilentlyContinue) {
            cross build --release --target $target
        } else {
            Write-Host "Warning: 'cross' not found. Install with: cargo install cross" -ForegroundColor Red
            Write-Host "Trying with cargo (may fail for cross-compilation)..." -ForegroundColor Yellow
            cargo build --release --target $target
        }
    }

    # Copy binary to dist
    $binaryPath = "target\$target\release\$binName$ext"
    $outputName = "$outputDir\engram-$platformName$ext"

    if (Test-Path $binaryPath) {
        Copy-Item $binaryPath $outputName

        # Generate SHA-256 checksum
        $hash = (Get-FileHash -Path $outputName -Algorithm SHA256).Hash
        Set-Content -Path "$outputName.sha256" -Value $hash.ToLower()

        Write-Host "✓ Built: $outputName" -ForegroundColor Green
    } else {
        Write-Host "✗ Failed: $binaryPath not found" -ForegroundColor Red
    }

    Write-Host ""
}

# Build all targets
foreach ($target in $targets) {
    Build-Target -target $target
}

Write-Host "===================================" -ForegroundColor Green
Write-Host "Build complete!" -ForegroundColor Green
Write-Host "===================================" -ForegroundColor Green
Write-Host ""
Write-Host "Binaries available in: $outputDir\" -ForegroundColor Cyan
Get-ChildItem -Path $outputDir -File | Format-Table Name, Length -AutoSize
