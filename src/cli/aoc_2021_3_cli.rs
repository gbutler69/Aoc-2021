use std::{
    fs::File,
    io::{self, BufRead},
    path::PathBuf,
};

use itertools::Itertools;

use aoc_2021::aoc_2021_3::{determine_gamma_epsilon_and_power_factors, Reading};

pub fn execute(path: PathBuf) -> Result<(), Box<dyn std::error::Error>> {
    let (gamma, epsilon, power) = determine_gamma_epsilon_and_power_factors(
        io::BufReader::new(File::open(path)?)
            .lines()
            .map_ok(|line| Reading::from(line.as_str()))
            .collect::<Result<Vec<_>, _>>()?,
    );
    println!("gamma {gamma} epsilon {epsilon} power {power}");
    Ok(())
}
