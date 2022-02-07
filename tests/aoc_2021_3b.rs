use std::process::Command;

use assert_cmd::prelude::*;
use predicates::prelude::predicate::str::is_match;

#[test]
fn it_works_with_the_test_input_file() -> Result<(), Box<dyn std::error::Error>> {
    Command::cargo_bin("aoc-2021")?
        .arg("day-3b")
        .arg("test-data/aoc-2021-3/input")
        .assert()
        .success()
        .stdout(
            is_match("^oxygen rating 1161 co2 rating 3621 life support rating 4203981\n$").unwrap(),
        );
    Ok(())
}
