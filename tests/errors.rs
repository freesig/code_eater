use std::process::Command;
use assert_cmd::prelude::*;
use predicates::prelude::*;

#[test]
fn missing_source() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("single_source")?;
    cmd.args(&["code", "", "", "rust", "code"]);
    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("Cannot find the markdown source at: "));
    let mut cmd = Command::cargo_bin("single_source")?;
    cmd.args(&["code", "isdbcibbcidbd/newkjnkn.md", "", "rust", "code"]);
    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("Cannot find the markdown source at: isdbcibbcidbd/newkjnkn.md"));
    Ok(())
}
