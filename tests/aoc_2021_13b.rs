use std::process::Command;

use assert_cmd::prelude::*;
use predicates::prelude::predicate::str::is_match;

#[test]
fn it_works_with_the_test_input_file() -> Result<(), Box<dyn std::error::Error>> {
    let expected = "|###  #  #  ##  #    ###   ##  ###   ## \n\
                         |#  # #  # #  # #    #  # #  # #  # #  #\n\
                         |#  # #### #  # #    #  # #    #  # #  #\n\
                         |###  #  # #### #    ###  #    ###  ####\n\
                         |# #  #  # #  # #    # #  #  # # #  #  #\n\
                         |#  # #  # #  # #### #  #  ##  #  # #  #\n";
    Command::cargo_bin("aoc-2021")?
        .arg("day-13b")
        .arg("test-data/aoc-2021-13/input")
        .assert()
        .success()
        .stdout(is_match(("^output\n".to_owned() + expected + "\n$").as_str()).unwrap());
    Ok(())
}
