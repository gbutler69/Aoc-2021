use std::process::Command;

use assert_cmd::prelude::*;
use predicates::prelude::predicate::str::contains;
use predicates::prelude::predicate::str::is_match;

#[test]
fn it_works_with_the_test_input_file_and_80_lifecycle_days(
) -> Result<(), Box<dyn std::error::Error>> {
    Command::cargo_bin("aoc-2021")?
        .arg("day-6")
        .arg("test-data/aoc-2021-6/input")
        .arg("80")
        .assert()
        .success()
        .stdout(is_match("^number of fish 380243\n$").unwrap());
    Ok(())
}

#[test]
fn it_errors_with_file_not_found_if_file_does_not_exist() -> Result<(), Box<dyn std::error::Error>>
{
    Command::cargo_bin("aoc-2021")?
        .arg("day-6")
        .arg("test-data/aoc-2021-6/input-DOES-NOT-EXIST")
        .arg("80")
        .assert()
        .failure()
        .stderr(contains("No such file or directory"));
    Ok(())
}
