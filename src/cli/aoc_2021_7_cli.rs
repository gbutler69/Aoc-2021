use std::{
    fs::File,
    io::{self, BufRead},
    path::PathBuf,
};

use itertools::Itertools;

use aoc_2021::aoc_2021_7::determine_position_and_minimum_fuel_required;

pub fn execute(path: PathBuf) -> Result<(), Box<dyn std::error::Error>> {
    let (position, fuel_cost) = determine_position_and_minimum_fuel_required(
        io::BufReader::new(File::open(path)?)
            .lines()
            .filter_ok(|line| !line.trim().is_empty())
            .take(1)
            .map_ok(|l| l.split(',').map(|v| v.to_owned()).collect::<Vec<_>>())
            .flatten_ok()
            .map_ok(|v| v.parse::<u32>().unwrap())
            .collect::<Result<Vec<_>, _>>()?,
    );
    println!("position {position} fuel cost {fuel_cost}");
    Ok(())
}
