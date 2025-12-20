# Engram CLI Migration Plan

**Goal**: Migrate engram-cli to engram-rs v0.4.1 and integrate all features from engram-builder

**Status**: Planning
**Date**: 2025-12-19

---

## Phase 1: Dependency Migration

### 1.1 Update Cargo.toml
- [ ] Update workspace dependencies to engram-rs
- [ ] Remove old engram-core and engram-vfs references
- [ ] Add cryptography dependencies (ed25519-dalek, sha2, hex, rand)
- [ ] Add serialization: toml support
- [ ] Add logging: tracing, tracing-subscriber
- [ ] Configure Git access token requirement in .env

**Files to modify:**
- `Cargo.toml` (workspace root)
- `crates/engram-cli/Cargo.toml`

**Dependencies to add:**
```toml
engram-rs = { git = "https://github.com/Manifest-Humanity/engram-core", branch = "engram-rs-migration" }
ed25519-dalek = { version = "2.1", features = ["rand_core"] }
sha2 = "0.10"
hex = "0.4"
rand = "0.8"
toml = "0.8"
tracing = "0.1"
tracing-subscriber = "0.1"
```

### 1.2 Create .env.example
- [ ] Create .env.example with GITHUB_TOKEN placeholder
- [ ] Document Git credential setup in README.md

---

## Phase 2: Code Structure Reorganization

### 2.1 Create Module Structure
```
crates/engram-cli/src/
├── main.rs           (CLI entry point, command routing)
├── commands/
│   ├── mod.rs
│   ├── pack.rs       (create archives)
│   ├── list.rs       (list files)
│   ├── info.rs       (show metadata)
│   ├── extract.rs    (extract files) - NEW
│   ├── verify.rs     (verify signatures/hashes) - NEW
│   ├── sign.rs       (add signatures) - NEW
│   ├── keygen.rs     (generate keypairs) - NEW
│   ├── query.rs      (SQL queries) - NEW
│   └── search.rs     (text search)
├── crypto/
│   ├── mod.rs
│   └── keys.rs       (keypair management from engram-builder)
├── manifest/
│   ├── mod.rs
│   └── builder.rs    (manifest creation/validation)
└── utils/
    ├── mod.rs
    ├── compression.rs (compression helpers)
    └── paths.rs      (path normalization)
```

### 2.2 Migrate from engram-builder
- [ ] Copy `keygen.rs` → `crypto/keys.rs` (adapt for engram-rs)
- [ ] Extract manifest logic from `builder.rs` → `manifest/builder.rs`
- [ ] Adapt signing/verification logic for new structure

---

## Phase 3: Command Implementation

### Priority 1: Foundation Commands

#### 3.1 keygen (Simple, no dependencies)
- [ ] Implement `commands/keygen.rs`
- [ ] Generate Ed25519 keypairs
- [ ] Save as hex-encoded files
- [ ] Test: roundtrip generation and loading

**Usage:**
```bash
engram keygen --private private.key --public public.key
```

#### 3.2 pack (Extend current implementation)
- [ ] Update `commands/pack.rs` to use engram-rs
- [ ] Add --compression flag (none|lz4|zstd)
- [ ] Add --encrypt flag (archive-level)
- [ ] Add --encrypt-per-file flag
- [ ] Add --manifest flag (load TOML manifest)
- [ ] Add --sign flag (sign with private key)
- [ ] Add --verbose flag (tracing output)
- [ ] Update tests for new features

**Breaking changes:**
- Uses engram-rs API instead of engram-core
- Manifest format changes (TOML → JSON in archive)

#### 3.3 verify (From engram-builder)
- [ ] Implement `commands/verify.rs`
- [ ] Verify Ed25519 signatures
- [ ] Verify SHA-256 file hashes from manifest
- [ ] Show per-file verification results with --verbose
- [ ] Exit code: 0 if valid, 1 if invalid

**Usage:**
```bash
engram verify archive.eng --public-key key.pub
engram verify archive.eng --check-hashes --verbose
```

#### 3.4 sign (From engram-builder)
- [ ] Implement `commands/sign.rs`
- [ ] Add signature to existing archive
- [ ] Update manifest.json with signature entry
- [ ] Support --signer identity field

**Usage:**
```bash
engram sign archive.eng --private-key key.priv --signer "Author Name"
```

### Priority 2: Enhanced Commands

#### 3.5 list (Extend current)
- [ ] Update `commands/list.rs` to use engram-rs
- [ ] Add --long flag (show sizes, compression, dates)
- [ ] Add --databases flag (filter .db/.sqlite files)
- [ ] Maintain backward compatibility (default simple list)

**Usage:**
```bash
engram list archive.eng
engram list archive.eng --long
engram list archive.eng --databases
```

#### 3.6 info (Extend current)
- [ ] Update `commands/info.rs` to use engram-rs
- [ ] Add --verify flag (check signatures + hashes)
- [ ] Add --manifest flag (show manifest only)
- [ ] Keep --inspect flag for detailed file info
- [ ] Show encryption mode if encrypted

**Usage:**
```bash
engram info archive.eng
engram info archive.eng --inspect
engram info archive.eng --verify
engram info archive.eng --manifest
```

#### 3.7 search (Keep current, add archive support)
- [ ] Update `commands/search.rs`
- [ ] Add --in-archive flag (search inside .eng files)
- [ ] Add --case-insensitive flag
- [ ] Default: search regular files (current behavior)

**Usage:**
```bash
engram search "pattern" file.txt
engram search "pattern" archive.eng --in-archive
```

### Priority 3: New Commands

#### 3.8 extract (Critical missing feature)
- [ ] Implement `commands/extract.rs`
- [ ] Extract all files to directory
- [ ] Extract specific files with --files
- [ ] Support --decrypt for encrypted archives
- [ ] Preserve directory structure
- [ ] Show progress with --verbose

**Usage:**
```bash
engram extract archive.eng --output ./extracted
engram extract archive.eng --files README.md LICENSE --output ./
engram extract archive.eng --output ./ --decrypt
```

#### 3.9 query (Leverage VFS)
- [ ] Implement `commands/query.rs`
- [ ] List databases with --list-databases
- [ ] Execute SQL with --database and --sql
- [ ] Output formats: json, csv, table (--output)
- [ ] Read-only safety (use VfsReader)

**Usage:**
```bash
engram query archive.eng --list-databases
engram query archive.eng --database data.db --sql "SELECT * FROM users"
engram query archive.eng --database data.db --sql "SELECT * FROM users" --output json
```

---

## Phase 4: Testing Strategy

### 4.1 Unit Tests
- [ ] Test keygen: generation, save, load roundtrip
- [ ] Test pack: compression selection, encryption, signing
- [ ] Test verify: signature validation, hash checking
- [ ] Test sign: adding signatures to archives
- [ ] Test extract: file extraction, decryption
- [ ] Test query: VFS database access

### 4.2 Integration Tests
- [ ] Test full workflow: pack → sign → verify → extract
- [ ] Test encrypted archives: pack --encrypt → extract --decrypt
- [ ] Test manifest workflow: pack --manifest → info --manifest
- [ ] Test VFS workflow: pack (with .db) → query --list-databases → query --sql
- [ ] Test backwards compatibility with existing .eng files

### 4.3 Migration Tests
- [ ] Verify old .eng files can still be read
- [ ] Test transition from engram-core to engram-rs API
- [ ] Validate manifest format migration (if needed)

---

## Phase 5: Documentation

### 5.1 Update README.md
- [ ] Document all new commands
- [ ] Add encryption examples
- [ ] Add signing/verification workflow
- [ ] Add VFS query examples
- [ ] Update building instructions
- [ ] Document .env setup for private repo access

### 5.2 Update CLAUDE.md
- [ ] Update dependency information (engram-rs v0.4.1)
- [ ] Document new command structure
- [ ] Add testing commands for new features
- [ ] Document manifest.toml format
- [ ] Add encryption/signing workflows
- [ ] Remove outdated engram-core references

### 5.3 Create Examples
- [ ] Create example manifest.toml files
- [ ] Create example signing workflow script
- [ ] Create example VFS query script
- [ ] Create example encrypted archive workflow

---

## Phase 6: Cleanup

### 6.1 Remove engram-builder from sam
- [ ] Verify all features migrated to engram-cli
- [ ] Archive engram-builder code (git tag)
- [ ] Remove from sam/crates/engram-builder
- [ ] Update sam workspace Cargo.toml
- [ ] Update any references in sam codebase

### 6.2 Code Quality
- [ ] Run cargo fmt
- [ ] Run cargo clippy
- [ ] Fix all warnings
- [ ] Update dependencies to latest compatible versions
- [ ] Run cargo test (all tests pass)

---

## Phase 7: Release

### 7.1 Pre-release Checklist
- [ ] All tests passing
- [ ] Documentation complete
- [ ] CHANGELOG.md updated
- [ ] Version bump (0.1.0 → 0.2.0)
- [ ] Tag release: v0.2.0

### 7.2 Post-release
- [ ] Build release binaries (Windows, Linux, macOS)
- [ ] Test binaries on clean systems
- [ ] Update project README with download links
- [ ] Archive old engram-builder tag

---

## Risk Mitigation

### Breaking Changes
- **API change**: engram-core → engram-rs
  - *Mitigation*: Both use same archive format, backward compatible

- **Manifest format**: engram-builder uses TOML, engram-rs uses JSON
  - *Mitigation*: CLI accepts TOML input, converts to JSON for archive

- **Encryption**: New feature, old archives won't have it
  - *Mitigation*: Optional flag, backward compatible

### Dependencies
- **Private GitHub repo**: Requires GITHUB_TOKEN
  - *Mitigation*: Document in README, provide .env.example

- **Git branch**: Using engram-rs-migration branch
  - *Mitigation*: Switch to main once merged upstream

### Testing
- **Complex integration**: Multiple features interact
  - *Mitigation*: Comprehensive integration test suite

- **Windows path handling**: Backslash normalization
  - *Mitigation*: Test on Windows, use normalize_path utility

---

## Timeline Estimate

| Phase | Estimated Time | Priority |
|-------|---------------|----------|
| 1. Dependencies | 30 min | P0 |
| 2. Structure | 1 hour | P0 |
| 3.1-3.4 (Priority 1) | 3 hours | P0 |
| 3.5-3.7 (Priority 2) | 2 hours | P1 |
| 3.8-3.9 (Priority 3) | 2 hours | P1 |
| 4. Testing | 2 hours | P0 |
| 5. Documentation | 1 hour | P1 |
| 6. Cleanup | 30 min | P2 |
| 7. Release | 30 min | P2 |
| **Total** | **~12 hours** | |

---

## Success Criteria

- ✅ All engram-builder features migrated
- ✅ All existing tests pass
- ✅ New tests cover all new features
- ✅ Documentation complete and accurate
- ✅ Builds successfully on Windows/Linux/macOS
- ✅ Backward compatible with existing .eng files
- ✅ engram-builder can be safely removed from sam

---

## Next Steps

1. Review this plan
2. Get approval for command structure
3. Create TodoWrite tasks for Phase 1
4. Begin implementation
