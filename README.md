# Wrench - Diagnostics CLI for Blackfall Labs Archive Formats

**Version:** 0.1.0
**License:** MIT OR Apache-2.0

Wrench is a diagnostics CLI for Engram, Cartridge, DataSpool, and DataCard file formats used across the Blackfall Labs ecosystem. It provides format-agnostic commands that automatically detect and handle Engram, Cartridge, DataSpool, and DataCard archives.

## 🚀 Quick Start

```bash
# Auto-detects format and shows info
wrench info archive.eng      # Engram
wrench info data.cart        # Cartridge
wrench list items.spool      # DataSpool
wrench extract doc.card --output ./    # DataCard

# Format-specific commands
wrench pack ./data --output archive.eng
wrench cartridge-create "mydata" "My Data"
wrench data-spool-build --output items.spool card1.card card2.card
```

## 📦 Supported Formats

| Format        | Ext      | Type        | Use Case                                           |
| ------------- | -------- | ----------- | -------------------------------------------------- |
| **Engram**    | `.eng`   | Immutable   | Cryptographically signed archives for preservation |
| **Cartridge** | `.cart`  | Mutable     | Working archives with in-place modifications       |
| **DataSpool** | `.spool` | Append-only | Item collections with byte-offset index            |
| **DataCard**  | `.card`  | Immutable   | BytePunch-compressed CML documents                 |

## 🔧 Installation

### From Source (Recommended)

```bash
# Clone repository
git clone https://github.com/blackfall-labs/wrench
cd wrench

# Build release binary
cargo build --release

# Binary location
target/release/wrench.exe  # Windows
target/release/wrench      # Linux/macOS

# Optional: Install to cargo bin
cargo install --path .

# Or copy to a directory in your PATH
# Windows
copy target\release\wrench.exe %LOCALAPPDATA%\Programs\

# Linux/macOS
sudo cp target/release/wrench /usr/local/bin/
# or for user install
mkdir -p ~/.local/bin
cp target/release/wrench ~/.local/bin/
```

### Prerequisites

- **Rust:** 2024 edition (Rust 1.75+)
- **Platform:** Windows, Linux, or macOS

### Verify Installation

```bash
# Check version
wrench --version

# Show help
wrench --help

# Test with a simple command
wrench keygen --private-key test.key --public-key test.pub
```

For detailed installation instructions including pre-built binaries and platform-specific guides, see [INSTALLATION.md](INSTALLATION.md).

## 🎯 Format Compatibility Matrix

| Command                | Engram | Cartridge | DataSpool | DataCard | Notes                              |
| ---------------------- | ------ | --------- | --------- | -------- | ---------------------------------- |
| **Universal Commands** |
| `info`                 | ✅     | ✅        | ✅        | ✅       | Auto-detects format                |
| `list`                 | ✅     | ✅        | ✅        | ✅       | Shows files/items                  |
| `extract`              | ✅     | ✅        | ✅        | ✅       | Extracts contents                  |
| `verify`               | ✅     | ✅        | ✅        | ✅       | Format-specific validation         |
| `search`               | ✅     | ✅        | ✅        | ⚠️       | DataCard: searches compressed data |
| `query`                | ✅     | ✅        | ❌        | ❌       | Requires SQLite VFS                |
| **Engram-Specific**    |
| `pack`                 | ✅     | ❌        | ❌        | ❌       | Create immutable archives          |
| `sign`                 | ✅     | ❌        | ❌        | ❌       | Ed25519 signatures                 |
| `keygen`               | ✅     | ❌        | ❌        | ❌       | Generate keypairs                  |
| **Cartridge-Specific** |
| `cartridge-create`     | ❌     | ✅        | ❌        | ❌       | Create mutable archive             |
| `cartridge-write`      | ❌     | ✅        | ❌        | ❌       | Write/update files                 |
| `cartridge-delete`     | ❌     | ✅        | ❌        | ❌       | Delete files                       |
| `cartridge-snapshot`   | ❌     | ✅        | ❌        | ❌       | Create snapshots                   |
| **DataSpool-Specific** |
| `data-spool-build`     | ❌     | ❌        | ✅        | ❌       | Build from cards                   |
| `data-spool-append`    | ❌     | ❌        | ✅        | ❌       | Append new cards                   |
| `data-spool-index`     | ❌     | ❌        | ✅        | ❌       | Show byte offsets                  |
| **DataCard-Specific**  |
| `data-card-compress`   | ❌     | ❌        | ❌        | ✅       | CML → DataCard                     |
| `data-card-decompress` | ❌     | ❌        | ❌        | ✅       | DataCard → CML                     |
| `data-card-validate`   | ❌     | ❌        | ❌        | ✅       | Validate structure                 |

## 📖 Command Reference

### Universal Commands (All Formats)

#### `info` - Display Archive Metadata

```bash
wrench info <archive> [options]
```

**Options:**

- `--inspect` - Show detailed per-file information
- `--verify` - Verify signatures and hashes
- `--manifest` - Show manifest only

**Examples:**

```bash
# Basic info
wrench info research.eng

# Full inspection with verification
wrench info research.eng --inspect --verify

# Show manifest only
wrench info research.eng --manifest
```

#### `list` - List Files/Items

```bash
wrench list <archive> [options]
```

**Options:**

- `--long`, `-l` - Show detailed information (size, compression, dates)
- `--databases`, `-d` - List only database files (.db, .sqlite)

**Examples:**

```bash
# Basic listing
wrench list archive.eng

# Detailed listing
wrench list archive.eng --long

# Show only databases
wrench list archive.cart --databases
```

#### `extract` - Extract Contents

```bash
wrench extract <archive> --output <path> [options]
```

**Options:**

- `--output`, `-o` - Output directory (required)
- `--files`, `-f` - Extract only specific files (space-separated)
- `--decrypt` - Decrypt encrypted archive with password (Engram only)

**Examples:**

```bash
# Extract all files
wrench extract archive.eng --output ./extracted

# Extract specific files
wrench extract archive.eng --output ./extracted --files data.db README.md

# Extract DataCard
wrench extract document.card --output ./output
```

#### `verify` - Verify Archive Integrity

```bash
wrench verify <archive> [options]
```

**Options:**

- `--public-key`, `-k` - Public key for signature verification (Engram)
- `--check-hashes` - Verify file hashes from manifest (Engram)

**Examples:**

```bash
# Basic verification
wrench verify archive.eng

# Verify with specific public key
wrench verify archive.eng --public-key pubkey.hex

# Verify file hashes
wrench verify archive.eng --check-hashes
```

#### `search` - Search Text Patterns

```bash
wrench search <pattern> <path> [options]
```

**Options:**

- `--in-archive` - Search inside archive files
- `--case-insensitive`, `-i` - Case-insensitive search

**Examples:**

```bash
# Search in archive
wrench search "error" logs.eng --in-archive

# Case-insensitive search
wrench search "TODO" project.cart -i
```

#### `query` - Query SQLite Databases

**Supported:** Engram, Cartridge only

```bash
wrench query <archive> [options]
```

**Options:**

- `--list-databases`, `-l` - List all databases in archive
- `--database`, `-d` - Database file path within archive
- `--sql`, `-s` - SQL query to execute
- `--format`, `-f` - Output format: json, csv, table (default: table)

**Examples:**

```bash
# List databases
wrench query archive.eng --list-databases

# Query database (table output)
wrench query archive.eng --database data.db --sql "SELECT * FROM users"

# Query with JSON output
wrench query archive.eng -d data.db -s "SELECT name, age FROM users" -f json

# Query with CSV output
wrench query archive.cart -d stats.db -s "SELECT * FROM metrics" -f csv
```

### Engram-Specific Commands

#### `pack` - Create Archive

```bash
wrench pack <path> [options]
```

**Options:**

- `--output`, `-o` - Output archive path
- `--compression`, `-c` - Compression method: none, lz4, zstd (default: lz4)
- `--manifest`, `-m` - Manifest file (manifest.toml)
- `--sign-key`, `-k` - Private key for signing
- `--encrypt` - Encrypt entire archive with password
- `--encrypt-per-file` - Encrypt each file individually

**Examples:**

```bash
# Pack directory with LZ4 compression
wrench pack ./data --output archive.eng

# Pack with manifest and signing
wrench pack ./data -o archive.eng -m manifest.toml -k private.hex

# Pack with Zstd compression
wrench pack ./data -o archive.eng -c zstd

# Pack with encryption
wrench pack ./data -o archive.eng --encrypt
```

#### `sign` - Sign Archive

```bash
wrench sign <archive> --private-key <path> [options]
```

**Options:**

- `--private-key`, `-k` - Private key file (required)
- `--signer`, `-s` - Signer identity

**Examples:**

```bash
# Sign archive
wrench sign archive.eng --private-key private.hex

# Sign with identity
wrench sign archive.eng -k private.hex -s "John Doe <john@example.com>"
```

#### `keygen` - Generate Ed25519 Keypair

```bash
wrench keygen --private-key <path> --public-key <path>
```

**Options:**

- `--private-key`, `-r` - Output path for private key (required)
- `--public-key`, `-u` - Output path for public key (required)

**Examples:**

```bash
# Generate keypair
wrench keygen --private-key private.hex --public-key public.hex
```

### Cartridge-Specific Commands

#### `cartridge-create` - Create Mutable Archive

```bash
wrench cartridge-create <slug> <title> [options]
```

**Options:**

- `--output`, `-o` - Output path (default: slug.cart)

**Examples:**

```bash
# Create Cartridge
wrench cartridge-create "my-project" "My Project"

# Create with custom path
wrench cartridge-create "my-project" "My Project" --output ./archives/project.cart
```

#### `cartridge-write` - Write/Update File

```bash
wrench cartridge-write <archive> <file-path> <source>
```

**Examples:**

```bash
# Write file to Cartridge
wrench cartridge-write project.cart "docs/README.md" ./README.md

# Update existing file
wrench cartridge-write project.cart "data/config.json" ./new-config.json
```

#### `cartridge-delete` - Delete File

```bash
wrench cartridge-delete <archive> <file-path>
```

**Examples:**

```bash
# Delete file from Cartridge
wrench cartridge-delete project.cart "old-data.txt"
```

#### `cartridge-snapshot` - Create Snapshot

```bash
wrench cartridge-snapshot <archive> --name <name> --description <desc> --snapshot-dir <path>
```

**Options:**

- `--name`, `-n` - Snapshot name (required)
- `--description`, `-d` - Snapshot description (required)
- `--snapshot-dir`, `-d` - Snapshot directory (required)

**Examples:**

```bash
# Create snapshot
wrench cartridge-snapshot project.cart \
  --name "v1.0.0" \
  --description "Release version 1.0.0" \
  --snapshot-dir ./snapshots
```

### DataSpool-Specific Commands

#### `data-spool-build` - Build Spool from Cards

```bash
wrench data-spool-build --output <path> <card1> <card2> ...
```

**Options:**

- `--output`, `-o` - Output spool path (required)

**Examples:**

```bash
# Build spool from card files
wrench data-spool-build --output items.spool doc1.card doc2.card doc3.card

# Build from wildcard
wrench data-spool-build -o items.spool docs/*.card
```

#### `data-spool-append` - Append Cards

```bash
wrench data-spool-append <spool> <card1> <card2> ...
```

**Examples:**

```bash
# Append new cards
wrench data-spool-append items.spool new1.card new2.card
```

#### `data-spool-index` - Show Index

```bash
wrench data-spool-index <spool>
```

**Examples:**

```bash
# Show byte offsets
wrench data-spool-index items.spool
```

### DataCard-Specific Commands

#### `data-card-compress` - Compress CML to DataCard

```bash
wrench data-card-compress <cml> --output <path> --dict <dict> --id <id> [options]
```

**Options:**

- `--output`, `-o` - Output card path (required)
- `--dict`, `-d` - BytePunch dictionary path (required)
- `--id` - Document ID (required)
- `--checksum` - Add CRC32 checksum

**Examples:**

```bash
# Compress CML document
wrench data-card-compress document.cml \
  --output document.card \
  --dict dictionary.json \
  --id "doc-001"

# Compress with checksum
wrench data-card-compress document.cml \
  -o document.card \
  -d dictionary.json \
  --id "doc-001" \
  --checksum
```

#### `data-card-decompress` - Decompress DataCard to CML

```bash
wrench data-card-decompress <card> --output <path> --dict <dict>
```

**Options:**

- `--output`, `-o` - Output CML path (required)
- `--dict`, `-d` - BytePunch dictionary path (required)

**Examples:**

```bash
# Decompress DataCard
wrench data-card-decompress document.card \
  --output document.cml \
  --dict dictionary.json
```

#### `data-card-validate` - Validate DataCard

```bash
wrench data-card-validate <card>
```

**Examples:**

```bash
# Validate card structure
wrench data-card-validate document.card
```

## 🎨 Workflow Examples

### Research Project Workflow

```bash
# 1. Create working Cartridge archive
wrench cartridge-create "research-2024" "Research Project 2024"

# 2. Add initial files
wrench cartridge-write research-2024.cart "notes/ideas.md" ./ideas.md
wrench cartridge-write research-2024.cart "data/experiments.db" ./experiments.db

# 3. Work iteratively (update files as needed)
wrench cartridge-write research-2024.cart "notes/ideas.md" ./ideas-updated.md

# 4. Create snapshot at milestone
wrench cartridge-snapshot research-2024.cart \
  --name "phase-1-complete" \
  --description "Completed initial experiments" \
  --snapshot-dir ./snapshots

# 5. Query embedded database
wrench query research-2024.cart \
  --database data/experiments.db \
  --sql "SELECT * FROM results WHERE status='completed'"

# 6. When ready to preserve, convert to immutable Engram
wrench pack ./extracted-cartridge --output research-2024-final.eng \
  --compression zstd \
  --manifest manifest.toml \
  --sign-key private.hex
```

### CML Document Processing Pipeline

```bash
# 1. Extract text from PDF
# (using byte-shredder-rs, not shown here)

# 2. Convert to CML format
# (using content-markup-language tools, not shown here)

# 3. Create BytePunch dictionary from corpus
# (using bytepunch-rs training, not shown here)

# 4. Compress individual CML documents to DataCards
wrench data-card-compress doc1.cml -o doc1.card -d corpus.dict --id "doc-001" --checksum
wrench data-card-compress doc2.cml -o doc2.card -d corpus.dict --id "doc-002" --checksum
wrench data-card-compress doc3.cml -o doc3.card -d corpus.dict --id "doc-003" --checksum

# 5. Build DataSpool collection
wrench data-spool-build --output documents.spool doc1.card doc2.card doc3.card

# 6. Verify DataSpool integrity
wrench verify documents.spool

# 7. Extract specific card from spool
wrench extract documents.spool --output ./output

# 8. View spool index
wrench data-spool-index documents.spool

# 9. Append new documents
wrench data-card-compress doc4.cml -o doc4.card -d corpus.dict --id "doc-004"
wrench data-spool-append documents.spool doc4.card
```

### Archive Migration and Verification

```bash
# 1. Create initial archive with old data
wrench pack ./legacy-data --output legacy.eng --compression lz4

# 2. Verify archive integrity
wrench verify legacy.eng

# 3. List contents
wrench list legacy.eng --long

# 4. Extract for processing
wrench extract legacy.eng --output ./working

# 5. Create mutable Cartridge for updates
wrench cartridge-create "legacy-updated" "Legacy Data Updated"

# 6. Copy files and make modifications
wrench cartridge-write legacy-updated.cart "data.json" ./working/data.json
# ... make updates ...

# 7. Query databases to verify changes
wrench query legacy-updated.cart -d data.db -s "SELECT COUNT(*) FROM records"

# 8. Create final immutable archive with stronger compression
wrench pack ./updated-data --output legacy-final.eng \
  --compression zstd \
  --manifest manifest.toml \
  --sign-key private.hex

# 9. Verify signatures
wrench verify legacy-final.eng --public-key public.hex --check-hashes

# 10. Search for specific content
wrench search "configuration" legacy-final.eng --in-archive
```

## 🏗️ Architecture

### Format Detection

Wrench automatically detects archive formats using magic bytes:

```rust
// Engram: PNG-style magic (8 bytes)
0x89 'E' 'N' 'G' 0x0D 0x0A 0x1A 0x0A

// Cartridge: Version header (8 bytes)
'C' 'A' 'R' 'T' 0x00 0x01 0x00 0x00

// DataSpool: Version marker (4 bytes)
'S' 'P' '0' '1'

// DataCard: Format marker (4 bytes)
'C' 'A' 'R' 'D'
```

### Trait System

Unified interface through Rust traits:

```rust
trait Archive {
    fn open(path: &Path) -> Result<Self>;
    fn info(&mut self) -> Result<ArchiveInfo>;
    fn list_files(&mut self) -> Result<Vec<FileEntry>>;
    fn extract(&mut self, output: &Path, files: Option<&[String]>) -> Result<()>;
    fn verify(&mut self) -> Result<bool>;
    fn search(&mut self, pattern: &str, case_insensitive: bool) -> Result<Vec<SearchResult>>;
}

trait MutableArchive: Archive {
    fn write_file(&mut self, path: &str, data: &[u8]) -> Result<()>;
    fn delete_file(&mut self, path: &str) -> Result<()>;
}

trait QueryableArchive: Archive {
    fn list_databases(&mut self) -> Result<Vec<String>>;
    fn query(&mut self, database: &str, sql: &str, format: OutputFormat) -> Result<String>;
}
```

### Module Structure

```
wrench/
├── src/
│   ├── main.rs              # CLI entry point
│   ├── commands/
│   │   ├── shared.rs        # Format-agnostic commands
│   │   ├── cartridge.rs     # Cartridge-specific
│   │   ├── dataspool.rs     # DataSpool-specific
│   │   ├── datacard.rs      # DataCard-specific
│   │   └── [engram commands]
│   ├── formats/
│   │   ├── traits.rs        # Archive traits
│   │   ├── detection.rs     # Magic byte detection
│   │   ├── engram.rs        # Engram wrapper
│   │   ├── cartridge.rs     # Cartridge wrapper
│   │   ├── dataspool.rs     # DataSpool wrapper
│   │   └── datacard.rs      # DataCard wrapper
│   ├── crypto/              # Ed25519 keys
│   ├── manifest/            # TOML → JSON conversion
│   └── utils/               # Compression, paths
└── Cargo.toml
```

## 📊 Performance Characteristics

| Format    | Read Speed | Write Speed     | Compression           | Searchable | Queryable        |
| --------- | ---------- | --------------- | --------------------- | ---------- | ---------------- |
| Engram    | Fast       | N/A (immutable) | High (Zstd)           | Yes        | Yes (SQLite VFS) |
| Cartridge | Fast       | Medium          | None                  | Yes        | Yes (SQLite VFS) |
| DataSpool | Very Fast  | Append-only     | N/A                   | Yes        | No               |
| DataCard  | Medium     | N/A (immutable) | Very High (BytePunch) | Limited    | No               |

## 🔒 Security Features

### Engram

- Ed25519 signatures (cryptographic authenticity)
- SHA-256 file hashing (integrity verification)
- AES-256-GCM encryption (confidentiality)
- Manifest with embedded signatures

### Cartridge

- Snapshot-based versioning
- Page-level integrity checks
- B-tree catalog structure

### DataSpool

- Append-only design (tamper-evident)
- Byte-offset index (efficient access)
- Companion SQLite database (queryable metadata)

### DataCard

- CRC32 checksums (data integrity)
- BytePunch compression (space-efficient)
- Immutable design (preserves original state)

## 🧪 Testing

```bash
# Run all tests
cargo test

# Run specific module tests
cargo test --test cli

# Run with output
cargo test -- --nocapture

# Run benchmarks (if available)
cargo bench
```

## 🤝 Contributing

Wrench is part of the Blackfall Labs ecosystem. Contributions welcome!

1. Follow Rust 2024 edition standards
2. Run `cargo fmt` and `cargo clippy` before committing
3. Write tests for new commands
4. Update documentation
5. Follow commit message format:

   ```
   <type>(<scope>): <subject>

   Generated with Claude Code
   ```

## 📝 License

Dual-licensed under MIT OR Apache-2.0

## 🔗 Related Projects

- **engram-rs** - Immutable signed archives
- **cartridge-rs** - Mutable page-based archives with S3 API
- **dataspool-rs** - Append-only item collections
- **datacard-rs** - BytePunch-compressed CML documents
- **bytepunch-rs** - Profile-aware compression
- **content-markup-language** - Structured knowledge format

## 📚 Resources

- [Engram Specification](../engram-specification/)
- [CML Documentation](../content-markup-language/)
- [Blackfall Labs Ecosystem](../CLAUDE.md)

## 🐛 Troubleshooting

### "Unknown archive format"

- Ensure file has correct magic bytes
- Check file isn't corrupted
- Verify file extension matches format

### "Failed to open archive"

- Check file exists and is readable
- Verify file permissions
- Ensure format library is properly installed

### "Signature verification failed"

- Verify using correct public key
- Check archive hasn't been modified
- Ensure manifest is present

### "Database not found in archive"

- List databases with `--list-databases`
- Check database path is correct
- Verify archive contains SQLite VFS

---

**Built with ❤️ by Blackfall Labs**
