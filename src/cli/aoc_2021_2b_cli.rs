use std::{
    fs::File,
    io::{self, BufRead},
    path::PathBuf,
};

use itertools::Itertools;

use aoc_2021::aoc_2021_2::command_from_text;
use aoc_2021::aoc_2021_2b::determine_depth_and_forward_movement_amount;

pub fn execute(path: PathBuf) -> Result<(), Box<dyn std::error::Error>> {
    let (final_depth, final_forward) = determine_depth_and_forward_movement_amount(
        io::BufReader::new(File::open(path)?)
            .lines()
            .map_ok(command_from_text)
            .collect::<Result<Vec<_>, _>>()?,
    );
    println!(
        "depth {} forward {} area {}",
        final_depth,
        final_forward,
        final_depth * final_forward
    );
    Ok(())
}
