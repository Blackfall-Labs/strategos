# Installation Guide

This guide covers all methods for installing Engram CLI on different platforms.

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
curl -fsSL https://raw.githubusercontent.com/blackfall-labs/engram-cli/main/scripts/install.sh | bash
```

This will:
1. Detect your platform and architecture automatically
2. Download the latest release binary
3. Verify the SHA-256 checksum
4. Install to `~/.local/bin/engram`
5. Optionally add the directory to your PATH

### Windows

Open PowerShell and run:

```powershell
irm https://raw.githubusercontent.com/blackfall-labs/engram-cli/main/scripts/install.ps1 | iex
```

This will:
1. Download the latest Windows binary
2. Verify the SHA-256 checksum
3. Install to `%LOCALAPPDATA%\engram\bin\engram.exe`
4. Add the directory to your user PATH

## Install via Cargo

If you have Rust installed, you can install directly from Git:

```bash
cargo install --git https://github.com/blackfall-labs/engram-cli engram-cli
```

This will compile from source and install to `~/.cargo/bin/engram` (or `%USERPROFILE%\.cargo\bin\engram.exe` on Windows).

### Prerequisites

- Rust 1.75 or later (install from [rustup.rs](https://rustup.rs))

## Pre-built Binaries

Download binaries from the [GitHub Releases page](https://github.com/blackfall-labs/engram-cli/releases/latest).

### Windows (x86_64)

1. Download `engram-Windows-x86_64.exe`
2. Rename to `engram.exe` (optional, for convenience)
3. Move to a directory in your PATH, or add the directory to PATH

**Example:**
```powershell
# Download
Invoke-WebRequest -Uri "https://github.com/blackfall-labs/engram-cli/releases/latest/download/engram-Windows-x86_64.exe" -OutFile "engram.exe"

# Move to a directory in PATH (e.g., C:\Users\YourName\bin)
Move-Item engram.exe C:\Users\$env:USERNAME\bin\

# Add to PATH (if not already)
$env:Path += ";C:\Users\$env:USERNAME\bin"
```

### macOS Intel (x86_64)

```bash
# Download
curl -LO https://github.com/blackfall-labs/engram-cli/releases/latest/download/engram-Darwin-x86_64

# Make executable
chmod +x engram-Darwin-x86_64

# Move to /usr/local/bin (or any directory in PATH)
sudo mv engram-Darwin-x86_64 /usr/local/bin/engram
```

### macOS Apple Silicon (ARM64)

```bash
# Download
curl -LO https://github.com/blackfall-labs/engram-cli/releases/latest/download/engram-Darwin-aarch64

# Make executable
chmod +x engram-Darwin-aarch64

# Move to /usr/local/bin (or any directory in PATH)
sudo mv engram-Darwin-aarch64 /usr/local/bin/engram
```

### Linux x86_64 (MUSL - Static Binary, Recommended)

```bash
# Download
curl -LO https://github.com/blackfall-labs/engram-cli/releases/latest/download/engram-Linux-x86_64-musl

# Make executable
chmod +x engram-Linux-x86_64-musl

# Move to /usr/local/bin (or ~/.local/bin for user install)
sudo mv engram-Linux-x86_64-musl /usr/local/bin/engram

# Or for user install
mkdir -p ~/.local/bin
mv engram-Linux-x86_64-musl ~/.local/bin/engram
```

### Linux x86_64 (GNU libc)

```bash
# Download
curl -LO https://github.com/blackfall-labs/engram-cli/releases/latest/download/engram-Linux-x86_64

# Make executable
chmod +x engram-Linux-x86_64

# Move to /usr/local/bin (or ~/.local/bin for user install)
sudo mv engram-Linux-x86_64 /usr/local/bin/engram
```

### Linux ARM64

```bash
# Download
curl -LO https://github.com/blackfall-labs/engram-cli/releases/latest/download/engram-Linux-aarch64

# Make executable
chmod +x engram-Linux-aarch64

# Move to /usr/local/bin (or ~/.local/bin for user install)
sudo mv engram-Linux-aarch64 /usr/local/bin/engram
```

## Building from Source

### Prerequisites

- Rust 1.75 or later (2024 edition)
- Git

### Steps

1. **Clone the repository:**
   ```bash
   git clone https://github.com/blackfall-labs/engram-cli
   cd engram-cli
   ```

2. **Build:**
   ```bash
   cargo build --release
   ```

3. **Binary location:**
   - Linux/macOS: `target/release/engram`
   - Windows: `target\release\engram.exe`

4. **Install (optional):**
   ```bash
   cargo install --path crates/engram-cli
   ```

   This installs to `~/.cargo/bin/engram` or `%USERPROFILE%\.cargo\bin\engram.exe`.

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
curl -fsSL https://raw.githubusercontent.com/blackfall-labs/engram-cli/main/scripts/install.sh | bash

# Add to PATH if not already
echo 'export PATH="$HOME/.local/bin:$PATH"' >> ~/.bashrc
source ~/.bashrc
```

#### Fedora/RHEL

```bash
# Install via quick install script
curl -fsSL https://raw.githubusercontent.com/blackfall-labs/engram-cli/main/scripts/install.sh | bash

# Add to PATH if not already
echo 'export PATH="$HOME/.local/bin:$PATH"' >> ~/.bashrc
source ~/.bashrc
```

#### Arch Linux

```bash
# Install via Cargo (AUR package coming soon)
cargo install --git https://github.com/blackfall-labs/engram-cli engram-cli
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
engram --version

# Show help
engram --help

# Test with a simple command
engram keygen --private-key test.key --public-key test.pub
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
chmod +x /path/to/engram
```

### Binary doesn't run on Linux

If you see "No such file or directory" on Linux, try the MUSL build (static binary):
```bash
curl -LO https://github.com/blackfall-labs/engram-cli/releases/latest/download/engram-Linux-x86_64-musl
chmod +x engram-Linux-x86_64-musl
sudo mv engram-Linux-x86_64-musl /usr/local/bin/engram
```

### Build from source fails

Ensure you have:
- Latest Rust: `rustup update`
- Git installed
- Internet connection (for dependencies)

## Uninstallation

### Quick Install Script

**Linux/macOS:**
```bash
rm ~/.local/bin/engram
```

**Windows:**
```powershell
Remove-Item "$env:LOCALAPPDATA\engram" -Recurse -Force
```

Then remove from PATH manually if needed.

### Cargo Install

```bash
cargo uninstall engram-cli
```

### Manual Install

Simply delete the binary from wherever you placed it.

## Next Steps

After installation:
1. Read the [README](README.md) for usage examples
2. Check out [CONTRIBUTING](CONTRIBUTING.md) if you want to contribute
3. Report issues at [GitHub Issues](https://github.com/blackfall-labs/engram-cli/issues)

## Support

For help:
- Check the [README](README.md)
- Open an [issue](https://github.com/blackfall-labs/engram-cli/issues)
- Consult the [CLAUDE.md](CLAUDE.md) for developer documentation
