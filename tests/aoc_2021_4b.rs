use std::process::Command;

use assert_cmd::prelude::*;
use predicates::prelude::predicate::str::is_match;

#[test]
fn it_works_with_the_test_input_file() -> Result<(), Box<dyn std::error::Error>> {
    Command::cargo_bin("aoc-2021")?
        .arg("day-4b")
        .arg("test-data/aoc-2021-4/input")
        .assert()
        .success()
        .stdout(
            is_match(
                "^last winning board 29 uncalled total 526 last called number 34 score 17884\n$",
            )
            .unwrap(),
        );
    Ok(())
}
