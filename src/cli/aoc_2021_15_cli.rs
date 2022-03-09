use std::{
    fs::File,
    io::{self, BufRead},
    path::PathBuf,
};

use itertools::Itertools;

use aoc_2021::aoc_2021_15::determine_danger_cost_of_least_cost_path;

pub fn execute(path: PathBuf) -> Result<(), Box<dyn std::error::Error>> {
    let input_lines = io::BufReader::new(File::open(path)?)
        .lines()
        .filter_ok(ignore_empty_or_all_whitespace_lines);
    let mut rows = Vec::new();
    for line in input_lines {
        let line = line?;
        rows.push(split_to_costs(line));
    }
    let least_cost = determine_danger_cost_of_least_cost_path(rows);
    println!("cost of least cost path {least_cost}");
    Ok(())
}

fn split_to_costs(line: String) -> Vec<u8> {
    line.chars()
        .map(|c| c.to_string().parse::<u8>())
        .collect::<Result<Vec<u8>, _>>()
        .unwrap()
}

#[allow(clippy::ptr_arg)]
fn ignore_empty_or_all_whitespace_lines(line: &String) -> bool {
    !line.trim().is_empty()
}
