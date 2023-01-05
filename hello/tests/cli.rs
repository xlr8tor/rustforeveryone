use assert_cmd::Command;
use predicates::prelude::*;

type TestError = Result<(), Box<dyn std::error::Error>>;
#[test]
fn runs() {
    let mut cmd = Command::cargo_bin("hello").unwrap();
    cmd.assert().success();
}

#[test]
fn runs_default_output() -> TestError {
    let mut cmd = Command::cargo_bin("hello")?;
    cmd.assert().success().stdout("Hello, world!\n");
    Ok(())
}

fn runs_with_option(args: &[&str]) -> TestError {
    let mut cmd = Command::cargo_bin("hello")?;
    cmd.args(args)
        .assert()
        .success()
        .stdout("Hello, Jeremiah!\n");

    Ok(())
}

#[test]
fn runs_with_input() -> TestError {
    runs_with_option(&["-n", "Jeremiah"])
}
