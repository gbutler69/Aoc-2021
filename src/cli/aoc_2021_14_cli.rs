use std::{
    fs::File,
    io::{self, BufRead},
    path::PathBuf,
};

use itertools::Itertools;

use aoc_2021::aoc_2021_14::{
    determine_polymer_after_n_steps_given_rules_and_starting_polymer, greatest_element_qty,
    least_element_qty,
};

pub fn execute(path: PathBuf) -> Result<(), Box<dyn std::error::Error>> {
    let mut input_lines = io::BufReader::new(File::open(path)?)
        .lines()
        .filter_ok(ignore_empty_or_all_whitespace_lines);
    let starting_polymer = split_to_polymer_vec(input_lines.next().unwrap()?);
    let mut rules = Vec::new();
    for line in input_lines {
        let line = line?;
        rules.push(split_to_rule(line));
    }
    let final_polymer = determine_polymer_after_n_steps_given_rules_and_starting_polymer(
        rules,
        starting_polymer,
        10,
    );
    let diff_score = greatest_element_qty(&final_polymer) - least_element_qty(&final_polymer);
    println!("final difference between most and least elements {diff_score}");
    Ok(())
}

fn split_to_rule(line: String) -> ((char, char), char) {
    let rule = line.split_once(" -> ").unwrap();
    let rule = (rule.0.split_at(1), rule.1);
    let rule = (
        (
            rule.0 .0.chars().next().unwrap(),
            rule.0 .1.chars().next().unwrap(),
        ),
        rule.1.chars().next().unwrap(),
    );
    rule
}

fn split_to_polymer_vec(polymer: String) -> Vec<char> {
    polymer.chars().collect()
}

#[allow(clippy::ptr_arg)]
fn ignore_empty_or_all_whitespace_lines(line: &String) -> bool {
    !line.trim().is_empty()
}
