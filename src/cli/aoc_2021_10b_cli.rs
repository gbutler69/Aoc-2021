use std::{
    fs::File,
    io::{self, BufRead},
    path::PathBuf,
};

use itertools::Itertools;

use aoc_2021::aoc_2021_10b::determine_incomplete_lines_score;

pub fn execute(path: PathBuf) -> Result<(), Box<dyn std::error::Error>> {
    let lines = io::BufReader::new(File::open(path)?)
        .lines()
        .filter_ok(|line| !line.trim().is_empty())
        .collect::<Result<Vec<_>, _>>()?;
    let error_score = determine_incomplete_lines_score(lines.iter().map(String::as_str));
    println!("error score {error_score}");
    Ok(())
}
