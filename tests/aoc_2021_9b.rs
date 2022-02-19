use std::process::Command;

use assert_cmd::prelude::*;
use predicates::prelude::predicate::str::is_match;

#[test]
fn it_works_with_the_test_input_file() -> Result<(), Box<dyn std::error::Error>> {
    Command::cargo_bin("aoc-2021")?
        .arg("day-9b")
        .arg("test-data/aoc-2021-9/input")
        .assert()
        .success()
        .stdout(is_match("^basin area 1317792\n$").unwrap());
    Ok(())
}
