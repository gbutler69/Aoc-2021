use std::process::Command;

use assert_cmd::prelude::*;

use predicates::prelude::predicate::str::is_match;

#[test]
fn it_works_with_the_test_input_file() -> Result<(), Box<dyn std::error::Error>> {
    Command::cargo_bin("aoc-2021")?
        .arg("day-16b")
        .arg("test-data/aoc-2021-16/input")
        .assert()
        .success()
        .stdout(is_match("^evaluated value of packet 9485076995911\n$").unwrap());
    Ok(())
}
