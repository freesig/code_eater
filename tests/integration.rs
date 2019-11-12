use std::process::Command;
use assert_cmd::prelude::*;
use tempdir::TempDir;
#[test]
fn ss_integration() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("single_source")?;
    let tmp_dir = TempDir::new("tmp")?;
    let manifest_dir = env!("CARGO_MANIFEST_DIR");
    let sample_test = format!("{}/{}", manifest_dir, "docs/sample_test.md");
    Command::new("cargo")
        .args(&["new", "doubler", "--lib"])
        .current_dir(tmp_dir.path())
        .output()?;
    cmd.args(&["code", &sample_test, "src/lib.rs", "rust", "code"])
        .current_dir(tmp_dir.path().join("doubler"));
    cmd.assert()
        .success();
    let mut cmd = Command::cargo_bin("single_source")?;
    cmd.args(&["code", &sample_test, "src/lib.rs", "rust", "test"])
        .current_dir(tmp_dir.path().join("doubler"));
    cmd.assert()
        .success();
    Command::new("cargo")
        .args(&["test"])
        .current_dir(tmp_dir.path().join("doubler"))
        .output()?;

    Ok(())
}
