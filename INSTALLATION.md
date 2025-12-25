# Installation Guide

This guide covers all methods for installing Strategos on different platforms.

## Table of Contents

- [Quick Install](#quick-install)
- [Install via Cargo](#install-via-cargo)
- [Pre-built Binaries](#pre-built-binaries)
- [Building from Source](#building-from-source)
- [Platform-Specific Instructions](#platform-specific-instructions)
- [Troubleshooting](#troubleshooting)
- [Uninstallation](#uninstallation)

## Quick Install

### Linux / macOS

```bash
curl -fsSL https://raw.githubusercontent.com/blackfall-labs/strategos/main/scripts/install.sh | bash
```

This will:
1. Detect your platform and architecture automatically
2. Download the latest release binary
3. Verify the SHA-256 checksum
4. Install to `~/.local/bin/strategos`
5. Optionally add the directory to your PATH

### Windows

Open PowerShell and run:

```powershell
irm https://raw.githubusercontent.com/blackfall-labs/strategos/main/scripts/install.ps1 | iex
```

This will:
1. Download the latest Windows binary
2. Verify the SHA-256 checksum
3. Install to `%LOCALAPPDATA%\strategos\bin\strategos.exe`
4. Add the directory to your user PATH

## Install via Cargo

If you have Rust installed, you can install directly from Git:

```bash
cargo install --git https://github.com/blackfall-labs/strategos strategos
```

This will compile from source and install to `~/.cargo/bin/strategos` (or `%USERPROFILE%\.cargo\bin\strategos.exe` on Windows).

### Prerequisites

- Rust 1.75 or later (2024 edition) - install from [rustup.rs](https://rustup.rs)

## Pre-built Binaries

Download binaries from the [GitHub Releases page](https://github.com/blackfall-labs/strategos/releases/latest).

### Windows (x86_64)

1. Download `strategos-Windows-x86_64.exe`
2. Rename to `strategos.exe` (optional, for convenience)
3. Move to a directory in your PATH, or add the directory to PATH

**Example:**
```powershell
# Download
Invoke-WebRequest -Uri "https://github.com/blackfall-labs/strategos/releases/latest/download/strategos-Windows-x86_64.exe" -OutFile "strategos.exe"

# Move to a directory in PATH (e.g., C:\Users\YourName\bin)
Move-Item strategos.exe C:\Users\$env:USERNAME\bin\

# Add to PATH (if not already)
$env:Path += ";C:\Users\$env:USERNAME\bin"
```

### macOS Intel (x86_64)

```bash
# Download
curl -LO https://github.com/blackfall-labs/strategos/releases/latest/download/strategos-Darwin-x86_64

# Make executable
chmod +x strategos-Darwin-x86_64

# Move to /usr/local/bin (or any directory in PATH)
sudo mv strategos-Darwin-x86_64 /usr/local/bin/strategos
```

### macOS Apple Silicon (ARM64)

```bash
# Download
curl -LO https://github.com/blackfall-labs/strategos/releases/latest/download/strategos-Darwin-aarch64

# Make executable
chmod +x strategos-Darwin-aarch64

# Move to /usr/local/bin (or any directory in PATH)
sudo mv strategos-Darwin-aarch64 /usr/local/bin/strategos
```

### Linux x86_64 (MUSL - Static Binary, Recommended)

```bash
# Download
curl -LO https://github.com/blackfall-labs/strategos/releases/latest/download/strategos-Linux-x86_64-musl

# Make executable
chmod +x strategos-Linux-x86_64-musl

# Move to /usr/local/bin (or ~/.local/bin for user install)
sudo mv strategos-Linux-x86_64-musl /usr/local/bin/strategos

# Or for user install
mkdir -p ~/.local/bin
mv strategos-Linux-x86_64-musl ~/.local/bin/strategos
```

### Linux x86_64 (GNU libc)

```bash
# Download
curl -LO https://github.com/blackfall-labs/strategos/releases/latest/download/strategos-Linux-x86_64

# Make executable
chmod +x strategos-Linux-x86_64

# Move to /usr/local/bin (or ~/.local/bin for user install)
sudo mv strategos-Linux-x86_64 /usr/local/bin/strategos
```

### Linux ARM64

```bash
# Download
curl -LO https://github.com/blackfall-labs/strategos/releases/latest/download/strategos-Linux-aarch64

# Make executable
chmod +x strategos-Linux-aarch64

# Move to /usr/local/bin (or ~/.local/bin for user install)
sudo mv strategos-Linux-aarch64 /usr/local/bin/strategos
```

## Building from Source

### Prerequisites

- Rust 1.75 or later (2024 edition)
- Git
- Local dependencies (cartridge-rs, dataspool-rs, datacard-rs, bytepunch-rs)

### Steps

1. **Clone the Blackfall Labs monorepo:**
   ```bash
   git clone https://github.com/blackfall-labs/blackfall-labs
   cd blackfall-labs/strategos
   ```

2. **Build:**
   ```bash
   cargo build --release
   ```

3. **Binary location:**
   - Linux/macOS: `target/release/strategos`
   - Windows: `target\release\strategos.exe`

4. **Install (optional):**
   ```bash
   cargo install --path .
   ```

   This installs to `~/.cargo/bin/strategos` or `%USERPROFILE%\.cargo\bin\strategos.exe`.

### Building for Multiple Platforms

#### Linux/macOS

```bash
./scripts/build-release.sh
```

#### Windows

```powershell
.\scripts\build-release.ps1
```

Binaries will be in the `dist/` directory.

## Platform-Specific Instructions

### Linux

#### Debian/Ubuntu

```bash
# Install via quick install script
curl -fsSL https://raw.githubusercontent.com/blackfall-labs/strategos/main/scripts/install.sh | bash

# Add to PATH if not already
echo 'export PATH="$HOME/.local/bin:$PATH"' >> ~/.bashrc
source ~/.bashrc
```

#### Fedora/RHEL

```bash
# Install via quick install script
curl -fsSL https://raw.githubusercontent.com/blackfall-labs/strategos/main/scripts/install.sh | bash

# Add to PATH if not already
echo 'export PATH="$HOME/.local/bin:$PATH"' >> ~/.bashrc
source ~/.bashrc
```

#### Arch Linux

```bash
# Install via Cargo (AUR package coming soon)
cargo install --git https://github.com/blackfall-labs/strategos strategos
```

### macOS

#### Homebrew (Coming Soon)

```bash
# Not yet available, use alternative methods
```

#### Manual Install

Use the quick install script or pre-built binaries (see above).

### Windows

#### Scoop (Coming Soon)

```powershell
# Not yet available, use alternative methods
```

#### Manual Install

Use the quick install PowerShell script or download the binary manually (see above).

## Verify Installation

After installation, verify it works:

```bash
# Check version
strategos --version

# Show help
strategos --help

# Test with a simple command
strategos keygen --private-key test.key --public-key test.pub
```

## Troubleshooting

### Command not found

**Linux/macOS:**
Ensure the install directory is in your PATH:
```bash
echo $PATH
export PATH="$HOME/.local/bin:$PATH"
```

Add to your shell profile (`~/.bashrc`, `~/.zshrc`, etc.) to make permanent:
```bash
echo 'export PATH="$HOME/.local/bin:$PATH"' >> ~/.bashrc
```

**Windows:**
Restart your terminal after installation. If still not working, verify PATH:
```powershell
$env:Path
```

### Permission denied

**Linux/macOS:**
```bash
chmod +x /path/to/strategos
```

### Binary doesn't run on Linux

If you see "No such file or directory" on Linux, try the MUSL build (static binary):
```bash
curl -LO https://github.com/blackfall-labs/strategos/releases/latest/download/strategos-Linux-x86_64-musl
chmod +x strategos-Linux-x86_64-musl
sudo mv strategos-Linux-x86_64-musl /usr/local/bin/strategos
```

### Build from source fails

Ensure you have:
- Latest Rust: `rustup update`
- Git installed
- All local dependencies available (cartridge-rs, dataspool-rs, etc.)
- Internet connection (for crates.io dependencies)

### Missing local dependencies

Strategos requires several local dependencies from the Blackfall Labs ecosystem:
```bash
# Make sure you cloned the full monorepo, not just strategos
git clone https://github.com/blackfall-labs/blackfall-labs
cd blackfall-labs

# All dependencies should be in sibling directories:
# - cartridge-rs/
# - dataspool-rs/
# - datacard-rs/
# - bytepunch-rs/
# - engram-rs/
```

## Uninstallation

### Quick Install Script

**Linux/macOS:**
```bash
rm ~/.local/bin/strategos
```

**Windows:**
```powershell
Remove-Item "$env:LOCALAPPDATA\strategos" -Recurse -Force
```

Then remove from PATH manually if needed.

### Cargo Install

```bash
cargo uninstall strategos
```

### Manual Install

Simply delete the binary from wherever you placed it.

## Next Steps

After installation:
1. Read the [README](README.md) for usage examples and command reference
2. Check out the [format compatibility matrix](README.md#-format-compatibility-matrix)
3. Try the [workflow examples](README.md#-workflow-examples)
4. Check [CONTRIBUTING](CONTRIBUTING.md) if you want to contribute
5. Report issues at [GitHub Issues](https://github.com/blackfall-labs/strategos/issues)

## Support

For help:
- Check the [README](README.md)
- Open an [issue](https://github.com/blackfall-labs/strategos/issues)
- Consult the [CLAUDE.md](CLAUDE.md) for developer documentation
