# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

**engram-cli** is a comprehensive command-line interface tool for managing Engram archives (`.eng` files). It is part of the **Blackfall Labs ecosystem** and implements the Engram specification for creating, inspecting, extracting, signing, and querying compressed archive files.

Engrams are universal archive formats designed for immutability, offline-first access, cryptographic verification, and long-term knowledge preservation.

**Version:** 0.2.0 (post-migration)
**Edition:** Rust 2024
**Primary Dependency:** engram-rs v0.3+ (unified library)

## Architecture

### Dependency Migration (v0.1 → v0.2)

**IMPORTANT**: This project has been migrated from the old split-package architecture to the unified `engram-rs` library:

**Old (v0.1):**
```toml
engram-core = { git = "...", package = "engram-core" }
engram-vfs  = { git = "...", package = "engram-vfs" }
```

**New (v0.2+):**
```toml
engram-rs = { git = "https://github.com/Manifest-Humanity/engram-core", branch = "engram-rs-migration" }
```

The `engram-rs` library is a unified package that combines:
- Archive read/write (formerly `engram-core`)
- VFS (Virtual File System) for SQLite databases (formerly `engram-vfs`)
- Manifest support with Ed25519 signatures
- Encryption support (AES-256-GCM)

### Module Structure

```
crates/engram-cli/src/
├── main.rs                 # CLI entry point, command routing
├── commands/
│   ├── mod.rs
│   ├── pack.rs            # Create archives
│   ├── list.rs            # List files
│   ├── info.rs            # Show metadata
│   ├── extract.rs         # Extract files (NEW in v0.2)
│   ├── verify.rs          # Verify signatures/hashes (NEW in v0.2)
│   ├── sign.rs            # Add signatures (NEW in v0.2)
│   ├── keygen.rs          # Generate keypairs (NEW in v0.2)
│   ├── query.rs           # SQL queries via VFS (NEW in v0.2)
│   └── search.rs          # Text search
├── crypto/
│   └── keys.rs            # Ed25519 keypair management
├── manifest/
│   └── builder.rs         # TOML manifest → JSON conversion
└── utils/
    ├── compression.rs     # Compression helpers
    └── paths.rs           # Path normalization
```

### Key Features (v0.2)

**Core Commands:**
1. **pack** - Create archives (with manifest, signing, compression options)
2. **list** - List contents (with --long, --databases flags)
3. **info** - Show metadata (with --inspect, --verify, --manifest flags)
4. **extract** - Extract files (with selective extraction)
5. **verify** - Verify signatures and SHA-256 hashes
6. **sign** - Add Ed25519 signatures to archives
7. **keygen** - Generate Ed25519 keypairs
8. **query** - Execute SQL on embedded SQLite databases
9. **search** - Text pattern search (in files or archives)

**Cryptography:**
- Ed25519 signatures for authenticity
- SHA-256 file hashing for integrity
- Hex-encoded key storage

**Compression:**
- None, LZ4 (fast), Zstd (best ratio)
- Auto-selection based on file size/type

**VFS (Virtual File System):**
- Read-only SQLite database access within archives
- No extraction required for queries
- Multiple database support

## Development Commands

### Building

```bash
# Build debug version
cargo build

# Build release version
cargo build --release

# Binary location after build
target/release/engram.exe  # Windows
target/release/engram      # Unix
```

### Testing

```bash
# Run all tests
cargo test

# Run specific test
cargo test pack_directory

# Run tests with output
cargo test -- --nocapture

# Run tests in release mode
cargo test --release
```

### Code Quality

```bash
# Format code
cargo fmt

# Lint with Clippy
cargo clippy

# Fix warnings automatically
cargo fix
```

## Private Repository Access

**CRITICAL**: This project depends on the private `engram-rs` repository.

### Setup

1. **Create `.env` file** (gitignored, see `.env.example`):
   ```
   GITHUB_TOKEN=ghp_your_personal_access_token_here
   ```

2. **Configure Git credentials**:
   ```bash
   git config --global url."https://${GITHUB_TOKEN}@github.com/".insteadOf "https://github.com/"
   ```

3. **Never commit**:
   - `.env` file
   - Access tokens in any form
   - Private keys

### Generating GitHub Token

1. Go to GitHub Settings → Developer settings → Personal access tokens → Tokens (classic)
2. Click "Generate new token (classic)"
3. Grant the "repo" scope (full control of private repositories)
4. Copy token and add to `.env`

## Command Implementation Details

### pack.rs

**Key Functions:**
- `pack()` - Main packing logic
- Handles files and directories recursively
- Normalizes Windows paths to forward slashes
- Supports TOML manifest loading and conversion
- **TODO**: Full encryption support
- **TODO**: Signature integration during packing

**Path Normalization:**
```rust
let archive_path = normalize_path(relative_path); // Converts \ to /
```

### list.rs

**Key Functions:**
- `list()` - List files in archive
- `--long` flag shows sizes, compression, ratios
- `--databases` flag filters to .db/.sqlite files

**Borrowing Pattern:**
```rust
// Clone list to avoid borrow conflicts with reader
let all_files: Vec<String> = reader.list_files().to_vec();
```

### info.rs

**Key Functions:**
- `info()` - Show archive metadata
- Calculates compression ratios
- Displays manifest if present
- `--verify` checks signatures
- `--inspect` shows per-file details

**Manifest Parsing:**
```rust
if let Ok(manifest) = serde_json::from_value::<engram_rs::Manifest>(manifest_value) {
    // Access manifest fields
}
```

### extract.rs

**Key Functions:**
- `extract()` - Extract files from archive
- Creates directory structure
- Selective extraction with `--files` flag
- **TODO**: Decryption support

### verify.rs

**Key Functions:**
- `verify()` - Verify signatures and hashes
- Uses `Manifest::verify_signatures()` from engram-rs
- SHA-256 hash verification for file integrity
- Exit code 0 if valid, 1 if invalid

**SHA-256 Hashing:**
```rust
use sha2::{Digest, Sha256};
let hash = hex::encode(Sha256::digest(&data));
```

### sign.rs

**Key Functions:**
- `sign()` - Add signature to archive manifest
- Loads private key and creates signature
- **TODO**: Write updated manifest back to archive

### keygen.rs

**Key Functions:**
- `keygen()` - Generate Ed25519 keypair
- Saves as hex-encoded files (64 chars = 32 bytes)
- Uses OsRng for cryptographically secure randomness

### query.rs

**Key Functions:**
- `query()` - Execute SQL on embedded databases
- Uses `VfsReader` from engram-rs
- Output formats: table (default), json, csv
- Read-only queries for safety

### search.rs

**Key Functions:**
- `search()` - Text pattern matching
- In-archive search with `--in-archive`
- Case-insensitive option
- Simple substring matching (not regex)

## engram-rs API Reference

### Core Types

```rust
use engram_rs::{
    ArchiveReader,      // Read .eng files
    ArchiveWriter,      // Create .eng files
    CompressionMethod,  // None, Lz4, Zstd
    Manifest,           // Archive metadata
    Author,             // Manifest author info
    VfsReader,          // SQLite VFS access
};
```

### Common Patterns

**Opening an Archive:**
```rust
let mut reader = ArchiveReader::open("archive.eng")?;
let files = reader.list_files().to_vec();  // Clone to avoid borrow issues
let data = reader.read_file("file.txt")?;
```

**Creating an Archive:**
```rust
let mut writer = ArchiveWriter::create("archive.eng")?;
writer.add_file("file.txt", b"content")?;
writer.add_file_from_disk("readme.md", Path::new("README.md"))?;
writer.finalize()?;  // MUST call to write central directory
```

**Reading Manifest:**
```rust
let manifest_value = reader.read_manifest()?.context("No manifest")?;
let manifest: Manifest = serde_json::from_value(manifest_value)?;
```

**VFS Database Access:**
```rust
let mut vfs = VfsReader::open("archive.eng")?;
let conn = vfs.open_database("data.db")?;
let mut stmt = conn.prepare("SELECT * FROM users")?;
```

## Manifest Format

### Input (TOML)

Users provide `manifest.toml`:
```toml
id = "my-archive"
name = "My Archive"
description = "Description here"
version = "1.0.0"
license = "MIT"
tags = ["data", "backup"]
capabilities = ["read", "query"]

[author]
name = "John Doe"
email = "john@example.com"
url = "https://example.com"
```

### Storage (JSON)

Converted and stored as `manifest.json` in archive:
```json
{
  "version": "0.4.0",
  "id": "my-archive",
  "name": "My Archive",
  "description": "Description here",
  "author": {
    "name": "John Doe",
    "email": "john@example.com",
    "url": "https://example.com"
  },
  "metadata": {
    "version": "1.0.0",
    "created": 1703001600,
    "license": "MIT",
    "tags": ["data", "backup"]
  },
  "capabilities": ["read", "query"],
  "files": [],
  "signatures": []
}
```

## Archive Format (Engram v0.3)

**Binary Structure:**
```
[Header: 64 bytes]
[File Data 1]
[File Data 2]
...
[Central Directory: 320 bytes per entry]
[Manifest (optional)]
[Signatures (optional)]
```

**Magic Bytes:** `0x89 'E' 'N' 'G' 0x0D 0x0A 0x1A 0x0A` (PNG-style)

**Header Fields:**
- Format version (major.minor)
- Central directory offset & size
- Entry count
- Content version
- CRC32 checksum

**Entry Fields (320 bytes):**
- Path (max 255 bytes UTF-8)
- Data offset
- Uncompressed size
- Compressed size
- CRC32
- Modified time
- Compression method
- Flags

## Important Implementation Notes

### Windows Path Handling

**CRITICAL**: Always normalize paths when packing:
```rust
use crate::utils::paths::normalize_path;
let archive_path = normalize_path(&relative_path);  // Converts \ to /
```

Archives store paths with forward slashes (`/`) regardless of platform.

### Borrowing Patterns

**Problem:** `ArchiveReader::list_files()` returns `&[String]`, which borrows the reader.

**Solution:** Clone the list before using with mutable operations:
```rust
let all_files = reader.list_files().to_vec();  // Clone
for file in &all_files {
    let data = reader.read_file(file)?;  // Now we can mutably borrow
}
```

### Error Handling

All commands return `anyhow::Result<()>` with context:
```rust
reader.read_file(path)
    .with_context(|| format!("Failed to read file `{}`", path))?;
```

Error messages should:
- Be user-friendly
- Include relevant file paths
- Use backticks for code/paths

### Testing Patterns

**Integration Tests** (`tests/cli.rs`):
```rust
use assert_cmd::cargo::cargo_bin_cmd;
use predicates::prelude::*;
use tempfile::TempDir;

#[test]
fn test_pack_directory() -> Result<(), Box<dyn std::error::Error>> {
    let temp_dir = TempDir::new()?;
    // Create test data
    fs::write(temp_dir.path().join("test.txt"), "content")?;

    // Run command
    let mut cmd = cargo_bin_cmd!("engram");
    cmd.arg("pack").arg(temp_dir.path());

    // Assert
    cmd.assert().success()
        .stdout(predicate::str::contains("Archive created successfully"));

    Ok(())
}
```

## Migration from engram-builder

All features from `../sam/crates/engram-builder` have been migrated:

| Feature | engram-builder | engram-cli | Status |
|---------|----------------|------------|--------|
| Pack archives | ✅ | ✅ | Migrated |
| Sign archives | ✅ | ✅ | Migrated |
| Verify signatures | ✅ | ✅ | Migrated |
| Keygen | ✅ | ✅ | Migrated |
| Manifest support | ✅ | ✅ | Migrated |
| List files | ❌ | ✅ | Enhanced |
| Extract files | ❌ | ✅ | New |
| Query DBs | ❌ | ✅ | New |
| Search | ❌ | ✅ | New |

**Action Items:**
- ✅ All features migrated
- ⏳ Remove engram-builder from sam repo
- ⏳ Update sam dependencies to use engram-cli

## Common Tasks

### Adding a New Command

1. Create `src/commands/mycommand.rs`:
   ```rust
   use anyhow::Result;
   use std::path::Path;

   pub fn mycommand(path: &Path) -> Result<()> {
       // Implementation
       Ok(())
   }
   ```

2. Add to `src/commands/mod.rs`:
   ```rust
   pub mod mycommand;
   ```

3. Add to `Commands` enum in `src/main.rs`:
   ```rust
   MyCommand {
       path: PathBuf,
   }
   ```

4. Add match arm in `main()`:
   ```rust
   Commands::MyCommand { path } => {
       commands::mycommand::mycommand(&path)?;
   }
   ```

5. Write tests in `tests/cli.rs`

### Updating Dependencies

```bash
# Update Cargo.lock
cargo update

# Check for outdated deps
cargo outdated  # (requires cargo-outdated)

# Update specific package
cargo update -p package-name
```

### Debugging

```bash
# Run with debug output
RUST_LOG=debug cargo run -- command args

# Run with backtrace
RUST_BACKTRACE=1 cargo run -- command args

# Debug specific test
RUST_LOG=debug cargo test test_name -- --nocapture
```

## Release Checklist

- [ ] All tests passing (`cargo test`)
- [ ] No compiler warnings (`cargo clippy`)
- [ ] Code formatted (`cargo fmt`)
- [ ] README.md updated
- [ ] CHANGELOG.md updated
- [ ] Version bumped in `Cargo.toml`
- [ ] Build release binary (`cargo build --release`)
- [ ] Test release binary on clean system
- [ ] Create git tag: `git tag v0.2.0`
- [ ] Push tag: `git push origin v0.2.0`

## Related Repositories

- **engram-specification**: Format specification (../engram-specification)
- **engram-core** (now engram-rs): Rust library (../engram-core)
- **sam**: SAM ecosystem (../sam) - remove engram-builder after migration

## Troubleshooting

### Build Errors

**Problem:** `failed to authenticate when downloading repository`
**Solution:** Check `.env` file and Git credential configuration

**Problem:** `libsqlite3-sys version conflict`
**Solution:** Ensure `rusqlite` version matches engram-rs (v0.31)

### Runtime Errors

**Problem:** `Failed to open archive`
**Solution:** Check file exists, has `.eng` extension, valid Engram format

**Problem:** `No manifest found`
**Solution:** Archive was created without `--manifest` flag

**Problem:** `Signature invalid`
**Solution:** Archive modified after signing, or wrong public key

## Support

For issues or questions:
- Check MIGRATION_PLAN.md for detailed migration notes
- Review README.md for usage examples
- Consult engram-specification for format details
- Report bugs at: https://github.com/Manifest-Humanity/engram-cli/issues
