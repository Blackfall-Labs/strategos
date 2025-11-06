use assert_cmd::cargo::*;
use predicates::prelude::*;
use std::fs;
use tempfile::TempDir;

/// Verifies that the `search` command returns an appropriate error when given a non-existent file path
#[test]
fn file_doesnt_exist() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = cargo_bin_cmd!("engram");

    cmd.arg("search").arg("foobar").arg("test/file/doesnt/exist");
    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("could not read file"));

    Ok(())
}

/// Tests packing a directory into an archive
#[test]
fn pack_directory() -> Result<(), Box<dyn std::error::Error>> {
    let temp_dir = TempDir::new()?;
    let source_dir = temp_dir.path().join("test_data");
    fs::create_dir(&source_dir)?;

    // Create test files
    fs::write(source_dir.join("file1.txt"), "Hello, World!")?;
    fs::write(source_dir.join("file2.txt"), "Test content")?;

    let output_archive = temp_dir.path().join("test.eng");

    let mut cmd = cargo_bin_cmd!("engram");
    cmd.arg("pack")
        .arg(&source_dir)
        .arg("-o")
        .arg(&output_archive);

    cmd.assert()
        .success()
        .stdout(predicate::str::contains("Archive created successfully"));

    // Verify archive was created
    assert!(output_archive.exists());

    Ok(())
}

/// Tests packing a single file into an archive
#[test]
fn pack_single_file() -> Result<(), Box<dyn std::error::Error>> {
    let temp_dir = TempDir::new()?;
    let source_file = temp_dir.path().join("test.txt");
    fs::write(&source_file, "Single file content")?;

    let output_archive = temp_dir.path().join("single.eng");

    let mut cmd = cargo_bin_cmd!("engram");
    cmd.arg("pack")
        .arg(&source_file)
        .arg("-o")
        .arg(&output_archive);

    cmd.assert()
        .success()
        .stdout(predicate::str::contains("Added: test.txt"))
        .stdout(predicate::str::contains("Archive created successfully"));

    assert!(output_archive.exists());

    Ok(())
}

/// Tests listing files in an archive
#[test]
fn list_archive() -> Result<(), Box<dyn std::error::Error>> {
    let temp_dir = TempDir::new()?;
    let source_dir = temp_dir.path().join("test_data");
    fs::create_dir(&source_dir)?;

    // Create test files
    fs::write(source_dir.join("alpha.txt"), "Alpha")?;
    fs::write(source_dir.join("beta.txt"), "Beta")?;

    let output_archive = temp_dir.path().join("test.eng");

    // Pack the directory
    let mut pack_cmd = cargo_bin_cmd!("engram");
    pack_cmd.arg("pack")
        .arg(&source_dir)
        .arg("-o")
        .arg(&output_archive);
    pack_cmd.assert().success();

    // List the archive
    let mut list_cmd = cargo_bin_cmd!("engram");
    list_cmd.arg("list").arg(&output_archive);

    list_cmd.assert()
        .success()
        .stdout(predicate::str::contains("alpha.txt"))
        .stdout(predicate::str::contains("beta.txt"));

    Ok(())
}

/// Tests showing basic info about an archive
#[test]
fn info_archive_basic() -> Result<(), Box<dyn std::error::Error>> {
    let temp_dir = TempDir::new()?;
    let source_dir = temp_dir.path().join("test_data");
    fs::create_dir(&source_dir)?;

    // Create test file
    fs::write(source_dir.join("test.txt"), "Test content for info")?;

    let output_archive = temp_dir.path().join("test.eng");

    // Pack the directory
    let mut pack_cmd = cargo_bin_cmd!("engram");
    pack_cmd.arg("pack")
        .arg(&source_dir)
        .arg("-o")
        .arg(&output_archive);
    pack_cmd.assert().success();

    // Show info
    let mut info_cmd = cargo_bin_cmd!("engram");
    info_cmd.arg("info").arg(&output_archive);

    info_cmd.assert()
        .success()
        .stdout(predicate::str::contains("Archive:"))
        .stdout(predicate::str::contains("Format Version:"))
        .stdout(predicate::str::contains("Total Files: 1"));

    Ok(())
}

/// Tests showing detailed inspection of an archive
#[test]
fn info_archive_inspect() -> Result<(), Box<dyn std::error::Error>> {
    let temp_dir = TempDir::new()?;
    let source_dir = temp_dir.path().join("test_data");
    fs::create_dir(&source_dir)?;

    // Create test file
    fs::write(source_dir.join("inspect.txt"), "Detailed inspection test")?;

    let output_archive = temp_dir.path().join("test.eng");

    // Pack the directory
    let mut pack_cmd = cargo_bin_cmd!("engram");
    pack_cmd.arg("pack")
        .arg(&source_dir)
        .arg("-o")
        .arg(&output_archive);
    pack_cmd.assert().success();

    // Show detailed info
    let mut info_cmd = cargo_bin_cmd!("engram");
    info_cmd.arg("info").arg(&output_archive).arg("--inspect");

    info_cmd.assert()
        .success()
        .stdout(predicate::str::contains("Detailed File Information:"))
        .stdout(predicate::str::contains("Compression:"))
        .stdout(predicate::str::contains("CRC32:"))
        .stdout(predicate::str::contains("Archive Structure:"))
        .stdout(predicate::str::contains("Central Directory Offset:"));

    Ok(())
}

/// Tests that `list` command fails gracefully with non-existent archive
#[test]
fn list_nonexistent_archive() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = cargo_bin_cmd!("engram");
    cmd.arg("list").arg("nonexistent.eng");

    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("failed to open archive"));

    Ok(())
}

/// Tests that `info` command fails gracefully with non-existent archive
#[test]
fn info_nonexistent_archive() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = cargo_bin_cmd!("engram");
    cmd.arg("info").arg("nonexistent.eng");

    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("failed to open archive"));

    Ok(())
}

/// Tests `search` command with actual file content
#[test]
fn search_finds_pattern() -> Result<(), Box<dyn std::error::Error>> {
    let temp_dir = TempDir::new()?;
    let test_file = temp_dir.path().join("search_test.txt");

    fs::write(&test_file, "First line\nSecond line with PATTERN\nThird line")?;

    let mut cmd = cargo_bin_cmd!("engram");
    cmd.arg("search").arg("PATTERN").arg(&test_file);

    cmd.assert()
        .success()
        .stdout(predicate::str::contains("Second line with PATTERN"));

    Ok(())
}

/// Tests `search` command when pattern is not found
#[test]
fn search_pattern_not_found() -> Result<(), Box<dyn std::error::Error>> {
    let temp_dir = TempDir::new()?;
    let test_file = temp_dir.path().join("search_test.txt");

    fs::write(&test_file, "First line\nSecond line\nThird line")?;

    let mut cmd = cargo_bin_cmd!("engram");
    cmd.arg("search").arg("NOTFOUND").arg(&test_file);

    cmd.assert()
        .success()
        .stdout(predicate::str::is_empty());

    Ok(())
}

/// Tests packing a directory with nested subdirectories
#[test]
fn pack_nested_directories() -> Result<(), Box<dyn std::error::Error>> {
    let temp_dir = TempDir::new()?;
    let source_dir = temp_dir.path().join("test_data");
    fs::create_dir(&source_dir)?;

    // Create nested structure
    let subdir = source_dir.join("subdir");
    fs::create_dir(&subdir)?;
    let nested_subdir = subdir.join("nested");
    fs::create_dir(&nested_subdir)?;

    fs::write(source_dir.join("root.txt"), "Root file")?;
    fs::write(subdir.join("sub.txt"), "Subdir file")?;
    fs::write(nested_subdir.join("nested.txt"), "Nested file")?;

    let output_archive = temp_dir.path().join("nested.eng");

    let mut cmd = cargo_bin_cmd!("engram");
    cmd.arg("pack")
        .arg(&source_dir)
        .arg("-o")
        .arg(&output_archive);

    cmd.assert()
        .success()
        .stdout(predicate::str::contains("root.txt"))
        .stdout(predicate::str::contains("subdir/sub.txt"))
        .stdout(predicate::str::contains("subdir/nested/nested.txt"))
        .stdout(predicate::str::contains("Packed 3 files"));

    // Verify all files are listed
    let mut list_cmd = cargo_bin_cmd!("engram");
    list_cmd.arg("list").arg(&output_archive);

    list_cmd.assert()
        .success()
        .stdout(predicate::str::contains("root.txt"))
        .stdout(predicate::str::contains("subdir/sub.txt"))
        .stdout(predicate::str::contains("subdir/nested/nested.txt"));

    Ok(())
}
