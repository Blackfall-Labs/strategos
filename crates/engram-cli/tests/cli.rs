use assert_cmd::cargo::*;
use predicates::prelude::*;

/// Verifies that the search command returns an appropriate error when given a non-existent file path
#[test]
fn file_doesnt_exist() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = cargo_bin_cmd!("engram");

    cmd.arg("search").arg("foobar").arg("test/file/doesnt/exist");
    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("could not read file"));

    Ok(())
}
