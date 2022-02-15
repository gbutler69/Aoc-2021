use std::{
    fs::File,
    io::{self, BufRead},
    path::PathBuf,
};

use itertools::Itertools;

use aoc_2021::aoc_2021_6b::determine_number_of_fish_after_x_days;

pub fn execute(path: PathBuf, number_of_cycles: u32) -> Result<(), Box<dyn std::error::Error>> {
    let number_of_fish = determine_number_of_fish_after_x_days(
        io::BufReader::new(File::open(path)?)
            .lines()
            .filter_ok(|line| !line.trim().is_empty())
            .take(1)
            .map_ok(|l| l.split(',').map(|v| v.to_owned()).collect::<Vec<_>>())
            .flatten_ok()
            .map_ok(|v| v.parse::<u32>().unwrap() as u8)
            .collect::<Result<Vec<_>, _>>()?,
        number_of_cycles,
    );
    println!("number of fish {number_of_fish}");
    Ok(())
}
