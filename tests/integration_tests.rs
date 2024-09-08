use std::process::Command;
use assert_cmd::prelude::*;
use predicates::prelude::*;
use tempfile::tempdir;
use std::fs;

#[test]
fn test_cli_basic() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("your_binary_name")?;
    cmd.assert().success();
    Ok(())
}

#[test]
fn test_cli_with_input() -> Result<(), Box<dyn std::error::Error>> {
    let temp_dir = tempdir()?;
    let input_file = temp_dir.path().join("input.txt");
    fs::write(&input_file, "test content")?;

    let mut cmd = Command::cargo_bin("your_binary_name")?;
    cmd.arg(input_file.to_str().unwrap())
        .assert()
        .success()
        .stdout(predicate::str::contains("test content"));

    Ok(())
}

#[test]
fn test_cli_with_invalid_input() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("your_binary_name")?;
    cmd.arg("nonexistent_file.txt")
        .assert()
        .failure()
        .stderr(predicate::str::contains("Error"));

    Ok(())
}
