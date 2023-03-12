use anyhow::{Result};
use assert_cmd::prelude::*;
use predicates::prelude::*;
use std::process::Command;

#[test]
fn authenticate_no_email_passed() -> Result<()> {
    let mut cmd = Command::cargo_bin("dekube")?;
    cmd.arg("authenticate").arg("-p").arg("password");
    cmd.assert().failure().stderr(predicate::str::contains("required arguments were not provided"));

    Ok(())
}

#[test]
fn authenticate_no_password_passed() -> Result<()> {
    let mut cmd = Command::cargo_bin("dekube")?;
    cmd.arg("authenticate").arg("-e").arg("email");
    cmd.assert().failure().stderr(predicate::str::contains("required arguments were not provided"));

    Ok(())
}

#[test]
fn authenticate_with_all_required_parameters() -> Result<()> {
    let mut cmd = Command::cargo_bin("dekube")?;
    cmd.arg("authenticate").arg("-e").arg("email").arg("-p").arg("password");
    cmd.assert().success();

    Ok(())
}