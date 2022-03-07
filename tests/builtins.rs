use assert_cmd::Command; // Run programs
use predicates::prelude::*;
use std::env;

#[test]
fn cd() {
    // TODO 获取环境变量 Home，通过 cd 命令切换到 Home 目录，并用 pwd 检测。
    let home = env::var("HOME").unwrap();

    let mut cmd = Command::cargo_bin("shl").unwrap();
    cmd.write_stdin(format!("cd {}\n pwd \n exit", home))
        .assert()
        .success()
        .stdout(predicate::str::diff(format!("{}", home)));

}