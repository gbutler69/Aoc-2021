use std::{
    fs::File,
    io::{self, BufRead},
    path::PathBuf,
};

use itertools::Itertools;

use aoc_2021::aoc_2021_11::determine_number_of_flashes_after_n_steps;

pub fn execute(path: PathBuf) -> Result<(), Box<dyn std::error::Error>> {
    let lines = io::BufReader::new(File::open(path)?)
        .lines()
        .filter_ok(|line| !line.trim().is_empty())
        .map_ok(|line| {
            line.chars()
                .map(|c| c.to_digit(10).unwrap() as u8)
                .collect::<Vec<_>>()
        })
        .collect::<Result<Vec<_>, _>>()?;
    let number_of_flashes = determine_number_of_flashes_after_n_steps(lines, 100);
    println!("number of flashes {number_of_flashes}");
    Ok(())
}
