# engram-cli

[![CI](https://github.com/blackfall-labs/engram-cli/actions/workflows/ci.yml/badge.svg)](https://github.com/blackfall-labs/engram-cli/actions/workflows/ci.yml)
[![Release](https://github.com/blackfall-labs/engram-cli/actions/workflows/release.yml/badge.svg)](https://github.com/blackfall-labs/engram-cli/actions/workflows/release.yml)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

A comprehensive CLI tool for managing Engram archives - create, inspect, extract, sign, and query `.eng` archive files.

## Supported Platforms

| Platform | Architecture | Binary Name |
|----------|-------------|-------------|
| Windows | x86_64 | `engram-Windows-x86_64.exe` |
| macOS | x86_64 (Intel) | `engram-Darwin-x86_64` |
| macOS | aarch64 (Apple Silicon) | `engram-Darwin-aarch64` |
| Linux | x86_64 (GNU) | `engram-Linux-x86_64` |
| Linux | x86_64 (MUSL) | `engram-Linux-x86_64-musl` |
| Linux | aarch64 (ARM64) | `engram-Linux-aarch64` |

## Features

- **Pack**: Create Engram archives from files or directories
- **Extract**: Extract files from archives
- **List**: Display contents of archives
- **Info**: Show metadata and statistics
- **Sign**: Cryptographically sign archives with Ed25519
- **Verify**: Verify signatures and file integrity
- **Keygen**: Generate Ed25519 keypairs for signing
- **Query**: Execute SQL queries on embedded SQLite databases
- **Search**: Search for text patterns within archives

## Installation

### Quick Install (Recommended)

**Linux / macOS:**
```bash
curl -fsSL https://raw.githubusercontent.com/blackfall-labs/engram-cli/main/scripts/install.sh | bash
```

**Windows (PowerShell):**
```powershell
irm https://raw.githubusercontent.com/blackfall-labs/engram-cli/main/scripts/install.ps1 | iex
```

### Install via Cargo

If you have Rust installed:

```bash
cargo install --git https://github.com/blackfall-labs/engram-cli engram-cli
```

### Pre-built Binaries

Download pre-built binaries from the [Releases page](https://github.com/blackfall-labs/engram-cli/releases/latest):

#### Windows
- Download `engram-Windows-x86_64.exe`
- Rename to `engram.exe`
- Add to your PATH or place in a directory that's already in PATH

#### macOS
**Intel (x86_64):**
```bash
# Download and install
curl -LO https://github.com/blackfall-labs/engram-cli/releases/latest/download/engram-Darwin-x86_64
chmod +x engram-Darwin-x86_64
sudo mv engram-Darwin-x86_64 /usr/local/bin/engram
```

**Apple Silicon (ARM64):**
```bash
# Download and install
curl -LO https://github.com/blackfall-labs/engram-cli/releases/latest/download/engram-Darwin-aarch64
chmod +x engram-Darwin-aarch64
sudo mv engram-Darwin-aarch64 /usr/local/bin/engram
```

#### Linux

**x86_64 (recommended - static binary):**
```bash
# Download and install
curl -LO https://github.com/blackfall-labs/engram-cli/releases/latest/download/engram-Linux-x86_64-musl
chmod +x engram-Linux-x86_64-musl
sudo mv engram-Linux-x86_64-musl /usr/local/bin/engram
```

**x86_64 (GNU libc):**
```bash
curl -LO https://github.com/blackfall-labs/engram-cli/releases/latest/download/engram-Linux-x86_64
chmod +x engram-Linux-x86_64
sudo mv engram-Linux-x86_64 /usr/local/bin/engram
```

**ARM64:**
```bash
curl -LO https://github.com/blackfall-labs/engram-cli/releases/latest/download/engram-Linux-aarch64
chmod +x engram-Linux-aarch64
sudo mv engram-Linux-aarch64 /usr/local/bin/engram
```

### Verify Installation

After installation, verify it works:

```bash
engram --version
engram --help
```

### Building from Source

```bash
# Clone the repository
git clone https://github.com/blackfall-labs/engram-cli
cd engram-cli

# Build with Cargo
cargo build --release

# Binary will be at: target/release/engram.exe (Windows) or target/release/engram (Unix)

# Optionally install to cargo bin directory
cargo install --path crates/engram-cli
```

## Commands

### Keygen - Generate Keypairs

Generate Ed25519 keypairs for signing archives:

```bash
engram keygen --private-key private.key --public-key public.key
```

**Output:**
```
Generating new Ed25519 keypair...
✓ Private key saved to: private.key
✓ Public key saved to: public.key

Keep your private key secure and never share it!
You can share your public key for signature verification.
```

### Pack - Create Archives

Create an Engram archive from files or directories:

```bash
# Pack a directory
engram pack my_data

# Pack with custom output
engram pack my_data -o archive.eng

# Pack with specific compression
engram pack my_data --compression zstd

# Pack with manifest and signing
engram pack my_data --manifest manifest.toml --sign-key private.key
```

**Options:**
- `-o, --output <PATH>` - Output archive path (default: input name + `.eng`)
- `-c, --compression <METHOD>` - Compression: `none`, `lz4` (default), `zstd`
- `-m, --manifest <PATH>` - Manifest file (manifest.toml)
- `-k, --sign-key <PATH>` - Private key for signing

**Example Output:**
```
Packing: my_data
Output: my_data.eng
  Added: file1.txt
  Added: subdir/file2.txt
  Added: data.json
Packed 3 files
Archive created successfully: my_data.eng
```

### List - List Archive Contents

List all files in an archive:

```bash
# Simple list
engram list archive.eng

# Detailed list with sizes and compression
engram list archive.eng --long

# List only database files
engram list archive.eng --databases
```

**Example Output (--long):**
```
file1.txt                                          1024       512    lz4  (50.0%)
data/large.json                                   10240      2048   zstd  (20.0%)
```

### Info - Archive Metadata

Display archive metadata and statistics:

```bash
# Basic info
engram info archive.eng

# Detailed inspection with per-file details
engram info archive.eng --inspect

# Show manifest only
engram info archive.eng --manifest

# Verify signatures
engram info archive.eng --verify
```

**Example Output:**
```
Archive: archive.eng
Format Version: 0.3
Total Files: 3
Content Version: 0
Total Size: 11264 bytes
Compressed: 2560 bytes (22.7%)

Manifest:
  ID: my-archive
  Name: My Archive
  Version: 1.0.0
  Author: John Doe
  Signatures: 1
```

### Extract - Extract Files

Extract files from an archive:

```bash
# Extract all files
engram extract archive.eng --output ./extracted

# Extract specific files
engram extract archive.eng --output ./extracted --files file1.txt data.json
```

**Example Output:**
```
Extracting to: ./extracted
  Extracted: file1.txt
  Extracted: subdir/file2.txt
  Extracted: data.json
Extraction complete
```

### Sign - Sign Archives

Add cryptographic signatures to archives:

```bash
engram sign archive.eng --private-key private.key

# With signer identity
engram sign archive.eng --private-key private.key --signer "John Doe"
```

**Example Output:**
```
Signing: archive.eng
  Signature added
  Signer: John Doe
  Public key: cfc6873ad182091d5c1cef96c3d88d7dd5055a24004c5f710ec93afcebff3baf
```

### Verify - Verify Archives

Verify signatures and file integrity:

```bash
# Verify signatures
engram verify archive.eng --public-key public.key

# Verify file hashes
engram verify archive.eng --check-hashes

# Both
engram verify archive.eng --public-key public.key --check-hashes
```

**Example Output:**
```
Verifying: archive.eng

Verifying signatures...
  ✓ Signature 1 valid

Verifying file hashes...
  ✓ file1.txt hash valid
  ✓ file2.txt hash valid
  ✓ data.json hash valid

✓ Verification successful
```

### Query - SQLite Database Queries

Query SQLite databases embedded in archives:

```bash
# List all databases
engram query archive.eng --list-databases

# Execute SQL query
engram query archive.eng --database data.db --sql "SELECT * FROM users"

# Output as JSON
engram query archive.eng --database data.db --sql "SELECT * FROM users" --format json

# Output as CSV
engram query archive.eng --database data.db --sql "SELECT * FROM users" --format csv
```

**Example Output (table format):**
```
Querying database: data.db
id | name | email
------------------------------------------------------------
1 | Alice | alice@example.com
2 | Bob | bob@example.com
```

### Search - Text Search

Search for text patterns in files:

```bash
# Search in regular file
engram search "pattern" file.txt

# Search inside archive
engram search "pattern" archive.eng --in-archive

# Case-insensitive search
engram search "pattern" file.txt --case-insensitive
```

**Example Output:**
```
file1.txt:
  This line contains the pattern we're looking for
  Another line with the pattern here
```

## Manifest Format

Create a `manifest.toml` file for your archives:

```toml
id = "my-archive"
name = "My Archive"
description = "A description of my archive"
version = "1.0.0"
license = "MIT"
tags = ["data", "backup"]
capabilities = ["read", "query"]

[author]
name = "John Doe"
email = "john@example.com"
url = "https://example.com"
```

Then pack with the manifest:

```bash
engram pack my_data --manifest manifest.toml
```

## Complete Workflow Example

```bash
# 1. Generate keypair for signing
engram keygen --private-key my.key --public-key my.pub

# 2. Create manifest
cat > manifest.toml <<EOF
id = "backup-2025"
name = "Backup Archive"
version = "1.0.0"

[author]
name = "Admin"
EOF

# 3. Pack directory with manifest and signing
engram pack backup_data --manifest manifest.toml --sign-key my.key

# 4. Verify the archive
engram info backup_data.eng --verify

# 5. List contents
engram list backup_data.eng --long

# 6. Query database (if archive contains .db files)
engram query backup_data.eng --list-databases
engram query backup_data.eng --database data.db --sql "SELECT COUNT(*) FROM records"

# 7. Extract specific files
engram extract backup_data.eng --output ./restore --files config.json

# 8. Verify with public key
engram verify backup_data.eng --public-key my.pub --check-hashes
```

## Testing

```bash
# Run all tests
cargo test

# Run specific test
cargo test pack_directory

# Run tests with output
cargo test -- --nocapture
```

## Command Aliases

- `ls` → `list`
- `i` → `info`
- `p` → `pack`
- `x` → `extract`
- `q` → `query`

## Development

### Project Structure

```
engram-cli/
├── crates/
│   └── engram-cli/
│       ├── src/
│       │   ├── main.rs          # CLI entry point
│       │   ├── commands/        # Command implementations
│       │   │   ├── pack.rs
│       │   │   ├── list.rs
│       │   │   ├── info.rs
│       │   │   ├── extract.rs
│       │   │   ├── sign.rs
│       │   │   ├── verify.rs
│       │   │   ├── keygen.rs
│       │   │   ├── query.rs
│       │   │   └── search.rs
│       │   ├── crypto/          # Cryptography (keypairs)
│       │   ├── manifest/        # Manifest handling
│       │   └── utils/           # Utilities
│       └── tests/               # Integration tests
└── Cargo.toml
```

### Building

```bash
# Debug build
cargo build

# Release build
cargo build --release

# Check without building
cargo check

# Format code
cargo fmt

# Lint
cargo clippy
```

## License

MIT

## Related Projects

- [engram-specification](https://github.com/blackfall-labs/engram-specification) - Engram format specification
- [engram-rs](https://github.com/blackfall-labs/engram-rs) - Core Rust library
