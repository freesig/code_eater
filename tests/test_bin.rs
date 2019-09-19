use std::process::Command;  // Run programs
use assert_cmd::prelude::*; // Add methods on commands

#[test]
fn generate_bin_and_code() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("single_source")?;
    cmd.arg("code")
        .arg("test.md")
        .arg("result.rs")
        .arg("rust");
    cmd.assert()
        .success();
    Ok(())
}

#[test]
fn generate_bin_and_md() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("single_source")?;
    cmd.arg("md")
        .arg("test.md")
        .arg("result.md");
    cmd.assert()
        .success();
    Ok(())
}
