use std::process::Command;

use assert_cmd::prelude::*;
use predicates::prelude::predicate::str::contains;
use predicates::prelude::predicate::str::is_match;

#[test]
fn it_works_with_the_test_input_file() -> Result<(), Box<dyn std::error::Error>> {
    Command::cargo_bin("aoc-2021")?
        .arg("day-10")
        .arg("test-data/aoc-2021-10/input")
        .assert()
        .success()
        .stdout(is_match("^error score 364389\n$").unwrap());
    Ok(())
}

#[test]
fn it_errors_with_file_not_found_if_file_does_not_exist() -> Result<(), Box<dyn std::error::Error>>
{
    Command::cargo_bin("aoc-2021")?
        .arg("day-10")
        .arg("test-data/aoc-2021-10/input-DOES-NOT-EXIST")
        .assert()
        .failure()
        .stderr(contains("No such file or directory"));
    Ok(())
}
