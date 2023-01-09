use assert_cmd::Command;
use predicates::prelude::*;
use std::error::Error;

const PRG: &str = "catr";
type TestResult = Result<(), Box<dyn Error>>;

#[test]
fn runs() -> TestResult {
    let mut cmd = Command::cargo_bin(PRG)?;
    cmd.assert().success();
    Ok(())
}

#[test]
fn usage() -> TestResult {
    for flag in &["-h", "--help"] {
        Command::cargo_bin(PRG)?
            .arg(flag)
            .assert()
            .stdout(predicate::str::contains("USAGE"));
    }
    Ok(())
}
