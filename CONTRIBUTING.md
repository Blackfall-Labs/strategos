# Contributing to Engram CLI

Thank you for your interest in contributing to Engram CLI! This document provides guidelines and instructions for contributing.

## Getting Started

### Prerequisites

- Rust 1.75 or later (2024 edition)
- Git
- A GitHub account

### Setting Up Your Development Environment

1. **Fork and clone the repository:**
   ```bash
   git clone https://github.com/YOUR_USERNAME/engram-cli
   cd engram-cli
   ```

2. **Build the project:**
   ```bash
   cargo build
   ```

3. **Run tests:**
   ```bash
   cargo test
   ```

4. **Run the CLI:**
   ```bash
   cargo run -- --help
   ```

## Development Workflow

### Making Changes

1. **Create a new branch:**
   ```bash
   git checkout -b feature/your-feature-name
   ```

2. **Make your changes** and ensure they follow the coding standards:
   ```bash
   # Format code
   cargo fmt

   # Run linter
   cargo clippy

   # Run tests
   cargo test
   ```

3. **Commit your changes:**
   ```bash
   git add .
   git commit -m "feat: add your feature description"
   ```

   Use conventional commit messages:
   - `feat:` - New feature
   - `fix:` - Bug fix
   - `docs:` - Documentation changes
   - `refactor:` - Code refactoring
   - `test:` - Adding or updating tests
   - `chore:` - Maintenance tasks

4. **Push to your fork:**
   ```bash
   git push origin feature/your-feature-name
   ```

5. **Create a Pull Request** on GitHub

### Code Style

- Follow Rust standard style (enforced by `rustfmt`)
- Use meaningful variable and function names
- Add comments for complex logic
- Write documentation for public APIs
- Keep functions focused and concise

### Testing

- Write unit tests for new functionality
- Add integration tests for commands in `tests/cli.rs`
- Ensure all tests pass before submitting PR
- Aim for good test coverage

Example test:
```rust
#[test]
fn test_pack_directory() -> Result<(), Box<dyn std::error::Error>> {
    let temp_dir = TempDir::new()?;
    fs::write(temp_dir.path().join("test.txt"), "content")?;

    let mut cmd = cargo_bin_cmd!("engram");
    cmd.arg("pack").arg(temp_dir.path());
    cmd.assert().success();

    Ok(())
}
```

### Documentation

- Update README.md if adding new features
- Update CLAUDE.md for architectural changes
- Add inline documentation for public functions
- Include usage examples where appropriate

## Pull Request Process

1. **Ensure your PR:**
   - Has a clear description of changes
   - References any related issues
   - Includes tests for new functionality
   - Updates documentation as needed
   - Passes all CI checks

2. **PR Title Format:**
   ```
   feat: add new command for X
   fix: resolve issue with Y
   docs: update installation instructions
   ```

3. **Wait for Review:**
   - A maintainer will review your PR
   - Address any requested changes
   - Once approved, your PR will be merged

## Reporting Issues

### Bug Reports

When reporting bugs, include:
- Clear description of the issue
- Steps to reproduce
- Expected vs actual behavior
- System information (OS, Rust version)
- Relevant logs or error messages

### Feature Requests

When requesting features, include:
- Clear description of the feature
- Use case and motivation
- Proposed implementation (if any)
- Examples of similar features elsewhere

## Project Structure

```
engram-cli/
├── crates/
│   └── engram-cli/
│       ├── src/
│       │   ├── main.rs          # CLI entry point
│       │   ├── commands/        # Command implementations
│       │   ├── crypto/          # Cryptography utilities
│       │   ├── manifest/        # Manifest handling
│       │   └── utils/           # Utility functions
│       └── tests/               # Integration tests
├── scripts/                     # Build and install scripts
├── .github/workflows/           # CI/CD workflows
└── Cargo.toml                   # Workspace configuration
```

## Adding a New Command

1. Create a new file in `src/commands/`:
   ```rust
   // src/commands/mycommand.rs
   use anyhow::Result;

   pub fn mycommand(args: &Args) -> Result<()> {
       // Implementation
       Ok(())
   }
   ```

2. Add to `src/commands/mod.rs`:
   ```rust
   pub mod mycommand;
   ```

3. Add to CLI enum in `src/main.rs`:
   ```rust
   #[derive(Subcommand)]
   enum Commands {
       // ...
       MyCommand {
           #[arg(short, long)]
           option: String,
       },
   }
   ```

4. Add match arm in `main()`:
   ```rust
   match &cli.command {
       // ...
       Commands::MyCommand { option } => {
           commands::mycommand::mycommand(&args)?;
       }
   }
   ```

5. Write tests in `tests/cli.rs`

## Building for Release

### Local Build

```bash
# Build release binary
cargo build --release

# Binary at: target/release/engram[.exe]
```

### Cross-Platform Builds

```bash
# Linux/macOS
./scripts/build-release.sh

# Windows
.\scripts\build-release.ps1
```

This creates binaries in the `dist/` directory for all platforms.

## License

By contributing to Engram CLI, you agree that your contributions will be licensed under the MIT License.

## Questions?

If you have questions about contributing, feel free to:
- Open an issue for discussion
- Reach out to maintainers
- Check the [README](README.md) and [CLAUDE.md](CLAUDE.md) for more details

Thank you for contributing!
