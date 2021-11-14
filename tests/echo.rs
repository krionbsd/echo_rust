use assert_cmd::Command;
use predicates::prelude::*;
use std::fs;

type TestResult = Result<(), Box<dyn std::error::Error>>;

fn run(args: &[&str], expected_file: &str) -> TestResult {
    let expected = fs::read_to_string(expected_file)?;
    Command::cargo_bin("echo_rust")?
        .args(args)
        .assert()
        .success()
        .stdout(expected);
    Ok(())
}

#[test]
fn dies_no_args() -> TestResult {
    Command::cargo_bin("echo_rust")?
        .assert()
        .failure()
        .stderr(predicate::str::contains("USAGE"));
    Ok(())
}

#[test]
fn hello1() -> TestResult {
    run(&["Hello there"], "tests/files/hello.txt")
}

#[test]
fn hello2() -> TestResult {
    run(&["Hello", "there"], "tests/files/hello.txt")
}

#[test]
fn hello3() -> TestResult {
    run(&["Hello there"], "tests/files/hello.txt")
}

#[test]
fn hello4() -> TestResult {
    run(&["Hello", "there"], "tests/files/hello.txt")
}
