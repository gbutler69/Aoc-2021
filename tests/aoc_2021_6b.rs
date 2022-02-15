use std::process::Command;

use assert_cmd::prelude::*;
use predicates::prelude::predicate::str::is_match;

#[test]
fn it_works_with_the_test_input_file_and_256_lifecycle_days(
) -> Result<(), Box<dyn std::error::Error>> {
    Command::cargo_bin("aoc-2021")?
        .arg("day-6b")
        .arg("test-data/aoc-2021-6/input")
        .arg("256")
        .assert()
        .success()
        .stdout(is_match("^number of fish 1708791884591\n$").unwrap());
    Ok(())
}
