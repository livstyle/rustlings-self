use assert_cmd::prelude::*;
use std::process::Command;

#[test]
fn run_single_test_success_with_output() {
    Command::cargo_bin("rustlings")
        .unwrap()
        .args(&["--nocapture", "run", "all_exercises"])
        // .current_dir("exercises/")
        .assert()
        // .code(0)
        .stdout(predicates::str::contains("THIS TEST TOO SHALL PASS"));
}
