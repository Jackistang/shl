use assert_cmd::Command; // Run programs
use predicates::prelude::*;

pub fn exec_command(stdin: &str, stdout: &str) {
    let mut cmd = Command::cargo_bin("shl").unwrap();
    cmd.arg("--test")
        .write_stdin(stdin)
        .assert()
        .success()
        .stdout(predicate::str::diff(stdout.to_owned()));
}